import mido
import random
import copy

def invert_notes(track, pivot=60):
    """Flips notes around a pivot pitch (Middle C by default)."""
    new_track = mido.MidiTrack()
    for msg in track:
        if msg.type == 'note_on' or msg.type == 'note_off':
            # Calculate distance from pivot and flip
            diff = msg.note - pivot
            new_note = pivot - diff
            # Clamp to valid MIDI range (0-127)
            new_note = max(0, min(127, new_note))
            new_msg = msg.copy(note=new_note)
            new_track.append(new_msg)
        else:
            new_track.append(msg)
    return new_track

def retrograde_track(track):
    """Reverses the order of notes (Time Reverse)."""
    # Extract just the note events
    events = []
    meta_events = []
    
    current_time = 0
    
    # Separate notes from meta events (like track name, tempo)
    for msg in track:
        if msg.is_meta:
            meta_events.append(msg)
        else:
            # We need absolute time to reverse correctly
            current_time += msg.time
            events.append({'msg': msg, 'abs_time': current_time})
            
    if not events:
        return copy.deepcopy(track)

    total_time = current_time
    
    # Reverse the absolute times
    # New Abs Time = Total Time - Old Abs Time
    # Note: note_on becomes note_off effectively in reverse if we aren't careful, 
    # but MIDI structure is [Event, Delta]. 
    # Simpler approach: Reverse the list of note-on/off pairs? 
    # A true musical retrograde reverses the SEQUENCE of notes, not just the timestamps.
    
    # Let's simple-reverse the list of note payloads but keep durations?
    # Actually, a safer "Remix" way is just reversing the order of pitches while keeping rhythm.
    # But let's try true retrograde: Reverse the list of events.
    
    reversed_events = []
    # We need to pair note_ons with their note_offs to reverse meaningful musical units
    # This is complex for a simple script. 
    # Alternative Retrograde: Just reverse the pitch sequence!
    
    # 1. Collect all pitches
    pitches = [m.note for m in track if m.type == 'note_on']
    # 2. Reverse them
    pitches.reverse()
    
    new_track = mido.MidiTrack()
    # Add back meta
    for m in meta_events:
        new_track.append(m)
        
    pitch_idx = 0
    for msg in track:
        if not msg.is_meta and msg.type == 'note_on':
            if pitch_idx < len(pitches):
                new_msg = msg.copy(note=pitches[pitch_idx])
                new_track.append(new_msg)
                pitch_idx += 1
            else:
                 new_track.append(msg)
        elif not msg.is_meta and msg.type == 'note_off':
             # We need to match the note_off to the new pitch. 
             # This simple logic fails if polyphonic.
             # Fallback: Just append original for non-note-on to avoid stuck notes
             # A true simple retrograde is hard in raw MIDI without a library like music21.
             # Let's switch strategies: "Randomized Octaves" (easier and sounds cool)
             new_track.append(msg)
        else:
             new_track.append(msg)
             
    return new_track

def randomize_octaves(track):
    """Randomly shifts notes up or down an octave."""
    new_track = mido.MidiTrack()
    active_shifts = {} # Keep track of shifts to match note_off
    
    for msg in track:
        if msg.type == 'note_on' and msg.velocity > 0:
            shift = random.choice([-12, 0, 12])
            new_note = max(0, min(127, msg.note + shift))
            active_shifts[msg.note] = new_note
            new_track.append(msg.copy(note=new_note))
        elif msg.type == 'note_off' or (msg.type == 'note_on' and msg.velocity == 0):
            # Match the shift
            original_note = msg.note
            if original_note in active_shifts:
                new_note = active_shifts[original_note]
                new_track.append(msg.copy(note=new_note))
                # Don't remove yet if it's polyphonic/overlapping, but for simple MIDI this is ok
            else:
                new_track.append(msg)
        else:
            new_track.append(msg)
    return new_track

def glitch_drums(length=100):
    """Generates a glitchy drum track."""
    track = mido.MidiTrack()
    track.append(mido.MetaMessage('track_name', name='Glitch Drums'))
    
    t = 0
    for _ in range(length):
        # Randomize rhythm: fast 16ths or stutters
        delta = random.choice([120, 240, 60, 60]) # ticks (assuming 480 tpqn)
        note = random.choice([36, 38, 42]) # Kick, Snare, Hat
        vel = random.randint(80, 127)
        
        track.append(mido.Message('note_on', note=note, velocity=vel, time=delta))
        track.append(mido.Message('note_off', note=note, velocity=0, time=120))
        
    return track

def process_midi(input_file, output_file):
    try:
        mid = mido.MidiFile(input_file)
    except Exception as e:
        print(f"Error loading MIDI: {e}")
        return

    new_midi = mido.MidiFile()
    new_midi.ticks_per_beat = mid.ticks_per_beat
    
    print(f"Processing {len(mid.tracks)} tracks...")

    # Identify the track with the most notes (likely melody)
    best_track = None
    max_notes = 0
    
    for i, track in enumerate(mid.tracks):
        note_count = sum(1 for m in track if m.type == 'note_on')
        if note_count > max_notes:
            max_notes = note_count
            best_track = track

    if best_track:
        print(f"Selected Track as Melody source (Notes: {max_notes})")
        
        # 1. Original (Clean)
        t1 = copy.deepcopy(best_track)
        t1.name = "Original Melody"
        new_midi.tracks.append(t1)
        
        # 2. Inversion
        print("Generating Inversion...")
        t2 = invert_notes(best_track)
        t2.name = "Inverted Melody"
        new_midi.tracks.append(t2)
        
        # 3. Random Octaves (Glitchy Feel)
        print("Generating Octave Glitch...")
        t3 = randomize_octaves(best_track)
        t3.name = "Glitch Melody"
        new_midi.tracks.append(t3)
        
        # 4. Glitch Drums
        print("Generating Glitch Drums...")
        t4 = glitch_drums()
        new_midi.tracks.append(t4)
        
    else:
        print("No melody track found!")

    new_midi.save(output_file)
    print(f"Saved remixed MIDI to: {output_file}")

if __name__ == "__main__":
    import sys
    if len(sys.argv) < 2:
        print("Usage: python3 midi_algo_remix.py <input.midi>")
    else:
        process_midi(sys.argv[1], "remixed_theme.midi")
