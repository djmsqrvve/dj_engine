# DJ Engine

DJ Engine is a custom game/visual novel framework focused on **procedural 2D character animation**, **palette‑driven corruption effects**, and **narrative‑heavy JRPGs** such as **doomexe**.

This repository holds the shared engine code and tools. Individual games live under the `games/` folder.

---

## Directory Layout

Planned structure on disk:

```text
C:\Users\Mike\Documents\dj_engine\
  README.md              # this file
  engine\                # core engine crates/modules (Rust / Bevy / tooling)
  tools\                 # asset + build-time tools (Aseprite pipeline, generators)
  games\
    dev\
      doomexe\           # doomexe game project (first target)
