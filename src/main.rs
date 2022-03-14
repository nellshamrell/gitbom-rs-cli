use gitbom::{HashAlgorithm, GitOid};
use clap::Parser;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufReader, Read};

#[derive(Parser)]
struct Cli {
    path: String
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let file = File::open(args.path)?;
    let file_length = file.metadata()?.len();

    let reader = BufReader::new(file);
    let new_gitoid = GitOid::new(HashAlgorithm::SHA1);


    let result = new_gitoid.generate_git_oid_from_buffer(reader, file_length as usize);
    println!("result: {:?}", result.unwrap());

    Ok(())
}
