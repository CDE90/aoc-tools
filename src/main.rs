use chrono::Datelike;
use clap::{Parser, Subcommand, ValueEnum};
use reqwest::blocking::Client;
use std::{fmt::Display, process::Command, str::FromStr};

#[derive(Parser)]
#[command(name = "aoc")]
#[command(about="Tools for Advent of Code", long_about=None)]
struct Aoc {
    #[command(subcommand)]
    command: Commands,
}

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
enum Language {
    #[clap(name = "rs")]
    Rust,
    #[clap(name = "ts")]
    TypeScript,
    #[clap(name = "py")]
    Python,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true, visible_alias = "n")]
    New {
        /// Day to create
        #[arg(short, long, value_parser = valid_day)]
        day: u8,

        /// Year to create
        #[arg(short, long, value_parser = valid_year)]
        year: Option<u16>,

        /// Language to use
        #[arg(short, long,default_value_t=Language::TypeScript, default_missing_value="ts", value_enum)]
        lang: Language,
    },

    #[command(arg_required_else_help = true, visible_alias = "r")]
    Run {
        /// Day to run
        #[arg(short, long, value_parser = valid_day)]
        day: u8,

        /// Language to use
        #[arg(short, long, default_value_t=Language::TypeScript, default_missing_value="ts", value_enum)]
        lang: Language,
    },
}

fn main() {
    let args = Aoc::parse();

    let py_template =
        String::from_utf8_lossy(include_bytes!("../templates/template.py")).to_string();
    let rs_template =
        String::from_utf8_lossy(include_bytes!("../templates/template.rs")).to_string();
    let ts_template =
        String::from_utf8_lossy(include_bytes!("../templates/template.ts")).to_string();
    let cargo_template =
        String::from_utf8_lossy(include_bytes!("../templates/template.Cargo.toml")).to_string();

    match args.command {
        Commands::New { day, year, lang } => {
            let year = year
                .unwrap_or_else(|| chrono::Local::now().year() as u16)
                .to_string();
            let day = day.to_string();

            std::fs::create_dir_all(&format!("{}", day)).unwrap();

            let aoc_session = dotenv::var("AOC_SESSION");
            match aoc_session {
                Ok(aoc_session) => {
                    if !std::path::Path::new(&format!("{}/input.txt", day)).exists() {
                        let input = Client::new()
                            .get(&format!(
                                "https://adventofcode.com/{}/day/{}/input",
                                year, day
                            ))
                            .header("Cookie", format!("session={}", aoc_session))
                            .send()
                            .unwrap()
                            .text()
                            .unwrap();

                        std::fs::write(format!("{}/input.txt", day), input.trim()).unwrap();
                    }
                }
                Err(e) => {
                    println!("AOC_SESSION not found in .env file: {}", e);
                }
            }

            match lang {
                Language::Rust => {
                    let path = format!("{}/solution.rs", day);
                    let template = rs_template.replace("{day}", &day);
                    std::fs::write(&path, template).unwrap();
                    println!("Created {}", path);
                    let cargo_toml_path = "Cargo.toml";
                    if !std::path::Path::new(&cargo_toml_path).exists() {
                        let cargo_toml = cargo_template.replace("{year}", &year);
                        std::fs::write(&cargo_toml_path, cargo_toml).unwrap();
                        println!("Created {}", cargo_toml_path);
                    }
                    let mut cargo_toml = std::fs::read_to_string(&cargo_toml_path).unwrap();
                    cargo_toml.push_str(&format!(
                        "\n[[bin]]\nname = \"aoc-{}\"\npath = \"{}/solution.rs\"\n",
                        day, day
                    ));
                    std::fs::write(&cargo_toml_path, cargo_toml).unwrap();
                    println!("Added {} to {}", path, cargo_toml_path);
                }
                Language::TypeScript => {
                    let path = format!("{}/solution.ts", day);
                    let template = ts_template.replace("{day}", &day);
                    std::fs::write(&path, template).unwrap();
                    println!("Created {}", path);
                }
                Language::Python => {
                    let path = format!("{}/solution.py", day);
                    let template = py_template.replace("{day}", &day);
                    std::fs::write(&path, template).unwrap();
                    println!("Created {}", path);
                }
            }
        }
        Commands::Run { day, lang } => {
            let day = day.to_string();

            match lang {
                Language::Rust => {
                    let path = format!("{}/solution.rs", day);
                    if !std::path::Path::new(&path).exists() {
                        println!("{} does not exist", path);
                        return;
                    }
                    let output = Command::new("cargo")
                        .arg("run")
                        .arg("--bin")
                        .arg(format!("aoc-{}", day))
                        .output()
                        .unwrap()
                        .stdout;
                    println!("{}:\n{}", path, String::from_utf8(output).unwrap().trim());
                }
                Language::TypeScript => {
                    let path = format!("{}/solution.ts", day);
                    if !std::path::Path::new(&path).exists() {
                        println!("{} does not exist", path);
                        return;
                    }
                    let output = Command::new("deno")
                        .arg("run")
                        .arg("-A")
                        .arg(format!("./{}/solution.ts", day))
                        .output()
                        .unwrap()
                        .stdout;
                    println!("{}:\n{}", path, String::from_utf8(output).unwrap().trim());
                }
                Language::Python => {
                    let path = format!("{}/solution.py", day);
                    if !std::path::Path::new(&path).exists() {
                        println!("{} does not exist", path);
                        return;
                    }
                    let output = Command::new("python")
                        .arg(format!("./{}/solution.py", day))
                        .output()
                        .unwrap()
                        .stdout;
                    println!("{}:\n{}", path, String::from_utf8(output).unwrap().trim());
                }
            }
        }
    }
}

// const RELEASE_TIMEZONE_OFFSET: i32 = -5 * 60 * 60;

fn convert_number<T: FromStr>(s: &str) -> Result<T, String>
where
    <T as FromStr>::Err: Display,
{
    s.parse::<T>().map_err(|err| format!("{}", err))
}

fn valid_day(s: &str) -> Result<u8, String> {
    convert_number(s).and_then(|day| {
        if is_valid_day(day) {
            Ok(day)
        } else {
            Err("invalid Advent of Code day".to_string())
        }
    })
}

fn valid_year(s: &str) -> Result<u16, String> {
    convert_number(s).and_then(|day| {
        if is_valid_year(day) {
            Ok(day)
        } else {
            Err("invalid Advent of Code year".to_string())
        }
    })
}

fn is_valid_year(year: u16) -> bool {
    year >= 2015
}

fn is_valid_day(day: u8) -> bool {
    (1..=25).contains(&day)
}
