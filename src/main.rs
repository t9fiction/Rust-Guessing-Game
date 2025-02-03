use rand::Rng;
use std::cmp::Ordering;
use std::fs::{self, OpenOptions};
use std::io::{self, Read, Write};
use std::time::Instant;

#[derive(Debug)]
struct GameStats {
    wins: u32,
    losses: u32,
    best_score: Option<u32>, // Best score in least attempts
}

impl GameStats {
    fn new() -> Self {
        let (wins, losses) = load_win_loss_stats();
        let best_score = load_best_score();
        GameStats {
            wins,
            losses,
            best_score,
        }
    }

    fn update_best_score(&mut self, attempts: u32) {
        if self.best_score.is_none() || attempts < self.best_score.unwrap() {
            self.best_score = Some(attempts);
            save_best_score(attempts);
            println!("🎉 New high score: {} attempts!", attempts);
        }
    }

    fn save_stats(&self) {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("stats.txt")
            .expect("Failed to open stats file");

        writeln!(file, "{} {}", self.wins, self.losses).expect("Failed to write stats");
    }

    fn display_stats(&self) {
        println!("📊 Updated Stats - Wins: {} | Losses: {}", self.wins, self.losses);
    }
}

fn main() {
    let mut stats = GameStats::new();

    println!("🏆 High Score: {:?} attempts", stats.best_score.unwrap_or(0));
    println!("📊 Wins: {} | Losses: {}", stats.wins, stats.losses);
    println!("Welcome to the Number Guessing Game!");

    loop {
        play_game(&mut stats);

        // Save stats after each game
        stats.save_stats();

        // Show updated stats immediately after the game
        stats.display_stats();

        // Ask if the player wants to play again
        loop {
            println!("Do you want to play again? (yes/y to start, no/n to exit)");
            let mut choice = String::new();
            io::stdin().read_line(&mut choice).expect("Failed to read input");

            let choice = choice.trim().to_lowercase();
            if choice == "yes" || choice == "y" {
                break; // Restart game
            } else if choice == "no" || choice == "n" {
                println!("Thanks for playing! 🎮");
                return;
            } else {
                println!("❌ Invalid input! Please enter yes/y or no/n.");
            }
        }
    }
}

fn play_game(stats: &mut GameStats) {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut attempts = 0;
    let max_attempts = 7;
    let start_time = Instant::now();

    println!("I'm thinking of a number between 1 and 100. Can you guess it?");

    loop {
        println!("Please input your guess. (Remaining tries: {}/{})", max_attempts - attempts, max_attempts);

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("❌ Please enter a valid number");
                continue;
            }
        };

        attempts += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("📉 Too small!"),
            Ordering::Greater => println!("📈 Too big!"),
            Ordering::Equal => {
                let elapsed_time = start_time.elapsed();
                println!(
                    "🎉 Congratulations! You guessed the number in {} attempts ({}s)!",
                    attempts,
                    elapsed_time.as_secs_f32()
                );

                stats.wins += 1;
                stats.update_best_score(attempts);
                return;
            }
        }

        if attempts == max_attempts {
            println!("💀 You've used all {} attempts! The number was {}. You lose!", max_attempts, secret_number);
            stats.losses += 1;
            return;
        }
    }
}

fn save_best_score(score: u32) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("highscore.txt")
        .expect("Failed to open file");

    writeln!(file, "{}", score).expect("Failed to write to file");
}

fn load_best_score() -> Option<u32> {
    if let Ok(mut file) = OpenOptions::new().read(true).open("highscore.txt") {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            return contents.trim().parse().ok();
        }
    }
    None
}

fn load_win_loss_stats() -> (u32, u32) {
    if let Ok(mut file) = OpenOptions::new().read(true).open("stats.txt") {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            let parts: Vec<&str> = contents.trim().split_whitespace().collect();
            if parts.len() == 2 {
                if let (Ok(wins), Ok(losses)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
                    return (wins, losses);
                }
            }
        }
    }
    (0, 0) // Default if no file exists
}
