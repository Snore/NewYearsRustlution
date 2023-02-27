#![allow(unused)]

use clap::Parser;
use anyhow::{Context, Result};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli
{
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf
}

#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<()> {
    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // let path = std::env::args().nth(2).expect("no path given");

    let args = Cli::parse();
    print!("You want me to search for this pattern: {} in this file: {}", args.pattern, args.path.display());
    
    // let result = std::fs::read_to_string(&args.path);

    // let content = match result 
    // {
    //     Ok(content) => { content },
    //     Err(error) => { panic!("Can't deal with {}, just exit here", error); }   
    // };

    // or

    // let content = std::fs::read_to_string(&args.path).unwrap();

    // but we don't need to panic and exit!
    // let content = match result 
    // {
    //     Ok(content) => { content },
    //     Err(error) => { return Err(error.into()); }   
    // };
    
    // or we can use the `?`
    // let content = std::fs::read_to_string(&args.path)?;

    // or we can use a CustomError defined by us
    let path = &args.path;
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Could not read file `{}`", path.display()))?;

    println!("\nResults:\n");
    // print the line when key word is found
    for line in content.lines()
    {
        if line.contains(&args.pattern)
        {
            println!("{}", line);
        }
    }

    Ok(())
}
