use gitbom::{HashAlgorithm, GitOid};
use clap::Parser;
use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Parser)]
struct Cli {
    path: String
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    println!("args: {:?}", args.path);

    let file = File::open(args.path)?;
    println!("file: {:?}", file);

    let reader = BufReader::new(file);

    let new_gitoid = GitOid {
        hash_algorithm: HashAlgorithm::SHA1
    };

    let result = new_gitoid.generate_git_oid_from_buffer(reader, 11);
    println!("result: {:?}", result);

    Ok(())
}
