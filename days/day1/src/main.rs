use clap::Parser;
use std::fs::read_to_string;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    input: String,    
}

fn main() {

    let args = Args::parse();
    println!("Args: {:?}", args);
    std::env::set_current_dir("./days/day1");

    let file = read_to_string(args.input)
        .expect("Could not read file");

    let mut totals: Vec<i32> = vec![];
    let mut current = 0;
    for line in file.lines() {
        if line == "" {
            totals.push(current);
            current = 0;
        } else {
            current += line.parse::<i32>().unwrap();            
        }
    }
    totals.push(current);
    current = 0;

    // Part 1
    let max = totals.iter().max().unwrap();
    println!("Part 1 Max: {}", max);

    // Part 2
    totals.sort_by(|a, b| b.cmp(a));
    let top3: i32 = totals.iter().take(3).sum();
    println!("Part 2: {}", top3);

}
