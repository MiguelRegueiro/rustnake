# 🐍 Rustnake

A modern, high-performance take on the classic Snake game, built with **Rust** for the terminal. Experience nostalgic Nokia-style gameplay enhanced with gradients, power-ups, and optimized CLI rendering.

## ✨ Key Features

* **Modern Visuals:** Colorful terminal display featuring a smooth gradient-colored snake.
* **Power-Up System:** 5 unique items to shift gameplay dynamics.
* **Nokia Logic:** Authentic wall-wrapping mechanics (no "wall" deaths).
* **Performance:** Optimized rendering using `crossterm` for zero-flicker gameplay.
* **Milestone Rewards:** Special food spawns at score milestones to keep the pace exciting.

---

## 🎮 Gameplay Preview

## 🎮 Gameplay Preview

| Difficulty Selection | Active Gameplay | Game Over Screen |
| :---: | :---: | :---: |
| ![Difficulty Menu](screenshots/difficultymenu.png) | ![Gameplay](screenshots/gameplay.png) | ![Game Over](screenshots/gameover.png) |

---

## 🚀 Quick Start

### Prerequisites

* [Rust & Cargo](https://rustup.rs/) (Latest stable)

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/rustnake.git
cd rustnake

# Run immediately
cargo run --release

```

> **Tip:** Running with the `--release` flag ensures the smoothest frame timings for the game loop.

---

## 🕹️ Controls

| Action | Key(s) |
| --- | --- |
| **Move** | `Arrow Keys` or `WASD` |
| **Pause** | `P` |
| **Mute SFX** | `M` |
| **Reset/Menu** | `SPACE` |
| **Quit** | `Q` |

---

## 🍎 Game Elements

### Food System

* **Basic Food** (●): +10 Points. Grows snake by 1 segment.
* **Special Food** (★): Spawns every **50 points**. A visual milestone reward!

### Power-Ups

Power-ups have a **30% spawn chance** and effects last for **100 ticks**.

| Icon | Type | Effect |
| --- | --- | --- |
| `>` | **Speed Boost** | Increases snake velocity temporarily. |
| `<` | **Slow Down** | Decreases velocity (ideal for tight maneuvers). |
| `$` | **Bonus** | Instant +50 points. |
| `+` | **Grow** | Adds 2 segments to the snake. |
| `-` | **Shrink** | Removes 2 segments (minimum 3 segments kept). |

---

## 🛠️ Technical Architecture

Rustnake is built with a modular approach to separate logic from rendering:

* **`core/`**: The "Brain." Handles the Snake struct, coordinate math, and collision physics.
* **`render/`**: The "Painter." Manages ANSI escape codes and gradient calculations.
* **`input/`**: Non-blocking input handling to ensure the game doesn't stutter while waiting for keys.

### Difficulty Mechanics

Horizontal movement is intentionally tuned faster than vertical movement to compensate for terminal character aspect ratios (which are usually taller than they are wide).

| Level | Horizontal Tick | Vertical Tick |
| --- | --- | --- |
| **Easy** | 150ms | 300ms |
| **Medium** | 100ms | 200ms |
| **Hard** | 60ms | 120ms |

---

## 🔧 Customization

Want to hack the game? Edit `src/utils/mod.rs` to change:

* `WIDTH` / `HEIGHT`: Change the board dimensions.
* `TICK_RATE`: Speed up or slow down the core loop.
* `COLORS`: Customize the snake's gradient.
---

