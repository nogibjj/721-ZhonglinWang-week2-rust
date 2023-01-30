/* An adding calculator that will take two integer and return the product*/

use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Zhonglin Wang",
    about = "multiply two integers"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Zhonglin Wang", about = "Add two integers")]
    Add {
        #[clap(short, long)]
        x: i32,
        #[clap(short, long)]
        y: i32,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Mul { x, y }) => {
            let product = calculator::mul(x, y);
            println!("Outcome is {}", product);
        }
        None => println!("Unavailable command, please use --help to see available commands"),
    }
}