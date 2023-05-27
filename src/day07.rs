use std::cell::RefCell;
use std::collections::HashMap;
use std::error;
use std::fs;
use std::rc::Rc;

use nom::bytes::complete::{tag, take_while};
use nom::character::complete::{alphanumeric1, multispace1};
use nom::combinator::map_res;
use nom::sequence::preceded;
use nom::IResult;

type DirectoryHandle = Rc<RefCell<Directory>>;

struct FileSystem {
    root: DirectoryHandle,
    // current: DirectoryHandle,
}

struct Directory {
    name: String,
    parent: Option<DirectoryHandle>,
    children: HashMap<String, DirectoryHandle>,
    files: HashMap<String, File>,
}

struct File {
    name: String,
    size: usize,
}

fn debug(directory: DirectoryHandle) {
    debug_recursive(directory, 0);
}

fn debug_recursive(directory: DirectoryHandle, indent: usize) {
    print!("{}", &" ".repeat(indent));
    println!("({})", directory.borrow().name);
    for (_, file) in directory.borrow().files.iter() {
        println!("{}{} {}", &" ".repeat(indent + 4), file.name, file.size);
    }
    for (_ ,sub_directory)in directory.borrow().children.iter() {
        debug_recursive(Rc::clone(&sub_directory), indent + 4);
    }
}

impl FileSystem {

    fn init() -> (FileSystem, DirectoryHandle) {
        // let root = Directory {
        //     name: String::from("/"),
        //     parent: None,
        //     children: HashMap::new(),
        //     files: HashMap::new(),
        // };
        let filesystem = FileSystem {
            root: Rc::new(RefCell::new(Directory {
                name: String::from("/"),
                parent: None,
                children: HashMap::new(),
                files: HashMap::new(),
            })),
        };
        let root = Rc::clone(&filesystem.root);
        (filesystem, root)
    }

    fn cd(&self, trgt: &str, current: DirectoryHandle) -> DirectoryHandle {
        match trgt {
            "/" => Rc::clone(&self.root),
            // unwrap consumes `self` which means the value `Option` is moved; a dereferenced value cannot be moved
            // as_ref casts `Option<DirectoryHandle>` to `Option<&DirectoryHandle>`
            ".." => Rc::clone(current.borrow().parent.as_ref().unwrap()),
            _ => Rc::clone(current.borrow().children.get(trgt).unwrap()),
        }
    }
    
    fn mkdir(trgt: &str, current: &DirectoryHandle) {
        current.borrow_mut().children.entry(String::from(trgt)).or_insert(
            Rc::new(RefCell::new(Directory {
                name: String::from(trgt),
                parent: Some(Rc::clone(&current)),
                children: HashMap::new(),
                files: HashMap::new(),
            }))
        );
    }

    fn add_file(trgt: &str, size: usize, current: &DirectoryHandle) {
        let name = String::from(trgt);
        current.borrow_mut().files.insert(name.clone(), File { name, size });
    }
}

fn parse_cd(line: &str) -> Option<&str> {
    let result: IResult<_,_>= tag("$ cd ")(line);
    result.ok().map(|(directory, _)| directory)
}

fn parse_ls(line: &str) -> Option<&str> {
    let result: IResult<_,_>= tag("$ ls")(line);
    result.ok().map(|(_, empty)| empty)
}

fn parse_directory(line: &str) -> Option<&str> {
    let result: IResult<_,_> = preceded(tag("dir "), alphanumeric1)(line);
    result.ok().map(|(_, directory)| directory)
}

fn parse_file(line: &str) -> Option<(usize, &str)> {
    let result: IResult<_,_> = map_res(take_while(is_digit), |s: &str| {s.parse::<usize>()})(line);
    if let Some((line, size)) = result.ok() {
        let result: IResult<_,_> = multispace1(line); 
        return result.ok().map(|(file_name, _)| (size, file_name));
    }
    None
}

fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

fn collect_sizes(current: DirectoryHandle, sizes: &mut Vec<usize>, limit: usize) -> usize {
    let mut total_size: usize = current.borrow().files.values().map(|f| f.size).sum(); 
    for (_, dir) in current.borrow().children.iter() {
        total_size += collect_sizes(Rc::clone(dir), sizes, limit);
    }
    if total_size <= limit {
        sizes.push(total_size);
    }
    total_size
}

pub fn part1(file_path: &str) -> Result<usize, Box<dyn error::Error>> {
    let puzzle_input = fs::read_to_string(file_path)?;
    let terminal_output = puzzle_input.split('\n');
    let (filesystem, mut current) = FileSystem::init();

    for line in terminal_output {
        if let Some(directory) = parse_cd(line) {
            current = filesystem.cd(directory, current);
        } else if let Some(_) = parse_ls(line) {
            continue;
        } else if let Some(directory) = parse_directory(line){
            FileSystem::mkdir(directory, &current);
        } else if let Some((size, file)) = parse_file(line) {
            FileSystem::add_file(file, size, &current);
        } else {
            panic!("TODO")
        }
    }

    let root = filesystem.cd("/", current);
    debug(Rc::clone(&root));
    let mut sizes = Vec::new();
    collect_sizes(root, &mut sizes, 100000);
    let mut total_size = 0;
    for size in sizes {
        total_size += size;
    }
    Ok(total_size)
}