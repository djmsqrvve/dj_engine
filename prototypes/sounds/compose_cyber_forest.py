import mido

# --- MUSICAL CONSTANTS ---
# MIDI Note Numbers: C3=48, C4=60, C5=72
TICKS_PER_BEAT = 480
SIXTEENTH = TICKS_PER_BEAT // 4
EIGHTH = TICKS_PER_BEAT // 2
QUARTER = TICKS_PER_BEAT

def create_chord(root, quality, octave=3):
    """Returns a list of note numbers for a chord."""
    base = root + (octave * 12)
    if quality == 'min9': # C Eb G Bb D
        return [base, base+3, base+7, base+10, base+14]
    elif quality == 'maj7': # Ab C Eb G
        return [base, base+4, base+7, base+11]
    elif quality == 'dim7': # G B D F
        return [base, base+4, base+7, base+10] # Dom7 actually
    return [base, base+7] # Power chord fallback

def generate_arpeggio(chord_notes, duration_beats):
    """Generates a harp-like arpeggio (The 'Rain' layer)."""
    events = []
    total_ticks = 0
    pattern_len = len(chord_notes)
    
    # Up-Down pattern: 0 1 2 3 4 3 2 1 ...
    # Make it fast (16th notes)
    steps = int(duration_beats * 4) 
    
    for i in range(steps):
        # Ping-pong through the notes
        idx = i % (pattern_len * 2 - 2)
        if idx >= pattern_len:
            idx = (pattern_len * 2 - 2) - idx
            
        note = chord_notes[idx]
        velocity = 60 + (i % 2) * 20 # 60, 80, 60, 80... pulse
        
        events.append(mido.Message('note_on', note=note, velocity=velocity, time=0))
        events.append(mido.Message('note_off', note=note, velocity=0, time=SIXTEENTH))
        
    return events

def generate_melody(chord_notes, scale_notes, duration_beats):
    """Generates a lead melody on top (The 'Spooky' layer)."""
    events = []
    # A simple melodic motif: Long note -> Short run -> Long note
    
    # 1. Target the 5th or 9th of the chord (Emotional notes)
    target_note = chord_notes[-1] + 12 # Top note of chord, up an octave
    
    # Beat 1: Long sustained emotional note
    events.append(mido.Message('note_on', note=target_note, velocity=95, time=0))
    events.append(mido.Message('note_off', note=target_note, velocity=0, time=QUARTER * 3)) # Hold for 3 beats
    
    # Beat 4: A quick run to lead into next bar
    # Find scale notes close to target
    run_start = target_note - 2 # Approx
    events.append(mido.Message('note_on', note=run_start, velocity=80, time=0))
    events.append(mido.Message('note_off', note=run_start, velocity=0, time=EIGHTH))
    
    events.append(mido.Message('note_on', note=run_start-2, velocity=75, time=0))
    events.append(mido.Message('note_off', note=run_start-2, velocity=0, time=EIGHTH))

    return events

def compose():
    mid = mido.MidiFile()
    track = mido.MidiTrack()
    track.name = "Cyber Forest Theme"
    mid.tracks.append(track)
    
    # Sound: Celesta (9) or Harp (47) or Fantasia (89)
    # Let's go with Harp (47) for that fluid water sound
    track.append(mido.Message('program_change', program=46, time=0)) # 46 is Orchestral Harp
    
    # --- PROGRESSION ---
    # C Minor 9 -> Ab Major 7 -> F Minor 9 -> G Dom 7
    # This loop repeats 4 times
    progression = [
        (0, 'min9'),  # C
        (8, 'maj7'),  # Ab (G#)
        (5, 'min9'),  # F
        (7, 'dim7')   # G
    ]
    
    # We need to merge Arpeggio + Melody into a single stream of events
    # This is tricky in a single track because they overlap. 
    # Mido uses "delta time" (time since last event). 
    # To do polyphony easily in Mido, it's safer to use TWO tracks and merge, 
    # OR simpler: just write the arpeggio track for now as it's the core texture.
    # User asked for "One stereo track".
    
    # Let's combine them by building a discrete list of absolute-time events and sorting.
    
    all_events = []
    
    current_beat = 0
    
    for _ in range(4): # 4 loops
        for root, quality in progression:
            chord = create_chord(root, quality, octave=4)
            
            # 1. Arpeggio Events (Background)
            arp_notes = generate_arpeggio(chord, 4) # 4 beats per chord
            t = current_beat * TICKS_PER_BEAT
            for msg in arp_notes:
                # msg.time in generate_arpeggio is duration (delta). 
                # We need to convert to absolute start/stop
                if msg.type == 'note_on':
                    all_events.append({'time': t, 'msg': msg})
                else:
                    t += msg.time # Increment time for next note
                    all_events.append({'time': t, 'msg': msg})

            # 2. Melody Events (Foreground)
            # High melody
            melody_chord = create_chord(root, quality, octave=5) 
            mel_msgs = generate_melody(melody_chord, [], 4)
            
            t_mel = current_beat * TICKS_PER_BEAT
            for msg in mel_msgs:
                if msg.type == 'note_on':
                    all_events.append({'time': t_mel, 'msg': msg})
                else:
                    t_mel += msg.time
                    all_events.append({'time': t_mel, 'msg': msg})
            
            current_beat += 4

    # Sort by absolute time
    all_events.sort(key=lambda x: x['time'])
    
    # Convert back to Delta Time
    last_time = 0
    for e in all_events:
        delta = e['time'] - last_time
        msg = e['msg'].copy(time=delta)
        track.append(msg)
        last_time = e['time']
        
    mid.save('cyber_forest_theme.midi')
    print("Composed 'cyber_forest_theme.midi'")

if __name__ == "__main__":
    compose()
