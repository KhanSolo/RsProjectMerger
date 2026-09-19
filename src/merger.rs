use std::fmt::Write;
use std::fs::File;
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
    let mut builder = String::new();
    for project_path in projects {
        append_project(&mut builder, project_path, output_path);
    }
    builder
}

pub fn append_project(builder:&mut String, project_path: &Path, output_full_path: &Path) {

}

pub fn append_header(builder:&mut String, file_path: &Path) {
    writeln!(builder, "// {}", file_path.display()).expect("could not append to builder");
}