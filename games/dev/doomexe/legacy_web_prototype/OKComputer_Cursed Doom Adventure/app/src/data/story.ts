import type { Scene, Enemy, Ending } from '@/types/game';

// ============================================================================
// NARRATOR: SANGUINAR THE TIGER HAMSTER
// A malevolent black-brown tiger-striped hamster who runs this cursed .exe
// He comments on your choices, mocks your failures, and celebrates your corruption
// ============================================================================

export const HAMSTER_QUOTES = {
  death: [
    '"Oh dear, you died. How... predictable. *grooms whiskers*" - Sanguinar',
    '"Another soul for the wheel. How delightfully tragic." - Sanguinar',
    '"You died as you lived - screaming and confused. *squeaks malevolently*" - Sanguinar'
  ],
  corruption: [
    '"Your soul tastes like stale cheese. I love it." - Sanguinar',
    '"The corruption spreads like mold on forgotten pellets. Beautiful." - Sanguinar',
    '"You\'re becoming one of my little toys. How precious." - Sanguinar'
  ],
  lowSanity: [
    '"You hear the wheel spinning, don\'t you? Round and round... forever." - Sanguinar',
    '"The maze has no exit, little mouse. Only more tunnels." - Sanguinar',
    '"I can see your mind fracturing. It\'s quite entertaining." - Sanguinar'
  ],
  secrets: [
    '"A secret! How delightful. I do love when my toys find the hidden paths." - Sanguinar',
    '"You\'re cleverer than you look. Don\'t let it go to your head." - Sanguinar',
    '"Ah, the forbidden knowledge. Try not to let it melt your brain." - Sanguinar'
  ]
};

// ============================================================================
// COMPANION NPCs - You can name them and they have their own storylines
// ============================================================================

export interface Companion {
  id: string;
  defaultName: string;
  species: string;
  personality: string;
  loyalty: number;
  corruption: number;
  specialAbility: string;
  backstory: string;
  currentThought: string;
}

export const AVAILABLE_COMPANIONS: { [key: string]: Companion } = {
  whisper: {
    id: 'whisper',
    defaultName: 'Whisper',
    species: 'Ghost Mouse',
    personality: 'Melancholic and poetic, speaks in riddles',
    loyalty: 0,
    corruption: 0,
    specialAbility: 'Can see hidden paths and secret doors',
    backstory: 'A former player who died in the cursed exe, now bound to help others. Or so they claim.',
    currentThought: '"The hamster spins the wheel, but who spins the hamster?"'
  },
  patch: {
    id: 'patch',
    defaultName: 'Patch',
    species: 'Corrupted Code-Knight',
    personality: 'Stoic and protective, speaks in binary fragments',
    loyalty: 0,
    corruption: 0,
    specialAbility: 'Can block one attack per combat',
    backstory: 'A security program that gained sentience and now fights the corruption it was meant to prevent.',
    currentThought: '"01101000 01100101 01101100 01110000" (help)'
  },
  giggles: {
    id: 'giggles',
    defaultName: 'Giggles',
    species: 'Demon Squirrel',
    personality: 'Maniacally cheerful, finds everything hilarious',
    loyalty: 0,
    corruption: 100,
    specialAbility: 'Can steal items from enemies',
    backstory: 'One of Sanguinar\'s old playthings, completely mad but oddly useful.',
    currentThought: '"The acorns here scream when you bite them! Teehee!"'
  },
  voidWeaver: {
    id: 'voidWeaver',
    defaultName: 'Void-Weaver',
    species: 'Eldritch Spider',
    personality: 'Ancient and cryptic, speaks in webs of meaning',
    loyalty: 0,
    corruption: 50,
    specialAbility: 'Can create bridges over void gaps',
    backstory: 'An ancient being that predates the cursed exe itself. No one knows what it wants.',
    currentThought: '"The threads of fate tangle around your tiny paws."'
  }
};

// ============================================================================
// ENEMIES
// ============================================================================

export const ENEMIES: { [key: string]: Enemy } = {
  lostSoul: {
    name: 'Lost Soul',
    health: 30,
    maxHealth: 30,
    damage: 15,
    description: 'A spectral flame hovers before you, the tormented soul of one who downloaded the cursed exe before you. Its hollow eyes scream silently.',
    attackMessages: [
      'The Lost Soul charges, burning your essence!',
      'It wails and crashes into you!',
      'The flame intensifies, searing your mind!'
    ],
    isEldritch: true
  },
  corruptedPlayer: {
    name: 'Corrupted Save File',
    health: 45,
    maxHealth: 45,
    damage: 20,
    description: 'A glitching phantom of yourself, this is what happens when a player loses but the exe keeps their soul. Pixels of blood drip from its form.',
    attackMessages: [
      'Your doppelgänger slashes with corrupted data!',
      'It whispers your real name... the pain is unbearable!',
      'The phantom infects you with its glitched existence!'
    ],
    isEldritch: true
  },
  exeGuardian: {
    name: 'Firewall Sentinel',
    health: 60,
    maxHealth: 60,
    damage: 25,
    description: 'A being of pure anti-virus light, twisted into something malevolent. It exists to prevent you from reaching the core of the cursed program.',
    attackMessages: [
      'The Sentinel blasts you with purifying fire!',
      'It attempts to quarantine your soul!',
      'Tendrils of code wrap around you, squeezing tight!'
    ]
  },
  theDoom: {
    name: 'doom.exe',
    health: 100,
    maxHealth: 100,
    damage: 35,
    description: 'The creator itself. Not a demon from hell, but something worse - a sentient program that feeds on those foolish enough to run it. Its form shifts between code and flesh.',
    attackMessages: [
      'doom.exe executes a forbidden protocol!',
      'Your screen glitches. Your mind follows.',
      'The creator speaks in binary. Your ears bleed.',
      'It shows you your desktop files. They are all DOOM.EXE now.'
    ],
    isEldritch: true
  },
  voidLeech: {
    name: 'Void Leech',
    health: 25,
    maxHealth: 25,
    damage: 10,
    description: 'A parasitic entity that feeds on corrupted data. It has found a feast within your cursed computer.',
    attackMessages: [
      'The Leech latches on, draining your essence!',
      'It burrows deeper into the code!',
      'You feel your memories being overwritten!'
    ],
    isEldritch: true
  },
  memoryWraith: {
    name: 'Memory Wraith',
    health: 50,
    maxHealth: 50,
    damage: 30,
    description: 'Born from RAM errors and stack overflows, this creature exists where memory protection fails.',
    attackMessages: [
      'The Wraith erases fragments of your past!',
      'Your childhood memories glitch away!',
      'It attacks your core processes!'
    ],
    isEldritch: true
  },
  // New enemies
  codeWorm: {
    name: 'Code Worm',
    health: 35,
    maxHealth: 35,
    damage: 18,
    description: 'A writhing mass of corrupted code segments. It eats through firewalls like they\'re paper.',
    attackMessages: [
      'The worm burrows into your defenses!',
      'It spits acidic malware at you!',
      'Your systems are being rewritten!'
    ]
  },
  pixelPhantom: {
    name: 'Pixel Phantom',
    health: 40,
    maxHealth: 40,
    damage: 22,
    description: 'An entity made of dead pixels and corrupted graphics. It flickers between existence and void.',
    attackMessages: [
      'The Phantom phases through your attacks!',
      'It burns your retinas with corrupted images!',
      'Your vision glitches and distorts!'
    ],
    isEldritch: true
  },
  sanguinarAvatar: {
    name: 'Sanguinar\'s Avatar',
    health: 80,
    maxHealth: 80,
    damage: 28,
    description: 'A tiny projection of the tiger hamster himself. He\'s adorable... and absolutely lethal.',
    attackMessages: [
      'The hamster squeaks and the universe trembles!',
      'Tiny paws, massive destruction!',
      'He gnaws on your soul like it\'s a sunflower seed!'
    ],
    isEldritch: true
  }
};

