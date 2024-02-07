# Arkham API

Welcome to Arkham API! This Rust project is designed to help you manage and categorize cards related to the Arkham Horror: The Card Game using Rust. It provides functionalities for initialization and searching cards based on different criteria.

## Table of Contents

- [Introduction](#introduction)
- [Setup](#setup)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## Introduction

Arkham API is structured with the following modules:

- `api`: Contains functionality related to interacting with APIs.
- `config`: Holds configuration settings.
- `handlers`: Provides handlers for different actions.
- `models`: Defines data models used in the project.
- `service`: Implements core business logic and services.
- `types`: Contains type definitions.
- `utils`: Houses utility functions.

## Setup

To set up Arkham API locally, follow these steps:

1. Clone the repository to your local machine.
2. Ensure you have Rust installed. You can install it from [the official Rust website](https://www.rust-lang.org/tools/install).
3. Navigate to the project directory in your terminal.
4. Run `cargo build` to build the project.
5. Once built, you can run the project using `cargo run`.

## Usage

Arkham API provides a command-line interface (CLI) for interacting with its functionalities. Here are the available commands:

- `init`: Initializes the project.
- `search`: Searches for cards based on specified criteria.

To use the CLI:

1. Run the project using `cargo run`.
2. Follow the prompts to enter a command (`init` or `search`) and provide necessary inputs.

## Contributing

Contributions are welcome! If you have any ideas, suggestions, or improvements, feel free to open an issue or submit a pull request.

## License

Arkham API is licensed under the [MIT License](LICENSE).
