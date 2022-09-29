use gitoid::{HashAlgorithm, GitOid, ObjectType::Blob};
use clap::{Parser, Subcommand};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Write};
use std::fs;
use walkdir::WalkDir;

const GITBOM_DIRECTORY: &str = ".bom";
const OBJECTS_DIRECTORY: &str = "objects";

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
            create_gitbom_file()?;

            let file = File::open(file)?;
            let generated_gitoid = create_gitoid_for_file(file);

            match generated_gitoid {
                Ok(gitoid) => {
                    println!("Generated GitOid: {}", gitoid.hash());
                    let gitoid_directories = create_gitoid_directory(&gitoid)?;
                    write_gitoid_file(&gitoid, gitoid_directories)?;
                    write_gitbom_file(&gitoid)?;
                },
                Err(e) => println!("Error generating the GitBOM: {:?}", e),
            }

            hash_gitbom_file()?;

            Ok(())
        },
        Commands::ArtifactTree { directory } => {
            println!("Generating GitBOM for {}", directory);
            create_gitbom_directory()?;
            create_gitbom_file()?;
            
            let mut count = 0;

            let mut gitoids_collected: Vec<GitOid> = Vec::new();

            // Generate GitOids for every file within directory
            for entry in WalkDir::new(directory) {
                let entry_clone = entry?.clone();

                if entry_clone.file_type().is_dir() {
                    continue;
                } else {
                    let file = File::open(entry_clone.path())?;
                    let generated_gitoid = create_gitoid_for_file(file);
                    match generated_gitoid {
                        Ok(gitoid) => {
                            println!("Generated GitOid: {}", gitoid.hash());
                            let gitoid_directories = create_gitoid_directory(&gitoid)?;
                            write_gitoid_file(&gitoid, gitoid_directories)?;
                            gitoids_collected.push(gitoid);
                            //write_gitbom_file(&gitoid)?;
                            count += 1;
                        },
                        Err(e) => println!("Error generating the GitBOM: {:?}", e),
                    }
                }
                
            }

           for gitoid in gitoids_collected {
               write_gitbom_file(&gitoid)?;
           } 

            hash_gitbom_file()?;
            println!("Generated GitBom for {} files", count);
            Ok(())
        }
    }
}

fn create_gitbom_directory() -> std::io::Result<()> {
    let directory_path = format!("{}/{}", GITBOM_DIRECTORY, OBJECTS_DIRECTORY);
    fs::create_dir_all(directory_path)?;
    Ok(())
}

fn create_gitbom_file() -> std::io::Result<()> {
    let file_path = format!("{}/gitbom_temp", GITBOM_DIRECTORY);
    File::create(file_path)?;
    Ok(())
}

fn create_gitoid_for_file(file: File) -> Result<GitOid, gitoid::Error> {
    let file_length = file.metadata()?.len();
    let reader = BufReader::new(file);
    GitOid::new_from_reader(HashAlgorithm::Sha256, Blob, reader, file_length as usize)
}

fn create_gitoid_directory(gitoid: &GitOid) -> std::io::Result<HashMap<String, String>> {
    let mut gitoid_directory = gitoid.hash().as_hex();

    // split off everything into a new string
    // except for the first 2 chars
    let rest_of_gitoid = gitoid_directory.split_off(2);
    let directory_path = format!("{}/{}/{}", GITBOM_DIRECTORY, OBJECTS_DIRECTORY, gitoid_directory);

    fs::create_dir_all(directory_path)?;

    let directory_strings = HashMap::from([
      (String::from("gitoid_shard"), gitoid_directory),
      (String::from("rest_of_gitoid"), rest_of_gitoid)
    ]);

    Ok(directory_strings)
}

fn write_gitoid_file(gitoid: &GitOid, gitoid_directories: HashMap<String, String>) -> std::io::Result<()> {
    let mut gitoid_file = File::create(gitoid_file_path(gitoid_directories))?;
    let gitoid_blob_string = format!("blob {}\n", gitoid.hash());
    gitoid_file.write_all(gitoid_blob_string.as_bytes())?;
    Ok(())
}

fn gitoid_file_path(gitoid_directories: HashMap<String, String>) -> String {
    return format!("{}/{}/{}/{}", GITBOM_DIRECTORY, OBJECTS_DIRECTORY, gitoid_directories["gitoid_shard"], gitoid_directories["rest_of_gitoid"]);
}

fn write_gitbom_file(gitoid: &GitOid) -> std::io::Result<()> {
    let gitbom_file_path = format!("{}/gitbom_temp", GITBOM_DIRECTORY);
    let mut gitbom_file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(gitbom_file_path)?;
    let gitoid_blob_string = format!("blob {}\n", gitoid.hash());
    gitbom_file.write_all(gitoid_blob_string.as_bytes())?;
    Ok(())
}

fn hash_gitbom_file() -> Result<(), gitoid::Error> {
    let gitbom_file_path = format!("{}/gitbom_temp", GITBOM_DIRECTORY);
    let gitbom_file = File::open(&gitbom_file_path)?;
    let gitoid = match create_gitoid_for_file(gitbom_file) {
        Ok(gitoid) => gitoid,
        Err(e) => return Err(e)
    };

    println!("GitOid for GitBOM file: {}", gitoid.hash());

    let gitoid_directories = create_gitoid_directory(&gitoid)?;

    let new_file_path = gitoid_file_path(gitoid_directories);
    fs::copy(&gitbom_file_path, new_file_path)?;

    fs::remove_file(gitbom_file_path)?;
    Ok(())
}
