use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// Advent date (1-25)
    #[arg(short, long)]
    date: u8,

    /// Part (1 or 2)
    #[arg(short, long)]
    part: u8,
}

fn main() {
    let args = Args::parse();
    if 1 <= args.date && args.date <= 25 {
        match advent::run(2023, args.date, args.part) {
            Ok(res) => println!("Day {}, Part {}: {}", args.date, args.part, res),
            Err(err) => eprintln!("{}", err),
        }
    } else {
        println!("Invalid date {}, expected number 1-25", args.date);
        std::process::exit(1)
    }
}
