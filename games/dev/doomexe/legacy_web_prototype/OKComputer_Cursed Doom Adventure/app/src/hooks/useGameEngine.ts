import { useState, useCallback, useEffect, useRef } from 'react';
import type { GameState, Player, Enemy, Ending, Companion } from '@/types/game';
import { ALL_SCENES, ENDINGS, AVAILABLE_COMPANIONS, COMPANION_DIALOGUE } from '@/data/story';

// Seeded RNG for consistent randomness
class SeededRNG {
  private seed: number;

  constructor(seed: number) {
    this.seed = seed;
  }

  next(): number {
    this.seed = (this.seed * 9301 + 49297) % 233280;
    return this.seed / 233280;
  }

  range(min: number, max: number): number {
    return Math.floor(this.next() * (max - min + 1)) + min;
  }

  roll(sides: number = 20): number {
    return this.range(1, sides);
  }

  chance(probability: number): boolean {
    return this.next() < probability / 100;
  }
}

// Default player state
const createDefaultPlayer = (): Player => ({
  health: 100,
  maxHealth: 100,
  sanity: 100,
  maxSanity: 100,
  corruption: 0,
  weapon: 'Fists',
  inventory: [],
  stats: {
    strength: 10,
    willpower: 10,
    arcane: 10
  },
  flags: {
    hasRitualDagger: false,
    hasBloodGem: false,
    hasEldritchTome: false,
    hasSoulJar: false,
    visitedRooms: [],
    sacrificedFollowers: 0,
    secretsFound: 0,
    doomCounter: 0,
    readForbiddenTexts: [],
    killedDemons: 0,
    acceptedFate: false,
    wasScreaming: false,
    warnedOther: false,
    secretPath: false,
    tookCommunion: false,
    realityTouched: false,
    acceptedHamster: false
  }
});

// Create initial game state
const createInitialGameState = (): GameState => ({
  player: createDefaultPlayer(),
  currentScene: 'start',
  gameLog: ['DOOM.EXE initialized...', 'Welcome to your doom.', '"Oh, this is going to be FUN!" - Sanguinar'],
  visitedScenes: ['start'],
  rngSeed: Date.now(),
  cursedExe: {
    glitchLevel: 0,
    messagesUnlocked: [],
    realityStability: 100
  },
  companions: [],
  discoveredSecrets: []
});

