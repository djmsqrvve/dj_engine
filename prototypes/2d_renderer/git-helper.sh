#!/bin/bash

# Git Helper for Bevy 2D Renderer Prototype
# Shows complete git status and provides commit assistance

set -e

echo "=========================================="
echo "🚀 Git Helper - Bevy 2D Renderer"
echo "=========================================="
echo ""

# Check if we're in a git repo
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    echo "❌ Not in a git repository"
    exit 1
fi

echo "📍 Current prototype: prototypes/2d_renderer"
echo "📂 Parent project: $(basename $(git rev-parse --show-toplevel))"
echo ""

# Check if we should show untracked files
SHOW_UNTRACKED=true
if [ "$1" = "--cached" ] || [ "$1" = "--staged" ]; then
    SHOW_UNTRACKED=false
fi

echo "=========================================="
echo "📝 Git Status (Relevant to This Prototype)"
echo "=========================================="
echo ""

if [ "$SHOW_UNTRACKED" = true ]; then
    # Show full status
    git status prototypes/2d_renderer/
else
    # Show only staged
    git diff --cached --stat prototypes/2d_renderer/
fi

echo ""
echo "=========================================="
echo "📊 File Statistics"
echo "=========================================="

# Count files in the prototype
if [ -d "prototypes/2d_renderer/" ]; then
    echo "📁 Total files in prototype:"
    find prototypes/2d_renderer/ -type f \
        ! -path "*/target/*" \
        ! -path "*/.git/*" \
        -name "*.rs" -o -name "*.toml" -o -name "*.md" -o -name "*.sh" -o -name "*.yml" | wc -l | xargs echo "  Rust source & configs:"
    
    echo "  Asset files:"
    find prototypes/2d_renderer/assets -type f 2>/dev/null | wc -l | xargs echo "    " || echo "    0"
    
    # Size of source code (excluding target)
    SIZE=$(find prototypes/2d_renderer/src -name "*.rs" -exec wc -c {} + 2>/dev/null | tail -1 | awk '{print $1}' || echo 0)
    echo "  Source code size: $(echo "scale=1; $SIZE/1024" | bc) KB"
fi

echo ""
echo "=========================================="
echo "✅ Quality Checks"
echo "=========================================="

# Run quick checks if code is not modified
if [ -z "$(git status --short prototypes/2d_renderer/)" ]; then
    echo "✓ No uncommitted changes"
else
    # Check for TODO/FIXME
    echo "🔍 Checking for TODOs/FIXMEs..."
    if grep -r "TODO\|FIXME" prototypes/2d_renderer/src/ 2>/dev/null; then
        echo "  ⚠️  Found TODO/FIXME items"
    else
        echo "  ✓ No TODO/FIXME items"
    fi
    
    # Check if tests pass
    echo "🧪 Running quick test check..."
    if cargo test --lib --quiet 2>/dev/null | grep -q "test result: ok"; then
        echo "  ✓ All tests passing"
    else
        echo "  ⚠️  Some tests may be failing"
    fi
fi

echo ""
echo "=========================================="
echo "🎯 Recommended Next Actions"
echo "=========================================="
echo ""

if [ -z "$(git status --short prototypes/2d_renderer/)" ]; then
    echo "🎉 All prototype files are committed!"
    echo ""
    echo "Options:"
    echo "  • Push to remote: git push"
    echo "  • Create PR/merge in parent project"
    echo "  • Continue development"
else
    echo "📝 To stage and commit this prototype:"
    echo ""
    echo "  1. Review changes:"
    echo "     git status prototypes/2d_renderer/"
    echo ""
    echo "  2. Add files (choose one):"
    echo "     git add prototypes/2d_renderer/              # Add all"
    echo "     git add -p prototypes/2d_renderer/           # Interactive"
    echo "     git add prototypes/2d_renderer/src/         # Source only"
    echo ""
    echo "  3. Commit:"
    echo "     git commit -m 'feat: Add Bevy 2D renderer prototype'"
    echo ""
    echo "  4. Or use a single command:"
    echo "     git add prototypes/2d_renderer/ && git commit -m 'feat: Add Bevy 2D renderer prototype with tests'"
    echo ""
    echo "💡 Pro tip: Write detailed commit messages:"
    echo "   git commit -m 'feat: Add Bevy 2D renderer prototype' \\"
    echo "              -m 'Includes: animated sprites, parallax backgrounds, 2D lighting,' \\"
    echo "              -m 'tilemap support, camera controls, and comprehensive test suite.'"
fi

echo ""
echo "=========================================="
echo "📚 Prototype Summary"
echo "=========================================="
echo ""
echo "This prototype implements:"
echo "  ✓ Bevy 0.14 2D rendering engine"
echo "  ✓ Animated sprite system (4-frame)"
echo "  ✓ 3-layer parallax backgrounds"
echo "  ✓ Mouse-following 2D point light"
echo "  ✓ ECS tilemap rendering (32x24)"
echo "  ✓ Smooth camera follow with zoom"
echo "  ✓ Mint Cyberpunk visual aesthetic"
echo "  ✓ 20 unit/integration tests"
echo "  ✓ Test automation & coverage"
echo "  ✓ CI/CD pipeline (GitHub Actions)"
echo "  ✓ Comprehensive documentation"
echo ""
echo "📖 See README.md for usage instructions"
echo "🧪 See TESTING.md for testing guide"
echo ""
echo "=========================================="
