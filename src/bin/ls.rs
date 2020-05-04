/*
 * A Rust implementation of ls functionality.
 *
 * Command line tool to display files in a directory.
 *
 * Author: Pranav Kumar(pmkumar)
 */
use colored::*;
use std::env;
use std::fs;
use std::path::Path;
use std::io;

fn main() {
    let argv : Vec<String> = env::args().collect();
    let argc : usize = argv.len();
    
    /* Get directory from command line. */
    let temp = ".".to_string();
    let dir : &String = match argc {
        1 => {
            &temp
        },
        
        2 => {
            &argv[1]
        },
        
        _ => {
            panic!("Error: invalid number of arguments!")
        }
    };
    
    list_dir(Path::new(&dir)).expect("Unable to list directory");
}


/*
 * List the contents of a given directory.
 *
 * @param dir Directory to list.
 * @return std::io::Result<()> : Ok(()) on success.
 */
fn list_dir(dir : &Path) -> io::Result<()> {
    let mut s = String::from("");
    if dir.is_dir() {
        /* Output each entry in directory. */
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let name = path.file_name()
                .unwrap();

            if path.is_dir() {
                /* Format dirs blue. */
                let formatted = format!("{}", name.to_str().unwrap())
                    .blue()
                    .bold();

                s.push_str(&format!("{}    ", formatted));
                
            } else {
                let formatted = format!("{}", name.to_str().unwrap());
                s.push_str(&format!("{}    ", formatted));
            }
        }
        println!("{}", s.trim());
    } else {
        /* If not dir, print file name. */
        println!("{}", dir.to_str().unwrap());
    }
    Ok(())
}
