use gitoid::{HashAlgorithm, GitOid, ObjectType::Blob};
use clap::{Parser, Subcommand};
use std::collections::HashMap;
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    console_subscriber::init();

    match &args.command {
        Commands::Bom { file } => {
            println!("Generating GitBOM for {}", file);
            create_gitbom_directory().await?;
            create_gitbom_file().await?;

            let file = tokio::fs::File::open(file).await?;
            let generated_gitoid = create_gitoid_for_file(file).await?;

            println!("Generated GitOid: {}", generated_gitoid.hash());
            let gitoid_directories = create_gitoid_directory(&generated_gitoid).await?;
            write_gitoid_file(&generated_gitoid, gitoid_directories).await?;
            write_gitbom_file(&generated_gitoid).await?;

            hash_gitbom_file().await?;

            Ok(())
        },
        Commands::ArtifactTree { directory } => {
            println!("Generating GitBOM for {}", directory);
            create_gitbom_directory().await?;
            create_gitbom_file().await?;
            
            let mut count = 0;

            // Generate GitOids for every file within directory
            // Then add to GitBom
            for entry in WalkDir::new(directory) {
                let entry_clone = entry?.clone();

                if entry_clone.file_type().is_dir() {
                    continue;
                } else {
                    let file = File::open(entry_clone.path()).await?;
                    let generated_gitoid = create_gitoid_for_file(file).await?;
                    println!("Generated GitOid: {}", generated_gitoid.hash());
                    let gitoid_directories = create_gitoid_directory(&generated_gitoid).await?;
                    write_gitoid_file(&generated_gitoid, gitoid_directories).await?;
                    write_gitbom_file(&generated_gitoid).await?;
                    count += 1;
                }
                
            }

            hash_gitbom_file().await?;
            println!("Generated GitBom for {} files", count);
            Ok(())
        }
    }
}

async fn create_gitbom_directory() -> std::io::Result<()> {
    let directory_path = format!("{}/{}", GITBOM_DIRECTORY, OBJECTS_DIRECTORY);
    fs::create_dir_all(directory_path).await?;
    Ok(())
}

async fn create_gitbom_file() -> std::io::Result<()> {
    let file_path = format!("{}/gitbom_temp", GITBOM_DIRECTORY);
    File::create(file_path).await?;
    Ok(())
}

async fn create_gitoid_for_file(file: File) -> Result<GitOid, gitoid::Error> {
    let file_length = file.metadata().await?.len();
    let res = GitOid::new_from_async_reader(HashAlgorithm::Sha256, Blob, file, file_length as usize).await?;
    Ok(res)
}

async fn create_gitoid_directory(gitoid: &GitOid) -> std::io::Result<HashMap<String, String>> {
    let mut gitoid_directory = gitoid.hash();

    // split off everything into a new string
    // except for the first 2 chars
    let rest_of_gitoid = gitoid_directory.split_off(2);
    let directory_path = format!("{}/{}/{}", GITBOM_DIRECTORY, OBJECTS_DIRECTORY, gitoid_directory);

    fs::create_dir_all(directory_path).await?;

    let directory_strings = HashMap::from([
      (String::from("gitoid_shard"), gitoid_directory),
      (String::from("rest_of_gitoid"), rest_of_gitoid)
    ]);

    Ok(directory_strings)
}

async fn write_gitoid_file(gitoid: &GitOid, gitoid_directories: HashMap<String, String>) -> std::io::Result<()> {
    let mut gitoid_file = File::create(gitoid_file_path(gitoid_directories)).await?;
    let gitoid_blob_string = format!("blob {}\n", gitoid.hash());
    gitoid_file.write_all(gitoid_blob_string.as_bytes()).await?;
    Ok(())
}

fn gitoid_file_path(gitoid_directories: HashMap<String, String>) -> String {
    return format!("{}/{}/{}/{}", GITBOM_DIRECTORY, OBJECTS_DIRECTORY, gitoid_directories["gitoid_shard"], gitoid_directories["rest_of_gitoid"]);
}

async fn write_gitbom_file(gitoid: &GitOid) -> std::io::Result<()> {
    let gitbom_file_path = format!("{}/gitbom_temp", GITBOM_DIRECTORY);
    let mut gitbom_file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(gitbom_file_path).await?;
    let gitoid_blob_string = format!("blob {}\n", gitoid.hash());
    gitbom_file.write_all(gitoid_blob_string.as_bytes()).await?;
    Ok(())
}

async fn hash_gitbom_file() -> Result<(), gitoid::Error> {
    let gitbom_file_path = format!("{}/gitbom_temp", GITBOM_DIRECTORY);
    let gitbom_file = File::open(&gitbom_file_path).await?;
    let gitoid = create_gitoid_for_file(gitbom_file).await?;

    println!("GitOid for GitBOM file: {}", gitoid.hash());

    let gitoid_directories = create_gitoid_directory(&gitoid).await?;

    let new_file_path = gitoid_file_path(gitoid_directories);
    fs::copy(&gitbom_file_path, new_file_path).await?;

    fs::remove_file(gitbom_file_path).await?;
    Ok(())
}
