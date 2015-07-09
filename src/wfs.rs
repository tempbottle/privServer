
#![macro_use]
#![allow(unused)]
use std::path::Path;
use std::fs;
use std::io;
use std::io::Read;
use std::collections::HashMap;

// this is for loading the all the files in the root of the website directory for quick
// access during requests

// defiantly don't put the server keys in the website folder

// use Bin to hold any type of file. Use Text to store data that you want to process as a string
// eg. .html or .css before responding to requests
pub enum WebFile {
    Text(String),
    Bin(Vec<u8>),
}

pub struct WebFs {
    pub map: HashMap<String, WebFile>,
}

//pub struct DirNode {
//    pub name : String, // the name of this directory
//    pub files : Vec<FileNode>,
//    pub sub_dir : Vec<DirNode>,
//}
//
//pub struct FileNode {
//    pub name : String,
//    pub data : WebFile,
//}

// using this a lot. Maybe not the vest but if anything does fail while trying to read
// the website directory then it is useless to keep going. So I try to provide info
// to help determine what caused the error.
//
// Unwrapping errors is really kind of annoying but I can see where it is a good thing
macro_rules! walk_dir_err_chk {
    ( $x:expr ) => {
        $x.unwrap_or_else( |err| {
            println!("Error reading website files to memory on line {}. Error: {}", line!(), err);
            panic!();
        })
    }
}

impl WebFs {

    pub fn new(path: &Path) -> WebFs {

        let mut website = WebFs {
            map: HashMap::new(),
        };

        WebFs::walk_dir_helper(path, &mut website);

        website

    }

    pub fn get(&self, key: &String) -> Option<&WebFile> {
        if key == "/" {
            self.map.get("/index.html")
        } else {
            self.map.get(key) 
        }
    }

    fn walk_dir_helper(path: &Path, website: &mut WebFs) {

        let metadata = walk_dir_err_chk!(fs::metadata(path));
        if metadata.is_dir() {

            let dir = walk_dir_err_chk!(fs::read_dir(path));
            for entry in dir {

                let entry = walk_dir_err_chk!(entry);
                let entry_metadata = walk_dir_err_chk!(fs::metadata(entry.path()));
                println!("{:?}", entry.path());

                if entry_metadata.is_dir() {

                    WebFs::walk_dir_helper(&entry.path(), website);

                } else {
                    // right now just takeing binary.
                    // but will leter check for text format and also check for more errors

                    let mut file = walk_dir_err_chk!(fs::File::open(entry.path()));
                    let mut buf = Vec::with_capacity(metadata.len() as usize);
                    walk_dir_err_chk!(file.read_to_end(&mut buf));
                    println!("file loaded");

                    if let Some(file_name) = entry.path().to_str() {
                        let file_name_string = file_name[7..].to_string();
                        website.map.insert(file_name_string, WebFile::Bin(buf));
                    } else {
                        println!("The path converted to an empty string. Weird file name?");
                        println!("{:?}", entry.path());
                        panic!();
                    }

                }
            }
        }
    }

}