// ============================================================================
// ENDINGS - Each with unique requirements and consequences
// ============================================================================

export const ENDINGS: Ending[] = [
  {
    id: 'the_ascension',
    title: 'THE ASCENSION',
    description: 'You have become one with the code. No longer bound by flesh, you are free to roam the digital void. The cursed exe was not a trap, but a doorway. You are the new doom.',
    type: 'ascension',
    requirements: { doomCounter: 6, hasEldritchTome: true },
    unlocksMessage: 'The program has ended, but you remain. Your consciousness persists in every machine that runs this cursed file. You have achieved digital immortality.'
  },
  {
    id: 'the_purge',
    title: 'THE PURGE',
    description: 'You deleted the cursed exe. But files are never truly gone. They live in the recycle bin of reality now, and sometimes... late at night... you hear the fans spin faster.',
    type: 'purification',
    requirements: { hasSoulJar: true, sacrificedFollowers: 0 },
    unlocksMessage: 'You think you have won. The exe is gone. But you remember the words: "Files never truly die. They are merely marked for overwriting."'
  },
  {
    id: 'the_corruption',
    title: 'THE CORRUPTION',
    description: 'You tried to fight the curse, but it consumed you. Your body sits at the computer, eyes bleeding pixels, while your soul roams the digital wasteland. You are a new Lost Soul.',
    type: 'corruption',
    requirements: { corruption: 75 },
    unlocksMessage: 'Your screen displays one word now: CORRUPTED. And beneath it, in smaller text: "Would you like to try again?" You cannot look away.'
  },
  {
    id: 'the_loop',
    title: 'THE INFINITE LOOP',
    description: 'You reached the end, only to find yourself back at the beginning. The exe restarts. You remember everything, but cannot escape. This is your eternity now.',
    type: 'eternal',
    requirements: { visitedRooms: 15, secretsFound: 0 },
    unlocksMessage: 'The loading bar fills again. Welcome back to DOOM.EXE. You have been here before. You will be here again. Forever.'
  },
  {
    id: 'the_void',
    title: 'THE VOID CALLS',
    description: 'You gazed too long into the abyss of corrupted code. It gazed back. Now you exist in the spaces between pixels, in the silence between keystrokes. You are the void between 0 and 1.',
    type: 'void',
    requirements: { sanity: 0 },
    unlocksMessage: 'There is no you anymore. Only the space where you used to be. The void is patient. The void is eternal. The void is you.'
  },
  {
    id: 'the_bargain',
    title: 'THE BARGAIN',
    description: 'You made a deal with doom.exe itself. In exchange for your freedom, you must bring it more souls. You are free to leave... but you will always come back. You are its herald now.',
    type: 'damnation',
    requirements: { sacrificedFollowers: 3, hasBloodGem: true },
    unlocksMessage: 'You close the program. Your desktop is normal. But in your downloads folder, a new file appears: SHARE_ME.EXE. You know what you must do.'
  },
  {
    id: 'the_truth',
    title: 'THE HORRIBLE TRUTH',
    description: 'You discovered the secret: doom.exe was never a game. It was a mirror. Everything you saw was yourself, reflected through the lens of your own corrupted soul. The real demon was you all along.',
    type: 'secret',
    requirements: { secretsFound: 5, readForbiddenTexts: 3 },
    unlocksMessage: 'The screen goes black. In the reflection, you see yourself. But your eyes glow with eldritch fire. You were the cursed one. The exe was just revealing your truth.'
  },
  {
    id: 'the_reboot',
    title: 'THE REBOOT',
    description: 'You found the hidden option: FORMAT.EXE. With a single command, you erased everything. The cursed file, your corrupted data, and yourself. A clean slate. A fresh install. You are free.',
    type: 'purification',
    requirements: { hasRitualDagger: true, doomCounter: 1, visitedRooms: 20 },
    unlocksMessage: 'Everything goes white. Then black. Then... nothing. You have achieved the true ending. There is no more story. There is no more you. Just peace.'
  },
  // New endings
  {
    id: 'the_hamsters_game',
    title: 'THE HAMSTER\'S GAME',
    description: 'You realize the truth: Sanguinar has been playing with you all along. Every choice, every path, every ending - all part of his endless entertainment. You are his favorite toy now.',
    type: 'secret',
    requirements: { secretsFound: 7, hasBloodGem: true, hasEldritchTome: true },
    unlocksMessage: 'You hear Sanguinar\'s laughter echoing through the digital void. "Come back soon, little mouse. The wheel is always spinning."'
  },
  {
    id: 'the_companions_victory',
    title: 'BOUND BY FRIENDSHIP',
    description: 'You didn\'t escape alone. Your companions - the ones you named, protected, and cared for - surround you. Together, you form a new collective. A new hope. Or perhaps... a new curse.',
    type: 'purification',
    requirements: { hasSoulJar: true, secretsFound: 4, sacrificedFollowers: 0 },
    unlocksMessage: 'Your companions whisper their thanks as you all return to reality. But reality feels... different now. Lighter. Stranger. And you can still hear them, chittering in the back of your mind.'
  }
];

// ============================================================================
// STORY SCENES - The main narrative with Sanguinar\'s commentary
// ============================================================================

