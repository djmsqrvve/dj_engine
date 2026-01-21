# Documentation Index – DJ Engine

**Version**: 1.0  
**Last Updated**: 2026-01-20  
**Purpose**: Navigate all DJ Engine documentation

---

## 📚 Documentation Library

Welcome! This index helps you find the right documentation for your task.

### For Different Roles

#### 👨‍💼 Project Manager / Designer
**Start here to understand the project scope and timeline:**
1. README.md (top-level overview)
2. PROJECT_PLAN.md (vision, roadmap, milestones)
3. ASSET_PIPELINE.md (how assets are created and used)

**Useful references:**
- CHANGELOG.md (what's complete, what's planned)
- ARCHITECTURE.md §8 (extensibility for future features)

#### 👨‍💻 Rust Developer
**Start here to understand the codebase and contribute:**
1. QUICKSTART.md (get set up in 15 minutes)
2. ARCHITECTURE.md (system design and how things connect)
3. CODING_STANDARDS.md (how to write code that fits the project)
4. WORKFLOW.md (development process and git workflow)

**For specific systems:**
- LUA_FFI.md (if working on scripting integration)
- ASSET_PIPELINE.md §4 (if working on asset loading)

#### 🎨 Game Designer / Lua Scripter
**Start here to understand game logic and customization:**
1. QUICKSTART.md (get the game running)
2. LUA_FFI.md (Lua API and scripting)
3. ASSET_PIPELINE.md (how to create and use assets)
4. ARCHITECTURE.md §2 (understand the event flow)

**For debugging:**
- LUA_FFI.md §7 (debugging Lua scripts)
- WORKFLOW.md §7 (local development setup)

#### 🎨 Artist / Asset Creator
**Start here to understand asset workflows:**
1. ASSET_PIPELINE.md (complete asset pipeline from Aseprite to game)
2. ARCHITECTURE.md (understand where your assets are used)
3. QUICKSTART.md (get the game running to preview your work)

---

## 📖 Document Guide

### Core Documentation

#### **README.md**
- **What**: Project overview
- **Who**: Everyone
- **Read time**: 5 min
- **Key sections**: Project vision, features, quick start

#### **PROJECT_PLAN.md**
- **What**: Detailed project plan with roadmap and scaffolding
- **Who**: Everyone (especially managers and architects)
- **Read time**: 30 min
- **Key sections**: Current state, gaps identified, 5-week roadmap, risk mitigation

#### **ARCHITECTURE.md**
- **What**: System design and how pieces fit together
- **Who**: Developers and architects
- **Read time**: 20 min
- **Key sections**: System overview, module dependencies, data flow, extensibility

#### **QUICKSTART.md**
- **What**: Get up and running in 15 minutes
- **Who**: New developers and contributors
- **Read time**: 15 min
- **Key sections**: Setup, common tasks, troubleshooting

---

### Development Standards

#### **CODING_STANDARDS.md**
- **What**: How to write code that fits the project
- **Who**: Rust developers
- **Read time**: 20 min
- **Key sections**: Naming conventions, error handling, testing, Bevy patterns
- **Why it matters**: Consistent style = easier code review

#### **WORKFLOW.md**
- **What**: Development process, code review, CI/CD
- **Who**: Everyone
- **Read time**: 20 min
- **Key sections**: Branches, commits, PRs, testing, release process
- **Why it matters**: Clear process = smooth collaboration

---

### Technical Specifications

#### **LUA_FFI.md**
- **What**: Lua ↔ Rust integration specification
- **Who**: Lua scripters and Rust developers (if touching scripting)
- **Read time**: 25 min
- **Key sections**: Exposed functions, type conversions, hot-reload, error handling
- **Why it matters**: Clear boundary = easier debugging and maintenance

#### **ASSET_PIPELINE.md**
- **What**: Asset creation workflow from Aseprite to runtime
- **Who**: Artists, asset creators, and developers
- **Read time**: 25 min
- **Key sections**: Directory structure, sprite export, palette format, loaders, validation
- **Why it matters**: Clear workflow = less time troubleshooting asset issues

---

### Historical & Tracking

