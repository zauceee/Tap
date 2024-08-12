use std::{env, io};
use std::fs::File;
use std::path::Path;
fn main() {
    let files: Vec<_> = env::args().collect();
    if files.len() == 1 {
     println!(r#"

 88888888888     d8888 8888888b.  
     888        d88888 888   Y88b 
     888       d88P888 888    888 
     888      d88P 888 888   d88P 
     888     d88P  888 8888888P"  
     888    d88P   888 888        
     888   d8888888888 888        
     888  d88P     888 888        
                                  
----------------------------------------
Usage:
tap [file name here] [file name here too if you want]
----------------------------------------

by kodeine<3
     "#)
    }
    for args in &files[1..]  {
        if Path::new(args).exists() {
            println!("{}, already exists, overwrite? (Y/N)", args);
            let mut input = String::new();

            io::stdin().read_line(&mut input).expect("Failed to read line");
            let response = input.trim().to_lowercase();
        match response.as_str() {
        "y" | "yes" => {
            File::create(args).expect("Couldn't make file.");
            continue;
        },
        "n" | "no" => {
            println!("You chose not to continue.");
            return ()
        },
        _ => {
            println!("Invalid response.");
            return ()
        },
        }
        
    } else {
        File::create(args).expect("Couldn't make file.");
    }
    println!("Files created.");
}
}
