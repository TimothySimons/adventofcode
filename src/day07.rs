use std::cell::RefCell;
use std::collections::HashMap;
use std::error;
use std::fs;
use std::rc::Rc;

type DirectoryHandle = Rc<RefCell<Directory>>;

struct FileSystem {
    root: DirectoryHandle,
    current: DirectoryHandle,
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

impl FileSystem {
    fn new() -> Self {
        let root = Rc::new(RefCell::new(Directory {
            name: String::from("/"),
            parent: None,
            children: HashMap::new(),
            files: HashMap::new(),
        }));
        let current = Rc::clone(&root);
        Self { root, current }
    }

    fn cd(&mut self, trgt: &str) {
        self.current = match trgt {
            "/" => Rc::clone(&self.root),
            // unwrap consumes `self` which means the value `Option` is moved; a dereferenced value cannot be moved
            // as_ref casts `Option<DirectoryHandle>` to `Option<&DirectoryHandle>`
            ".." => Rc::clone(self.current.borrow().parent.as_ref().unwrap()),
            _ => Rc::clone(self.current.borrow().children.get(trgt).unwrap()),
        }
    }

    fn mkdir(&mut self, trgt: &str) {
        self.current
            .borrow_mut()
            .children
            .entry(String::from(trgt))
            .or_insert(Rc::new(RefCell::new(Directory {
                name: String::from(trgt),
                parent: Some(Rc::clone(&self.current)),
                children: HashMap::new(),
                files: HashMap::new(),
            })));
    }

    fn add_file(&mut self, trgt: &str, size: usize) {
        let name = String::from(trgt);
        self.current
            .borrow_mut()
            .files
            .insert(name.clone(), File { name, size });
    }

    #[allow(dead_code)]
    fn debug(directory: &DirectoryHandle) {
        Self::debug_recursive(directory, 0);
    }

    #[allow(dead_code)]
    fn debug_recursive(directory: &DirectoryHandle, indent: usize) {
        print!("{}", &" ".repeat(indent));
        println!("({})", directory.borrow().name);
        for (_, file) in directory.borrow().files.iter() {
            println!("{}{} {}", &" ".repeat(indent + 4), file.name, file.size);
        }
        for (_, sub_directory) in directory.borrow().children.iter() {
            Self::debug_recursive(sub_directory, indent + 4);
        }
    }
}

mod puzzle_parser {
    use nom::bytes::complete::{tag, take_while};
    use nom::character::complete::{alphanumeric1, multispace1};
    use nom::combinator::map_res;
    use nom::sequence::preceded;
    use nom::IResult;

    pub(crate) fn parse_cd(line: &str) -> Option<&str> {
        let result: IResult<_, _> = tag("$ cd ")(line);
        result.ok().map(|(directory, _)| directory)
    }

    pub(crate) fn parse_ls(line: &str) -> Option<&str> {
        let result: IResult<_, _> = tag("$ ls")(line);
        result.ok().map(|(_, empty)| empty)
    }

    pub(crate) fn parse_directory(line: &str) -> Option<&str> {
        let result: IResult<_, _> = preceded(tag("dir "), alphanumeric1)(line);
        result.ok().map(|(_, directory)| directory)
    }

    pub(crate) fn parse_file(line: &str) -> Option<(usize, &str)> {
        let result: IResult<_, _> = map_res(take_while(is_digit), str::parse)(line);
        if let Some((line, size)) = result.ok() {
            let result: IResult<_, _> = multispace1(line);
            return result.ok().map(|(file_name, _)| (size, file_name));
        }
        None
    }

    fn is_digit(c: char) -> bool {
        c.is_ascii_digit()
    }
}

fn construct_filesystem(terminal_output: &str) -> FileSystem {
    let mut filesystem = FileSystem::new();
    for line in terminal_output.split('\n') {
        if let Some(directory) = puzzle_parser::parse_cd(line) {
            filesystem.cd(directory);
        } else if puzzle_parser::parse_ls(line).is_some() {
            continue;
        } else if let Some(directory) = puzzle_parser::parse_directory(line) {
            filesystem.mkdir(directory);
        } else if let Some((size, file)) = puzzle_parser::parse_file(line) {
            filesystem.add_file(file, size);
        } else {
            panic!("TODO")
        }
    }
    filesystem
}

fn calc_size(filesystem: FileSystem, predicate: impl Fn(&&usize) -> bool) -> usize {
    let mut memory = Vec::new();
    record_sizes_recursive(&filesystem.root, &mut memory);
    memory.iter().filter(predicate).sum()
}

fn record_sizes_recursive(current: &DirectoryHandle, memory: &mut Vec<usize>) -> usize {
    let mut total_size: usize = current.borrow().files.values().map(|f| f.size).sum();
    for (_, dir) in current.borrow().children.iter() {
        total_size += record_sizes_recursive(dir, memory);
    }
    memory.push(total_size);
    total_size
}

pub fn part1(file_path: &str) -> Result<usize, Box<dyn error::Error>> {
    let puzzle_input = fs::read_to_string(file_path)?;
    let filesystem = construct_filesystem(&puzzle_input);
    Ok(calc_size(filesystem, |&x| *x <= 100_000))
}
