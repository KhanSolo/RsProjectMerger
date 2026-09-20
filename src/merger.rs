use std::fmt::Write;
use std::fs::{self, File, read_to_string};
use std::io::{BufRead, BufReader, Result};
use std::path::{Path, PathBuf};

use crate::utils::is_ignored_directory;

pub fn find_projects_in_solution(solution_path: &Path) -> Result<Vec<PathBuf>> {
    let solution_directory = solution_path.parent().unwrap_or(Path::new("."));
    let mut projects = Vec::<PathBuf>::new();

    let solution_file = File::open(solution_path)?;
    let reader = BufReader::new(solution_file);

    for line in reader.lines() {
        let line = line?;
        // Typical .sln line:
        // Project("{GUID}") = "MyProject", "MyProject\MyProject.csproj", "{GUID}"
        if !line.starts_with("Project(") {
            continue;
        }
        if let Some(raw_line) = line.split(',').nth(1) {
            let proj_line = raw_line.trim().trim_matches('"');
            if proj_line.ends_with(".csproj") {
                let relative_path = Path::new(proj_line);
                let full_project_path = solution_directory.join(relative_path);
                if full_project_path.exists() {
                    projects.push(full_project_path);
                }
            }
        }
    }
    projects.sort();
    Ok(projects)
}

pub fn build_merged_file(projects: Vec<&Path>, output_path: &Path) -> String {
    let mut builder = String::with_capacity(projects.len() * 1024);
    for project_path in projects {
        append_project(&mut builder, project_path, output_path);
    }
    builder
}

pub fn append_project(builder: &mut String, project_path: &Path, output_full_path: &Path) {
    let project_directory = project_path.parent().unwrap_or(Path::new("."));

    // .csproj
    let Ok(relative_path) = project_path.strip_prefix(project_directory) else {
        eprintln!("Cannot get relative_path");
        return; // todo: create error
    };
    append_header(builder, relative_path);

    let Ok(file_all_text) = read_to_string(project_path) else {
        eprintln!("Cannot get file_all_text");
        return; // todo: create error
    };
    writeln!(builder, "{}", file_all_text.trim()).expect("could not append to builder");
    writeln!(builder, "").expect("could not append to builder"); // new line

    // .cs files
    let source_files = collect_files(project_directory).unwrap_or_default(); // todo
    let source_files: Vec<&Path> = source_files.iter().map(|p| p.as_path()).collect();

    for source_file in source_files
    // todo : remove duplication with csproj part
    {
        let Ok(relative_path) = source_file.strip_prefix(project_directory) else {
            eprintln!("Cannot get relative_path");
            return; // todo: create error
        };
        append_header(builder, relative_path);

        let Ok(file_all_text) = read_to_string(project_path) else {
            eprintln!("Cannot get file_all_text");
            return; // todo: create error
        };
        writeln!(builder, "{}", file_all_text.trim()).expect("could not append to builder");
        writeln!(builder, "").expect("could not append to builder"); // new line      
    }
}

pub fn append_header(builder: &mut String, file_path: &Path) {
    writeln!(builder, "// {}", file_path.display()).expect("could not append to builder");
    writeln!(builder, "").expect("could not append to builder"); // new line
}

fn collect_files(path: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut files: Vec<PathBuf> = vec![];
    let mut dirs = vec![path.to_path_buf()];

    while let Some(current_dir) = dirs.pop() {
        for entry in fs::read_dir(current_dir)? {
            let entry = entry?;
            let path = entry.path();

            if is_ignored_directory(&path) {
                println!("is_ignored_directory {}", path.display());
                continue;
            }

            if path.is_dir() {
                println!("dirs.push {}", path.display());
                dirs.push(path);
            } else if path.is_file() {
                println!("path.is_file() {}", path.display());
                if let Some(ext) = path.extension() {
                    println!(" ext.len: {}", &ext.len());
                    if let Some(e) = ext.to_str()
                        && e == "cs"
                    {
                        println!(" files.push {}", path.display());
                        files.push(path);
                    }
                }
            }
        }
    }
    files.sort();
    println!("files.len() : {}", &files.len());
    Ok(files)
}
