/*
 * A Rust implementation of wget functionality.
 *
 * Command line tool to download a file via HTTP/S.
 *
 * Author: Pranav Kumar(pmkumar)
 */

extern crate reqwest;

use bytes::Bytes;
use reqwest::Url;

use std::env;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::path::Path;
use std::io::{BufWriter, Write};

fn main() {
    let argv : Vec<String> = env::args().collect();
    let argc : usize = argv.len();

    /* Get URL from command line. */
    let url : &String = match argc {
        2 => {
            &argv[1]
        },
        
        _ => {
            panic!("Error: invalid number of arguments!")
        }
    };

    /* Get file name and contents. */
    let file_name : OsString = get_file_name(url);
    let bytes : Bytes = download(url);

    /* Write bytes to file. */
    create_file(&file_name, &bytes).expect("Error in creating file!");
    println!("Success!");
}

/*
 * Create a file with the given name and contents.
 *
 * @param file_name Name of the file to create.
 * @param contents  Bytes to fill the file with.
 * @return std::io::Result<()> : Ok(()) on success.
 */
fn create_file(file_name : &OsStr, contents : &Bytes) -> std::io::Result<()> {
    /* Write with buffered writer. */
    let mut f = BufWriter::new(fs::File::create(file_name).unwrap());
    f.write(contents)?;
    Ok(())
}

/*
 * Extract the file name from a URL.
 *
 * Note: If no file name specified, defaults to `index.html`.
 *
 * @param url URL to parse.
 * @return OsString : File name.
 */
fn get_file_name(url : &String) -> OsString {
    /* Parse URL. */
    match Url::parse(url) {
        Ok(url) => {

            /* Extract file name. */
            let path_name = url.path().to_string();
            let path = Path::new(&path_name);
            let osstr = path.file_name();

            match osstr {
                /* Convert to OsString. */
                Some(osstrfinal) => osstrfinal.to_os_string(),

                /* Default to index.html. */
                None => OsString::from("index.html")
            }
        },
        _ => panic!("No file name")
    }
}

/*
 * Download file from a URL.
 *
 * TODO: look into async download.
 *
 * @param url URL to download from.
 * @return Bytes : bytes content of file.
 */
fn download(url : &String) -> Bytes {
    /* Block on GET request. */
    if let Ok(response) = reqwest::blocking::get(url) {

        /* Failed to connect. */
        if !response.status().is_success() {
            panic!("Error: request unsuccessful");
        }

        /* Extract bytes. */
        if let Ok(body) = response.bytes() {
            return body;
        }
    }
    panic!("Error while downloading");
}