export function useGameEngine() {
  const [gameState, setGameState] = useState<GameState>(createInitialGameState);
  const [isCombat, setIsCombat] = useState(false);
  const [currentEnemy, setCurrentEnemy] = useState<Enemy | null>(null);
  const [enemyHealth, setEnemyHealth] = useState(0);
  const [combatLog, setCombatLog] = useState<string[]>([]);
  const [gameOver, setGameOver] = useState(false);
  const [currentEnding, setCurrentEnding] = useState<Ending | null>(null);
  const [activeCompanion, setActiveCompanion] = useState<Companion | null>(null);
  const lastSaveTime = useRef<number>(0);
  const gameOverTriggered = useRef<boolean>(false);

  const rng = new SeededRNG(gameState.rngSeed);

  // Add message to game log (with throttling)
  const addToLog = useCallback((message: string, isHamsterQuote: boolean = false) => {
    setGameState(prev => {
      // Avoid duplicate messages
      const lastMessage = prev.gameLog[prev.gameLog.length - 1];
      const newMessage = isHamsterQuote ? `"${message}" - Sanguinar` : message;
      if (lastMessage === newMessage) {
        return prev;
      }
      return {
        ...prev,
        gameLog: [...prev.gameLog.slice(-30), newMessage]
      };
    });
  }, []);

  // Check if choice is available
  const isChoiceAvailable = useCallback((choice: any, player: Player): boolean => {
    // Check required items
    if (choice.requiredItems) {
      for (const item of choice.requiredItems) {
        if (!player.inventory.includes(item)) {
          return false;
        }
      }
    }

    // Check required flags
    if (choice.requiredFlags) {
      for (const [flag, value] of Object.entries(choice.requiredFlags)) {
        const playerFlag = player.flags[flag as keyof typeof player.flags];
        if (playerFlag !== value) {
          return false;
        }
      }
    }

    // Check stat requirements
    if (choice.statCheck) {
      const playerStat = player.stats[choice.statCheck.stat as keyof typeof player.stats];
      if (playerStat < choice.statCheck.threshold) {
        return false;
      }
    }

    return true;
  }, []);

  // Apply choice effects
  const applyChoiceEffects = useCallback((choice: any, player: Player, currentCompanion: Companion | null): { player: Player; companion: Companion | null } => {
    let newPlayer = { ...player };
    let newCompanion = currentCompanion;

    // Add items
    if (choice.addItems) {
      newPlayer.inventory = [...newPlayer.inventory, ...choice.addItems];
      choice.addItems.forEach((item: string) => addToLog(`Acquired: ${item}`));
    }

    // Remove items
    if (choice.removeItems) {
      newPlayer.inventory = newPlayer.inventory.filter((item: string) => !choice.removeItems!.includes(item));
    }

    // Set flags
    if (choice.setFlags) {
      Object.entries(choice.setFlags).forEach(([flag, value]) => {
        if (flag in newPlayer.flags) {
          (newPlayer.flags as any)[flag] = value;
        }
      });
    }

    // Health changes
    if (choice.healthChange) {
      newPlayer.health = Math.max(0, Math.min(newPlayer.maxHealth, newPlayer.health + choice.healthChange));
      if (choice.healthChange < 0) {
        addToLog(`Took ${-choice.healthChange} damage!`);
      } else {
        addToLog(`Healed ${choice.healthChange} health!`);
      }
    }

    // Sanity changes (with better balance)
    if (choice.sanityChange) {
      newPlayer.sanity = Math.max(0, Math.min(newPlayer.maxSanity, newPlayer.sanity + choice.sanityChange));
      if (choice.sanityChange < 0) {
        addToLog(`Lost ${-choice.sanityChange} sanity!`);
        if (newPlayer.sanity < 30 && newPlayer.sanity > 0) {
          addToLog('You hear whispers in the static...', true);
        }
      } else {
        addToLog(`Gained ${choice.sanityChange} sanity!`);
      }
    }

    // Corruption changes
    if (choice.corruptionChange) {
      newPlayer.corruption = Math.max(0, Math.min(100, newPlayer.corruption + choice.corruptionChange));
      if (choice.corruptionChange > 0) {
        addToLog(`Gained ${choice.corruptionChange} corruption!`);
        if (newPlayer.corruption > 50) {
          addToLog('The corruption spreads through your veins...', true);
        }
      }
    }

    // Companion loyalty changes
    if (choice.companionLoyalty && newCompanion) {
      newCompanion.loyalty = Math.max(0, Math.min(100, newCompanion.loyalty + choice.companionLoyalty));
    }

    return { player: newPlayer, companion: newCompanion };
  }, [addToLog]);

  // Handle random events
  const handleRandomEvent = useCallback((choice: any): string => {
    if (!choice.randomEvents || choice.randomEvents.length === 0) {
      return choice.nextScene;
    }

    const totalWeight = choice.randomEvents.reduce((sum: number, event: any) => sum + event.chance, 0);
    let roll = rng.range(1, totalWeight);

    for (const event of choice.randomEvents) {
      roll -= event.chance;
      if (roll <= 0) {
        addToLog(`Random Event: ${event.event}`);
        if (event.sanityDamage) {
          setGameState(prev => ({
            ...prev,
            player: {
              ...prev.player,
              sanity: Math.max(0, prev.player.sanity - event.sanityDamage!)
            }
          }));
        }
        return event.nextScene || choice.nextScene;
      }
    }

    return choice.nextScene;
  }, [addToLog, rng]);

  // Recruit a companion
  const recruitCompanion = useCallback((companionId: string, customName?: string) => {
    const template = AVAILABLE_COMPANIONS[companionId];
    if (!template) return;

    const newCompanion: Companion = {
      ...template,
      name: customName || template.defaultName,
      isActive: true
    };

    setActiveCompanion(newCompanion);
    setGameState(prev => ({
      ...prev,
      companions: [...prev.companions, newCompanion]
    }));

    addToLog(`${newCompanion.name} has joined you!`);
    addToLog(`"${newCompanion.personality}" - ${newCompanion.name}`, true);
  }, [addToLog]);

  // Get companion dialogue
  const getCompanionDialogue = useCallback((eventType: string): string | null => {
    if (!activeCompanion) return null;
    
    const dialogues = COMPANION_DIALOGUE[activeCompanion.id]?.[eventType];
    if (!dialogues || dialogues.length === 0) return null;
    
    return dialogues[Math.floor(Math.random() * dialogues.length)];
  }, [activeCompanion]);

  // Load game
  const loadGame = useCallback(() => {
    const saveData = localStorage.getItem('doomExeSave');
    if (saveData) {
      try {
        const parsed = JSON.parse(saveData);
        setGameState(parsed.gameState);
        // Restore active companion
        const activeComp = parsed.gameState.companions?.find((c: Companion) => c.isActive);
        setActiveCompanion(activeComp || null);
        addToLog('Game loaded.');
      } catch (error) {
        addToLog('Failed to load save.');
      }
    } else {
      addToLog('No save found.');
    }
  }, [addToLog]);

  // Reset game
  const resetGame = useCallback(() => {
    setGameState(createInitialGameState());
    setIsCombat(false);
    setCurrentEnemy(null);
    setEnemyHealth(0);
    setCombatLog([]);
    setGameOver(false);
    setCurrentEnding(null);
    setActiveCompanion(null);
    gameOverTriggered.current = false;
    lastSaveTime.current = 0;
  }, []);

  // Check for ending conditions (with game over prevention)
  const checkEndings = useCallback((player: Player) => {
    if (gameOverTriggered.current) return;

    // First check - prevent The Void Calls from triggering too easily
    if (player.sanity <= 0) {
      // Only trigger void ending if corruption is also high
      if (player.corruption >= 60) {
        gameOverTriggered.current = true;
        setGameOver(true);
        setCurrentEnding(ENDINGS.find(e => e.id === 'the_void') || null);
        addToLog('ENDING UNLOCKED: THE VOID CALLS');
        addToLog('"The void is patient. The void is eternal. The void is you." - Sanguinar', true);
      }
      return;
    }

    // Check other endings
    for (const ending of ENDINGS) {
      if (ending.id === 'the_void') continue; // Skip void ending, handled above
      
      if (ending.requirements) {
        let meetsRequirements = true;
        
        for (const [key, value] of Object.entries(ending.requirements)) {
          if (key in player.flags) {
            const flagValue = player.flags[key as keyof typeof player.flags];
            if (flagValue !== value) {
              meetsRequirements = false;
              break;
            }
          } else if (key in player) {
            const playerValue = (player as any)[key];
            if (playerValue < value) {
              meetsRequirements = false;
              break;
            }
          }
        }

        if (meetsRequirements) {
          gameOverTriggered.current = true;
          setGameOver(true);
          setCurrentEnding(ending);
          addToLog(`ENDING UNLOCKED: ${ending.title}`);
          addToLog(`"${ending.description}" - Sanguinar`, true);
          return;
        }
      }
    }
  }, [addToLog]);

  // Make a choice
  const makeChoice = useCallback((choiceIndex: number) => {
    if (gameOverTriggered.current) return;

    const currentScene = ALL_SCENES[gameState.currentScene];
    if (!currentScene || !currentScene.choices[choiceIndex]) return;

    const choice = currentScene.choices[choiceIndex];
    
    // Check if choice is available
    if (!isChoiceAvailable(choice, gameState.player)) {
      addToLog('You cannot make that choice!');
      return;
    }

    // Apply choice effects
    const { player: updatedPlayer, companion: updatedCompanion } = applyChoiceEffects(choice, gameState.player, activeCompanion);
    
    // Update companion if changed
    if (updatedCompanion) {
      setActiveCompanion(updatedCompanion);
    }

    // Handle random events
    const nextSceneId = handleRandomEvent(choice);

    // Update game state
    setGameState(prev => ({
      ...prev,
      player: updatedPlayer,
      currentScene: nextSceneId,
      visitedScenes: [...prev.visitedScenes, nextSceneId],
      rngSeed: prev.rngSeed + 1
    }));

    addToLog(`> ${choice.text}`);

    // Companion commentary
    if (activeCompanion && choice.text) {
      const companionComment = getCompanionDialogue('choice_made');
      if (companionComment) {
        addToLog(companionComment);
      }
    }

    // Check for game over conditions
    checkEndings(updatedPlayer);

  }, [gameState, isChoiceAvailable, applyChoiceEffects, handleRandomEvent, addToLog, activeCompanion, getCompanionDialogue, checkEndings]);

  // Combat actions
  const combatAttack = useCallback(() => {
    if (!currentEnemy || !isCombat || gameOverTriggered.current) return;

    const player = gameState.player;
    let damage = rng.range(5, 15) + Math.floor(player.stats.strength / 5);
    
    // Companion bonus
    if (activeCompanion && activeCompanion.loyalty > 50) {
      damage += Math.floor(activeCompanion.loyalty / 10);
      const companionAttack = getCompanionDialogue('combat_start');
      if (companionAttack) {
        addToLog(companionAttack);
      }
    }
    
    const newEnemyHealth = Math.max(0, enemyHealth - damage);

    setCombatLog(prev => [...prev, `You attack for ${damage} damage!`]);
    setEnemyHealth(newEnemyHealth);

    if (newEnemyHealth <= 0) {
      // Enemy defeated
      setCombatLog(prev => [...prev, `${currentEnemy.name} has been defeated!`]);
      
      // Companion victory line
      if (activeCompanion) {
        const victoryLine = getCompanionDialogue('combat_victory');
        if (victoryLine) {
          addToLog(victoryLine);
        }
      }
      
      setIsCombat(false);
      setCurrentEnemy(null);
      
      // Update player stats
      setGameState(prev => ({
        ...prev,
        player: {
          ...prev.player,
          flags: {
            ...prev.player.flags,
            killedDemons: prev.player.flags.killedDemons + 1
          }
        }
      }));

      // Move to victory scene
      setTimeout(() => {
        makeChoice(0); // Use first choice as victory
      }, 1000);
    } else {
      // Enemy counterattack
      let enemyDamage = rng.range(currentEnemy.damage - 5, currentEnemy.damage + 5);
      
      // Companion defense
      if (activeCompanion && activeCompanion.id === 'patch' && activeCompanion.loyalty > 30) {
        const blocked = rng.chance(30);
        if (blocked) {
          addToLog(`${activeCompanion.name} blocks the attack!`);
          enemyDamage = 0;
        }
      }
      
      const newPlayerHealth = Math.max(0, player.health - enemyDamage);
      
      if (enemyDamage > 0) {
        setCombatLog(prev => [...prev, `${currentEnemy.name} attacks for ${enemyDamage} damage!`]);
      }
      
      setGameState(prev => ({
        ...prev,
        player: {
          ...prev.player,
          health: newPlayerHealth
        }
      }));

      if (newPlayerHealth <= 0) {
        gameOverTriggered.current = true;
        setGameOver(true);
        setCurrentEnding(ENDINGS.find(e => e.id === 'the_void') || null);
        addToLog('You have been defeated in combat.');
        addToLog('"And they were doing so well!" - Sanguinar', true);
      }
    }
  }, [currentEnemy, isCombat, enemyHealth, gameState.player, rng, addToLog, activeCompanion, getCompanionDialogue, makeChoice]);

  const combatFlee = useCallback(() => {
    if (!isCombat || gameOverTriggered.current) return;

    if (rng.chance(40)) {
      setCombatLog(prev => [...prev, 'You successfully fled!']);
      setIsCombat(false);
      setCurrentEnemy(null);
      setGameState(prev => ({
        ...prev,
        currentScene: 'ethernet_corridor' // Fallback scene
      }));
    } else {
      setCombatLog(prev => [...prev, 'Failed to flee!']);
      combatAttack(); // Enemy gets a free hit
    }
  }, [isCombat, rng, combatAttack]);

  // Start combat
  const startCombat = useCallback((enemy: Enemy) => {
    if (gameOverTriggered.current) return;
    
    setIsCombat(true);
    setCurrentEnemy(enemy);
    setEnemyHealth(enemy.health);
    setCombatLog([`Combat started with ${enemy.name}!`, enemy.description]);
    
    // Companion combat start line
    if (activeCompanion) {
      const startLine = getCompanionDialogue('combat_start');
      if (startLine) {
        addToLog(startLine);
      }
    }
  }, [activeCompanion, addToLog, getCompanionDialogue]);

  // Get current scene
  const currentScene = ALL_SCENES[gameState.currentScene];

  // Auto-save on state change (with throttling)
  useEffect(() => {
    const timeout = setTimeout(() => {
      const now = Date.now();
      if (now - lastSaveTime.current >= 10000) { // 10 second cooldown
        lastSaveTime.current = now;
        const saveData = {
          gameState,
          timestamp: now
        };
        localStorage.setItem('doomExeSave', JSON.stringify(saveData));
        addToLog('Game saved.');
      }
    }, 1000);
    return () => clearTimeout(timeout);
  }, [gameState, addToLog]);

  // Initialize combat when entering combat scene
  useEffect(() => {
    if (currentScene?.isCombat && currentScene.enemy && !isCombat && !gameOverTriggered.current) {
      startCombat(currentScene.enemy);
    }
  }, [currentScene, isCombat, startCombat]);

  return {
    gameState,
    currentScene,
    isCombat,
    currentEnemy,
    enemyHealth,
    combatLog,
    gameOver,
    currentEnding,
    activeCompanion,
    makeChoice,
    combatAttack,
    combatFlee,
    resetGame,
    saveGame: () => {
      const now = Date.now();
      if (now - lastSaveTime.current >= 5000) {
        lastSaveTime.current = now;
        const saveData = {
          gameState,
          timestamp: now
        };
        localStorage.setItem('doomExeSave', JSON.stringify(saveData));
        addToLog('Game saved.');
      }
    },
    loadGame,
    addToLog,
    isChoiceAvailable,
    recruitCompanion
  };
}
