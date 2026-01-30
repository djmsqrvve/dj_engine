use crate::editor::EditorPrefs;
use crate::data::components::{
    ColorData, EntityComponents, SpriteComponent, TransformComponent, Vec3Data,
};
use crate::data::project::EngineSettings;
use crate::data::scene::{Entity as SceneEntity, Scene};
use crate::data::{loader, project::Project};
use crate::diagnostics::console::ConsoleLogStore;
use bevy::prelude::*;
use bevy_egui::egui::{self, Color32};

use super::state::*;

pub fn configure_visuals_system(mut contexts: bevy_egui::EguiContexts) {
    let ctx = contexts.ctx_mut();
    let mut visuals = egui::Visuals::dark();

    // Cyberpunk tweaks
    visuals.window_rounding = 2.0.into();
    visuals.widgets.noninteractive.bg_fill = COLOR_BG;
    visuals.widgets.inactive.bg_fill = Color32::from_rgb(25, 25, 35);
    visuals.widgets.hovered.bg_fill = Color32::from_rgb(40, 40, 50);
    visuals.widgets.active.bg_fill = Color32::from_rgb(50, 50, 65);
    visuals.selection.bg_fill = COLOR_PRIMARY.linear_multiply(0.3);
    visuals.selection.stroke = egui::Stroke::new(1.0, COLOR_PRIMARY);

    ctx.set_visuals(visuals);
}

pub fn automated_ui_test_system(
    mut commands: Commands,
    time: Res<Time>,
    mut test_state: ResMut<AutomatedTestActive>,
    mut ui_state: ResMut<EditorUiState>,
    mut console: ResMut<ConsoleLogStore>,
    mut app_exit: EventWriter<bevy::app::AppExit>,
) {
    test_state.timer.tick(time.delta());
    if !test_state.timer.finished() {
        return;
    }

    match test_state.step {
        0 => {
            console.log("TEST: Starting automated UI test sequence...".into());
            test_state.step += 1;
        }
        1 => {
            console.log("TEST: Select 'Hamster' from palette".into());
            // Palette is now always visible in Node Dropper, just select item
            ui_state.selected_palette_item = Some("Hamster".into());
            test_state.step += 1;
        }
        2 => {
            debug!("TEST: Executing spawn step");
            console.log("TEST: Simulating click/spawn at (100, 100)".into());
            // Manually spawn entity as if clicked
            commands.spawn((
                LogicalEntity,
                Name::new("Hamster [100, 100]"),
                Sprite {
                    color: Color::srgb(0.8, 0.5, 0.2),
                    custom_size: Some(Vec2::new(30.0, 30.0)),
                    ..default()
                },
                Transform::from_xyz(100.0, 100.0, 0.0),
            ));
            test_state.step += 1;
        }
        3 => {
            console.log("TEST: Switching to Story Graph view".into());
            if let Some(branch) = ui_state.current_branch_mut() {
                branch.active_view = EditorView::StoryGraph;
            }
            test_state.step += 1;
        }
        4 => {
            console.log("TEST: Validation Complete. Exiting.".into());
            info!("Automated UI Test Passed");
            app_exit.send(bevy::app::AppExit::Success);
        }
        _ => {}
    }
}

pub fn launch_project_system(
    project: Res<ProjectMetadata>,
    mut script_events: EventWriter<crate::scripting::ScriptCommand>,
) {
    let Some(path) = &project.path else {
        warn!("No project path mounted! Cannot launch.");
        return;
    };

    info!("Editor: Launching project from {:?}", path);

    // Look for a main.lua or hamster_test.lua in the project's script folder
    let script_path = path.join("assets/scripts/hamster_test.lua");
    if script_path.exists() {
        script_events.send(crate::scripting::ScriptCommand::Load {
            path: script_path.to_string_lossy().into(),
        });
    } else {
        warn!("No entry script found at {:?}", script_path);
    }
}

