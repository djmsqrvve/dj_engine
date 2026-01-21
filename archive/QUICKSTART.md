# Quick Start Guide – DJ Engine

**For**: New developers joining the project  
**Time**: 15–20 minutes  
**Goal**: Get the project running locally and understand the layout

---

## 1. Prerequisites (5 minutes)

### Required Software

- **Rust** (latest stable): https://rustup.rs/
  ```bash
  rustup update stable
  rustup default stable
  ```

- **Git**: https://git-scm.com/
  ```bash
  git --version
  ```

- **VS Code** with Rust extensions (optional but recommended)
  - Install Rust Analyzer extension
  - Install CodeLLDB for debugging

### Verify Installation

```bash
rustc --version           # Should be 1.75+
cargo --version           # Should be 1.75+
git --version             # Should be 2.40+
```

---

## 2. Clone & Setup (5 minutes)

### Clone Repository

```bash
git clone https://github.com/yourusername/dj_engine.git
cd dj_engine
```

### Check Initial Build

```bash
# Verify workspace compiles
cargo check --workspace

# Run tests
cargo test --workspace

# If everything passes, you're ready! ✅
```

**First compile**: Takes 2–3 minutes (Bevy is large). Subsequent builds are faster.

---

## 3. Project Layout (3 minutes)

```
dj_engine/
├── README.md                    # Project overview
├── PROJECT_PLAN.md              # Complete technical plan
├── ARCHITECTURE.md              # System design
├── CODING_STANDARDS.md          # Code style guide
├── WORKFLOW.md                  # Development workflow
├── LUA_FFI.md                   # Lua integration guide
├── ASSET_PIPELINE.md            # Asset workflow
├── CHANGELOG.md                 # Version history
├── Cargo.toml                   # Workspace manifest
├── Cargo.lock                   # Dependency lock (committed)
│
├── engine/                      # Shared engine code
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs               # Currently mostly empty; scaffolding done
│
├── tools/                       # Build tools (planned)
│
└── games/
    └── dev/
        └── doomexe/             # Main game (Hamster Narrator)
            ├── Cargo.toml
            ├── README.md
            ├── assets/          # Sprites, shaders, scripts, palettes
            └── src/
                └── main.rs      # Entry point
```

---

## 4. Common Development Tasks

### Starting Development on a Feature

```bash
# Get latest code
git fetch origin
git checkout main
git pull origin main

# Create feature branch
git checkout -b feature/your-feature-name

# Verify build
cargo check --workspace
```

**Branch naming**: See WORKFLOW.md §2

### Running the Game

```bash
# In the project root
cd games/dev/doomexe
cargo run

# Or with logging
RUST_LOG=debug cargo run
```

**Note**: Game window will show (mostly empty in Milestone 1).

### Checking Code Quality

```bash
# Format code (required before commit)
cargo fmt --all

# Check for issues
cargo clippy --workspace -- -D warnings

# Run tests
cargo test --workspace

# Shortcut: Run all checks
cargo fmt --all && cargo clippy --workspace -- -D warnings && cargo test --workspace
```

### Creating a Commit

```bash
# Make your changes
# ...

# Stage changes
git add .

# Commit with message (see WORKFLOW.md §3 for format)
git commit -m "feat(scope): description"

# Example:
git commit -m "feat(animation): add breathing animation system"
```

### Opening a Pull Request

1. Push your branch:
   ```bash
   git push origin feature/your-feature-name
   ```

2. Go to GitHub, create Pull Request
3. Link to related issue
4. Describe what you changed and why
5. Request review from a team member

---

## 5. Key Documentation

Read in this order:

| Document | Purpose | Time |
|----------|---------|------|
| **PROJECT_PLAN.md** | "Why are we building this?" and "What's the plan?" | 20 min |
| **ARCHITECTURE.md** | System design and how pieces fit together | 15 min |
| **CODING_STANDARDS.md** | How to write code that fits the project | 10 min |
| **WORKFLOW.md** | Git flow, code review, and team process | 10 min |
| **LUA_FFI.md** | How Rust talks to Lua (if doing Lua work) | 15 min |
| **ASSET_PIPELINE.md** | How assets flow from Aseprite to game | 15 min |

**Total reading time**: ~85 minutes for full understanding.

---

## 6. Understanding the Code

### Where to Look First

**For rendering/animation**:
→ `engine/src/` (currently mostly empty; structure defined in PROJECT_PLAN.md §2.1)

**For Lua integration**:
→ `engine/scripting/` (coming in Milestone 1)

**For game logic**:
→ `games/dev/doomexe/src/` (your custom code)

**For assets**:
→ `games/dev/doomexe/assets/` (sprites, shaders, scripts, palettes)

### Example: Tracing a Feature

Let's say you want to understand how "corruption changes" work:

1. **Read the spec**: PROJECT_PLAN.md §7 (Milestone 1 scope)
2. **Check the architecture**: ARCHITECTURE.md §2.3 (data flow)
3. **Look at the Lua API**: LUA_FFI.md §2.1 (`set_corruption()`)
4. **See the type definition**: ARCHITECTURE.md §5 (`HamsterNarrator`)
5. **Find the system**: `engine/src/rendering/` (when implemented)

This top-down approach prevents getting lost in code.

---

## 7. Common Questions

### "Where do I start implementing?"

→ See PROJECT_PLAN.md §7.3 (implementation order)  
→ Week 1 = module scaffolding  
→ Week 2–5 = feature implementation

