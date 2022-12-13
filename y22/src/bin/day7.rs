use std::{cell::RefCell, collections::HashMap, rc::Rc};

fn main() {
    let file_content = std::fs::read_to_string("res/day7.txt").unwrap();
    let root = Dir::new_ref("/", None);
    let mut current = root.clone();
    let mut dirs = vec![];
    let mut it = file_content.lines().peekable();
    while it.peek().is_some() {
        let line = it.next().unwrap();
        let command = line.split_whitespace().collect::<Vec<_>>();
        match command[1] {
            "cd" => {
                let name = command[2];
                current = match name {
                    "/" => root.clone(),
                    ".." => current.borrow().parent.clone().unwrap(),
                    _ => current.borrow().sub_dirs.get(name).unwrap().clone(),
                };
            }
            "ls" => {
                while it.peek().is_some() && !it.peek().unwrap().starts_with("$") {
                    let entry = it.next().unwrap().split_whitespace().collect::<Vec<_>>();
                    let name = entry[1];
                    match entry[0] {
                        "dir" => {
                            let sub_dir = Dir::new_ref(name, Some(current.clone()));
                            dirs.push(sub_dir.clone());
                            current
                                .borrow_mut()
                                .sub_dirs
                                .insert(name.to_owned(), sub_dir);
                        }
                        _ => {
                            let size = entry[0].parse::<u64>().expect("invalid entry");
                            current.borrow_mut().files.insert(name.to_owned(), size);
                        }
                    }
                }
            }
            _ => panic!("Unknown command: {line}"),
        }
    }
    let mut sizes = dirs
        .into_iter()
        .map(|d| d.borrow().total_size())
        .collect::<Vec<_>>();
    let size = sizes.iter().filter(|s| **s <= 100000).sum::<u64>();
    println!("{size}");

    let total = 70000000;
    let required = 30000000;
    let free = total - root.borrow().total_size();
    let need = required - free;
    sizes.sort();
    let remove = sizes.iter().find(|s| **s >= need).unwrap();
    println!("{remove}");
}

type DirRef = Rc<RefCell<Dir>>;

struct Dir {
    #[allow(unused)]
    name: String,
    files: HashMap<String, u64>,
    sub_dirs: HashMap<String, DirRef>,
    parent: Option<DirRef>,
}

impl Dir {
    fn new(name: &str, parent: Option<DirRef>) -> Self {
        Dir {
            name: name.to_owned(),
            files: HashMap::new(),
            sub_dirs: HashMap::new(),
            parent,
        }
    }
    fn new_ref(name: &str, parent: Option<DirRef>) -> DirRef {
        Rc::new(RefCell::new(Dir::new(name, parent)))
    }

    fn total_size(&self) -> u64 {
        self.files.values().sum::<u64>()
            + self
                .sub_dirs
                .values()
                .map(|d| d.borrow().total_size())
                .sum::<u64>()
    }
}
