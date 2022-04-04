use gitbom::{HashAlgorithm, GitOid};
use clap::{Parser, Subcommand};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Write};
use std::fs;
use walkdir::WalkDir;

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
            create_gitbom_directory()?;

            let file = File::open(file)?;
            let generated_gitoid = create_gitoid_for_file(file);

            match generated_gitoid {
                Ok(gitoid) => {
                    println!("Generated GitOid: {}", gitoid.hex_hash());
                    let gitoid_directories = create_gitoid_directory(&gitoid)?;
                    write_gitoid_file(&gitoid, gitoid_directories)?;
                },
                Err(e) => println!("Error generating the GitBOM: {:?}", e),
            }

            Ok(())
        },
        Commands::ArtifactTree { directory } => {
            println!("Generating GitBOM for {}", directory);
            create_gitbom_directory()?;
            
            let mut count = 0;

            // Generate GitOids for every file within directory
            // Then add to GitBom
            for entry in WalkDir::new(directory) {
                let file = File::open(entry?.path())?;
                let generated_gitoid = create_gitoid_for_file(file);
                match generated_gitoid {
                    Ok(gitoid) => {
                        println!("Generated GitOid: {}", gitoid.hex_hash());
                        let gitoid_directories = create_gitoid_directory(&gitoid)?;
                        write_gitoid_file(&gitoid, gitoid_directories)?;
                        count += 1;
                    },
                    Err(e) => println!("Error generating the GitBOM: {:?}", e),
                }
            }

            // TODO: tGenerate GitOid for the GitBom file itself?
            println!("Generated GitBom for {} files", count);
            Ok(())
        }
    }
}

fn create_gitbom_directory() -> std::io::Result<()> {
    let directory_path = String::from(".bom/object");
    fs::create_dir_all(directory_path)?;
    Ok(())
}

fn create_gitoid_for_file(file: File) -> Result<GitOid, std::io::Error> {
    let file_length = file.metadata()?.len();
    let reader = BufReader::new(file);
    GitOid::new_from_reader(HashAlgorithm::SHA256, reader, file_length as usize)
}

fn create_gitoid_directory(gitoid: &GitOid) -> std::io::Result<HashMap<String, String>> {
    let mut gitoid_directory = gitoid.hex_hash();

    // split off everything into a new string
    // except for the first 2 chars
    let rest_of_gitoid = gitoid_directory.split_off(2);
    let directory_path = format!(".bom/object/{}", gitoid_directory);

    fs::create_dir_all(directory_path)?;

    let directory_strings = HashMap::from([
      (String::from("gitoid_shard"), gitoid_directory),
      (String::from("rest_of_gitoid"), rest_of_gitoid)
    ]);

    Ok(directory_strings)
}

fn write_gitoid_file(gitoid: &GitOid, gitoid_directories: HashMap<String, String>) -> std::io::Result<()> {
    let file_path = format!(".bom/object/{}/{}", gitoid_directories["gitoid_shard"], gitoid_directories["rest_of_gitoid"]);

    let mut gitoid_file = File::create(file_path)?;
    let gitoid_blob_string = format!("blob {}", gitoid.hex_hash());
    gitoid_file.write_all(gitoid_blob_string.as_bytes())?;
    Ok(())
}

