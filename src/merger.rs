use std::fmt::Write;
use std::fs::{File,read_to_string};
use std::io::{BufRead, BufReader, Result};
use std::path::{Path, PathBuf};

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
    let mut builder = String::with_capacity(projects.len()*1024);
    for project_path in projects {
        append_project(&mut builder, project_path, output_path);
    }
    builder
}

pub fn append_project(builder:&mut String, project_path: &Path, output_full_path: &Path) {
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
    let _ = writeln!(builder, "{}", file_all_text.trim());
    let _ = writeln!(builder, ""); // new line

    // .cs files
    let source_files: Vec<&Path> = vec![];
    /*   var sourceFiles = Directory
    .EnumerateFiles(projectDirectory, "*.cs", SearchOption.AllDirectories)
    .Where(file => !IsIgnoredDirectory(file))
    .Where(file => !Path.GetFullPath(file).Equals(outputPath, StringComparison.OrdinalIgnoreCase))
    .OrderBy(file => file, StringComparer.OrdinalIgnoreCase)
    .ToList(); */        
    for source_file in source_files
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

pub fn append_header(builder:&mut String, file_path: &Path) {
    writeln!(builder, "// {}", file_path.display()).expect("could not append to builder");
    writeln!(builder, "").expect("could not append to builder");
}