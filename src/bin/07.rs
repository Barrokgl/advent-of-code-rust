use std::{cell::RefCell, rc::Rc};

struct Folder {
    pub parent: Option<Rc<RefCell<Folder>>>,
    pub name: String,
    pub files: Vec<Rc<RefCell<File>>>,
    pub folders: Vec<Rc<RefCell<Folder>>>,
    pub size: i32,
}

struct File {
    pub size: i32,
    pub name: String,
}

impl File {
    fn new(size: i32, name: String) -> File {
        File { size, name }
    }
}

impl Folder {
    fn new(name: String, parent: Option<Rc<RefCell<Folder>>>) -> Folder {
        Folder {
            name,
            parent,
            files: vec![],
            folders: vec![],
            size: 0,
        }
    }

    fn add_file(&mut self, file: Rc<RefCell<File>>) {
        self.files.push(Rc::clone(&file));

        let file_ref = file.as_ref().borrow();
        self.size = self.size + file_ref.size;
    }

    fn add_folder(&mut self, folder: Rc<RefCell<Folder>>) {
        self.folders.push(Rc::clone(&folder));
    }

    fn get_folder_size(&self) -> i32 {
        self.size
            + self
                .folders
                .iter()
                .map(|folder| folder.borrow().get_folder_size())
                .sum::<i32>()
    }

    fn get_total_size(&self, threshhold: i32) -> i32 {
        let folder_size = match self.get_folder_size() {
            n if n < threshhold => n,
            _ => 0,
        };

        folder_size
            + self
                .folders
                .iter()
                .map(|folder| folder.borrow().get_total_size(threshhold))
                .sum::<i32>()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let threshhold = 100000;
    let root_folder = Rc::new(RefCell::new(Folder::new("/".to_string(), None)));
    let mut curr_folder = Rc::clone(&root_folder);
    for line in input.lines() {
        match line {
            s if s.starts_with("$ cd /") => { // skip root folder
            }
            s if s.starts_with("$ cd") => {
                if s == "$ cd .." {
                    let current_clone = Rc::clone(&curr_folder);
                    curr_folder = Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
                } else {
                    let folder_name = s.replace("$ cd ", "");
                    let new_folder = Folder::new(
                        folder_name.trim().to_string(),
                        Some(Rc::clone(&curr_folder)),
                    );
                    let new_folder_ref = Rc::new(RefCell::new(new_folder));
                    let new_folder_copy = Rc::clone(&new_folder_ref);
                    curr_folder.borrow_mut().add_folder(new_folder_ref);

                    curr_folder = new_folder_copy;
                }
            }
            s if s.starts_with("$ ls") => {}
            s if s.starts_with("dir") => {}
            s => {
                let tokens = s.split(" ").collect::<Vec<_>>();
                let size = tokens
                    .first()
                    .and_then(|size| size.parse::<i32>().ok())
                    .unwrap();
                let name = tokens.last().map(|s| s.trim()).unwrap();
                let file = Rc::new(RefCell::new(File::new(size, String::from(name))));
                curr_folder.borrow_mut().add_file(file);
            }
        }
    }
    let sum = root_folder.borrow().get_total_size(threshhold) as u32;
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
