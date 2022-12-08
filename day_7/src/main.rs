use std::{
    borrow::Borrow,
    cell::{Ref, RefCell},
    rc::Rc,
};

fn main() {
    process_commands(read_data());
}

// install nom
// parse through the command list
// lines that start with $ are cmds
// everything else is output
// when cd set the working_dir to directory
// when ls add all the files and directories to cwd

fn process_commands(data: &str) {
    let mut root = Rc::new(RefCell::new(Directory {
        name: String::from("/"),
        files: vec![],
        directories: vec![],
        parent: None,
    }));
    let mut node = root.clone();

    for line in data.lines() {
        let mut stroop = line.split(' ');
        let (Some("$"), Some("cd"), Some(directory_name), None) = (stroop.next(), stroop.next(), stroop.next(), stroop.next()) else {
            panic!("aaaaaaaaaaaaaaaah");
        };
        if directory_name == ".." {
            let parent = node.borrow_mut().parent.clone().unwrap();
            node = parent;
        } else {
            let child = node
                .borrow_mut()
                .directories
                .push(Rc::new(RefCell::new(Directory {
                    name: String::from("/"),
                    files: vec![],
                    directories: vec![],
                    parent: Some(node),
                })))
                .clone();
            // node = child;
        }
    }

    // how to use nom
    // set up an alt to either return command or output
    // but I need a common type they both belong to??
}

fn read_data() -> &'static str {
    include_str!("../data/example_data")
}

type NodeHandle = Rc<RefCell<Directory>>;

#[derive(Clone, Default, Debug)]
struct Directory {
    name: String,
    files: Vec<File>,
    directories: Vec<NodeHandle>,
    parent: Option<NodeHandle>,
}

#[derive(Clone, Debug)]
struct File {
    name: String,
    size: u64,
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_part_one_example() {
        assert_eq!(1, 2);
    }
}
