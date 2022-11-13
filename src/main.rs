use clap::Parser;

fn main() {
    let cli = Cli::parse();
    let min_files = cli.number_of_files;
    for arg in cli.directories {
        if is_dir_empty_enough_to_delete(&arg, min_files) {
            println!("delete {}", arg);
            if let Err(_) = std::fs::remove_dir_all(arg.clone()) {
                eprintln!("could not delete {}", arg);
            }
        }
    }
}

#[derive(Parser)]
#[command(name = "dircleaner")]
#[command(author = "seahawk1986 <seahawk1986@gmx.de>")]
#[command(version = "0.0.1")]
#[command(about = "clean directories containing not more than the given number of files")]
#[command(long_about = None)]
struct Cli {
    #[arg(short, long, default_value_t = 2)]
    number_of_files: i32,
    directories: Vec<String>,
}

fn is_dir_empty_enough_to_delete(dir: &String, min_files: i32) -> bool {
    if let Ok(dr) = std::fs::read_dir(dir) {
        let mut n_files = 0;
        let mut preserve_child_dir = false;
        for entry in dr {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    let file_name = path.file_name().unwrap();
                    // ignore current and parent directory
                    if file_name == "." || file_name == ".." {
                        continue;
                    }
                    if let Some(p) = path.to_str() {
                        let delete_dir = is_dir_empty_enough_to_delete(&p.to_string(), min_files);
                        if delete_dir {
                            println!("delete {}", p);
                            if let Err(_) = std::fs::remove_dir_all(path.clone()) {
                                eprintln!("could not delete {}", p);
                                preserve_child_dir = true;
                            }
                            continue;
                        } else {
                            // println!("directory {} should be preserved", p);
                            preserve_child_dir |= !delete_dir;
                        }
                    }
                }
                n_files += 1;
            }
        }
        if n_files > min_files || preserve_child_dir {
            return false;
        }
    } else {
        eprintln!("could not read from directory {}", dir);
        return false;
    }
    true
}