#### **CHANGELOG.md**
- **What**: Version history and release notes
- **Who**: Everyone (especially for understanding what's done)
- **Read time**: 10 min
- **Key sections**: v0.1.0 planned features, release checklist

#### **DOCUMENTATION_INDEX.md** (This file)
- **What**: Navigation guide for all documentation
- **Who**: Everyone
- **Read time**: 5 min

---

## 🗺️ Reading Paths

### Path 1: New Developer (First Day)
1. **QUICKSTART.md** (15 min) – Get set up
2. **PROJECT_PLAN.md** (15 min) – Understand the vision
3. **ARCHITECTURE.md** (15 min) – Understand the design
4. **CODING_STANDARDS.md** (15 min) – Learn the code style
5. **WORKFLOW.md** (15 min) – Learn the process

**Total**: ~90 minutes of reading + setup

### Path 2: Existing Developer (New Feature)
1. **WORKFLOW.md §2** (2 min) – Branch naming
2. Relevant technical doc (LUA_FFI.md, ASSET_PIPELINE.md, etc.) (10–15 min)
3. **CODING_STANDARDS.md** (quick reference) (2 min)
4. Code and test locally
5. **WORKFLOW.md §3–5** (5 min) – Commit and PR process

**Total**: ~30 minutes reading + development

### Path 3: Designer/Lua Scripter
1. **QUICKSTART.md** (15 min) – Get game running
2. **LUA_FFI.md** (20 min) – Learn Lua API
3. **ASSET_PIPELINE.md** (15 min) – Understand assets
4. Write scripts and test with hot-reload
5. **LUA_FFI.md §7** (debug when needed)

**Total**: ~50 minutes reading + scripting

### Path 4: Artist/Asset Creator
1. **QUICKSTART.md** (15 min) – Get game running
2. **ASSET_PIPELINE.md §2** (15 min) – Sprite export process
3. **ASSET_PIPELINE.md §3** (10 min) – Palette format
4. Create assets in Aseprite
5. **ASSET_PIPELINE.md §7** (validate before commit)

**Total**: ~40 minutes reading + asset creation

---

## 🔍 Quick Lookup by Topic

### Want to understand...

| Topic | Document | Section |
|-------|----------|---------|
| Project vision & scope | PROJECT_PLAN.md | Executive Summary |
| System architecture | ARCHITECTURE.md | System Overview §1 |
| Module structure | ARCHITECTURE.md | Module Dependencies §4 |
| Development process | WORKFLOW.md | Development Phases §1 |
| Git workflow | WORKFLOW.md | Branch Naming §2, Commits §3 |
| Code style | CODING_STANDARDS.md | General Principles §1 |
| Error handling | CODING_STANDARDS.md | Error Handling §5 |
| Testing | CODING_STANDARDS.md | Testing §8 |
| Lua scripting | LUA_FFI.md | Exposed Functions §2 |
| Hot-reload | LUA_FFI.md | Hot-Reload Semantics §5 |
| Asset creation | ASSET_PIPELINE.md | Sprite Export §2 |
| Asset loading | ASSET_PIPELINE.md | Runtime Asset Loading §5 |
| Debugging Lua | LUA_FFI.md | Debugging §7 |
| Debugging assets | ASSET_PIPELINE.md | Troubleshooting §10 |
| Release process | WORKFLOW.md | Release Process §11 |
| Versioning | CHANGELOG.md | Semantic Versioning |
| First time setup | QUICKSTART.md | Prerequisites §1, Setup §2 |
| Common dev tasks | QUICKSTART.md | Common Development Tasks §4 |
| Troubleshooting | QUICKSTART.md | Troubleshooting §11 |

---

## 📋 Document Checklist

### Before Starting Development

- [ ] Read QUICKSTART.md (get set up)
- [ ] Read PROJECT_PLAN.md (understand vision)
- [ ] Read ARCHITECTURE.md (understand design)
- [ ] Read CODING_STANDARDS.md (learn style)
- [ ] Read WORKFLOW.md (learn process)
- [ ] Run `cargo check --workspace` (verify build)
- [ ] Ask questions if anything is unclear

### Before Writing Code

- [ ] Read relevant technical doc (LUA_FFI.md, ASSET_PIPELINE.md, etc.)
- [ ] Read the GitHub Issue you're working on
- [ ] Create feature branch with proper naming (WORKFLOW.md §2)
- [ ] Make sure you understand the acceptance criteria

### Before Committing Code

- [ ] Run `cargo fmt --all` (format)
- [ ] Run `cargo clippy --workspace -- -D warnings` (lint)
- [ ] Run `cargo test --workspace` (test)
- [ ] Write commit message per WORKFLOW.md §3
- [ ] Self-review your code
- [ ] Reference related issues in commit message

### Before Opening a PR

- [ ] Verify your code follows CODING_STANDARDS.md
- [ ] Link to related GitHub Issue
- [ ] Write clear PR description
- [ ] Request review from appropriate person
- [ ] Be ready to address feedback

### Before Merging

- [ ] At least one approval from reviewer
- [ ] All feedback addressed
- [ ] All checks passing (format, lint, tests)
- [ ] Update CHANGELOG.md (if applicable)

---

## 🆘 Getting Help

### I can't find an answer to my question

1. **Search existing documentation** – Use Ctrl+F to find keywords
2. **Check the table above** (Quick Lookup by Topic)
3. **Ask in team Slack** – Provide context (what you tried, what doc you read)
4. **Create a GitHub Discussion** – If it's a complex question
5. **Open a GitHub Issue** – If it's a bug or documentation gap

### Documentation is unclear or outdated

Please help us improve! You have two options:

1. **Create a GitHub Issue** with title `docs: [section] needs clarity`
2. **Submit a PR** with improvements (see WORKFLOW.md)

Even small improvements (typos, clearer examples) are welcome!

### I disagree with a design decision

That's healthy! Here's the process:

1. **Read the decision** in the relevant document (usually PROJECT_PLAN.md or ARCHITECTURE.md)
2. **Understand the reasoning** – Look for comments explaining why
3. **Discuss with team** – Slack first, then GitHub Issue if it's a big decision
4. **Create an RFC** (Request for Comments) if it affects multiple systems

We're always open to better ideas!

---

## 📊 Documentation Statistics

| Document | Lines | Topics | Read Time |
|----------|-------|--------|-----------|
| PROJECT_PLAN.md | 600+ | 13 parts, 50+ topics | 30 min |
| ARCHITECTURE.md | 400+ | 11 sections | 20 min |
| CODING_STANDARDS.md | 500+ | 15 topics | 20 min |
| WORKFLOW.md | 400+ | 14 sections | 20 min |
| LUA_FFI.md | 500+ | 12 sections | 25 min |
| ASSET_PIPELINE.md | 500+ | 11 sections | 25 min |
| QUICKSTART.md | 300+ | 12 sections | 15 min |
| CHANGELOG.md | 200+ | Version tracking | 10 min |
| **TOTAL** | **3,400+** | **100+ topics** | **~2–3 hours** |

---

## 🎯 Success Metrics

### How do I know I've read enough?

You're ready to start contributing when you can answer these:

1. ✅ **What is the project?** (PROJECT_PLAN.md Executive Summary)
2. ✅ **What am I building this week?** (PROJECT_PLAN.md §7.3)
3. ✅ **How do systems connect?** (ARCHITECTURE.md)
4. ✅ **How do I write code?** (CODING_STANDARDS.md §1–2)
5. ✅ **How do I contribute?** (WORKFLOW.md §1–2)

---

## 📞 Document Ownership

| Document | Owner | Updates |
|----------|-------|---------|
| PROJECT_PLAN.md | Architect | When scope changes |
| ARCHITECTURE.md | Architect | When systems change |
| CODING_STANDARDS.md | Tech Lead | When standards update |
| WORKFLOW.md | Tech Lead | When process changes |
| LUA_FFI.md | Scripting Lead | When API changes |
| ASSET_PIPELINE.md | Asset Lead | When workflow changes |
| CHANGELOG.md | Release Manager | With each release |
| QUICKSTART.md | Anyone | When setup changes |

---

## 🔗 External References

### Useful Resources

- **Bevy Engine Documentation**: https://docs.rs/bevy/latest/bevy/
- **Rust Book**: https://doc.rust-lang.org/book/
- **Mlua Documentation**: https://docs.rs/mlua/latest/mlua/
- **Semantic Versioning**: https://semver.org/
- **Keep a Changelog**: https://keepachangelog.com/

---

## Version History

- **v1.0** (2026-01-20) – Initial documentation suite created

---

## Conclusion

You now have a complete, integrated documentation suite for DJ Engine. Every document serves a purpose, references others where relevant, and is written for a specific audience.

**Start with QUICKSTART.md, then follow your role's reading path above.**

Welcome to the team! 🚀
