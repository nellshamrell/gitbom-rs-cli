# gitbom-rs-cli
Experimental CLI for generating a gitbom document

## Commands

```
gitbom-cli bom <path_to_file> # Creates a gitbom for a single file
gitbom-cli artifact-tree # Creates a gitbom for all files in a directory (still to be implemented)
```

## Usage

```bash
$ git clone git@github.com:nellshamrell/gitbom-rs-cli.git
$ cd gitbom-rs-cli
$ cargo build
$ ./target/debug/gitbom-cli bom src/main.rs
```

Feel free to substitute any file you want to generate a gitbom for for src/

## Where do the generated GitBOMs live?

Look in `directory_you_ran_the_cli_in/.bom/object`
