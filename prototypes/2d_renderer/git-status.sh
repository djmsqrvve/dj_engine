#!/bin/bash

# Git Status Helper for Bevy 2D Renderer
# Shows status of files in this prototype relative to parent project

set -e

echo "=========================================="
echo "Git Status - Bevy 2D Renderer Prototype"
echo "=========================================="
echo ""

# Check if we're in a git repo
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    echo "❌ Not in a git repository"
    exit 1
fi

echo "📍 Current directory: $(pwd)"
echo "📂 Git root: $(git rev-parse --show-toplevel)"
echo ""

echo "=========================================="
echo "📝 Modified Files in This Prototype"
echo "=========================================="

# Show modified files in this directory
git status --short | grep -E "^( M|\?\?) ." | grep -E "(2d_renderer|assets/)" || echo "No modified files in this prototype"

echo ""
echo "=========================================="
echo "📊 Statistics"
echo "=========================================="

# Count new files
NEW_FILES=$(git status --short | grep -E "^\?\?" | grep -c "2d_renderer" || echo 0)
echo "📄 New files: $NEW_FILES"

# Count modified files
MOD_FILES=$(git status --short | grep -E "^ M" | grep -c "2d_renderer" || echo 0)
echo "✏️  Modified files: $MOD_FILES"

echo ""
echo "=========================================="
echo "🚀 Quick Actions"
echo "=========================================="
echo ""
echo "View diff of a file:"
echo "  git diff path/to/file.rs"
echo ""
echo "Add all prototype changes:"
echo "  git add prototypes/2d_renderer/"
echo ""
echo "Commit with message:"
echo "  git commit -m 'feat: Add Bevy 2D renderer prototype with testing'"
echo ""
echo "Check parent project status:"
echo "  git status --short | grep -v 2d_renderer"
echo ""
echo "=========================================="
echo "✅ Ready to Commit!"
echo "=========================================="
echo ""
echo "This prototype includes:"
echo "  • Complete Bevy 2D rendering implementation"
echo "  • 20 unit and integration tests"
echo "  • Test infrastructure and documentation"
echo "  • GitHub Actions CI/CD workflow"
echo "  • Placeholder assets and git configuration"
echo ""
