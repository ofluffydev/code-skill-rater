# Code Skill Rater

Code Skill Rater is a tool that evaluates a GitHub user's programming skill level based on the content of their repositories' README files. It uses AI to analyze the README files and provides a rating along with suggestions for improvement.

## Features

- Fetches repositories and README files for a given GitHub user.
- Uses AI to analyze README content and infer the user's programming skill level.
- Provides a rating (e.g., Beginner, Intermediate, Expert) and suggestions for improvement.

## Prerequisites

- Rust (edition 2024)
- A valid `.env` file with the following keys:
  - `ACCOUNT_ID`: Your Cloudflare account ID.
  - `API_KEY`: Your Cloudflare API key.
  - `RUST_LOG`: Set to `info` for logging.

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/code-skill-rater.git
   cd code-skill-rater
   ```

2. Install dependencies:
   ```bash
   cargo build
   ```

3. Create a `.env` file in the root directory with the required keys:
   ```dotenv
   ACCOUNT_ID=your_account_id
   API_KEY=your_api_key
   RUST_LOG=info
   ```

## Usage

1. Run the program with a GitHub username as an argument:
   ```bash
   cargo run -- <github_username>
   ```

2. Example:
   ```bash
   cargo run -- fluffydev
   ```

3. The program will output a rating and suggestions based on the user's repositories.

## Development

- To run the tests:
  ```bash
  cargo test
  ```

- To enable detailed logging, set `RUST_LOG=debug` in your `.env` file.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.

## License

This project is licensed under the GNU AGPL License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with Rust and powered by Cloudflare's AI API.
- Inspired by the need to evaluate programming skills based on documentation quality.
