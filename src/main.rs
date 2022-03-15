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
                    write_gitbom(&r)?;
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

fn write_gitbom(gitoid: &str) -> std::io::Result<()> {
    let mut gitoid_directory = String::from(gitoid);
    let rest_of_gitoid = gitoid_directory.split_off(2);

    let directory_path = format!(".bom/object/{}", gitoid_directory);
    fs::create_dir_all(directory_path)?;

    let file_path = format!(".bom/object/{}/{}", gitoid_directory, rest_of_gitoid);
    let mut gitoid_file = File::create(file_path);

    Ok(())
}
