# Rustnake

A classic Snake game implemented in Rust for the command line interface with enhanced features and modern gameplay elements.

## Features

- Classic snake gameplay with Nokia-style wall wrapping
- Five different power-ups for varied gameplay
- Multiple difficulty levels (Easy, Medium, Hard)
- Score tracking during gameplay
- Colorful terminal display with gradient-colored snake
- Sound effects using terminal bell
- Pause functionality
- Optimized rendering with efficient updates
- Special food items that appear at score milestones

## Installation

1. Make sure you have Rust installed on your system (latest stable version recommended)
2. Clone or download this repository
3. Navigate to the project directory
4. Run the game with:
   ```bash
   cargo run
   ```
   
Alternatively, you can use the provided `run.sh` script:
```bash
chmod +x run.sh
./run.sh
```

## Game Controls

### During Gameplay:
- **Arrow Keys** or **WASD** - Change snake direction
- **P** - Pause/Unpause the game
- **M** - Mute/Unmute sound effects
- **SPACE** - Go back to the difficulty selection menu
- **Q** - Quit the game at any time

### During Menu Navigation:
- **Arrow Keys** or **WASD** / **1-3** - Navigate difficulty selection
- **ENTER** or **SPACE** - Confirm selection

### After Game Over:
- **SPACE** - Restart the game with the same difficulty
- **Q** - Quit the game

## Game Elements

### Food System
- **Basic Food** (●) - Red circle that increases your score by 10 points and grows your snake by one segment
- **Special Food** (★) - Appears when your score is divisible by 50, providing visual milestone recognition

### Power-Up System
Collect power-ups to gain special abilities and advantages:

- **Speed Boost** (`>`) - Blue symbol that temporarily increases your snake's speed
- **Slow Down** (`<`) - Cyan symbol that temporarily decreases your snake's speed (can be strategic!)
- **Extra Points** (`$`) - Yellow symbol that adds 50 points to your score
- **Grow** (`+`) - Green symbol that adds 2 segments to your snake
- **Shrink** (`-`) - Magenta symbol that removes 2 segments from your snake (minimum 3 segments maintained)

Power-ups appear randomly with a 30% chance when initially generated and a 2% chance each tick. Their effects last for 100 game ticks.

## Game Mechanics

- The snake moves continuously at different speeds based on difficulty level:
  - **Easy**: Slower movement (150ms horizontal, 300ms vertical)
  - **Medium**: Default movement (100ms horizontal, 200ms vertical) 
  - **Hard**: Faster movement (60ms horizontal, 120ms vertical)
- Horizontal movement is faster than vertical movement for authentic snake game feel
- The snake wraps around screen edges (pass through walls to opposite side)
- Game ends when the snake collides with itself (no wall collisions due to wrapping)
- The snake's head is bright green, with body segments transitioning from green to yellow to dark gray toward the tail

## How to Play

1. Launch the game using `cargo run`
2. Select your preferred difficulty level (1-3)
3. Control the snake using arrow keys or WASD to collect food and power-ups
4. Eat the red food (● or ★) to grow and increase your score
5. Collect power-ups for special effects and advantages
6. Avoid colliding with your own body
7. Press `P` to pause the game at any time
8. When game over, press `SPACE` to play again or `Q` to quit

## Strategy Tips

- Pay attention to power-up appearances - they can be game-changers
- The "Shrink" power-up reduces your snake length, making collisions less likely
- Use the "Slow Down" power-up strategically when you're in a tight spot
- Higher difficulties offer greater challenges but the same scoring system
- The star food (★) appears every 50 points as a visual milestone

## Dependencies

- `crossterm`: For cross-platform terminal manipulation
- `rand`: For random food and power-up placement
- `serde` and `toml`: For configuration (when needed)

## Customization

You can customize various aspects of the game:

- **Board Size**: Modify the `WIDTH` and `HEIGHT` constants in `src/utils/mod.rs`
- **Game Speed**: Adjust tick rates in the `get_tick_rates` method in `src/core/mod.rs`
- **Power-up Behavior**: Modify probabilities and effects in the power-up functions in `src/core/mod.rs`
- **Food Appearance**: Change symbols and conditions in the drawing function in `src/render/mod.rs`

## File Structure

- `Cargo.toml` - Project dependencies and metadata
- `src/main.rs` - Main game loop and orchestration
- `src/core/` - Core game logic (Snake, Game entities)
- `src/render/` - Rendering and UI functionality
- `src/input/` - Input handling
- `src/utils/` - Shared types and constants
- `run.sh` - Convenience script to run the game
- `.gitignore` - Files and directories to be ignored by Git

## Installation for Beginners

### Prerequisites
1. Install Rust: Visit [https://rustup.rs](https://rustup.rs) and follow the instructions
2. Verify installation: Open a terminal and run:
   ```bash
   rustc --version
   cargo --version
   ```

### Installing Rustnake

#### Method 1: From Source (Recommended)
1. Navigate to the project directory:
   ```bash
   cd /path/to/rustnake
   ```
2. Build and install:
   ```bash
   cargo build --release
   sudo cp target/release/rustnake /usr/local/bin/
   ```
3. Run the game:
   ```bash
   rustnake
   ```

#### Method 2: Using Cargo Install
1. Install directly:
   ```bash
   cargo install --path .
   ```
2. Run the game:
   ```bash
   rustnake
   ```

#### Method 3: Quick Run (No Installation)
- Just run `cargo run` in the project directory to play without installing

### Updating Rustnake

#### If installed from source:
```bash
cd /path/to/rustnake
git pull origin master
cargo build --release
sudo cp target/release/rustnake /usr/local/bin/
```

#### If installed via cargo:
```bash
cargo install --path . --force
```

### Uninstalling Rustnake

#### If installed to /usr/local/bin:
```bash
sudo rm /usr/local/bin/rustnake
```

#### If installed via cargo:
```bash
cargo uninstall rustnake
```

### Troubleshooting

- If the game doesn't run, ensure Rust is properly installed
- If colors don't display correctly, ensure your terminal supports ANSI escape codes
- If experiencing input lag, try adjusting the input_cooldown value in the main function