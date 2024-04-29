pub mod main_code;

use crate::main_code::core::game::Game;
use clap::Parser;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the game
    #[arg(short, long)]
    game: String,

    /// Use GUI?
    #[arg(short, long, default_value_t = true)]
    use_gui: bool,

    /// Turn pause
    #[arg(short = 'p', long, default_value_t = 0)]
    turn_pause: i8,

    /// Seed for random number generation
    #[arg(short, long)]
    seed: Option<u64>,
}

fn main() {
    let args = Args::parse();

    let seed = args.seed.unwrap_or_else(|| {
        // Get current time in milliseconds since the UNIX epoch
        let elapsed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        elapsed.as_millis() as u64 // Convert to u64
    });

    println!("Game: {}", args.game);
    println!("Use GUI: {}", args.use_gui);
    println!("Turn pause: {}", args.turn_pause);
    println!("Seed: {}", seed);

    let game = Game::new();
    game.main();

    // ... rest of your program
}
