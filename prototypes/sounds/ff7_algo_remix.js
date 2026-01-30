// FF7 Main Theme - Algorithmic Logic
// 1. INPUT: The extracted notes
let melody = note("g5 ~ _ ~ e5 ~ _ ~ _ d5 ~ f5 ~ e5").scale("Minor")
let bass = note("c3 g3 c4 b3 a3 g3").scale("Minor")

// 2. PROCESS: The Algorithm Stack
stack(
  // LAYER A: The "Memory" (Pad)
  melody
    .slow(2)
    .s("triangle")
    .jux(rev) // Reverse in right ear
    .lpf(sine.range(500, 2000).slow(8)) // Breathing filter
    .delay(0.8).decay(0.8) // Long echo
    .gain(0.5),

  // LAYER C: The "Glitch" (Lead)
  melody
    .s("sawtooth")
    .chunk(4, x => x.fast(2)) // Stutter chunks
    .sometimesBy(0.25, x => x.add(n(12))) // Random octaves
    .clip(1) // Distortion
    .lpf(3000)
    .gain(0.6),

  // LAYER B: The "Drive" (Rhythm)
  bass
    .struct("x(3,8)") // Euclidean Rhythm (3 hits per 8 steps)
    .s("square")
    .lpf(400) // Deep bass
    .gain(0.8)
)
.play()
