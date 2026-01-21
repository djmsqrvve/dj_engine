import { useEffect, useRef, useState } from 'react';
import { useGameEngine } from '@/hooks/useGameEngine';
import { Button } from '@/components/ui/button';
import { ScrollArea } from '@/components/ui/scroll-area';
import { Progress } from '@/components/ui/progress';
import { Badge } from '@/components/ui/badge';
import { Separator } from '@/components/ui/separator';
import { 
  Heart, 
  Brain, 
  Flame, 
  Shield, 
  Sword, 
  Package, 
  Save, 
  RotateCcw,
  Skull,
  Eye,
  Key,
  AlertTriangle,
  Terminal,
  Power,
  Mouse,
  Users,
  Sparkles,
  MessageSquare,
  Volume2,
  VolumeX
} from 'lucide-react';
import { HAMSTER_QUOTES } from '@/data/story';
import './App.css';

function App() {
  const {
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
    saveGame,
    loadGame,
    isChoiceAvailable,
    recruitCompanion
  } = useGameEngine();

  const [glitchText, setGlitchText] = useState('');
  const [showGlitch, setShowGlitch] = useState(false);
  const [showCompanionName, setShowCompanionName] = useState(false);
  const [companionName, setCompanionName] = useState('');
  const [hamsterQuote, setHamsterQuote] = useState<string | null>(null);
  const [showNarrator, setShowNarrator] = useState(true);
  const logRef = useRef<HTMLDivElement>(null);

  // Glitch effect
  useEffect(() => {
    const interval = setInterval(() => {
      if (Math.random() < 0.1) {
        const glitchChars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_+-=[]{}|;:,.<>?`~';
        const randomText = Array.from({ length: 20 }, () => glitchChars[Math.floor(Math.random() * glitchChars.length)]).join('');
        setGlitchText(randomText);
        setShowGlitch(true);
        setTimeout(() => setShowGlitch(false), 100);
      }
    }, 2000);
    return () => clearInterval(interval);
  }, []);

  // Auto-scroll log
  useEffect(() => {
    if (logRef.current) {
      logRef.current.scrollTop = logRef.current.scrollHeight;
    }
  }, [gameState.gameLog]);

  // Random Sanguinar quotes
  useEffect(() => {
    const interval = setInterval(() => {
      if (!gameOver && Math.random() < 0.05) {
        const quoteType = Object.keys(HAMSTER_QUOTES)[Math.floor(Math.random() * Object.keys(HAMSTER_QUOTES).length)];
        const quotes = (HAMSTER_QUOTES as any)[quoteType];
        const randomQuote = quotes[Math.floor(Math.random() * quotes.length)];
        setHamsterQuote(randomQuote);
        setTimeout(() => setHamsterQuote(null), 4000);
      }
    }, 15000);
    return () => clearInterval(interval);
  }, [gameOver]);

  // Format scene text with proper spacing
  const formatSceneText = (text: string[]) => {
    return text.map((line, index) => {
      if (line === '') {
        return <br key={index} />;
      }
      return (
        <p key={index} className="scene-text">
          {line}
        </p>
      );
    });
  };

  // Companion name input
  const handleCompanionNaming = (companionId: string) => {
    if (companionName.trim()) {
      recruitCompanion(companionId, companionName.trim());
      setShowCompanionName(false);
      setCompanionName('');
    }
  };

  // Player stats component
  const PlayerStats = () => (
    <div className="stats-panel">
      <div className="stat-row">
        <div className="stat-item">
          <Heart className="stat-icon health" size={16} />
          <span className="stat-label">HP</span>
          <Progress 
            value={(gameState.player.health / gameState.player.maxHealth) * 100} 
            className="stat-bar health-bar"
          />
          <span className="stat-value">{gameState.player.health}/{gameState.player.maxHealth}</span>
        </div>
        <div className="stat-item">
          <Brain className="stat-icon sanity" size={16} />
          <span className="stat-label">SAN</span>
          <Progress 
            value={(gameState.player.sanity / gameState.player.maxSanity) * 100} 
            className="stat-bar sanity-bar"
          />
          <span className="stat-value">{gameState.player.sanity}/{gameState.player.maxSanity}</span>
        </div>
        <div className="stat-item">
          <Flame className="stat-icon corruption" size={16} />
          <span className="stat-label">COR</span>
          <Progress 
            value={gameState.player.corruption} 
            className="stat-bar corruption-bar"
          />
          <span className="stat-value">{gameState.player.corruption}%</span>
        </div>
      </div>
      
      <Separator className="my-2" />
      
      <div className="stat-row">
        <div className="stat-item compact">
          <Sword className="stat-icon" size={14} />
          <span className="stat-label">STR</span>
          <span className="stat-value">{gameState.player.stats.strength}</span>
        </div>
        <div className="stat-item compact">
          <Shield className="stat-icon" size={14} />
          <span className="stat-label">WIL</span>
          <span className="stat-value">{gameState.player.stats.willpower}</span>
        </div>
        <div className="stat-item compact">
          <Eye className="stat-icon" size={14} />
          <span className="stat-label">ARC</span>
          <span className="stat-value">{gameState.player.stats.arcane}</span>
        </div>
      </div>

      <Separator className="my-2" />

      <div className="inventory">
        <div className="inventory-header">
          <Package size={14} />
          <span>Inventory ({gameState.player.inventory.length})</span>
        </div>
        <div className="inventory-items">
          {gameState.player.inventory.length === 0 ? (
            <span className="empty-inventory">Empty</span>
          ) : (
            gameState.player.inventory.map((item, index) => (
              <Badge key={index} variant="outline" className="inventory-item">
                {item}
              </Badge>
            ))
          )}
        </div>
      </div>

      <Separator className="my-2" />

      <div className="flags">
        {gameState.player.flags.hasRitualDagger && (
          <Badge variant="destructive" className="flag-badge">
            <Key size={12} /> Dagger
          </Badge>
        )}
        {gameState.player.flags.hasBloodGem && (
          <Badge variant="destructive" className="flag-badge">
            <Flame size={12} /> Blood Gem
          </Badge>
        )}
        {gameState.player.flags.hasEldritchTome && (
          <Badge variant="destructive" className="flag-badge">
            <Skull size={12} /> Tome
          </Badge>
        )}
        {gameState.player.flags.hasSoulJar && (
          <Badge variant="destructive" className="flag-badge">
            <AlertTriangle size={12} /> Soul Jar
          </Badge>
        )}
        {gameState.player.flags.realityTouched && (
          <Badge variant="outline" className="flag-badge">
            <Sparkles size={12} /> Reality-Touched
          </Badge>
        )}
      </div>
    </div>
  );

  // Companion UI
  const CompanionPanel = () => {
    if (!activeCompanion) return null;

    return (
      <div className="companion-panel">
        <div className="companion-header">
          <Users size={14} />
          <span>{activeCompanion.name} ({activeCompanion.species})</span>
        </div>
        <div className="companion-stats">
          <div className="companion-loyalty">
            <span>Loyalty: {activeCompanion.loyalty}/100</span>
            <Progress value={activeCompanion.loyalty} className="loyalty-bar" />
          </div>
          <div className="companion-corruption">
            <span>Corruption: {activeCompanion.corruption}%</span>
          </div>
        </div>
        <div className="companion-thought">
          <MessageSquare size={12} />
          <span>{activeCompanion.currentThought}</span>
        </div>
        <div className="companion-ability">
          <Sparkles size={12} />
          <span>{activeCompanion.specialAbility}</span>
        </div>
      </div>
    );
  };

  // Combat UI
  const CombatUI = () => {
    if (!currentEnemy || !isCombat) return null;

    const enemyHealthPercent = (enemyHealth / currentEnemy.maxHealth) * 100;

    return (
      <div className="combat-overlay">
        <div className="combat-panel">
          <div className="enemy-info">
            <h3 className="enemy-name">{currentEnemy.name}</h3>
            <p className="enemy-description">{currentEnemy.description}</p>
            <div className="enemy-health">
              <Progress value={enemyHealthPercent} className="enemy-health-bar" />
              <span>{enemyHealth}/{currentEnemy.maxHealth}</span>
            </div>
          </div>

          <div className="combat-log">
            {combatLog.map((log, index) => (
              <p key={index} className="combat-log-entry">
                {log}
              </p>
            ))}
          </div>

          <div className="combat-actions">
            <Button 
              onClick={combatAttack}
              className="combat-btn attack-btn"
              variant="destructive"
            >
              <Sword size={16} /> Attack
            </Button>
            <Button 
              onClick={combatFlee}
              className="combat-btn flee-btn"
              variant="outline"
            >
              Flee
            </Button>
          </div>
        </div>
      </div>
    );
  };

  // Ending screen
  const EndingScreen = () => {
    if (!currentEnding || !gameOver) return null;

    return (
      <div className="ending-overlay">
        <div className="ending-panel">
          <div className="ending-header">
            <Skull className="ending-icon" size={48} />
            <h1 className="ending-title">{currentEnding.title}</h1>
          </div>
          <p className="ending-description">{currentEnding.description}</p>
          {currentEnding.unlocksMessage && (
            <p className="ending-message">{currentEnding.unlocksMessage}</p>
          )}
          <div className="ending-stats">
            <div className="stat-display">
              <span>Layers Explored: {gameState.visitedScenes.length}</span>
            </div>
            <div className="stat-display">
              <span>Secrets Found: {gameState.player.flags.secretsFound}</span>
            </div>
            <div className="stat-display">
              <span>Final Corruption: {gameState.player.corruption}%</span>
            </div>
          </div>
          <div className="ending-actions">
            <Button onClick={resetGame} className="ending-btn">
              <RotateCcw size={16} /> Play Again
            </Button>
          </div>
        </div>
      </div>
    );
  };

  // Companion naming modal
  const CompanionNamingModal = () => {
    if (!showCompanionName) return null;

    return (
      <div className="naming-overlay">
        <div className="naming-panel">
          <h3>Name Your Companion</h3>
          <input
            type="text"
            value={companionName}
            onChange={(e) => setCompanionName(e.target.value)}
            placeholder="Enter a name..."
            maxLength={20}
            className="naming-input"
            autoFocus
          />
          <div className="naming-actions">
            <Button 
              onClick={() => handleCompanionNaming('whisper')}
              disabled={!companionName.trim()}
            >
              Confirm
            </Button>
            <Button 
              onClick={() => setShowCompanionName(false)}
              variant="outline"
            >
              Cancel
            </Button>
          </div>
        </div>
      </div>
    );
  };

  // Sanguinar narrator quote
  const SanguinarQuote = () => {
    if (!hamsterQuote) return null;

    return (
      <div className="sanguinar-quote">
        <div className="quote-icon">
          <Mouse size={20} />
        </div>
        <div className="quote-text">
          {hamsterQuote}
        </div>
      </div>
    );
  };

  return (
    <div className="app">
      {/* Glitch overlay */}
      {showGlitch && (
        <div className="glitch-overlay">
          <span className="glitch-text">{glitchText}</span>
        </div>
      )}

      {/* Header */}
      <header className="app-header">
        <div className="header-content">
          <Terminal className="header-icon" size={24} />
          <h1 className="app-title">DOOM.EXE</h1>
          <div className="header-subtitle">
            <span className="version">v6.66</span>
            <span className="status">RUNNING</span>
            <Badge className="hamster-badge">
              <Mouse size={12} /> Sanguinar
            </Badge>
          </div>
        </div>
        <div className="header-actions">
          <Button 
            onClick={() => setShowNarrator(!showNarrator)}
            size="sm" 
            variant="ghost" 
            className="header-btn"
            title={showNarrator ? 'Mute Sanguinar' : 'Unmute Sanguinar'}
          >
            {showNarrator ? <Volume2 size={14} /> : <VolumeX size={14} />}
          </Button>
          <Button 
            onClick={saveGame} 
            size="sm" 
            variant="ghost" 
            className="header-btn"
          >
            <Save size={14} />
          </Button>
          <Button 
            onClick={loadGame} 
            size="sm" 
            variant="ghost" 
            className="header-btn"
          >
            Load
          </Button>
          <Button 
            onClick={resetGame} 
            size="sm" 
            variant="ghost" 
            className="header-btn danger"
          >
            <Power size={14} />
          </Button>
        </div>
      </header>

      <div className="app-content">
        {/* Left panel - Stats & Companion */}
        <aside className="left-panel">
          <PlayerStats />
          <Separator className="my-2" />
          <CompanionPanel />
          
          {/* Companion recruitment buttons */}
          {!activeCompanion && gameState.visitedScenes.length > 3 && (
            <div className="companion-recruitment">
              <h4 className="recruitment-title">Find Companion</h4>
              <Button 
                onClick={() => setShowCompanionName(true)}
                className="recruit-btn"
                size="sm"
              >
                <Users size={14} /> Summon Whisper
              </Button>
            </div>
          )}
        </aside>

        {/* Center panel - Main game */}
        <main className="main-panel">
          {currentScene && (
            <div className="scene">
              <div className="scene-header">
                <h2 className="scene-title">{currentScene.title}</h2>
                <Badge variant="outline" className="scene-id">
                  Layer: {gameState.visitedScenes.length}
                </Badge>
              </div>

              <ScrollArea className="scene-text-container">
                <div className="scene-content">
                  {formatSceneText(currentScene.text)}
                </div>
              </ScrollArea>

              {!isCombat && !gameOver && (
                <div className="choices">
                  {currentScene.choices.map((choice, index) => {
                    const available = isChoiceAvailable(choice, gameState.player);
                    return (
                      <Button
                        key={index}
                        onClick={() => makeChoice(index)}
                        disabled={!available}
                        className={`choice-btn ${!available ? 'disabled' : ''}`}
                        variant={available ? 'default' : 'ghost'}
                      >
                        {choice.text}
                        {choice.requiredItems && (
                          <span className="choice-requirement">
                            Requires: {choice.requiredItems.join(', ')}
                          </span>
                        )}
                        {choice.statCheck && (
                          <span className="choice-requirement">
                            {choice.statCheck.stat.toUpperCase()} {choice.statCheck.threshold}+
                          </span>
                        )}
                      </Button>
                    );
                  })}
                </div>
              )}
            </div>
          )}
        </main>

        {/* Right panel - Game log & Narrator */}
        <aside className="right-panel">
          {/* Sanguinar Quote Display */}
          {showNarrator && <SanguinarQuote />}
          
          <div className="log-header">
            <Terminal size={16} />
            <span>System Log</span>
          </div>
          <ScrollArea className="log-container">
            <div ref={logRef} className="log-content">
              {gameState.gameLog.map((log, index) => (
                <p key={index} className={`log-entry ${log.includes('Sanguinar') ? 'hamster-entry' : ''}`}>
                  <span className="log-timestamp">[{new Date().toLocaleTimeString()}]</span>
                  <span className="log-text">{log}</span>
                </p>
              ))}
            </div>
          </ScrollArea>

          {/* Secrets Discovered */}
          {gameState.discoveredSecrets.length > 0 && (
            <div className="secrets-panel">
              <h4 className="secrets-title">
                <Key size={14} /> Secrets Found ({gameState.discoveredSecrets.length})
              </h4>
              {gameState.discoveredSecrets.map((secret, index) => (
                <Badge key={index} className="secret-badge">
                  {secret}
                </Badge>
              ))}
            </div>
          )}
        </aside>
      </div>

      {/* Combat overlay */}
      <CombatUI />

      {/* Ending overlay */}
      <EndingScreen />

      {/* Companion naming modal */}
      <CompanionNamingModal />

      {/* Footer */}
      <footer className="app-footer">
        <div className="footer-content">
          <span className="footer-text">
            DOOM.EXE © 1993-{new Date().getFullYear()} doom. All rights reserved.
          </span>
          <span className="footer-text glitchy">
            "This program is not responsible for any corruption, digital or otherwise." - Sanguinar
          </span>
        </div>
      </footer>
    </div>
  );
}

export default App;
