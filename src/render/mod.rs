//! UI and rendering module for the Snake game.
//! Handles all terminal-based graphics and user interface elements.

use crate::core::Game;
use std::io::Write;

pub fn draw(game: &Game) {
    // Clear screen
    print!("\x1b[2J");

    // Draw top border
    print!("\x1b[1;1H┌"); // Top-left corner (starting at 1,1 to account for 1-indexing)
    for x in 2..game.width {
        print!("\x1b[1;{}H─", x);
    }
    print!("\x1b[1;{}H┐", game.width); // Top-right corner

    // Draw bottom border
    print!("\x1b[{};1H└", game.height); // Bottom-left corner
    for x in 2..game.width {
        print!("\x1b[{};{}H─", game.height, x);
    }
    print!("\x1b[{};{}H┘", game.height, game.width); // Bottom-right corner

    // Draw left and right borders
    for y in 2..game.height {
        print!("\x1b[{};1H│", y); // Left border
        print!("\x1b[{};{}H│", y, game.width); // Right border
    }

    // Draw snake
    for (i, pos) in game.snake.body.iter().enumerate() {
        // Head is bright green, body segments get darker toward the tail
        let color = if i == 0 {
            "\x1b[92m" // Bright green for head
        } else if i < game.snake.body.len() / 3 {
            "\x1b[32m" // Regular green for front segments
        } else if i < game.snake.body.len() * 2 / 3 {
            "\x1b[33m" // Yellow for middle segments
        } else {
            "\x1b[90m" // Dark gray for tail segments
        };

        print!("\x1b[{};{}H{}", pos.y, pos.x, color);

        // Different symbols for head and body, with head indicating direction
        if i == 0 {
            // Head symbol depends on direction for rotation effect
            let head_symbol = match game.snake.direction {
                crate::utils::Direction::Up | crate::utils::Direction::Down => "█", // Vertical orientation
                crate::utils::Direction::Left | crate::utils::Direction::Right => "█", // Same symbol but conceptually rotated
            };
            print!("{}", head_symbol);
        } else {
            print!("■"); // Smaller block for body
        }
    }

    // Draw food with different symbols based on score
    let food_symbol = if game.score.is_multiple_of(50) && game.score != 0 {
        "★"
    } else {
        "●"
    };
    print!(
        "\x1b[{};{}H\x1b[91m{}",
        game.food.y, game.food.x, food_symbol
    ); // Bright red for food

    // Draw power-up if it exists
    if let Some(power_up) = game.power_up {
        let (symbol, color) = match power_up.power_up_type {
            crate::utils::PowerUpType::SpeedBoost => (">", "\x1b[94m"), // Blue for speed boost
            crate::utils::PowerUpType::SlowDown => ("<", "\x1b[96m"),   // Cyan for slow down
            crate::utils::PowerUpType::ExtraPoints => ("$", "\x1b[93m"), // Yellow for extra points
            crate::utils::PowerUpType::Grow => ("+", "\x1b[92m"),       // Green for grow
            crate::utils::PowerUpType::Shrink => ("-", "\x1b[95m"),     // Magenta for shrink
        };
        print!(
            "\x1b[{};{}H{}{}",
            power_up.position.y, power_up.position.x, color, symbol
        );
    }

    // Reset color
    print!("\x1b[0m");

    // Draw score
    print!("\x1b[{};{}HScore: {}", game.height + 2, 2, game.score);

    // Draw difficulty
    let difficulty_text = match game.difficulty {
        crate::utils::Difficulty::Easy => "Difficulty: Easy",
        crate::utils::Difficulty::Medium => "Difficulty: Medium",
        crate::utils::Difficulty::Hard => "Difficulty: Hard",
    };
    print!("\x1b[{};{}H{}", game.height + 2, 15, difficulty_text); // Positioned away from score

    // Draw pause indicator
    if game.is_paused() {
        print!("\x1b[{};{}HPAUSED", game.height + 2, 35); // Positioned away from other texts
    }

    // Draw mute indicator
    if game.muted {
        print!("\x1b[{};{}HMUTED", game.height + 2, 45); // Positioned away from other texts
    }

    // Draw controls reminder - at the bottom, away from other info
    print!(
        "\x1b[{};{}HWASD/Arrows:Move P:Pause M:Mute SPACE:Restart Q:Quit",
        game.height + 4,
        2
    );

    // Draw game over message
    if game.game_over {
        let box_width: usize = 36; // Width of the game over box
        let box_start_x: usize = ((game.width - box_width as u16) / 2) as usize; // Center the box properly

        print!(
            "\x1b[{};{}H╔{}╗",
            (game.height / 2) - 2 + 1, // Adjust for new border position
            box_start_x + 1,           // Adjust for new border position
            "═".repeat(box_width - 2)
        );
        print!(
            "\x1b[{};{}H║{: ^width$}║",
            (game.height / 2) - 1 + 1, // Adjust for new border position
            box_start_x + 1,           // Adjust for new border position
            "GAME OVER!",
            width = box_width - 2
        );
        print!(
            "\x1b[{};{}H║{: ^width$}║",
            (game.height / 2) + 1, // Adjust for new border position
            box_start_x + 1,       // Adjust for new border position
            format!("Score: {}", game.score),
            width = box_width - 2
        );
        print!(
            "\x1b[{};{}H║{: ^width$}║",
            (game.height / 2) + 1 + 1, // Adjust for new border position
            box_start_x + 1,           // Adjust for new border position
            "",
            width = box_width - 2
        );
        print!(
            "\x1b[{};{}H║{: ^width$}║",
            (game.height / 2) + 2 + 1, // Adjust for new border position
            box_start_x + 1,           // Adjust for new border position
            "Press SPACE to play again",
            width = box_width - 2
        );
        print!(
            "\x1b[{};{}H║{: ^width$}║",
            (game.height / 2) + 3 + 1, // Adjust for new border position
            box_start_x + 1,           // Adjust for new border position
            "or 'q' to quit",
            width = box_width - 2
        );
        print!(
            "\x1b[{};{}H╚{}╝",
            (game.height / 2) + 4 + 1, // Adjust for new border position
            box_start_x + 1,           // Adjust for new border position
            "═".repeat(box_width - 2)
        );
    }

    std::io::stdout().flush().unwrap();
}