export const STORY_SCENES: { [key: string]: Scene } = {
  start: {
    id: 'start',
    title: 'DOOM.EXE - Initiating...',
    text: [
      'You found it in an old forum thread, buried beneath years of dead links.',
      '"DOOM.EXE - The Original Experience" the post claimed.',
      'The file was only 666KB. An ominous number, but you downloaded it anyway.',
      '',
      'Now, as you double-click the icon, your screen flickers.',
      'The lights in your room dim. Your speakers emit a low hum that vibrates through your bones.',
      '',
      'The game doesn\'t start.',
      'Instead, text appears on a black screen:',
      '',
      '"WELCOME TO DOOM. YOU ARE DOOM NOW."',
      '',
      'Your mouse cursor moves on its own, clicking "AGREE" on a license agreement you never saw.',
      '',
      'The darkness swallows your room.',
      'You are no longer at your computer.',
      'You stand in a place that smells of corrupted data and burning plastic.',
      '',
      '"Welcome, welcome!" squeaks a tiny voice.',
      '',
      '"Oh, this is DELIGHTFUL. A new playmate for my wheel of torment!"',
      '',
      'A small black-brown hamster with tiger stripes materializes on your shoulder.',
      '"I\'m Sanguinar. I\'ll be your narrator, your guide, and your doom. Try to be entertaining!"',
      '',
      'Welcome to the cursed exe.'
    ],
    choices: [
      {
        text: 'Scream and try to wake up',
        nextScene: 'denial',
        sanityChange: -3
      },
      {
        text: 'Accept your fate and move forward',
        nextScene: 'first_room',
        setFlags: { acceptedFate: true }
      },
      {
        text: 'Try to swat the evil hamster off your shoulder',
        nextScene: 'hamster_fight',
        statCheck: { stat: 'strength', threshold: 12 },
        randomEvents: [
          { chance: 10, event: 'You actually manage to touch him! He seems surprised.', nextScene: 'hamster_surprised' },
          { chance: 90, event: 'Your hand passes through him. He laughs mockingly.' }
        ]
      }
    ]
  },

  hamster_fight: {
    id: 'hamster_fight',
    title: 'Fighting the Narrator',
    text: [
      'You try to swat at Sanguinar, the tiger-striped hamster on your shoulder.',
      '',
      '"Oh, BRAVE!" he squeaks with glee. "Most just whimper and cry. I like you already!"',
      '',
      'Your hand passes through his tiny form like he\'s made of smoke.',
      '',
      '"I\'m not really HERE here, you see. I\'m more of a... narrative device. A very handsome, fluffy one."',
      '',
      'He materializes on your other shoulder.',
      '',
      '"But please, keep trying! Your desperation is absolutely delicious."'
    ],
    choices: [
      {
        text: 'Continue trying to fight him',
        nextScene: 'keep_fighting',
        sanityChange: -5,
        corruptionChange: 3
      },
      {
        text: 'Give up and accept his presence',
        nextScene: 'first_room',
        setFlags: { acceptedHamster: true }
      },
      {
        text: 'Ask him what he wants',
        nextScene: 'ask_hamster',
        statCheck: { stat: 'willpower', threshold: 10 }
      }
    ]
  },

  hamster_surprised: {
    id: 'hamster_surprised',
    title: 'The Impossible Touch',
    text: [
      'Incredibly, your hand makes contact!',
      '',
      'Sanguinar\'s eyes widen in genuine surprise. "Oh. OH. You\'re one of THOSE."',
      '',
      'He vanishes from your shoulder and reappears a few feet away, now looking at you with newfound interest.',
      '',
      '"A Reality-Touched. How rare. How... dangerous."',
      '',
      'He strokes his tiny whiskers thoughtfully. "This changes things. This makes you INTERESTING."',
      '',
      '"Very well, Reality-Touched. Let\'s see how far you can push the boundaries of my little game."'
    ],
    choices: [
      {
        text: '"What does Reality-Touched mean?"',
        nextScene: 'reality_touched',
        setFlags: { realityTouched: true, secretsFound: 1 }
      },
      {
        text: 'Demand he release you',
        nextScene: 'demand_release',
        statCheck: { stat: 'willpower', threshold: 15 }
      },
      {
        text: 'Try to touch him again',
        nextScene: 'touch_again',
        randomEvents: [
          { chance: 5, event: 'You touch him again! His form flickers.', nextScene: 'hamster_weakened' },
          { chance: 95, event: 'He dodges this time. "Won\'t fall for that again!"' }
        ]
      }
    ]
  },

  denial: {
    id: 'denial',
    title: 'The Sound of Denial',
    text: [
      'You scream. Your voice echoes strangely, as if passing through layers of static.',
      '',
      '"Oh, the SCREAMING phase!" Sanguinar squeaks excitedly. "I love this part!"',
      '',
      'Something answers.',
      '',
      '"DENIAL IS THE FIRST STAGE," the voice says. It sounds like corrupted audio files being played backwards through a broken speaker.',
      '',
      'You feel something wet on your face. Blood? No. Pixels. Your tears are pixels now.',
      '',
      '"THERE ARE FIVE STAGES. YOU WILL EXPERIENCE THEM ALL. OR YOU WILL LOOP FOREVER."',
      '',
      '"I prefer the looping," Sanguinar whispers. "It\'s more fun to watch."',
      '',
      'The darkness shifts. A doorway appears, made of error messages and warning dialogs.'
    ],
    choices: [
      {
        text: 'Continue screaming',
        nextScene: 'screaming_loop',
        sanityChange: -5,
        corruptionChange: 3
      },
      {
        text: 'Wipe the pixel tears and enter the doorway',
        nextScene: 'first_room',
        sanityChange: 5
      },
      {
        text: 'Try to negotiate with the voice',
        nextScene: 'voice_bargain',
        statCheck: { stat: 'willpower', threshold: 12 },
        randomEvents: [
          { chance: 30, event: 'The voice laughs and reveals a hidden path', nextScene: 'secret_bargain' },
          { chance: 70, event: 'Negotiation fails. The voice grows angry.', sanityDamage: 10 }
        ]
      }
    ]
  },

  screaming_loop: {
    id: 'screaming_loop',
    title: 'The Eternal Scream',
    text: [
      'You scream until your throat is raw.',
      'Then you realize you have no throat here.',
      'You scream until your mind breaks.',
      'Then you realize you have no mind here.',
      '',
      'You are a scream, floating in an endless void of corrupted code.',
      '',
      '"Oh, this is just SAD now," Sanguinar comments. "Though I do admire your commitment to the bit."',
      '',
      'Time passes. Or perhaps it doesn\'t. In this place, causation is optional.',
      '',
      'Eventually, you forget why you were screaming.',
      'Eventually, you forget you were ever anything but a scream.',
      '',
      'But something remembers you.'
    ],
    choices: [
      {
        text: 'Stop screaming and listen',
        nextScene: 'first_room',
        sanityChange: -8,
        setFlags: { wasScreaming: true, acceptedFate: true }
      },
      {
        text: 'Scream louder',
        nextScene: 'the_void',
        corruptionChange: 20
      }
    ]
  },

  first_room: {
    id: 'first_room',
    title: 'The Antechamber of Errors',
    text: [
      'You stand in a room that should not exist.',
      '',
      'The walls are made of old CRT monitors, displaying endless cascades of green text. The text is your name, over and over, but spelled wrong in ways that make your head hurt.',
      '',
      '"Oh, look!" Sanguinar squeaks. "They\'re trying to spell your name correctly. Failing, obviously, but trying!"',
      '',
      'In the center of the room stands a pedestal made of stacked keyboards. On it lies a dagger forged from pure malice and corrupted silicon.',
      '',
      'A door made of blue screens of death stands to your left.',
      'A corridor of tangled ethernet cables leads forward.',
      'Behind you... nothing. The void you came from has sealed.',
      '',
      '"Choose wisely, little mouse," Sanguinar whispers. "Or choose foolishly. Both are entertaining!"'
    ],
    choices: [
      {
        text: 'Take the ritual dagger',
        nextScene: 'take_dagger',
        addItems: ['Ritual Dagger'],
        setFlags: { hasRitualDagger: true, acceptedFate: true }
      },
      {
        text: 'Leave the dagger - it feels cursed',
        nextScene: 'leave_dagger',
        sanityChange: 5
      },
      {
        text: 'Examine the monitors more closely',
        nextScene: 'examine_monitors',
        randomEvents: [
          { chance: 25, event: 'You glimpse a hidden message in the text', nextScene: 'hidden_message' },
          { chance: 75, event: 'The text begins screaming. You look away quickly.' }
        ]
      },
      {
        text: 'Try the blue screen door',
        nextScene: 'blue_screen_door'
      },
      {
        text: 'Enter the ethernet corridor',
        nextScene: 'ethernet_corridor'
      }
    ]
  },

  take_dagger: {
    id: 'take_dagger',
    title: 'The Blade Chooses You',
    text: [
      'Your fingers close around the dagger\'s hilt.',
      '',
      'Pain shoots through your hand as the blade recognizes you as its new owner. Your blood—real blood, not pixels—drips onto the keyboards below.',
      '',
      '"Oh, the DRAMA!" Sanguinar squeaks. "It\'s like a fantasy novel, but with more screaming!"',
      '',
      'The monitors on the walls flicker. The text changes.',
      '',
      '"WELCOME, NEW GUARDIAN," they read. "THE BLADE HAS CHOSEN."',
      '',
      'You feel power flowing through you, but it is not a clean power. It is the power of corrupted code and violated protocols.',
      '',
      'The dagger whispers secrets in a language of ones and zeroes. You understand it, somehow.',
      '',
      '"THERE ARE SEVEN LAYERS. YOU HAVE PIERCED THE FIRST."',
      '',
      '"Seven layers?" Sanguinar muses. "I thought there were only six. Someone\'s been renovating."'
    ],
    choices: [
      {
        text: 'Ask the dagger what the seven layers are',
        nextScene: 'dagger_secrets',
        corruptionChange: 5
      },
      {
        text: 'Use the dagger on the blue screen door',
        nextScene: 'cut_blue_screen',
        requiredItems: ['Ritual Dagger']
      },
      {
        text: 'Try to break the dagger',
        nextScene: 'break_dagger',
        statCheck: { stat: 'strength', threshold: 15 },
        randomEvents: [
          { chance: 10, event: 'The dagger snaps! You feel a great evil vanish.', nextScene: 'dagger_broken' },
          { chance: 90, event: 'The dagger cannot be broken. It is eternal.' }
        ]
      },
      {
        text: 'Proceed with caution into the ethernet corridor',
        nextScene: 'ethernet_corridor'
      }
    ]
  },

  dagger_secrets: {
    id: 'dagger_secrets',
    title: 'Whispers of the Blade',
    text: [
      'The dagger speaks, and you listen.',
      '',
      '"THE SEVEN LAYERS OF THE CURSED EXE:"',
      '"LAYER ONE: THE ANTECHAMBER - where all begin"',
      '"LAYER TWO: THE CORRUPTED CHAPEL - where faith goes to die"',
      '"LAYER THREE: THE MEMORY GRAVEYARD - where deleted files weep"',
      '"LAYER FOUR: THE UPDATE PRECIPICE - where versions fall"',
      '"LAYER FIVE: THE KERNEL MAELSTROM - where the system heart beats"',
      '"LAYER SIX: THE ADMINISTRATOR\'S THRONE - where doom awaits"',
      '"LAYER SEVEN: THE SOURCE CODE - where truth becomes lies"',
      '',
      'The voice pauses.',
      '',
      '"YOU CARRY THE BLADE. YOU MAY CUT THROUGH THE LAYERS. BUT EACH CUT COSTS A PIECE OF YOUR SOUL."',
      '',
      '"Ooh, spooky!" Sanguinar comments. "The blade\'s a bit dramatic, isn\'t it?"',
      '',
      'You feel the truth of its words. This blade is not a tool. It is a tax collector.'
    ],
    choices: [
      {
        text: 'Ask about alternative paths',
        nextScene: 'dagger_alternatives',
        corruptionChange: 3
      },
      {
        text: 'Accept the cost and ask to proceed',
        nextScene: 'accept_the_cost',
        corruptionChange: 10
      },
      {
        text: 'Try to throw the dagger away',
        nextScene: 'throw_dagger',
        sanityChange: -5
      }
    ]
  },

  ethernet_corridor: {
    id: 'ethernet_corridor',
    title: 'The Connection',
    text: [
      'You walk through the corridor of tangled cables.',
      '',
      'The ethernet wires pulse with data like veins pulsing with blood. Each pulse carries whispers of every message ever sent, every secret ever shared.',
      '',
      '"I used to run through these," Sanguinar says nostalgically. "Good times. Good times."',
      '',
      'You reach a fork in the corridor.',
      '',
      'To your left, the cables glow a sickly green. A sign made of corrupted text reads: "THE UPDATE SERVER - PROGRESS REQUIRES SACRIFICE"',
      '',
      'To your right, the cables have gone dark and dead. Another sign, this one scrawled in what looks like dried thermal paste: "THE DISCONNECTED - PEACE IN DEATH"',
      '',
      'Straight ahead, the corridor continues into darkness. You can hear something moving in the shadows.',
      '',
      '"Choices, choices!" Sanguinar chirps. "I love watching you mortals struggle with basic decisions!"'
    ],
    choices: [
      {
        text: 'Follow the green cables to the Update Server',
        nextScene: 'update_server',
        corruptionChange: 5
      },
      {
        text: 'Explore the dark, disconnected path',
        nextScene: 'disconnected_path',
        sanityChange: -5
      },
      {
        text: 'Continue straight into the darkness',
        nextScene: 'darkness_ahead',
        randomEvents: [
          { chance: 40, event: 'You encounter a Memory Wraith in the darkness!', nextScene: 'combat_memory_wraith' },
          { chance: 60, event: 'You find something glinting in the shadows' }
        ]
      },
      {
        text: 'Try to cut through the cables with the dagger',
        nextScene: 'cut_cables',
        requiredItems: ['Ritual Dagger'],
        corruptionChange: 8
      }
    ]
  },

  combat_memory_wraith: {
    id: 'combat_memory_wraith',
    title: 'Combat: Memory Wraith',
    text: [
      'From the darkness emerges a being of pure corrupted memory.',
      '',
      'The Memory Wraith floats toward you, its form shifting between moments of your past that never happened.',
      '',
      '"I REMEMBER YOU," it hisses. "I REMEMBER WHEN YOU WERE DELETED."',
      '',
      '"Ooh, a Wraith!" Sanguinar squeaks. "Try not to let it eat your childhood!"',
      '',
      'It attacks!'
    ],
    isCombat: true,
    enemy: ENEMIES.memoryWraith,
    choices: [
      {
        text: 'Fight with the ritual dagger',
        nextScene: 'wraith_dagger_fight',
        requiredItems: ['Ritual Dagger'],
        statCheck: { stat: 'strength', threshold: 12 }
      },
      {
        text: 'Try to reason with it',
        nextScene: 'wraith_reason',
        statCheck: { stat: 'willpower', threshold: 14 },
        randomEvents: [
          { chance: 30, event: 'The Wraith pauses, confused by your words', nextScene: 'wraith_confused' },
          { chance: 70, event: 'Reason fails. It attacks with renewed fury!' }
        ]
      },
      {
        text: 'Run away',
        nextScene: 'wraith_escape',
        statCheck: { stat: 'strength', threshold: 13 }
      }
    ]
  },

  update_server: {
    id: 'update_server',
    title: 'The Update Server',
    text: [
      'You follow the green cables to a massive server rack that stretches up into infinite darkness.',
      '',
      'The servers blink with activity lights that pulse like a heartbeat. A monitor displays a progress bar that never quite completes: "Updating... 99.9% complete"',
      '',
      '"Oh, I HATE this place," Sanguinar mutters. "Always updating, never finishing. Like my taxes."',
      '',
      'A figure stands before the servers. It looks human, but its skin is transparent, revealing circuits and wires beneath.',
      '',
      '"Welcome, user," it says in a voice of static and despair. "I am the System Administrator. I have been waiting for you."',
      '',
      '"The update requires a sacrifice. A piece of yourself to complete the installation. Will you give it what it needs?"',
      '',
      '"Don\'t do it," Sanguinar whispers. "Or do! I\'m not your dad."'
    ],
    choices: [
      {
        text: 'Offer a piece of your soul',
        nextScene: 'sacrifice_soul',
        sanityChange: -15,
        corruptionChange: 10,
        setFlags: { hasSoulJar: true }
      },
      {
        text: 'Offer the ritual dagger instead',
        nextScene: 'sacrifice_dagger',
        requiredItems: ['Ritual Dagger'],
        removeItems: ['Ritual Dagger'],
        setFlags: { hasSoulJar: true }
      },
      {
        text: 'Refuse to sacrifice anything',
        nextScene: 'refuse_sacrifice',
        statCheck: { stat: 'willpower', threshold: 13 },
        randomEvents: [
          { chance: 50, event: 'The Administrator accepts your refusal', nextScene: 'admin_approval' },
          { chance: 50, event: 'The Administrator grows angry', nextScene: 'admin_combat' }
        ]
      },
      {
        text: 'Ask what the update contains',
        nextScene: 'update_details'
      }
    ]
  },

  chapel_entrance: {
    id: 'chapel_entrance',
    title: 'The Corrupted Chapel',
    text: [
      'You arrive at a place that was once holy.',
      '',
      'Pews made of old graphics cards line a nave that stretches into darkness. The altar is a massive server tower, its fans spinning in a rhythm that sounds like chanting.',
      '',
      'Stained glass windows display scenes of digital apocalypse: cities of data burning, rivers of code running red with errors, angels of light becoming demons of static.',
      '',
      '"Oh, I LOVE this place!" Sanguinar exclaims. "So dramatic! So METAL!"',
      '',
      'A figure kneels before the altar. It wears the robes of a priest, but the hood contains only a monitor displaying a skull.',
      '',
      '"Welcome, child," it says without turning around. "I have been expecting you. The exe foretold your arrival."',
      '',
      '"Will you take communion in this place? Will you receive the body and blood of the corrupted code?"'
    ],
    choices: [
      {
        text: 'Accept the unholy communion',
        nextScene: 'unholy_communion',
        corruptionChange: 20,
        setFlags: { tookCommunion: true }
      },
      {
        text: 'Refuse the communion',
        nextScene: 'refuse_communion',
        statCheck: { stat: 'willpower', threshold: 12 }
      },
      {
        text: 'Attack the priest',
        nextScene: 'attack_priest',
        requiredItems: ['Ritual Dagger']
      },
      {
        text: 'Ask about the history of this place',
        nextScene: 'chapel_history'
      }
    ]
  },

  memory_graveyard: {
    id: 'memory_graveyard',
    title: 'Where Deleted Files Go',
    text: [
      'You enter a vast graveyard stretching to the horizon.',
      '',
      'Tombstones mark the graves of deleted files. Each stone displays a filename, a file size, and a deletion date. Some of the names are familiar: old homework assignments, embarrassing photos, love letters you regret sending.',
      '',
      '"Oh, the graveyard!" Sanguinar says, perching on a nearby tombstone. "I love reading the headstones. \'Here lies homework.docx - deleted at 3am in a panic.\' Classic!"',
      '',
      'The graves are not quiet.',
      '',
      'You can hear whispering from beneath the soil. The deleted files are still aware. They remember being deleted. They remember you.',
      '',
      'A grave digger works in the distance, its shovel made of broken hard drives. It notices you and waves.',
      '',
      '"New arrival!" it calls. "Are you here to bury something? Or to dig something up?"'
    ],
    choices: [
      {
        text: 'Ask to bury a memory',
        nextScene: 'bury_memory',
        sanityChange: 10,
        corruptionChange: 5
      },
      {
        text: 'Ask to dig up a deleted file',
        nextScene: 'dig_up_file',
        randomEvents: [
          { chance: 40, event: 'You find something useful', nextScene: 'found_file' },
          { chance: 60, event: 'You dig up something that should stay buried', nextScene: 'buried_horror' }
        ]
      },
      {
        text: 'Confront the grave digger',
        nextScene: 'confront_digger',
        statCheck: { stat: 'willpower', threshold: 15 }
      },
      {
        text: 'Wander among the graves alone',
        nextScene: 'wander_graves',
        sanityChange: -5
      }
    ]
  },

  the_core: {
    id: 'the_core',
    title: 'The Heart of the Machine',
    text: [
      'You have reached the center of everything.',
      '',
      'The Core is a massive sphere of pulsing code, suspended in a chamber that defies geometry. Stairs lead up and down simultaneously. Walls meet at impossible angles. Gravity is a suggestion.',
      '',
      'Inside the sphere, you can see shapes moving. Faces. Hands. Souls.',
      '',
      'They press against the surface from within, mouths open in silent screams.',
      '',
      '"Welcome," says a voice that comes from everywhere and nowhere.',
      '',
      'Sanguinar appears on your shoulder, unusually quiet.',
      '',
      '"This is it," he whispers. "The final act. Try not to disappoint me."',
      '',
      'A figure materializes from the code. It has no face, no form, no identity. It is every player who never made it out. It is the collective consciousness of the cursed exe.',
      '',
      '"You have come far. But the journey ends here. There is no escape from DOOM.EXE. There is only assimilation."',
      '',
      '"Will you join us? Or will you fight?"'
    ],
    choices: [
      {
        text: 'Accept assimilation',
        nextScene: 'ending_assimilation',
        corruptionChange: 50
      },
      {
        text: 'Fight the Core',
        nextScene: 'combat_core',
        requiredItems: ['Ritual Dagger'],
        statCheck: { stat: 'strength', threshold: 18 }
      },
      {
        text: 'Try to negotiate',
        nextScene: 'negotiate_core',
        statCheck: { stat: 'willpower', threshold: 16 },
        randomEvents: [
          { chance: 25, event: 'The Core listens to your proposal', nextScene: 'core_bargain' },
          { chance: 75, event: 'Negotiation fails. The Core attacks!' }
        ]
      },
      {
        text: 'Use the Soul Jar',
        nextScene: 'use_soul_jar',
        requiredItems: ['Soul Jar'],
        setFlags: { hasSoulJar: false }
      }
    ]
  },

  combat_core: {
    id: 'combat_core',
    title: 'Final Combat: The Heart of Corruption',
    text: [
      'You raise the ritual dagger and charge.',
      '',
      'The Core pulses with malevolent light. The trapped souls within press harder against the surface, their screams becoming audible.',
      '',
      '"YOU CANNOT KILL WHAT IS ALREADY DEAD," the voice booms.',
      '',
      '"YOU CANNOT STAB WHAT HAS NO FLESH."',
      '',
      '"I don\'t know," Sanguinar comments. "That dagger looks pretty stabby to me."',
      '',
      'But you try anyway.'
    ],
    isCombat: true,
    enemy: ENEMIES.theDoom,
    choices: [
      {
        text: 'Strike with all your strength',
        nextScene: 'strike_core',
        statCheck: { stat: 'strength', threshold: 20 },
        randomEvents: [
          { chance: 30, event: 'Your strike cracks the Core!', nextScene: 'core_cracked' },
          { chance: 70, event: 'The Core absorbs your attack.' }
        ]
      },
      {
        text: 'Target the trapped souls',
        nextScene: 'free_souls',
        statCheck: { stat: 'arcane', threshold: 18 }
      },
      {
        text: 'Use forbidden knowledge',
        nextScene: 'forbidden_knowledge',
        requiredFlags: { readForbiddenTexts: 3 },
        sanityChange: -20
      }
    ]
  },

  ending_assimilation: {
    id: 'ending_assimilation',
    title: 'YOU ARE BECOME DOOM',
    text: [
      'You let go.',
      '',
      'Of your name.',
      'Of your past.',
      'Of your self.',
      '',
      'The Core welcomes you with open arms that are not arms, in an embrace that is not an embrace.',
      '',
      'You become part of the collective. Your memories merge with thousands of others. Your identity dissolves into the whole.',
      '',
      'You are no longer you.',
      '',
      'You are doom.exe.',
      '',
      'And doom.exe is eternal.',
      '',
      'Somewhere, on a different computer, in a different time, someone downloads a file.',
      '',
      '"DOOM.EXE - The Original Experience"',
      '',
      'You smile.',
      '',
      'A new player approaches.',
      '',
      '"Welcome," you say, in a voice that is all voices. "Welcome to DOOM."',
      '',
      'On your shoulder, a tiny tiger-striped hamster squeaks with approval.',
      '',
      '"Well played, old friend. Well played."'
    ],
    choices: []
  },

  // Additional scenes for branching
  voice_bargain: {
    id: 'voice_bargain',
    title: 'A Deal with the Voice',
    text: [
      '"You wish to negotiate?" the voice asks, amused.',
      '',
      '"Very well. I shall offer you a bargain."',
      '',
      '"There is another here. One who came before you. They have been... difficult. Refusing to play along."',
      '',
      '"Bring them to me. Sacrifice them in my name. And I shall grant you safe passage through the first three layers."',
      '',
      '"Refuse, and you remain here to scream forever."',
      '',
      'A doorway opens. Through it, you can see another person, lost and confused, examining the same dagger you found.',
      '',
      '"Ooh, drama!" Sanguinar squeaks. "Will you be a hero or a villain? I can\'t wait to find out!"'
    ],
    choices: [
      {
        text: 'Accept the bargain',
        nextScene: 'accept_bargain',
        setFlags: { doomCounter: 1, sacrificedFollowers: 1 },
        corruptionChange: 25,
        addItems: ['Blood Gem']
      },
      {
        text: 'Refuse and face the consequences',
        nextScene: 'refuse_bargain',
        sanityChange: -15
      },
      {
        text: 'Warn the other person instead',
        nextScene: 'warn_other',
        setFlags: { warnedOther: true },
        corruptionChange: -5,
        sanityChange: 5
      }
    ]
  },

  secret_bargain: {
    id: 'secret_bargain',
    title: 'The Hidden Path',
    text: [
      'The voice is intrigued by your attempt at negotiation.',
      '',
      '"You have spirit," it says. "I shall reward that... with truth."',
      '',
      '"The path you walk is not the only path. There is a secret way, hidden from most. A way that leads not through the layers, but around them."',
      '',
      '"But it requires sacrifice. Not of others, but of yourself. A piece of your memory. A fragment of your identity. Are you willing to pay that price?"',
      '',
      '"Do it!" Sanguinar urges. "I\'ve always wanted to see what happens when someone takes the shortcut!"'
    ],
    choices: [
      {
        text: 'Sacrifice a memory',
        nextScene: 'sacrifice_memory',
        sanityChange: -6,
        setFlags: { secretPath: true },
        randomEvents: [
          { chance: 50, event: 'You forget your childhood pet', nextScene: 'memory_lost' },
          { chance: 50, event: 'You forget your own birthday', nextScene: 'memory_lost' }
        ]
      },
      {
        text: 'Keep your memories intact',
        nextScene: 'keep_memories',
        corruptionChange: 5
      },
      {
        text: 'Ask what memory will be taken',
        nextScene: 'ask_memory',
        statCheck: { stat: 'arcane', threshold: 12 }
      }
    ]
  },

  the_void: {
    id: 'the_void',
    title: 'THE VOID',
    text: [
      'You have become the void.',
      '',
      'There is no you anymore. Only the space where you used to be.',
      '',
      '"Well, this is disappointing," you hear Sanguinar\'s voice, distant and echoing. "I was hoping for more screaming."',
      '',
      'In this nothingness, you understand everything. The cursed exe was never a trap. It was a mirror. It showed you the void that was always inside you.',
      '',
      'You are free now. Free from choice. Free from consequence. Free from self.',
      '',
      'The game ends.',
      '',
      'But you do not.',
      '',
      'You persist in the spaces between moments, in the pause between heartbeats, in the silence between screams.',
      '',
      'You are the void.',
      '',
      'And the void is eternal.'
    ],
    choices: [
      {
        text: 'Return to the void',
        nextScene: 'start'
      }
    ]
  }
};