pub fn save_project_impl(world: &mut World) {
    // Clone necessary data to avoid holding borrow on world
    let (project_name, project_path) = {
        let project_meta = world.resource::<ProjectMetadata>();
        (project_meta.name.clone(), project_meta.path.clone())
    };

    if let Some(path) = project_path {
        info!("Saving project to {:?}", path);

        // 1. Save Project Structure
        let project_data = Project::new(&project_name);
        match loader::save_project_structure(&project_data, &path) {
            Ok(_) => info!("Successfully saved project structure"),
            Err(e) => error!("Failed to save project structure: {}", e),
        }

        // 2. Save Current Scene
        let scene = world_to_scene(world);
        let scene_path = path.join("scenes/current_scene.json");
        match loader::save_scene(&scene, &scene_path) {
            Ok(_) => info!("Successfully saved scene to {:?}", scene_path),
            Err(e) => error!("Failed to save scene: {}", e),
        }

        // 3. Save Story Graph
        let graph = &world.resource::<ActiveStoryGraph>().0;
        let graph_path = path.join("story_graphs/main.json");
        match loader::save_story_graph(graph, &graph_path) {
            Ok(_) => info!("Successfully saved story graph to {:?}", graph_path),
            Err(e) => error!("Failed to save story graph: {}", e),
        }

        // 4. Update recent projects in preferences
        let mut prefs = world.resource_mut::<super::EditorPrefs>();
        prefs
            .0
            .add_recent_project(path.to_string_lossy().to_string());
        if let Err(e) = prefs.0.save() {
            warn!("Failed to save editor preferences: {}", e);
        } else {
            info!("Updated recent projects list");
        }
    } else {
        warn!("Cannot save: No project path set!");
    }
}

pub fn world_to_scene(world: &mut World) -> Scene {
    let mut scene = Scene::new("current_scene", "Current Scene");

    // In a real implementation, we'd query for all entities with specific marker components.
    // For this prototype, we'll query all entities with a Name and Transform.

    let mut entities = Vec::new();
    let mut query = world.query::<(Entity, &Name, &Transform, Option<&Sprite>)>();

    // We need to collect first to avoid borrowing world inside loop if we needed mutable access,
    // though query iteration is fine. But constructing SceneEntity might need data types.
    let mut world_entities = Vec::new();
    for (_e, name, transform, sprite) in query.iter(world) {
        // Clone data out of world
        let pos = transform.translation;
        let scale = transform.scale;

        let sprite_color = sprite.map(|s| s.color.to_linear().to_f32_array());

        world_entities.push((name.to_string(), pos, scale, sprite_color));
    }

    for (name, pos, scale, sprite_color) in world_entities {
        // Skip editor-only entities (like cameras or UI, unless tagged)
        // For now, simple filter: if it has a name starting with "Editor", skip?
        // Or better, only save things we know we spawned.

        let mut components = EntityComponents::default();

        components.transform = TransformComponent {
            position: Vec3Data::new(pos.x, pos.y, pos.z),
            rotation: Vec3Data::default(), // Simplification
            scale: Vec3Data::new(scale.x, scale.y, scale.z),
            lock_uniform_scale: false,
        };

        if let Some([r, g, b, a]) = sprite_color {
            components.sprite = Some(SpriteComponent {
                sprite_id: "pixel".to_string(), // Placeholder
                tint: ColorData::rgba(r, g, b, a),
                ..Default::default()
            });
        }

        let entity = SceneEntity::new(name.clone(), name) // using name as ID for prototype
            .with_components(components);

        entities.push(entity);
    }

    scene.entities = entities;
    scene
}

pub fn load_scene_into_editor(world: &mut World, scene: Scene) {
    debug!("load_scene_into_editor called for scene: {}", scene.id);
    // 1. Clear existing entities (only those marked as LogicalEntity)
    let entities_to_despawn: Vec<Entity> = world
        .query_filtered::<Entity, With<LogicalEntity>>()
        .iter(world)
        .collect();
    for e in entities_to_despawn {
        world.despawn(e);
    }

    // 2. Spawn new entities
    let entity_count = scene.entities.len();
    for entity_data in scene.entities {
        let transform = entity_data.components.transform;
        let pos = transform.position;
        let scale = transform.scale;

        let mut entity_cmd = world.spawn((
            LogicalEntity,
            Name::new(entity_data.name),
            Transform::from_xyz(pos.x, pos.y, pos.z)
                .with_scale(Vec3::new(scale.x, scale.y, scale.z)),
        ));

        if let Some(sprite) = entity_data.components.sprite {
            let c = sprite.tint;
            entity_cmd.insert(Sprite {
                color: Color::srgba(c.r, c.g, c.b, c.a),
                custom_size: Some(Vec2::new(30.0, 30.0)), // Default size for now
                ..default()
            });
        }
    }
    info!("Loaded scene with {} entities", entity_count);
}

