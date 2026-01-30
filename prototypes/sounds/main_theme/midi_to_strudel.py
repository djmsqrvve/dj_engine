import sys
import struct

def read_variable_length(data, offset):
    value = 0
    while True:
        byte = data[offset]
        offset += 1
        value = (value << 7) | (byte & 0x7F)
        if not (byte & 0x80):
            break
    return value, offset

def midi_note_to_strudel(note_number):
    notes = ['c', 'db', 'd', 'eb', 'e', 'f', 'gb', 'g', 'ab', 'a', 'bb', 'b']
    octave = (note_number // 12) - 1
    name = notes[note_number % 12]
    return f"{name}{octave}"

def parse_midi(file_path):
    with open(file_path, 'rb') as f:
        data = f.read()

    if data[0:4] != b'MThd':
        print("Not a valid MIDI file")
        return

    # Parse Header
    header_len = struct.unpack('>I', data[4:8])[0]
    fmt, tracks, division = struct.unpack('>HHH', data[8:8+header_len])
    print(f"// Format: {fmt}, Tracks: {tracks}, Division: {division}")
    
    offset = 8 + header_len
    
    all_notes = []

    for t in range(tracks):
        if offset >= len(data): break
        if data[offset:offset+4] != b'MTrk':
            break
        
        track_len = struct.unpack('>I', data[offset+4:offset+8])[0]
        offset += 8
        track_end = offset + track_len
        
        current_time = 0
        last_status = 0
        
        # Simple tracker for active notes to calculate duration if needed
        # For Strudel "note" pattern, we mainly care about the sequence or specific offsets
        
        track_events = []
        
        event_count = 0
        while offset < track_end:
            delta, offset = read_variable_length(data, offset)
            current_time += delta
            
            if offset >= len(data): break
            
            status = data[offset]
            
            # Debug first few events
            if event_count < 1:
                print(f"// Track {t} Start: Delta {delta}, Status {hex(status)}")
            event_count += 1

            if status & 0x80:
                last_status = status
                offset += 1
            else:
                status = last_status
                
            cmd = status & 0xF0
            
            if cmd == 0x80: # Note Off
                note = data[offset]
                vel = data[offset+1]
                offset += 2
                track_events.append({'time': current_time, 'type': 'off', 'note': note})
            elif cmd == 0x90: # Note On
                note = data[offset]
                vel = data[offset+1]
                offset += 2
                if vel > 0:
                    track_events.append({'time': current_time, 'type': 'on', 'note': note, 'vel': vel})
                else:
                    track_events.append({'time': current_time, 'type': 'off', 'note': note})
            elif cmd == 0xF0: # Sysex (skip)
                # Sysex length is variable
                length, offset = read_variable_length(data, offset)
                offset += length
            elif cmd == 0xFF: # Meta event
                meta_type = data[offset]
                offset += 1
                length, offset = read_variable_length(data, offset)
                offset += length
            else: # Other control change, program change etc
                # Most are 2 bytes, Program Change and Channel Pressure are 1
                if cmd == 0xC0 or cmd == 0xD0:
                    offset += 1
                else:
                    offset += 2
        
        # Force alignment for next track
        offset = track_end

        # Quantization Logic
        grid_step = division // 4 # 16th note
        if grid_step == 0: grid_step = 120 # Fallback
        
        # Determine track duration
        max_tick = 0
        if track_events:
            max_tick = track_events[-1]['time']
            
        # Rasterize
        # We need to know when notes end. The current parser only has 'on' events stored fully.
        # Let's rebuild the event list to track duration or just use Note Offs from the raw parse.
        # Actually, the previous loop stored 'off' events too.
        
        # State: currently playing notes
        active_notes = {} # note_num -> start_tick
        
        # We will build a sequence of strings
        sequence = []
        
        # Sort events by time
        track_events.sort(key=lambda x: x['time'])
        
        event_idx = 0
        
        for tick in range(0, max_tick + grid_step, grid_step):
            # Process events up to this tick
            while event_idx < len(track_events) and track_events[event_idx]['time'] <= tick:
                ev = track_events[event_idx]
                if ev['type'] == 'on':
                    active_notes[ev['note']] = tick
                elif ev['type'] == 'off':
                    if ev['note'] in active_notes:
                        del active_notes[ev['note']]
                event_idx += 1
            
            # Determine what to play at this step
            # For monophonic reduction, pick the lowest note (often bass/melody root) or highest? 
            # Let's pick the highest note for melody tracks, lowest for bass?
            # Default to highest for now as it captures melody better.
            
            if active_notes:
                # check if any note *started* exactly here or close to it?
                # Strudel: "c4" is a trigger. "~" is rest. "_" is sustain? Strudel uses "~" for silence.
                # If we just output notes at every step, it retrigger.
                # Ideally: Start of note -> "c4". Sustain -> "_".
                
                # Find the note that started most recently or is highest
                # Let's just pick highest.
                best_note = max(active_notes.keys())
                start_tick = active_notes[best_note]
                
                # If the note started within this grid step (or close enough), trigger it.
                # Else, sustain it? Or just retrigger?
                # Retriggering fast 16ths sounds robotic but is accurate for "sequencer" style.
                # Let's use "_" for sustain if it started earlier.
                
                if abs(start_tick - tick) < grid_step:
                     sequence.append(midi_note_to_strudel(best_note))
                else:
                     # sequence.append("_") # Sustain
                     sequence.append("~") # Actually, let's just use rests for gaps to keep it punchy, or retrigger if long?
                     # A safer bet for "remix" is just retrigger or nothing.
                     # Let's try explicit sustain "_" if strudel supports it in mini notation?
                     # Strudel mini-notation: "c ~" -> c lasts half. "c _" -> c lasts full.
                     sequence.append("_")
            else:
                sequence.append("~")

        # Simplify output: combine consecutive rests? 
        # For now, just print the raw sequence chunked.
        
        if len(sequence) > 16: # Only long tracks
            # Filter empty tracks (all ~)
            if all(c == '~' or c == '_' for c in sequence):
                continue

            print(f"// Track {t} (Generated)")
            print('s("midi")')
            print('.note("')
            
            # Print in chunks of 16 (1 bar of 16ths)
            for j in range(0, len(sequence), 16):
                chunk = sequence[j:j+16]
                print(" ".join(chunk))
            
            print('")')
            print()

if __name__ == "__main__":
    parse_midi(sys.argv[1])
