//A command-line tool to play Marco Polo
use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Changhao Li", about = "A Marco Polo game")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Changhao Li")]
    Marco {
        #[clap(short, long)]
        name: String,
    },
}

fn main() {
    let args = Cli::parse();
    if let Some(Commands::Marco { name }) = args.command {
        let result = hello::marco_polo(&name);
        println!("{}", result);
    } else {
        println!("No command was given.")
    }
}
