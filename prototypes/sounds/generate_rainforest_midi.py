import mido
import random

def generate_track():
    track = mido.MidiTrack()
    track.name = "Cyber Rain"
    
    # --- 1. SOUND DESIGN ---
    # Marimba (12) or Vibraphone (11) or Kalimba (108) fit the "Rainforest" vibe best.
    # Let's use Marimba (12) for that woody, plucky sound.
    track.append(mido.Message('program_change', program=12, time=0))
    
    # Heavy Reverb for the "City Distance" feel
    track.append(mido.Message('control_change', control=91, value=90, time=0))
    # Delay/Chorus for "Cyber" texture
    track.append(mido.Message('control_change', control=93, value=70, time=0))

    # --- 2. MUSIC THEORY ---
    # C Minor Pentatonic (C Eb F G Bb) + added 9th (D) for color
    # Base MIDI notes: 48(C3), 51(Eb3), 53(F3), 55(G3), 58(Bb3), 60(C4)...
    scale = [48, 50, 51, 53, 55, 58, 60, 62, 63, 65, 67, 70, 72]
    
    current_note_idx = 6 # Start Middle C
    
    # --- 3. ALGORITHM ---
    # Generate 1 minute of rain
    total_ticks = 0
    max_ticks = 480 * 100 # 100 beats approx
    
    while total_ticks < max_ticks:
        # A. RHYTHM (Raindrops)
        # Instead of a grid, use variable gaps.
        # Short gap (fast drops) vs Long gap (pause in rain)
        delta = random.choice([60, 120, 120, 240, 480]) 
        # Add slight "humanize" jitter
        delta += random.randint(-10, 10)
        delta = max(20, delta)
        
        # B. MELODY (Random Walk)
        # Move up/down by 1 or 2 steps in the scale, or stay.
        step = random.choice([-2, -1, -1, 0, 1, 1, 2])
        current_note_idx += step
        # Clamp to scale range
        current_note_idx = max(0, min(len(scale)-1, current_note_idx))
        
        note = scale[current_note_idx]
        
        # C. DYNAMICS (Wind Swells)
        # Swell velocity based on pitch? Higher pitch = softer (distant chime)?
        # Or random swells. Let's do random swells.
        velocity = random.randint(50, 90)
        
        # Occasional "Thunder" low note?
        if random.random() < 0.05: # 5% chance
            low_note = scale[0] - 12 # Octave down
            track.append(mido.Message('note_on', note=low_note, velocity=100, time=delta))
            track.append(mido.Message('note_off', note=low_note, velocity=0, time=480)) # Long sustain
            total_ticks += 480
        else:
            # Normal raindrop
            track.append(mido.Message('note_on', note=note, velocity=velocity, time=delta))
            track.append(mido.Message('note_off', note=note, velocity=0, time=120)) # Short decay
            total_ticks += delta + 120

    return track

def save_file():
    mid = mido.MidiFile()
    mid.tracks.append(generate_track())
    mid.save('cyber_rain.midi')
    print("Generated 'cyber_rain.midi'")

if __name__ == "__main__":
    save_file()
