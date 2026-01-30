import mido
import random
import argparse
import sys

# --- CONSTANTS ---
TICKS_PER_BEAT = 480
SIXTEENTH = TICKS_PER_BEAT // 4
EIGHTH = TICKS_PER_BEAT // 2
QUARTER = TICKS_PER_BEAT

class CyberForestGenerator:
    def __init__(self, seed=None):
        if seed:
            random.seed(seed)
        
        # The Palette of "Spooky/Cyber" Chords
        # Format: (Root relative to C, Quality)
        self.chord_pool = {
            'tonic': [(0, 'min9'), (0, 'min11')],       # C minor variations
            'subdom': [(5, 'min9'), (8, 'maj7'), (1, 'halfdim')], # Fm, AbMaj, Ddim
            'dom': [(7, 'dim7'), (10, 'maj7#11'), (3, 'maj7')]    # Gdim, BbMaj, EbMaj
        }

    def _create_chord_notes(self, root, quality, octave=3):
        base = root + (octave * 12)
        if quality == 'min9':      return [base, base+3, base+7, base+10, base+14]
        if quality == 'min11':     return [base, base+3, base+7, base+10, base+17]
        if quality == 'maj7':      return [base, base+4, base+7, base+11]
        if quality == 'maj7#11':   return [base, base+4, base+7, base+11, base+18]
        if quality == 'dim7':      return [base, base+4, base+7, base+10] # Dom7b9 sound
        if quality == 'halfdim':   return [base, base+3, base+6, base+10] 
        return [base, base+7, base+12] # Fallback

    def _generate_arpeggio(self, chord_notes, duration_beats):
        """Generates the background rain texture."""
        events = []
        steps = int(duration_beats * 4) # 16th notes
        pattern_len = len(chord_notes)
        
        # Randomized direction for variety
        mode = random.choice(['updown', 'up', 'random'])
        
        for i in range(steps):
            if mode == 'updown':
                idx = i % (pattern_len * 2 - 2)
                if idx >= pattern_len: idx = (pattern_len * 2 - 2) - idx
            elif mode == 'up':
                idx = i % pattern_len
            else:
                idx = random.randint(0, pattern_len-1)
                
            note = chord_notes[idx]
            # Humanize velocity: swell and fade
            velocity = 50 + int(30 * (1 - abs((i % 16) - 8)/8)) # Breathing pulse
            velocity += random.randint(-10, 10)
            velocity = max(20, min(127, velocity))
            
            events.append(mido.Message('note_on', note=note, velocity=velocity, time=0))
            events.append(mido.Message('note_off', note=note, velocity=0, time=SIXTEENTH))
        return events

    def _generate_melody(self, chord_notes, duration_beats):
        """Generates the lead melody."""
        events = []
        target_note = chord_notes[-1] + 12
        
        # Variation 1: Sustained emotive note
        if random.random() > 0.5:
            events.append(mido.Message('note_on', note=target_note, velocity=90, time=0))
            events.append(mido.Message('note_off', note=target_note, velocity=0, time=int(QUARTER * (duration_beats - 1))))
            
            # Ending run
            run_note = target_note - 2
            events.append(mido.Message('note_on', note=run_note, velocity=75, time=0))
            events.append(mido.Message('note_off', note=run_note, velocity=0, time=QUARTER))
        
        # Variation 2: Sparse droplets
        else:
            steps = int(duration_beats * 2) # 8th notes
            current_time = 0
            for i in range(steps):
                if random.random() < 0.3: # 30% chance of note
                    note = random.choice(chord_notes) + 12
                    events.append(mido.Message('note_on', note=note, velocity=80, time=current_time))
                    events.append(mido.Message('note_off', note=note, velocity=0, time=EIGHTH))
                    current_time = 0 # Reset delta since we used it
                else:
                    current_time += EIGHTH # Accumulate rest time
            
            # If we ended with accumulated time, just pass (handled by absolute sorter)
                    
        return events

    def generate(self, filename="generated_forest.midi", loops=4):
        mid = mido.MidiFile()
        track = mido.MidiTrack()
        mid.tracks.append(track)
        
        # Setup: Harp
        track.append(mido.Message('program_change', program=46, time=0))
        track.append(mido.Message('control_change', control=91, value=95, time=0)) # Deep Reverb

        all_events = []
        current_beat = 0
        
        # Generate Progression on the fly
        progression = []
        for _ in range(loops):
            # i - VI - iv - V structure
            progression.append(random.choice(self.chord_pool['tonic']))
            progression.append(random.choice(self.chord_pool['subdom']))
            progression.append(random.choice(self.chord_pool['subdom']))
            progression.append(random.choice(self.chord_pool['dom']))

        print(f"Generating {len(progression)} bars...")

        for root, quality in progression:
            chord = self._create_chord_notes(root, quality, octave=4)
            
            # Arp
            arp_evs = self._generate_arpeggio(chord, 4)
            t = current_beat * TICKS_PER_BEAT
            for msg in arp_evs:
                if msg.type == 'note_on': all_events.append({'time': t, 'msg': msg})
                else: 
                    t += msg.time
                    all_events.append({'time': t, 'msg': msg})
            
            # Melody
            mel_chord = self._create_chord_notes(root, quality, octave=5)
            mel_evs = self._generate_melody(mel_chord, 4)
            t_mel = current_beat * TICKS_PER_BEAT
            for msg in mel_evs:
                # Handle rests manually for melody if needed, but simplistic approach here:
                if msg.type == 'note_on': all_events.append({'time': t_mel, 'msg': msg})
                else: 
                    t_mel += msg.time
                    all_events.append({'time': t_mel, 'msg': msg})
            
            current_beat += 4

        # Sort and Write
        all_events.sort(key=lambda x: x['time'])
        last_time = 0
        for e in all_events:
            delta = e['time'] - last_time
            track.append(e['msg'].copy(time=delta))
            last_time = e['time']
            
        mid.save(filename)
        print(f"Saved to {filename}")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Generate Cyber Forest Music")
    parser.add_argument("--out", type=str, default="gen_forest.midi", help="Output filename")
    parser.add_argument("--loops", type=int, default=4, help="Number of chord loops (4 bars each)")
    parser.add_argument("--seed", type=int, default=None, help="Random seed")
    
    args = parser.parse_args()
    
    gen = CyberForestGenerator(seed=args.seed)
    gen.generate(filename=args.out, loops=args.loops)
