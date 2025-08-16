use clap::{Parser, Subcommand};
use rand::{distributions::Alphanumeric, Rng};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Parser)]
#[command(name = "rust_url_shortener")]
#[command(about = "A simple URL shortener", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Shorten a URL
    Shorten {
        /// URL to shorten
        url: String,
        /// Code length
        #[arg(short, long, default_value_t = 6)]
        length: usize,
        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },
    /// Fetch original URL from code
    Fetch {
        /// Code to fetch
        code: String,
        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },
}

struct UrlMap {
    map: HashMap<String, String>,
}

impl UrlMap {
    fn new() -> Self {
        let mut map = HashMap::new();
        if let Ok(mut file) = File::open("url_map.json") {
            let mut data = String::new();
            let _ = file.read_to_string(&mut data);
            map = serde_json::from_str(&data).unwrap_or_default();
        }
        Self { map }
    }

    fn save(&self) {
        if let Ok(mut file) = File::create("url_map.json") {
            let _ = file.write_all(serde_json::to_string(&self.map).unwrap().as_bytes());
        }
    }

    fn shorten(&mut self, url: String, length: usize, verbose: bool) -> String {
        let code: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .map(char::from)
            .collect();

        if verbose {
            println!("Verbose: shortening URL `{}` with code length {}", url, length);
        }

        self.map.insert(code.clone(), url);
        self.save();
        code
    }

    fn fetch(&self, code: &str, verbose: bool) -> Option<&String> {
        if verbose {
            println!("Verbose: fetching URL for code `{}`", code);
        }
        self.map.get(code)
    }
}

fn main() {
    let cli = Cli::parse();
    let mut url_map = UrlMap::new();

    match cli.command {
        Commands::Shorten { url, length, verbose } => {
            let code = url_map.shorten(url, length, verbose);
            println!("Shortened URL: {}", code);
        }
        Commands::Fetch { code, verbose } => {
            if let Some(url) = url_map.fetch(&code, verbose) {
                println!("Original URL: {}", url);
            } else {
                println!("No URL found for code `{}`", code);
            }
        }
    }
}

