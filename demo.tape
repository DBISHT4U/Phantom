# Phantom Demo — VHS Recording Script
# Install: brew install charmbracelet/tap/vhs
# Run: vhs demo.tape
# Output: demo.gif (add to README)

Output demo.gif
Set FontSize 13
Set Width 1200
Set Height 700
Set Theme "Catppuccin Mocha"
Set Padding 15
Set TypingSpeed 30ms
Set CursorBlink false

# Start
Type "# 👻 Phantom — The fastest username recon tool"
Enter
Sleep 1s

# Scan a username
Type "phantom torvalds --found-only"
Enter
Sleep 12s

# Category filter
Type "# Filter by category"
Enter
Sleep 500ms
Type "phantom torvalds --category dev,security --found-only"
Enter
Sleep 8s

# List categories
Type "phantom --list-categories"
Enter
Sleep 3s

# Export
Type "# Export to JSON"
Enter
Sleep 500ms
Type "phantom torvalds --output results.json --found-only"
Enter
Sleep 10s

# Show JSON
Type "cat results.json | head -20"
Enter
Sleep 3s

# End
Type "# 481 sites. Single binary. Seconds, not minutes. 👻"
Enter
Sleep 2s
