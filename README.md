# gitbom-rs-cli

Experimental CLI for generating a gitbom document

This uses the (also experimental) [gitbom-rs](https://github.com/git-bom/gitbom-rs) library.

This project was heavily based on [fkautz/gitbom-cli](https://github.com/fkautz/gitbom-cli) (a Go implementation of a GitBOM CLI).

**This is very much still a work in progress!**

## What is GitBOM?

To quote [the GitBOM website](https://gitbom.dev/):

```
GitBOM is a minimalistic scheme for build tools to:
1. Build a compact artifact tree, tracking every source code file incorporated into each build artifact
2. Embed a unique, content addressable reference for that artifact tree, the GitBOM identifier, into the artifact at build time
```

For information, see [the website](https://gitbom.dev/) and the [list of GitBOM resources](https://gitbom.dev/resources/)

## Commands

```
gitbom-cli bom <path_to_file> # Creates a gitbom for a single file
gitbom-cli artifact-tree # Creates a gitbom for all files in a directory
```

## Usage

```bash
$ git clone git@github.com:nellshamrell/gitbom-rs-cli.git
$ cd gitbom-rs-cli
$ cargo build
```

### Generating a GitBOM for a specific file

```bash
$ ./target/debug/gitbom-cli bom src/main.rs
```

Feel free to substitute any file path you want to for src/main.rs

### Generating a GitBOM for a directory

```bash
$ ./target/debug/gitbom-cli artifact-tree src
```

Feel free to substitute any directory path you want to for src.

## Where do the generated GitBOMs live?

Look in `directory_you_ran_the_cli_in/.bom/object/`
