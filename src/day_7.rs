use std::collections::HashMap;

struct Dir {
    dirs: HashMap<String, Dir>,
    files: HashMap<String, usize>,
}

impl Dir {
    fn new() -> Self {
        Dir {
            dirs: HashMap::new(),
            files: HashMap::new(),
        }
    }

    fn add_file(&mut self, path: &Vec<&str>, name: String, size: usize) {
        self.get_dir(path).files.insert(name, size);
    }

    fn get_dir(&mut self, path: &Vec<&str>) -> &mut Dir {
        let cwd = self;
        path.iter()
            .map(|name| name.to_string())
            .fold(cwd, |acc, f| {
                if !acc.dirs.contains_key(&f) {
                    acc.dirs.insert(f.clone(), Dir::new());
                }
                acc.dirs.get_mut(&f).unwrap()
            })
    }

    fn get_size(&self) -> usize {
        let mut sum: usize = 0;
        for file in self.files.iter() {
            sum += file.1;
        }
        for dir in self.dirs.iter() {
            sum += dir.1.get_size();
        }
        sum
    }

    fn get_dirs_smaller_than_n(&self, max_size: usize, sizes: &mut Vec<usize>) -> usize {
        let mut sum: usize = 0;
        for file in self.files.iter() {
            sum += file.1;
        }
        for dir in self.dirs.iter() {
            sum += dir.1.get_dirs_smaller_than_n(max_size, sizes);
        }
        if sum <= max_size {
            sizes.push(sum);
        }
        sum
    }

    fn find_smallest_dir_above_n(&self, min_size: usize, result: &mut usize) {
        for dir in self.dirs.iter() {
            dir.1.find_smallest_dir_above_n(min_size, result);
            let s = dir.1.get_size();
            if s > min_size && s < *result {
                *result = s;
            }
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        println!("--- FS ---");
        self.print_recursive(0);
    }

    fn print_recursive(&self, depth: usize) {
        for dir in self.dirs.iter() {
            for _ in 0..depth * 2 {
                print!(" ");
            }
            println!("- dir {}", dir.0);
            dir.1.print_recursive(depth + 1);
        }
        for file in self.files.iter() {
            for _ in 0..depth * 2 {
                print!(" ");
            }
            println!("- {: <16} {: <8}", file.0, file.1); 
        }
    }
}

pub fn solve() {
    let input = include_str!("input/7.txt");

    let mut fs = Dir::new();

    let mut cwd: Vec<&str> = Vec::new();

    fs.dirs.insert("/".to_string(), Dir::new());

    cwd.push("/");

    input.lines().for_each(|line| {
        if line.starts_with("$ cd") {
            let new_dir = line.split("cd ").last().expect("cd missing arg");
            match new_dir {
                "/" => { cwd.clear(); cwd.push("/") },
                ".." => _ = cwd.pop(),
                _ => cwd.push(new_dir),
            }
        } else if line.starts_with("$ ls") {
            // list beginning
        } else {
            let mut entry = line.split(" ");
            let first = entry.nth(0).unwrap().clone();
            let last = entry.last().unwrap().clone();
            if line.starts_with("dir") {

            } else {
                fs.add_file(&cwd, last.to_string(), first.parse::<usize>().unwrap());
            }
        }
    });

    let mut sizes = Vec::new();
    fs.get_dirs_smaller_than_n(100000, &mut sizes);
    let part1 = sizes.iter().sum::<usize>();

    // fs.print();

    sizes.clear();
    let size_to_clear = 30000000 - (70000000 - fs.get_size());

    // println!("space to clear: {size_to_clear}");
    let mut result: usize = usize::MAX;
    fs.find_smallest_dir_above_n(size_to_clear, &mut result);

    println!("Day  7: {: >10} {: >10}", part1, result);
}