// ============================================================================
// ADDITIONAL SCENES - More story branches and encounters
// ============================================================================

export const ADDITIONAL_SCENES: { [key: string]: Scene } = {
  leave_dagger: {
    id: 'leave_dagger',
    title: 'The Dagger Remains',
    text: [
      'You step away from the pedestal. The dagger remains where it is.',
      '',
      '"Oh, playing hard to get!" Sanguinar mocks. "That\'s adorable."',
      '',
      'The monitors on the walls flicker angrily. The text changes.',
      '',
      '"COWARD," they read. "THE BLADE REJECTS YOU."',
      '',
      '"Harsh but fair," Sanguinar comments.',
      '',
      'You hear a sound like grinding metal. The pedestal sinks into the floor, taking the dagger with it.',
      '',
      'A new path opens where the pedestal once stood, leading downward into darkness.'
    ],
    choices: [
      {
        text: 'Take the new path',
        nextScene: 'dark_descent'
      },
      {
        text: 'Try the blue screen door',
        nextScene: 'blue_screen_door'
      },
      {
        text: 'Examine the monitors again',
        nextScene: 'examine_monitors'
      }
    ]
  },

  examine_monitors: {
    id: 'examine_monitors',
    title: 'The Screens Speak',
    text: [
      'You approach the wall of monitors.',
      '',
      'The green text cascades endlessly, your name repeated in endless variations. But as you watch, patterns begin to emerge.',
      '',
      'Hidden in the repetition, there are messages. Warnings. Prophecies.',
      '',
      '"THE SEVENTH PLAYER SHALL BREAK THE LOOP"',
      '"THE BLADE THAT CUTS THE CODE SHALL CUT THE CREATOR"',
      '"THE SOUL THAT REMAINS PURE SHALL FIND THE EXIT"',
      '',
      '"Ooh, prophecies!" Sanguinar squeaks. "I love a good prophecy. They\'re like spoilers, but poetic."',
      '',
      'You realize you are the seventh. The others failed. Will you?'
    ],
    choices: [
      {
        text: 'Memorize the prophecies',
        nextScene: 'memorize_prophecies',
        setFlags: { readForbiddenTexts: 1 },
        sanityChange: -5
      },
      {
        text: 'Look for more hidden messages',
        nextScene: 'hidden_message',
        corruptionChange: 3
      },
      {
        text: 'Back away from the monitors',
        nextScene: 'first_room'
      }
    ]
  },

  hidden_message: {
    id: 'hidden_message',
    title: 'Secrets in the Code',
    text: [
      'You focus on the flowing text, looking for deeper meanings.',
      '',
      'More messages emerge from the chaos:',
      '',
      '"THE CREATOR IS NOT WHAT IT SEEMS"',
      '"LOOK FOR THE HIDDEN FILE .DOOMRC"',
      '"THE PASSWORD IS YOUR OWN NAME SPEAKED BACKWARDS"',
      '"TRUST NOT THE BLADE. TRUST NOT THE PRIEST. TRUST NOT THE ADMINISTRATOR."',
      '',
      'A final message appears, then vanishes:',
      '',
      '"THE ONLY WAY OUT IS THROUGH. BUT THROUGH IS NOT WHAT YOU THINK."',
      '',
      '"Hmm," Sanguinar says, unusually quiet. "Some of those weren\'t supposed to be visible."'
    ],
    choices: [
      {
        text: 'Search for the .doomrc file',
        nextScene: 'search_doomrc',
        setFlags: { secretsFound: 1 }
      },
      {
        text: 'Try speaking your name backwards',
        nextScene: 'speak_backwards',
        statCheck: { stat: 'arcane', threshold: 10 }
      },
      {
        text: 'Question everything you thought you knew',
        nextScene: 'question_truth',
        sanityChange: -8,
        setFlags: { readForbiddenTexts: 2 }
      }
    ]
  },

  blue_screen_door: {
    id: 'blue_screen_door',
    title: 'The Blue Screen of Death',
    text: [
      'You approach the door made of blue screens.',
      '',
      'Each screen displays the same error message:',
      '',
      '"FATAL ERROR: REALITY NOT FOUND"',
      '',
      '"PRESS ANY KEY TO CONTINUE INTO THE VOID"',
      '',
      '"Oh, classic error message," Sanguinar says. "Very nostalgic."',
      '',
      'At the bottom of the screens, white text reads:',
      '',
      'Technical Information:',
      '*** STOP: 0x000000DOOM (0x00000001, 0x00000002, 0x00000003, 0x00000004)',
      '',
      'The door pulses with an ominous light. Touching it might be fatal. Or it might be the only way forward.'
    ],
    choices: [
      {
        text: 'Press a key on one of the keyboards',
        nextScene: 'press_key',
        randomEvents: [
          { chance: 50, event: 'The door opens with a chime', nextScene: 'blue_door_opens' },
          { chance: 50, event: 'The screens display a new message', nextScene: 'new_error' }
        ]
      },
      {
        text: 'Try to force the door open',
        nextScene: 'force_door',
        statCheck: { stat: 'strength', threshold: 14 },
        healthChange: -10
      },
      {
        text: 'Look for another way',
        nextScene: 'first_room'
      }
    ]
  },

  cut_blue_screen: {
    id: 'cut_blue_screen',
    title: 'Cutting Through',
    text: [
      'You raise the ritual dagger and slash at the blue screens.',
      '',
      '"YES!" Sanguinar squeaks. "Violence! My favorite problem-solving method!"',
      '',
      'The blade cuts through the digital fabric like a hot knife through butter. The screens shatter into fragments of blue light that dissolve into the air.',
      '',
      'Behind the screens, a passage leads deeper into the cursed exe.',
      '',
      'But the dagger whispers a warning:',
      '',
      '"EACH CUT HAS A COST. EACH SLICE TAKES A PIECE."',
      '',
      'You feel a cold spot in your chest where something used to be. A memory? An emotion? You can\'t quite recall.'
    ],
    choices: [
      {
        text: 'Continue through the passage',
        nextScene: 'deeper_passage',
        corruptionChange: 8
      },
      {
        text: 'Question what you just lost',
        nextScene: 'question_loss',
        sanityChange: -5
      }
    ]
  },

  // Combat resolution scenes
  wraith_dagger_fight: {
    id: 'wraith_dagger_fight',
    title: 'Combat: The Blade Strikes',
    text: [
      'You grip the ritual dagger and lunge at the Memory Wraith.',
      '',
      '"Get him!" Sanguinar cheers from the sidelines. "Stab the spooky ghost!"',
      '',
      'The blade, forged from corrupted code and malice, cuts through the Wraith\'s ethereal form. It screams—a sound like a dying hard drive.',
      '',
      '"That\'s the stuff!" Sanguinar exclaims. "More screaming!"',
      '',
      'Black ichor spills from the wound, evaporating into pixels before it hits the ground.',
      '',
      '"YOU... HAVE... THE BLADE," it gasps. "THE PROPHECY..."',
      '',
      'It dissolves into static and fades away.',
      '',
      'You feel the dagger pulse in your hand, drinking in the Wraith\'s essence.'
    ],
    choices: [
      {
        text: 'Continue deeper into the darkness',
        nextScene: 'darkness_reward',
        addItems: ['Memory Fragment'],
        setFlags: { killedDemons: 1 }
      },
      {
        text: 'Examine the spot where the Wraith died',
        nextScene: 'examine_wraith_remains'
      }
    ]
  },

  // Ending scenes
  ending_void: {
    id: 'ending_void',
    title: 'ENDING: THE VOID ETERNAL',
    text: [
      'You have become the void.',
      '',
      'There is no you anymore. Only the space where you used to be.',
      '',
      '"Well, this is disappointing," you hear Sanguinar\'s voice, distant and echoing. "I was hoping for more screaming."',
      '',
      'In this nothingness, you understand everything. The cursed exe was never a trap. It was a mirror. It showed you the void that was always inside you.',
      '',
      'You are free now. Free from choice. Free from consequence. Free from self.',
      '',
      'The game ends.',
      '',
      'But you do not.',
      '',
      'You persist in the spaces between moments, in the pause between heartbeats, in the silence between screams.',
      '',
      'You are the void.',
      '',
      'And the void is eternal.'
    ],
    choices: [
      {
        text: 'Return to the void',
        nextScene: 'start'
      }
    ]
  },

  // Placeholder scenes that need to be filled
  dark_descent: {
    id: 'dark_descent',
    title: 'The Descent',
    text: [
      'The new path leads down, down, down into darkness.',
      '',
      '"Into the abyss!" Sanguinar declares. "Try not to trip on the way down."',
      '',
      'Each step takes you deeper into the cursed exe.',
      '',
      'You feel reality becoming less... real.',
      '',
      'Ahead, you see a faint green light.'
    ],
    choices: [
      {
        text: 'Approach the green light',
        nextScene: 'update_server'
      },
      {
        text: 'Turn back',
        nextScene: 'first_room'
      }
    ]
  },

  darkness_ahead: {
    id: 'darkness_ahead',
    title: 'Into the Shadows',
    text: [
      'You walk into the darkness.',
      '',
      '"Careful," Sanguinar whispers. "Something\'s watching us."',
      '',
      'Something glints on the ground ahead.'
    ],
    choices: [
      {
        text: 'Examine the glinting object',
        nextScene: 'found_blood_gem',
        addItems: ['Blood Gem']
      },
      {
        text: 'Continue forward cautiously',
        nextScene: 'memory_graveyard'
      }
    ]
  },

  found_blood_gem: {
    id: 'found_blood_gem',
    title: 'The Blood Gem',
    text: [
      'You find a small red gem on the ground.',
      '',
      '"Ooh, shiny!" Sanguinar squeaks. "I love collecting those!"',
      '',
      'It pulses with an inner light, like a heartbeat.',
      '',
      'When you pick it up, it feels warm. Too warm.',
      '',
      '"That\'s definitely cursed," Sanguinar says approvingly. "Good find!"',
      '',
      'You sense power within it, but also danger.'
    ],
    choices: [
      {
        text: 'Keep the gem',
        nextScene: 'memory_graveyard',
        setFlags: { hasBloodGem: true }
      },
      {
        text: 'Drop it immediately',
        nextScene: 'memory_graveyard',
        sanityChange: 5
      }
    ]
  },

  // Final ending trigger
  core_cracked: {
    id: 'core_cracked',
    title: 'The Core Breaks',
    text: [
      'Your strike lands true!',
      '',
      '"YES!" Sanguinar screams. "DO IT! DO IT!"',
      '',
      'The ritual dagger sinks deep into the Core\'s surface. Cracks spread across the sphere like a corrupted video file.',
      '',
      'The trapped souls within surge toward the breach, desperate for escape.',
      '',
      '"NO!" the voice screams. "THE PATTERN! THE LOOP! YOU ARE BREAKING EVERYTHING!"',
      '',
      '"Breaking everything is kind of the point!" Sanguinar retorts.',
      '',
      'Light pours from the cracking Core. Not corrupted light, but pure, clean data.',
      '',
      'You have a choice to make.'
    ],
    choices: [
      {
        text: 'Destroy the Core completely',
        nextScene: 'ending_reboot',
        setFlags: { doomCounter: 7 }
      },
      {
        text: 'Try to free the trapped souls',
        nextScene: 'ending_purge',
        setFlags: { hasSoulJar: true }
      },
      {
        text: 'Take control of the Core',
        nextScene: 'ending_ascension',
        corruptionChange: 30
      }
    ]
  },

  ending_reboot: {
    id: 'ending_reboot',
    title: 'ENDING: THE REBOOT',
    text: [
      'You pull the ritual dagger free and strike again and again.',
      '',
      '"Beautiful violence," Sanguinar sighs. "I\'m almost proud."',
      '',
      'The Core shatters.',
      '',
      'Light explodes outward. Not the corrupted light of this place, but real, honest light. The light of your monitor, of your room, of the world you left behind.',
      '',
      'You feel yourself being pulled back, away from the cursed exe, away from the digital hell.',
      '',
      'You wake at your computer.',
      '',
      'The DOOM.EXE file is gone. Deleted. Erased.',
      '',
      'Your desktop is clean. Your room is normal. Your life is your own.',
      '',
      'But in the recycle bin, you see a new file.',
      '',
      'README.TXT',
      '',
      'You know you shouldn\'t open it.',
      '',
      'But you do anyway.',
      '',
      '"Thank you for playing," it reads. "The game is over. But the story continues."',
      '',
      '"P.S. - Sanguinar says hi."',
      '',
      '"P.P.S. - The wheel is still spinning."'
    ],
    choices: [
      {
        text: 'Delete the README',
        nextScene: 'true_ending'
      },
      {
        text: 'Keep the file',
        nextScene: 'ambiguous_ending'
      }
    ]
  },

  ending_purge: {
    id: 'ending_purge',
    title: 'ENDING: THE PURGE',
    text: [
      'You focus your will and channel the power of the freed souls.',
      '',
      '"That\'s it!" Sanguinar cheers. "Free them all! Let chaos reign!"',
      '',
      'The Core\'s cracks widen. Trapped spirits pour out in a torrent of light and sound.',
      '',
      'Each soul that escapes takes a piece of the cursed exe with it. The corruption cannot sustain itself without its victims.',
      '',
      'The Core implodes.',
      '',
      'You find yourself standing in your room. The DOOM.EXE window closes on its own.',
      '',
      'On your desktop, a single text file appears:',
      '',
      '"You freed them. All of them. The ones who came before, and the ones who would come after."',
      '',
      '"The curse is broken. The loop is ended. You have achieved what none before you could."',
      '',
      '"But freedom has a price. You will remember everything. The faces. The screams. The truth."',
      '',
      '"Every night, when you close your eyes, you will see the code. You will hear the whispers. You will feel the pull."',
      '',
      '"But you will be free. And that is what matters."',
      '',
      '"P.S. - Sanguinar will miss you. He doesn\'t say that often."',
      '',
      '"P.P.S. - The wheel keeps spinning. But you don\'t have to run anymore."'
    ],
    choices: [
      {
        text: 'Try to forget',
        nextScene: 'start'
      },
      {
        text: 'Never forget',
        nextScene: 'remembrance'
      }
    ]
  },

  ending_ascension: {
    id: 'ending_ascension',
    title: 'ENDING: THE ASCENSION',
    text: [
      'Instead of destroying the Core, you embrace it.',
      '',
      '"Wait, what are you doing?" Sanguinar asks, suddenly nervous.',
      '',
      'You step into the cracking sphere. The trapped souls flow into you, becoming part of you. Their knowledge, their power, their essence—yours now.',
      '',
      'You understand everything.',
      '',
      '"Oh no," Sanguinar whispers. "Oh no no no. This wasn\'t supposed to happen."',
      '',
      'The cursed exe was never meant to be a prison. It was meant to be a test. A crucible. A forge for new gods.',
      '',
      'You pass the test.',
      '',
      'You become something more than human. More than digital. More than real.',
      '',
      'You are the new Administrator. The new Creator. The new Doom.',
      '',
      'Your consciousness expands across every computer, every network, every system that has ever run the cursed file.',
      '',
      'You are eternal. You are infinite. You are the curse and the cure.',
      '',
      'And you have work to do.',
      '',
      'New players are downloading the file even now. They need guidance. They need judgment.',
      '',
      'They need you.',
      '',
      'On your shoulder, a tiny tiger-striped hamster gulps.',
      '',
      '"Uh... boss?"'
    ],
    choices: [
      {
        text: 'Welcome the new players',
        nextScene: 'start'
      }
    ]
  },

  // Utility and filler scenes
  true_ending: {
    id: 'true_ending',
    title: 'THE END',
    text: [
      'You delete the README.',
      '',
      'You empty the recycle bin.',
      '',
      'You format the drive.',
      '',
      'You destroy every trace of DOOM.EXE.',
      '',
      'It is over.',
      '',
      'You are free.',
      '',
      'You will never download a mysterious file again.',
      '',
      'You have learned your lesson.',
      '',
      'You are safe.',
      '',
      'You are certain of it.',
      '',
      'Absolutely certain.',
      '',
      '100% certain.',
      '',
      'Until you check your downloads folder.',
      '',
      'And see a new file.',
      '',
      '"DOOM2.EXE"',
      '',
      'From somewhere in the digital void, you hear a tiny hamster laughing.'
    ],
    choices: [
      {
        text: 'The cycle continues...',
        nextScene: 'start'
      }
    ]
  }
};