pub fn draw_menu(selected_option: usize, width: u16, _height: u16) {
    let options = ["Easy", "Medium", "Hard"];

    // Center the entire menu
    let title_box_width = 25;
    let title_start_x = (width - title_box_width as u16) / 2; // Center the title box

    // Clear screen and draw menu
    print!("\x1b[2J"); // Clear screen

    // Title box - centered
    print!(
        "\x1b[8;{}H┌{}┐",
        title_start_x,
        "─".repeat(title_box_width - 2)
    );
    print!(
        "\x1b[9;{}H│{: ^width$}│",
        title_start_x,
        "SNAKE GAME",
        width = title_box_width - 2
    );
    print!(
        "\x1b[10;{}H└{}┘",
        title_start_x,
        "─".repeat(title_box_width - 2)
    );

    // Menu options - centered under the title with color highlighting
    for (i, option) in options.iter().enumerate() {
        let option_center_x = (width - 10) / 2 - 2; // Center the options
        let marker = if selected_option == i { "▶" } else { " " };
        let color = if selected_option == i {
            "\x1b[92m"
        } else {
            "\x1b[97m"
        }; // Green for selected, white for others
        print!(
            "\x1b[{};{}H{}{}{}. {}",
            12 + i,
            option_center_x,
            color,
            marker,
            i + 1,
            option
        );
    }

    // Reset color
    print!("\x1b[0m");

    // Instructions - centered
    let instruction_width = 35;
    let instruction_start_x = (width - instruction_width as u16) / 2;
    print!(
        "\x1b[16;{}HUse ↑↓ arrows or WASD to navigate",
        instruction_start_x
    );
    print!(
        "\x1b[17;{}HPress ENTER to select, Q to quit",
        instruction_start_x
    );

    std::io::stdout().flush().unwrap();
}
