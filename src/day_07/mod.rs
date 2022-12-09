use crate::timer;

pub fn solve() -> () {
    timer::timeit(|| run());
}

#[derive(Debug)]
struct Directory {
    size: u32,
    parent: usize,
}

impl Directory {
    pub fn new(parent: usize) -> Directory {
        Directory { size: 0, parent }
    }
}

fn run() -> () {
    let input: &str = include_str!("__input.txt");

    const ROOT_IDX: usize = 0;

    const FS_SIZE: u32 = 70_000_000;
    const UPDATE_SIZE: u32 = 30_000_000;

    let mut current_dir: usize = ROOT_IDX;
    let mut fs: Vec<Directory> = vec![Directory::new(ROOT_IDX)];

    for line in input.lines() {
        let output: Vec<&str> = line.split_whitespace().collect();

        // cd instructions
        if &output[1] == &"cd" {
            match output[2] {
                "/" => current_dir = ROOT_IDX,
                ".." => current_dir = fs[current_dir].parent,
                _ => {
                    fs.push(Directory::new(current_dir));
                    current_dir = fs.len() - 1;
                }
            }
        }
        // Ignore everything expect files.
        else {
            match &output[0].parse::<u32>() {
                Ok(file_size) => {
                    let mut tmp: usize = current_dir;
                    loop {
                        let dir: &mut Directory = &mut fs[tmp];
                        dir.size += file_size;

                        if tmp == ROOT_IDX {
                            break;
                        }

                        tmp = dir.parent;
                    }
                }
                Err(_) => (),
            }
        }
    }

    let sizes: Vec<u32> = fs.iter().map(|dir: &Directory| dir.size).collect();

    println!(
        "Part 1: {}",
        sizes.iter().filter(|size| *size < &100_000).sum::<u32>()
    );

    let available_space: u32 = FS_SIZE - fs[ROOT_IDX].size;
    let needed_space: u32 = UPDATE_SIZE - available_space;
    let mut min_del_size: u32 = FS_SIZE;

    for size in sizes {
        if size < min_del_size && size > needed_space {
            min_del_size = size;
        }
    }

    println!("Part 2: {}", min_del_size);
}
