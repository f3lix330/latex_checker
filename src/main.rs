use regex::Regex;
use std::collections::HashMap;
use std::path::PathBuf;
use std::{env, fs};
use walkdir::WalkDir;
use colored::Colorize;

#[cfg(windows)]
fn enable_virtual_terminal_processing() {
    use winapi_util::console::Console;

    if let Ok(mut term) = Console::stdout() {
        let _ = term.set_virtual_terminal_processing(true);
    }
    if let Ok(mut term) = Console::stderr() {
        let _ = term.set_virtual_terminal_processing(true);
    }
}

fn main() {
    #[cfg(windows)]
    enable_virtual_terminal_processing();

    match env::args().nth(1) {
        Some(wd) => {
            let files = get_all_files(PathBuf::from(&wd), get_list(&wd, String::from("exclude_files")));
            let allowed_words = get_list(&wd, String::from("allow_words"));
            match files {
                Ok(files) => {
                    for file in files {
                        println!("{} {}", "Reading file:".blue(), get_file_name(&file).blue());
                        let result = read_file(&file, &allowed_words);
                        match result {
                            Err(lines) => {
                                for (i, line) in lines {
                                    println!("{}{} {}",i.to_string().yellow(), ":".yellow(), line.yellow());
                                }
                            }
                            Ok(err) => {
                                println!("{}", err.green());
                            }
                        }
                    }
                }
                Err(err) => {
                    println!("{}", err.red());
                }
            }
        }
        None => {
            println!("{}", "No path given".red());
        }
    }
}

fn get_all_files<'a>(path: PathBuf, excludes: Vec<String>) -> Result<Vec<PathBuf>, String> {
    let mut file_paths: Vec<PathBuf> = Vec::new();
    for entry in WalkDir::new(&path)
        .sort_by(|a, b| b.file_name().cmp(a.file_name()))
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if let Some(file_name) = entry.file_name().to_str() {
            if entry.path().is_file() && file_name.ends_with(".tex") && !excludes.iter().any(|exclude| entry.path().to_str().unwrap_or("").contains(exclude)){
                println!(
                    "Found latex file: {}",
                    get_file_name(&entry.clone().into_path())
                );
                file_paths.push(entry.into_path());
            }
        }
    }
    if file_paths.is_empty() {
        Err(String::from("No latex files found"))
    } else {
        Ok(file_paths)
    }
}

fn read_file(path: &PathBuf, allowed: &Vec<String>) -> Result<String, Vec<(usize, String)>> {
    let mut faulty_lines: HashMap<usize, String> = HashMap::new();
    let file = fs::read_to_string(path);
    let re = Regex::new(r"\b\p{Lu}+\b").unwrap();
    match file {
        Ok(file) => {
            let mut lines: HashMap<usize, String> = HashMap::new();
            for (i, line) in file.lines().enumerate() {
                lines.insert(i, line.to_string());
            }
            for (i, line) in lines.iter() {
                for cap in re.find_iter(line) {
                    let acronym = cap.as_str().to_string();
                    if acronym.len() >= 2 && !allowed.contains(&acronym){
                        faulty_lines.insert(*i, acronym);
                    }
                }
            }
        }
        Err(err) => {
            return Ok(err.to_string());
        }
    }

    if faulty_lines.is_empty() {
        Ok(format!("No faulty lines found in {}", get_file_name(&path)).to_string())
    } else {
        let mut sorted: Vec<(usize, String)> = faulty_lines.iter().map(|(&k, v)| (k, v.clone())).collect();
        sorted.sort_by_key(|(key, _)| *key);
        Err(sorted)
    }
}

fn get_file_name(path: &PathBuf) -> String {
    match path.file_name() {
        Some(file_name) => match file_name.to_str() {
            Some(file_name) => String::from(file_name),
            None => String::from(""),
        },
        None => String::from(""),
    }
}

fn get_list(wd: &String, name: String) -> Vec<String> {
    let mut list: Vec<String> = Vec::new();
    let mut path = PathBuf::from(wd);
    path.push(name + ".txt");
    let file_result = fs::read_to_string(path);
    match file_result {
        Ok(file) => {
            for line in file.lines() {
                list.push(line.trim().to_string());
            }
        }
        _ => {}
    }
    list
}