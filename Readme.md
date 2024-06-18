# Magic-Eight-Cli

This project is a simple command-line application written in Rust that simulates the concepts behing the classic Magic Eight Ball toy, except on a command line interface, and with no ball. Ask any yes/no question, and the Magic-Eight-Cli will provide you with a randomized response.


## Features

- Provides 38 responses categorized into positive, non-committal, and negative answers.
- Freely adjust those responses as you see fit prior to compiling for your own fun usage!
- Written in Rust for funsies.

## Getting Started

### Prerequisites

To build and run this project, you need to have Rust installed on your machine. If you don't have Rust installed, you can get it from [here](https://www.rust-lang.org/tools/install).

### Installation

1. Clone the repository:

```bash
git clone https://github.com/stopthatrightmeow/magic-eight-cli.git
cd magic-eight-cli
```

2. Build the project:

```bash
cargo build --release
```

3. Run the application:

```bash
./target/release/magic-eight-cli
```

## Usage

Run the application and type your name, then your yes or no question at the prompt. Press Enter to receive a response from the Magic-Eight-Cli.

_Example:_

```bash
$ ./target/release/magic-eight-cli
```

_Example Output:_
```
What is your name?
stopthatrightmeow
What do you want to know?
Will it rain tomorrow?
stopthatrightmeow asks Will it rain tomorrow?

Don't count on it
```

## Responses

The Magic-Eight-Cli can respond with one of the following:

Positive Responses:

- "Yes, definitely"
- "It is certain"
- "Without a doubt"
- "Yes, absolutely"
- "You may rely on it"
- "As I see it, yes"
- "Most likely"
- "Outlook good"
- "Yes"
- "Signs point to yes"
- "Affirmative"
- "Absolutely"
- "Indubitably"
- "Positively"
- "Certainly"
- "Yes, indeed"
- "Sure thing"
- "All signs say yes"
- "Decidedly so"
- "No doubt about it"

Non-committal Responses:

- "Reply hazy, try again"
- "Ask again later"
- "Better not tell you now"
- "Cannot predict now"
- "Concentrate and ask again"

Negative Responses:

- "Don't count on it"
- "My reply is no"
- "My sources say no"
- "Outlook not so good"
- "Very doubtful"
- "Absolutely not"
- "Definitely not"
- "Negative"
- "Unlikely"
- "Chances aren't good"
- "I wouldn't bet on it"
- "Don't hold your breath"
- "My intuition says n

## Disclaimer

This product, magic-eight-cli, is not affiliated with, endorsed by, or sponsored by the makers of the Magic Eight Ball®. Magic Eight Ball® is a registered trademark of Mattel, Inc. This tool is a generic decision-making application designed to provide yes/no answers and is independently developed.