### "How do I add a new system?"

→ ARCHITECTURE.md §3 (plugin architecture)  
→ Create module in `engine/src/`  
→ Implement `bevy::app::Plugin` trait  
→ Add to `App` in `main.rs`

### "How do I debug Lua scripts?"

→ LUA_FFI.md §7 (debugging)  
→ Use `log()` function in Lua  
→ Run with `RUST_LOG=debug`

### "Can I use feature X right now?"

→ Check CHANGELOG.md §0.1.0 (what's planned for Milestone 1)  
→ Or see ARCHITECTURE.md §10 (known limitations)

### "Where do I get help?"

→ Check relevant documentation first  
→ Ask in team Slack/Discord  
→ Create GitHub Discussion  
→ Search existing GitHub Issues

---

## 8. Development Workflow Checklist

Before starting work:

- [ ] Read PROJECT_PLAN.md (understand the vision)
- [ ] Read ARCHITECTURE.md (understand the design)
- [ ] Run `cargo check --workspace` (verify build)
- [ ] Create feature branch (`feature/your-name`)

Before pushing code:

- [ ] Run `cargo fmt --all` (format)
- [ ] Run `cargo clippy --workspace` (lint)
- [ ] Run `cargo test --workspace` (test)
- [ ] Review your own code
- [ ] Commit with clear message

Before merging:

- [ ] Wait for code review (at least 1 approval)
- [ ] Address all feedback
- [ ] Squash commits if requested
- [ ] Update CHANGELOG.md (if feature/fix)

---

## 9. Environment Setup (Optional)

### Recommended VS Code Extensions

```json
// .vscode/extensions.json
{
  "recommendations": [
    "rust-lang.rust-analyzer",
    "vadimcn.vscode-lldb",
    "tamasfe.even-better-toml",
    "serayuzgur.crates"
  ]
}
```

Install all at once:
```bash
code --install-extension rust-lang.rust-analyzer
code --install-extension vadimcn.vscode-lldb
code --install-extension tamasfe.even-better-toml
code --install-extension serayuzgur.crates
```

### Useful VS Code Settings

`.vscode/settings.json`:
```json
{
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.checkOnSave.extraArgs": ["--", "-D", "warnings"],
  "[rust]": {
    "editor.formatOnSave": true
  }
}
```

This makes VS Code auto-check and auto-format on save.

---

## 10. Next Steps (After This Guide)

### Immediate (Today)

- [ ] Clone the repo and run `cargo check`
- [ ] Read PROJECT_PLAN.md and ARCHITECTURE.md
- [ ] Explore the directory structure
- [ ] Run `cargo test --workspace`

### This Week

- [ ] Read CODING_STANDARDS.md
- [ ] Read WORKFLOW.md
- [ ] Set up VS Code with recommended extensions
- [ ] Join the team Slack/Discord
- [ ] Create a simple test commit to practice the workflow

### Before Starting Feature Work

- [ ] Read all documentation relevant to your task
- [ ] Ask questions if anything is unclear
- [ ] Review the weekly plan and commit to a task
- [ ] Create a GitHub Issue for tracking

---

## 11. Troubleshooting

### Build Fails with "Bevy not found"

```bash
# Make sure you're in the right directory
cd dj_engine

# Update dependencies
cargo update

# Clean and rebuild
cargo clean
cargo build --workspace
```

### Clippy Warnings Won't Go Away

```bash
# Make sure you ran formatter
cargo fmt --all

# Check clippy specifically
cargo clippy --workspace -- -D warnings

# Update Rust
rustup update stable
```

### Test Failures on First Run

```bash
# Some tests require assets. Make sure assets/ directory exists.
ls games/dev/doomexe/assets/

# Run specific test with logging
RUST_LOG=debug cargo test test_name -- --nocapture
```

### Git Conflicts on Cargo.lock

```bash
# Don't manually edit Cargo.lock
# Instead, let cargo handle it
git checkout --theirs Cargo.lock
cargo update
git add Cargo.lock
```

---

## 12. Quick Reference

### Most Common Commands

```bash
# Check if code compiles
cargo check --workspace

# Run all tests
cargo test --workspace

# Format code (REQUIRED before commit)
cargo fmt --all

# Check for issues
cargo clippy --workspace -- -D warnings

# Run the game
cd games/dev/doomexe && cargo run

# View documentation
cargo doc --no-deps --open

# Update dependencies
cargo update
```

### Git Quick Reference

```bash
# Create branch
git checkout -b feature/name

# Check status
git status

# Stage changes
git add .

# Commit
git commit -m "type(scope): message"

# Push to GitHub
git push origin feature/name

# Update from main
git fetch origin
git rebase origin/main
```

---

## Final Tips

1. **Read before you code** – The docs exist to save you time
2. **Ask questions** – If documentation is unclear, it's a documentation bug
3. **Test locally** – `cargo test` before pushing
4. **Commit often** – Small commits are easier to review
5. **Communicate** – Tell the team what you're working on

---

## Welcome! 🎮

You're now ready to contribute to DJ Engine. The team is excited to have you. Don't hesitate to ask questions in Slack or create a discussion on GitHub.

**Happy coding!**

---

**Questions?** Open a GitHub Discussion or ask in the team Slack.  
**Found an issue?** Check TROUBLESHOOTING above, then create a GitHub Issue.  
**Have feedback on this guide?** Open a PR with improvements!
