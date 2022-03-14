use gitbom::{HashAlgorithm, GitOid};
use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::{BufReader};
use std::fs;

/// A GitBom CLI written in Rust
#[derive(Parser)]
#[clap(name = "gitbom")]
#[clap(about = "A CLI for creating GitBom documents", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Creates a GitBOM for a single file
    #[clap(arg_required_else_help = true)]
    Bom {
        /// File to generate a GitBOM for
        file: String
    },

    /// Creates a GitBOM for a directory
    #[clap(arg_required_else_help = true)]
    ArtifactTree {
        /// Directory to generate a GitBOM for
        directory: String
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    match &args.command {
        Commands::Bom { file } => {
            println!("Generating GitBOM for {}", file);
            let file = File::open(file)?;
            let file_length = file.metadata()?.len();
            let reader = BufReader::new(file);
            let new_gitoid = GitOid::new(HashAlgorithm::SHA1);


            let result = new_gitoid.generate_git_oid_from_buffer(reader, file_length as usize);

            match result {
                Ok(r) => {
                    create_bom_directory()?;
                    println!("{}", r);
                    
                },
                Err(e) => println!("Error generating the GitBOM: {:?}", e),
            }

            Ok(())
        },
        Commands::ArtifactTree { directory } => {
            println!("Generating GitBOM for {}", directory);
            println!("Not implemented yet. Patience.");
            Ok(())
        }
    }
}

fn create_bom_directory() -> std::io::Result<()> {
    fs::create_dir(".bom")?;
    Ok(())
}
