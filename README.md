# Rust URL Shortener

![Rust](images/Rust.png)

A simple Rust CLI tool to shorten URLs and fetch the original URL. Supports custom code lengths and verbose output.

---

## Legend of Flags

| Flag                               | Description                                                    |
| ---------------------------------- | -------------------------------------------------------------- |
| `-v`, `--verbose`                  | Print detailed logs about the operation (shortening/fetching). |
| `-l <length>`, `--length <length>` | Specify the length of the shortened URL code (default: 6).     |
| `> <file>`                         | Redirect output to a file instead of printing to terminal.     |

---

## Installation

1. Clone the repository:

```bash
git clone https://github.com/AlexKrechmer/rust_url_shortener.git
cd rust_url_shortener
Build the project:

bash
Copy
Edit
cargo build --release
Usage
Shorten a URL
Shorten a URL with default length (6):

bash
Copy
Edit
cargo run -- shorten "https://github.com/AlexKrechmer/Rust_A24_Quiz"
Verbose output:

bash
Copy
Edit
cargo run -- shorten -v "https://github.com/AlexKrechmer/Rust_A24_Quiz"
Custom length (8 characters):

bash
Copy
Edit
cargo run -- shorten -v -l 8 "https://github.com/AlexKrechmer/Rust_A24_Quiz"
Redirect output to a file:

bash
Copy
Edit
cargo run -- shorten -v -l 8 "https://github.com/AlexKrechmer/Rust_A24_Quiz" > output.txt
Fetch the Original URL
Fetch the original URL using the shortened code:

bash
Copy
Edit
cargo run -- fetch <code>
Verbose fetch:

bash
Copy
Edit
cargo run -- fetch -v <code>
Redirect fetch output to a file:

bash
Copy
Edit
cargo run -- fetch -v <code> > fetch_output.txt
Examples
Shorten with verbose, length 8:

mathematica
Copy
Edit
Verbose: shortening URL `https://github.com/AlexKrechmer/Rust_A24_Quiz` with code length 8
Shortened URL: X9aBcD7q
Fetch using the code X9aBcD7q:

mathematica
Copy
Edit
Verbose: fetching URL for code `X9aBcD7q`
Original URL: https://github.com/AlexKrechmer/Rust_A24_Quiz
Notes
The shortened URL code is now randomly generated.

Redirecting with > captures standard output.

To capture verbose logs too, redirect stderr:

bash
Copy
Edit
cargo run -- shorten -v -l 8 "<URL>" > output.txt 2>&1
```
