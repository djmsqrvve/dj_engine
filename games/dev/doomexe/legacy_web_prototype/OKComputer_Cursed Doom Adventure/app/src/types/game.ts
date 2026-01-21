export interface Player {
  health: number;
  maxHealth: number;
  sanity: number;
  maxSanity: number;
  corruption: number;
  weapon: string;
  inventory: string[];
  stats: {
    strength: number;
    willpower: number;
    arcane: number;
  };
  flags: {
    hasRitualDagger: boolean;
    hasBloodGem: boolean;
    hasEldritchTome: boolean;
    hasSoulJar: boolean;
    visitedRooms: string[];
    sacrificedFollowers: number;
    secretsFound: number;
    doomCounter: number;
    readForbiddenTexts: string[];
    killedDemons: number;
    acceptedFate: boolean;
    wasScreaming: boolean;
    warnedOther: boolean;
    secretPath: boolean;
    tookCommunion: boolean;
    realityTouched: boolean;
    acceptedHamster: boolean;
  };
}

export interface Companion {
  id: string;
  name: string;
  defaultName: string;
  species: string;
  personality: string;
  loyalty: number;
  corruption: number;
  specialAbility: string;
  backstory: string;
  currentThought: string;
  isActive: boolean;
}

export interface Enemy {
  name: string;
  health: number;
  maxHealth: number;
  damage: number;
  description: string;
  attackMessages: string[];
  isEldritch?: boolean;
}

export interface Choice {
  text: string;
  nextScene: string;
  requiredItems?: string[];
  requiredFlags?: { [key: string]: boolean | number };
  statCheck?: { stat: keyof Player['stats']; threshold: number };
  setFlags?: { [key: string]: boolean | number };
  removeItems?: string[];
  addItems?: string[];
  healthChange?: number;
  sanityChange?: number;
  corruptionChange?: number;
  randomEvents?: { chance: number; event: string; nextScene?: string; sanityDamage?: number }[];
}

export interface Scene {
  id: string;
  title: string;
  text: string[];
  choices: Choice[];
  isCombat?: boolean;
  enemy?: Enemy;
  onEnter?: (player: Player) => Player;
  ambientSound?: string;
  corruptionLevel?: number;
}

export interface GameState {
  player: Player;
  currentScene: string;
  gameLog: string[];
  visitedScenes: string[];
  ending?: string;
  rngSeed: number;
  cursedExe: {
    glitchLevel: number;
    messagesUnlocked: string[];
    realityStability: number;
  };
  companions: Companion[];
  discoveredSecrets: string[];
}

export type EndingType = 'ascension' | 'damnation' | 'corruption' | 'purification' | 'eternal' | 'void' | 'secret';

export interface Ending {
  id: string;
  title: string;
  description: string;
  type: EndingType;
  requirements?: { [key: string]: boolean | number };
  unlocksMessage?: string;
}
