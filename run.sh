#!/bin/bash

# Script to run the rustnake game

echo "Starting Rustnake Game..."
echo "Controls: Arrow keys or wasd to move, 'p' to pause, 'm' to mute, 'space' to return to menu, 'q' to quit"
echo "Note: Walls wrap around (Nokia style) - only colliding with yourself ends the game!"
echo "Press any key to start..."
read -n 1 -s

# Run the game
cargo run