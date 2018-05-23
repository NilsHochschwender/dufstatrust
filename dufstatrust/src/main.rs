#![allow(unused)]
use std::io::{Error, ErrorKind};
use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};
use std::env;
use std::process;
use std::cell::RefCell;
use std::rc::Rc;
use std::fmt;

macro_rules! str {
    ($expression:expr) => (
        String::from($expression);    
    )
}

#[derive(Debug)]


//struct for the cli-options
struct CLIOptions{
    NULline: bool,
    all: bool,
    apparentSize: bool,
    blockSize: String,
    bytes: bool,
    total: bool,
    dereferenceArgs: String,
    maxDepth: i32,
    files0FromF: String,
    H: bool,
    humanReadable: bool,
    inodes: bool,
    kilobyte: bool,
    dereference: bool,
    countLinks: bool,
    megabyte: bool,
    noDereference: bool,
    separateDirs: bool,
    si: bool,
    summarize: bool,
    threshhold: i64,
    time: bool,
    timeWord: String,
    timeStyle: String,
    excludeFromFile: PathBuf,
    exclude: String,
    oneFileSystem: bool,
}
impl Default for CLIOptions {
    fn default() -> CLIOptions {
        let mut block = str!("1024");
        let keys = ["DU_BLOCK_SIZE", "BLOCK_SIZE", "BLOCKSIZE"];
        if let Ok(val) = env::var(keys[0]) {
            block = str!(val);
        }
        if let Ok(val) = env::var(keys[1]) {
            block = str!(val);
        }
        if let Ok(val) = env::var(keys[2]) {
            block = str!(val);
        }
        CLIOptions{
            NULline: false, 
            all: false,
            apparentSize: false,
            blockSize: str!(block),
            bytes: false,
            total: false,
            dereferenceArgs: str!(""),
            maxDepth: -1,
            files0FromF: str!(""),
            H: false,
            humanReadable: false,
            inodes: false,
            kilobyte: false,
            dereference: false,
            countLinks: false,
            megabyte: false,
            noDereference: true,
            separateDirs: true,
            si: false,
            summarize: false,
            threshhold: 0,
            time: false,
            timeWord: str!(""),
            timeStyle: str!(""),
            excludeFromFile: PathBuf::new(),
            exclude: str!(""),
            oneFileSystem: false,
        }
    }
}
//for the FileInformations
#[derive(Debug, Clone)]
struct DUFileStruct{
    filepath: PathBuf,
    filesize: u64,
}
//struct for Dir Informations and the included Files
#[derive(Debug)]
struct DUDirStruct{
    dirpath: PathBuf,
    files: Vec<DUFileStruct>,
    dirsize: u64,
    subdirs: Rc<RefCell<Option<Vec<DUDirStruct>>>>,
}
impl DUDirStruct {

    fn getSize(&mut self){
        match self.subdirs.borrow_mut().as_mut(){
            Some(x) => {
                for i in x.iter_mut(){
                    i.getSize();
                    self.dirsize += i.dirsize;
                }
            }
            None => {}
        }
        for i in self.files.iter(){
             self.dirsize += i.filesize;       
        }
    }
    
}
impl fmt::Display for DUDirStruct{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn goIntoStruct(root: &DUDirStruct) -> Vec<DUDirStruct> {
            let mut dirs: Vec<DUDirStruct> = vec![DUDirStruct{
                dirpath: root.dirpath.clone().to_path_buf(),
                files: root.files.clone(),
                dirsize: root.dirsize,
                subdirs: Rc::new(RefCell::new(None)),
            }];
            match root.subdirs.borrow().as_ref(){
                Some(x) => {
                    for i in x.iter(){
                        dirs.append(&mut goIntoStruct(&i));
                    }
                }
                None => {}
            }
            dirs
        }
        let mut dirs: Vec<DUDirStruct> = goIntoStruct(self);
        dirs.reverse();
        for dir in dirs.iter(){
            writeln!(f, "{}{:10}{}", dir.dirpath.to_str().unwrap_or(""), "", dir.dirsize)?;
            for file in dir.files.iter(){
                writeln!(f, "\t{}{:10}{}",file.filepath.to_str().unwrap_or(""), "", file.filesize)?;
            }
        }
        write!(f, "")
    }
}
fn readDir(root: &Path) -> DUDirStruct {
    let mut dirs = DUDirStruct{
            dirpath: root.to_path_buf(),
            files: Vec::new(),
            dirsize: 0,
            subdirs: Rc::new(RefCell::new(Some(Vec::new()))),
        };
    let mut inserts: i32 = 0;
    if root.is_dir(){
        if let Ok(entries) = fs::read_dir(root) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(file_type) = entry.file_type() {
                        if file_type.is_dir() {
                            match dirs.subdirs.borrow_mut().as_mut(){
                                Some(x) =>  {
                                    x.push(readDir(&entry.path()));
                                    inserts += 1;
                                },
                                None => {},
                            }
                        } else {
                            if let Ok(metadata) = entry.metadata(){
                                dirs.files.push(
                                DUFileStruct{
                                    filepath: entry.path().to_path_buf(),
                                    filesize: metadata.len(),
                                });
                            }
                        }
                    }
                }
            }
        }
    }
    if inserts == 0{
        let waste = dirs.subdirs.replace(None);
    }
    dirs
}


fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();
    //let path =
    //    if Path::new(&args[args.len() - 1]).is_dir() == true {
    //        Path::new(&args.pop().unwrap()).to_path_buf()
    //    } else {
    //        env::current_dir().unwrap().to_path_buf()
    //    };
    let path = env::current_dir().unwrap();
    let mut clio: CLIOptions = Default::default();
    let mut dirs = readDir(&path);
    dirs.getSize();
    println!("{}", dirs);
}
