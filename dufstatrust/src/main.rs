#![allow(unused)]
use std::io::{Error, ErrorKind};
use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};
use std::env;
use std::process;

macro_rules! str {
    ($expression:expr) => (
        String::from($expression);    
    )
}

#[derive(Debug)]
struct DUFileStruct{
    filepath: PathBuf,
    filesize: u64,
}
struct DUDirStruct{
    dirpath: PathBuf,
    files: Vec<DUFileStruct>,
    dirsize: u64,
}

impl DUDirStruct {
    fn getSize(&mut self){
        for i in self.files.iter(){
            self.dirsize += i.filesize;
        }
    }
    fn readDir(root: &Path) -> Vec<DUDirStruct> {
        let mut dirs = vec![
            DUDirStruct{
                dirpath: root.to_path_buf(),
                files: Vec::new(),
                dirsize: 0,
            }];
        if root.is_dir(){
            if let Ok(entries) = fs::read_dir(root) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        if let Ok(file_type) = entry.file_type() {
                            if file_type.is_dir() {
                                dirs.extend(readDir(&entry.path()));
                            } else {
                              dirs[0].files.push(
                                  DUDirStruct{
                                      filepath: entry.path().to_path_buf(),
                                      filesize: entry.metadata()?.len(),
                                  });
                            }
                        }
                    }
                }
            }
        }
        dirs
    }
}

fn main() {
    let path = match env::args().skip(1).next(){
        Some(arg) => arg,
        None => env::current_dir().unwrap(),
    }
    let path = Path::new(&path);

}
