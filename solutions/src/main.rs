use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// Puzzle year
    #[arg(short, long)]
    year: u32,

    /// Advent day (1-25)
    #[arg(short, long)]
    date: u8,

    /// Part (1 or 2)
    #[arg(short, long)]
    part: u8,
}

fn main() {
    let args = Args::parse();
    if ![2023u32, 2024, 2025].contains(&args.year) {
        eprintln!("Unsupported year {}, expected 2023, 2024, or 2025", args.year);
        std::process::exit(1);
    }
    if !(1..=25).contains(&args.date) {
        eprintln!("Invalid date {}, expected number 1-25", args.date);
        std::process::exit(1);
    }

    match solutions::run(args.year, args.date, args.part) {
        Ok(res) => println!("Year {}, Day {}, Part {}: {}", args.year, args.date, args.part, res),
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}
