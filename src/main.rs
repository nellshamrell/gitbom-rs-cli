use gitbom::GitBom;
use gitoid::{HashAlgorithm, GitOid, ObjectType::Blob};
use clap::{Parser, Subcommand};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Write};
use std::fs;
use std::path::Path;
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
            create_gitbom_file(HashAlgorithm::Sha1)?;
            create_gitbom_file(HashAlgorithm::Sha256)?;

            let file_contents = fs::read_to_string(file)?;

            generate_and_write_gitoid(&file_contents, HashAlgorithm::Sha1)?;
            generate_and_write_gitoid(&file_contents, HashAlgorithm::Sha256)?;


            hash_gitbom_file(HashAlgorithm::Sha1)?;
            hash_gitbom_file(HashAlgorithm::Sha256)?;

            Ok(())
        },
        Commands::ArtifactTree { directory } => {
            println!("Generating GitBOM for {}", directory);
            create_gitbom_directory()?;
            create_gitbom_file(HashAlgorithm::Sha1)?;
            create_gitbom_file(HashAlgorithm::Sha256)?;
            
            let mut count = 0;

            let gitbom = GitBom::new();

            // Generate GitOids for every file within directory
            for entry in WalkDir::new(directory) {
                let entry_clone = entry?.clone();

                if entry_clone.file_type().is_dir() {
                    continue;
                } else {
                    let file_contents = fs::read_to_string(entry_clone.path())?;

//                    let generated_sha1_gitoid = generate_and_write_gitoid(&file_contents, HashAlgorithm::Sha1)?;
                    let generated_gitoid = generate_and_write_gitoid(&file_contents, HashAlgorithm::Sha256)?;
                    gitbom.add(generated_gitoid);
                    count += 1;
                }
                
            }

           for gitoid in gitbom.get_oids() {
               write_gitbom_file(&gitoid, HashAlgorithm::Sha256)?;
           } 

            hash_gitbom_file(HashAlgorithm::Sha1)?;
            hash_gitbom_file(HashAlgorithm::Sha256)?;
            println!("Generated GitBom for {} files", count);
            Ok(())
        }
    }
}

fn create_gitbom_directory() -> std::io::Result<()> {
    let directory_path = format!("{}/{}", GITBOM_DIRECTORY, OBJECTS_DIRECTORY);
    //Check if .bom/objects directory already exists
    let dir_exists: bool = Path::new(&directory_path).is_dir();
    if dir_exists {
        println!("GitBOM directory already exists");
    } else {
        fs::create_dir_all(directory_path)?;
        println!("Created GitBOM directory");
    }
    Ok(())
}

fn create_gitbom_file(hash_algorithm: HashAlgorithm) -> std::io::Result<()> {
    let file_path = format!("{}/gitbom_{}_temp", GITBOM_DIRECTORY, hash_algorithm);
    let mut gitbom_file = File::create(file_path)?;
    let header_text = format!("gitoid:blob:{}\n", hash_algorithm).to_lowercase();
    gitbom_file.write_all(header_text.as_bytes())?;
    Ok(())
}

fn create_gitoid_for_file(file_contents: &str, hash_algorithm: HashAlgorithm) -> GitOid {
    GitOid::new_from_str(hash_algorithm, Blob, file_contents)
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

fn write_gitbom_file(gitoid: &GitOid, hash_algorithm: HashAlgorithm) -> std::io::Result<()> {
    let gitbom_file_path = format!("{}/gitbom_{}_temp", GITBOM_DIRECTORY, hash_algorithm);
    let mut gitbom_file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(gitbom_file_path)?;
    let gitoid_blob_string = format!("blob {}\n", gitoid.hash());
    gitbom_file.write_all(gitoid_blob_string.as_bytes())?;
    Ok(())
}

fn hash_gitbom_file(hash_algorithm: HashAlgorithm) -> Result<(), gitoid::Error> {
    let gitbom_file_path = format!("{}/gitbom_{}_temp", GITBOM_DIRECTORY, hash_algorithm);
    let file_contents = fs::read_to_string(gitbom_file_path.clone())?;
    let generated_gitoid = create_gitoid_for_file(&file_contents, hash_algorithm);

    println!("GitOid for {:?} GitBOM file: {}", generated_gitoid.hash_algorithm(), generated_gitoid.hash());

    let gitoid_directories = create_gitoid_directory(&generated_gitoid)?;

    let new_file_path = gitoid_file_path(gitoid_directories);
    fs::copy(&gitbom_file_path, new_file_path)?;

    fs::remove_file(gitbom_file_path)?;
    Ok(())
}

fn generate_and_write_gitoid(file_contents: &str, hash_algorithm: HashAlgorithm) -> std::io::Result<GitOid> {
    let generated_gitoid = create_gitoid_for_file(&file_contents, hash_algorithm);
    println!("Generated {:?} GitOid: {}", generated_gitoid.hash_algorithm(), generated_gitoid.hash());
    let gitoid_directories = create_gitoid_directory(&generated_gitoid)?;

    write_gitoid_file(&generated_gitoid, gitoid_directories)?;
    write_gitbom_file(&generated_gitoid, generated_gitoid.hash_algorithm())?;

    Ok(generated_gitoid)
}
