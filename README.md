# RegexRustler

## Overview
RegexRustler is a command-line interface (CLI) tool written in Rust designed for highlighting regex patterns in text files. Leveraging Rust's powerful regex and colored crates, it provides a fast and user-friendly way to visualize regex matches in your terminal.

## Features
- Efficient regex pattern matching within text files.
- Colorful highlighting of matched regex patterns in terminal output.
- User-friendly CLI for easy interaction.
- Customizable highlighting options for different regex patterns.
- Fast and reliable processing, thanks to Rust's performance.

## Getting Started

### Prerequisites
- Ensure you have the Rust programming language installed. [Rust Installation Guide](https://www.rust-lang.org/tools/install).

### Installation
Clone the RegexRustler repository and build the project:
```bash
git clone https://github.com/eyallampel1/RegexRustler.git
cd RegexRustler
cargo build --release
```

### Usage
To use RegexRustler, run it with a file path and a regex pattern:
```bash
cargo run --release <file_path> <regex_pattern>
```
Example:
```bash
cargo run --release sample.txt "\\w+"
```

## Contributing
Contributions to RegexRustler are warmly welcomed. Feel free to fork the repository, make changes, and submit pull requests. You can also create issues for bugs or feature suggestions.

## License
RegexRustler is made available under the [MIT License](LICENSE).

## Acknowledgements
- Special thanks to the Rust community for the excellent programming language and tools.
- Appreciation to the authors of the `regex` and `colored` crates used in this project.
Remember to customize any part of this README to better fit your project specifics, and ensure all links and references are accurate and relevant to your project.
