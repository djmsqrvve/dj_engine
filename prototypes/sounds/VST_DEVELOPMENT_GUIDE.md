# Developing the Cyber Forest VST with Rust & nih-plug

**Goal:** Create a standalone `.vst3` plugin that generates "Cyber Forest" MIDI patterns in real-time, compatible with Ableton Live (and other DAWs).

**Chosen Stack:** Rust + [`nih-plug`](https://github.com/robbert-vdh/nih-plug).
**Reasoning:** Rust offers memory safety and high performance without the extreme complexity of C++/JUCE. `nih-plug` is a modern, ergonomic framework that simplifies VST3/CLAP development.

---

## 1. Prerequisites

Before starting, ensure you have the Rust toolchain installed:

```bash
# Install Rust (standard installer)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install the bundling tool (for creating the .vst3 folder structure)
cargo install cargo-nih-plug
```

## 2. Project Setup

`nih-plug` provides a template to get started quickly.

```bash
# 1. Create a new Cargo project
cargo new cyber_forest_vst
cd cyber_forest_vst

# 2. Add dependencies to Cargo.toml
[dependencies]
nih_plug = { git = "https://github.com/robbert-vdh/nih-plug.git" }
# We'll need `rand` for the algorithmic generation
rand = "0.8" 
```

## 3. Architecture: Porting Python to Rust

The logic from `generators/cyber_forest.py` needs to change from "Offline File Generation" to "Real-time Block Processing".

### The Mental Shift
*   **Python Script:** "Generate 1 minute of music -> Save to File."
*   **VST Plugin:** "The host (Ableton) asks for audio/MIDI for the next 5 milliseconds. Fill this buffer."

### Core Components to Implement

1.  **The `NoteEvent` Struct:**
    Instead of `mido` messages, we push `nih_plug::midi::NoteEvent` into the output buffer.

2.  **The State Machine:**
    Because we are generating live, we need to track *where* we are in the song.
    ```rust
    struct CyberForestPlugin {
        params: Arc<CyberForestParams>,
        
        // State
        current_beat: f64,
        current_chord_index: usize,
        progression: Vec<Chord>,
        rng: StdRng,
    }
    ```

3.  **The Process Block:**
    This is the heart of the VST.
    ```rust
    fn process(
        &mut self,
        buffer: &mut Buffer,
        aux: &mut AuxiliaryBuffers,
        context: &mut ProcessContext<Self>,
    ) -> ProcessStatus {
        
        // 1. Get transport info (Are we playing? What is the BPM?)
        let transport = context.transport();
        
        // 2. If playing, calculate where we are in the bar
        if transport.playing {
            // 3. If we crossed a 16th note boundary since the last buffer...
            if self.crossed_16th_boundary() {
                // 4. Call our algorithmic logic
                let note = self.generate_next_arpeggio_note();
                
                // 5. Send MIDI event to host
                context.send_event(NoteEvent::NoteOn { 
                    note, 
                    velocity: 0.8, 
                    timing: 0 // offset within this buffer
                });
            }
        }
        
        ProcessStatus::KeepAlive
    }
    ```

## 4. Building the VST

`nih-plug` uses a specialized build command to package the binary correctly for DAWs.

```bash
# Build the VST3
cargo xtask bundle cyber_forest_vst --release
```

**Output:**
This will create a `target/bundled/cyber_forest_vst.vst3` directory.
*   **Windows:** Copy this to `C:\Program Files\Common Files\VST3`
*   **Mac:** Copy to `/Library/Audio/Plug-Ins/VST3/`
*   **Linux:** Copy to `~/.vst3/`

## 5. Next Steps (Roadmap)

1.  **Phase 1: The "Hello World" Note:**
    Build a simple plugin that just sends a C3 note every beat. This confirms the Rust-to-Ableton pipeline works.
2.  **Phase 2: The Logic Port:**
    Translate the `CyberForestGenerator` Python class into a Rust struct that manages the Chord Pool and Arpeggiator state.
3.  **Phase 3: Parameters:**
    Expose "Intensity" and "Rain Amount" knobs to Ableton so you can automate them in real-time.

---
**Reference:** [nih-plug Repository](https://github.com/robbert-vdh/nih-plug)
