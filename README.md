# Number Guessing Game

Welcome to the Number Guessing Game! This is a simple and fun game where the computer randomly selects a number between 1 and 100, and you have to guess it within a limited number of attempts. The game tracks your wins, losses, and best scores, providing an engaging experience each time you play.

## Features

- **Random Number Generation**: The game randomly generates a number between 1 and 100.
- **Limited Attempts**: You have a maximum of 7 attempts to guess the correct number.
- **Real-time Feedback**: The game gives feedback on whether your guess is too low, too high, or correct.
- **Stats Tracking**: Your wins, losses, and best score are recorded and displayed after each game.
- **High Score Saving**: The game saves your best score in a file for future reference.
- **Replay Option**: You can choose to play again or exit the game after each round.

## Requirements

To run this application, you need to have the following:

- Rust installed on your machine. If you haven't installed Rust yet, you can follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

## Installation

1. Clone this repository to your local machine using:
   ```bash
   git clone https://github.com/yourusername/number-guessing-game.git
   cd number-guessing-game
   ```

2. Build the project using Cargo:
   ```bash
   cargo build --release
   ```

3. Run the game:
   ```bash
   cargo run
   ```

## Gameplay Instructions

1. When prompted, enter your guess (a number between 1 and 100).
2. The game will inform you if your guess is too low, too high, or correct.
3. If you guess the correct number, the game will show how many attempts it took.
4. After each game, your stats will be displayed, and you can choose to play again or exit.

## File Structure

- `src/main.rs`: The main game logic and implementation.
- `highscore.txt`: A file that stores the highest score achieved in the game.
- `stats.txt`: A file that records the total number of wins and losses.

## Contributing

Contributions are welcome! If you want to contribute to the project, feel free to fork the repository and submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgements

- Thanks to the Rust community for providing excellent documentation and support.
