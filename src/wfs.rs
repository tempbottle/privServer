
#![allow(unused)]
use std::path::Path;
use std::fs;
use std::io;
use std::io::Read;

// This struct is a node for representing a filsystem.
// It loads all the files for the web page into memory into this structure
//
// It hold a vector of files in the directory the node represents.
// It also holds a vector of other nodes that are subdirectories
// of this node.

pub enum WebFile {
    Text(String),
    Bin(Vec<u8>),
}

pub struct DirNode {
    pub name : String, // the name of this directory
    pub files : Vec<FileNode>,
    pub sub_dir : Vec<DirNode>,
}

pub struct FileNode {
    pub name : String,
    pub data : WebFile,
}

impl DirNode {

    pub fn new(path: &Path) -> DirNode {
        //DirNode { name: String::new(), files: Vec::new(), sub_dir: Vec::new() }
        DirNode::walk_dir_helper(path).unwrap()
    }

    fn walk_dir_helper(path: &Path) -> io::Result<DirNode> {

        let mut dn = DirNode {
            name: path.to_str().unwrap().to_string(),
            files: Vec::new(),
            sub_dir: Vec::new()
        };

        let metadata = try!(fs::metadata(path));

        if metadata.is_dir() {
            for entry in try!(fs::read_dir(path)) {
                let entry = try!(entry);
                let entry_metadata = try!(fs::metadata(entry.path()));
                println!("{:?}", entry.path());
                if entry_metadata.is_dir() {
                    dn.sub_dir.push(try!(DirNode::walk_dir_helper(&entry.path())));
                } else { // right now just takeing binary. but will leter check for text format and also check for more errors
                    let mut file = fs::File::open(entry.path()).unwrap();
                    let mut buf = Vec::new();
                    file.read_to_end(&mut buf).unwrap();
                    println!("file loaded");

                    let file_node = FileNode {
                        name: entry.path().to_str().unwrap().to_string(),
                        data: WebFile::Bin(buf),
                    };

                    dn.files.push(file_node);
                }
            }
            Ok(dn)
        }
        else {
            Err(io::Error::new(io::ErrorKind::InvalidInput, "walk_dir_helper Called with no Directory"))
        }
    }
}
