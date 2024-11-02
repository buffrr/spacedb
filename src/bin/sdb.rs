use clap::{Parser, Subcommand};
use spacedb::{Result, db::Database};

#[derive(Parser)]
#[command(name = "sdb", about = "A utility for interacting with spacedb files")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Prints the root of the tree in the specified .sdb database
    Root {
        /// Path to the .sdb database file
        filename: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Root { filename } => {
            print_tree_root(filename)?;
        }
    }

    Ok(())
}

fn print_tree_root(filename: &str) -> Result<()> {
    // Open the database file
    let db = Database::open(filename)?;

    // Get and print the tree root
    let mut snapshot = db.begin_read()?;
    println!("Tree root: {}", hex::encode(snapshot.root()?));

    Ok(())
}