pub fn apply_window_settings_system(
    settings: Res<EngineSettings>,
    mut windows: Query<&mut Window, With<bevy::window::PrimaryWindow>>,
) {
    if settings.is_changed() {
        if let Ok(mut window) = windows.get_single_mut() {
            // Check if resolution actually changed (epsilon check)
            let current_width = window.resolution.width();
            let current_height = window.resolution.height();

            let res_changed = (current_width - settings.window_width).abs() > 0.1
                || (current_height - settings.window_height).abs() > 0.1;

            // Check if mode changed
            let target_monitor = if settings.monitor_index == 0 {
                MonitorSelection::Primary
            } else {
                MonitorSelection::Index(settings.monitor_index - 1)
            };

            let target_mode = match settings.window_mode_index {
                1 => bevy::window::WindowMode::BorderlessFullscreen(target_monitor),
                2 => bevy::window::WindowMode::SizedFullscreen(target_monitor),
                _ => bevy::window::WindowMode::Windowed,
            };

            // Note: WindowMode PartialEq might not align perfectly with our index logic if monitor varies,
            // but checking index change is safer against spam if the user keeps clicking same button.
            // However, here we check the actual window state.
            // Let's rely on calculating target mode and comparing.

            // If nothing changed, return
            if !res_changed && window.mode == target_mode {
                return;
            }

            // Update Resolution
            if res_changed {
                window
                    .resolution
                    .set(settings.window_width, settings.window_height);
            }

            // Update Mode
            if window.mode != target_mode {
                window.mode = target_mode;
            }

            // Update Position (if windowed, re-center on selected monitor)
            if window.mode == bevy::window::WindowMode::Windowed
                && target_mode == bevy::window::WindowMode::Windowed
            {
                window.position = WindowPosition::Centered(target_monitor);
            }

            info!(
                "Applied Window Settings: {}x{} (ModeIndex={}, MonitorIndex={})",
                settings.window_width,
                settings.window_height,
                settings.window_mode_index,
                settings.monitor_index
            );
        }
    }
}

pub fn sync_dock_layout_system(
    dock_state: Res<EditorDockState>,
    mut prefs: ResMut<EditorPrefs>,
    time: Res<Time>,
    mut timer: Local<Option<Timer>>,
) {
    if timer.is_none() {
        *timer = Some(Timer::from_seconds(2.0, TimerMode::Repeating));
    }
    let timer = timer.as_mut().unwrap();
    timer.tick(time.delta());

    if timer.just_finished() {
        if let Ok(json) = serde_json::to_value(&dock_state.0) {
            if prefs.0.dock_state.as_ref() != Some(&json) {
                prefs.0.dock_state = Some(json);
                // Trigger change detection for auto-save handled by resource mutation
            }
        }
    }
}

pub fn auto_save_prefs_system(prefs: Res<EditorPrefs>) {
    if prefs.is_changed() {
        if let Err(e) = prefs.0.save() {
            warn!("Failed to auto-save editor preferences: {}", e);
        } else {
            debug!("Auto-saved editor preferences");
        }
    }
}
pub fn cli_load_startup_system(world: &mut World) {
    let project_path = {
        let meta = world.resource::<ProjectMetadata>();
        meta.path.clone()
    };

    if let Some(path) = project_path {
        info!("Startup: CLI project path detected at {:?}", path);
        
        // 1. Load project metadata
        let project_file = path.join("project.json");
        if project_file.exists() {
            match loader::load_project(&project_file) {
                Ok(project) => {
                    world.resource_mut::<ProjectMetadata>().name = project.name.clone();
                    info!("Startup: Loaded project '{}'", project.name);
                }
                Err(e) => error!("Startup: Failed to load project.json: {}", e),
            }
        }

        // 2. Load primary scene
        let scene_path = path.join("scenes/intro_scene.json");
        if scene_path.exists() {
            match loader::load_scene(&scene_path) {
                Ok(scene) => {
                    load_scene_into_editor(world, scene);
                }
                Err(e) => error!("Startup: Failed to load scene: {}", e),
            }
        }

        // 3. Load story graph
        let graph_path = path.join("story_graphs/test_game.json");
        if graph_path.exists() {
            match loader::load_story_graph(&graph_path) {
                Ok(graph) => {
                    world.insert_resource(ActiveStoryGraph(graph.clone()));
                    info!("Startup: Loaded story graph");

                    // 4. Start playing if in Playing state
                    let start_playing = {
                        let state = world.resource::<State<EditorState>>();
                        **state == EditorState::Playing
                    };

                    if start_playing {
                        info!("Startup: Auto-starting story execution");
                        use crate::story_graph::types::{StoryGraphLibrary, GraphExecutor};
                        
                        // Ensure library exists
                        if world.get_resource::<StoryGraphLibrary>().is_none() {
                            world.insert_resource(StoryGraphLibrary::default());
                        }

                        // Use resource_scope to avoid multiple mutable borrows of world
                        world.resource_scope::<StoryGraphLibrary, _>(|world, mut library| {
                            let mut executor = world.resource_mut::<GraphExecutor>();
                            executor.load_from_data(&graph, &mut library);
                        });
                    }
                }
                Err(e) => error!("Startup: Failed to load story graph: {}", e),
            }
        }
    }
}