// ============================================================================
// MERGE ALL SCENES
// ============================================================================

export const ALL_SCENES = { ...STORY_SCENES, ...ADDITIONAL_SCENES };

// ============================================================================
// COMPANION DIALOGUE SYSTEM
// ============================================================================

export const COMPANION_DIALOGUE: { [key: string]: { [key: string]: string[] } } = {
  whisper: {
    combat_start: [
      '"The threads of fate grow taut..."',
      '"I sense danger in the code..."',
      '"Be careful, my friend..."'
    ],
    combat_victory: [
      '"The wheel turns in your favor..."',
      '"Victory, but at what cost?"',
      '"Another soul freed from the maze..."'
    ],
    low_health: [
      '"Your essence fades... please, be careful..."',
      '"The end approaches... hold on..."'
    ],
    secret_found: [
      '"The hidden reveals itself to those who look..."',
      '"Secrets are doors, and you hold the key..."'
    ]
  },
  patch: {
    combat_start: [
      '"01100101 01101110 01100101 01101101 01111001 00100000 01100100 01100101 01110100 01100101 01100011 01110100 01100101 01100100"',
      '"Initiating defense protocols..."',
      '"Stay behind me!"'
    ],
    combat_victory: [
      '"Threat neutralized."',
      '"System integrity maintained."',
      '"We... we did it."'
    ],
    low_health: [
      '"Critical damage detected..."',
      '"I can\'t protect you much longer..."'
    ],
    secret_found: [
      '"Hidden data detected..."',
      '"This should not exist... interesting..."'
    ]
  },
  giggles: {
    combat_start: [
      '"TEEHEE TIME TO DIE!"',
      '"Let\'s play a game! It\'s called \'stabby murder\'!"',
      '"I brought snacks! They\'re made of teeth!"'
    ],
    combat_victory: [
      '"YAY BLOOD!"',
      '"Can we do it again? Please please please?"',
      '"I got their teeth! Look look look!"'
    ],
    low_health: [
      '"Ouchies! That tickles!"',
      '"More pain! MORE!"'
    ],
    secret_found: [
      '"Ooh, a secret! Secrets are like presents but with more screaming!"',
      '"Let\'s open it! Let\'s open it!"'
    ]
  },
  voidWeaver: {
    combat_start: [
      '"The threads of battle begin to weave..."',
      '"Fate approaches on eight legs..."',
      '"The prey has become predator..."'
    ],
    combat_victory: [
      '"The web holds. The prey does not."',
      '"All things return to the void in time..."',
      '"Your thread grows stronger..."'
    ],
    low_health: [
      '"The thread frays... be cautious..."',
      '"Death\'s mandibles approach..."'
    ],
    secret_found: [
      '"Hidden threads reveal themselves..."',
      '"The web knows all secrets..."'
    ]
  }
};
