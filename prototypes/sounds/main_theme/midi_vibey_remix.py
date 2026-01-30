import mido
import random
import copy

def make_vibey(track):
    new_track = mido.MidiTrack()
    
    # --- 1. SETUP THE VIBE (Program Changes) ---
    # Set Instrument to Electric Piano 1 (General MIDI #5 -> index 4)
    # This gives that soft, warm Rhodes sound.
    new_track.append(mido.Message('program_change', program=4, time=0))
    
    # Add Reverb (Control Change 91). Value 90/127 (High Reverb)
    new_track.append(mido.Message('control_change', control=91, value=100, time=0))
    
    # Add Chorus (Control Change 93). Value 60/127 (Moderate Chorus for width)
    new_track.append(mido.Message('control_change', control=93, value=60, time=0))

    # --- 2. PROCESS NOTES ---
    for msg in track:
        if msg.type == 'note_on' or msg.type == 'note_off':
            # Create a copy to modify
            new_msg = msg.copy()
            
            # A. TRANSPOSITION (Darker)
            # Shift down 2 semitones
            new_msg.note = max(0, min(127, new_msg.note - 2))
            
            # B. TIME STRETCH (Slower, lazier)
            # Slow down by 20%
            new_msg.time = int(new_msg.time * 1.3)
            
            # C. SOFTEN VELOCITY
            # If it's a note ON (velocity > 0), make it softer
            if msg.type == 'note_on' and msg.velocity > 0:
                # Range 40-75 is "Soft" to "Mezzo-Piano"
                # Add random fluctuation for human feel
                new_msg.velocity = random.randint(45, 75)
            
            new_track.append(new_msg)
            
        elif msg.is_meta:
            # Keep tempo/names but maybe slow tempo meta events too if we want
            new_track.append(msg)
        else:
            new_track.append(msg)
            
    return new_track

def process_midi(input_file, output_file):
    try:
        mid = mido.MidiFile(input_file)
    except Exception as e:
        print(f"Error: {e}")
        return

    new_midi = mido.MidiFile()
    new_midi.ticks_per_beat = mid.ticks_per_beat

    # Find the main melody track again
    best_track = None
    max_notes = 0
    for track in mid.tracks:
        note_count = sum(1 for m in track if m.type == 'note_on')
        if note_count > max_notes:
            max_notes = note_count
            best_track = track

    if best_track:
        print("Found melody. Applying Cyber-Tokyo filters...")
        vibey_track = make_vibey(best_track)
        vibey_track.name = "Vibey Melody"
        new_midi.tracks.append(vibey_track)
        
        new_midi.save(output_file)
        print(f"Done. Saved to {output_file}")
    else:
        print("Could not find a melody track.")

if __name__ == "__main__":
    import sys
    process_midi(sys.argv[1], "vibey_theme.midi")
