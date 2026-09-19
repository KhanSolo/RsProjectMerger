use std::fmt::Write;
use std::path::Path;

pub fn find_projects_in_solution(input_path: &Path) -> Vec<&Path> {
    todo!()
}

pub fn build_merged_file(builder:&mut String, projects:Vec<&Path>, output_path:&Path) -> () {
    for project_path in projects  {
        append_project(builder, project_path, output_path);
    }
}

pub fn append_project(builder:&mut String, project_path: &Path, output_full_path: &Path) {

}

pub fn append_header(builder:&mut String, file_path: &Path) {
    writeln!(builder, "// {}", file_path.display()).expect("could not append to builder");
}