use std::env;
use std::fmt::Write;
use std::fs::{self, create_dir_all};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

static IGNORED_DIRECTORIES: &'static [&str] = &["bin", "obj", ".git", ".vs"];

#[derive(Copy, Clone, PartialEq)]
enum ExtensionType {
    Csproj,
    Sln,
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    // args[0] - executable
    if args.len() < 2 || args.len() > 3 {
        print_usage();
        return ExitCode::FAILURE;
    }

    let input_path = Path::new(&args[1]);
    if !input_path.exists() {
        eprintln!("Input file not found {}", input_path.display());
        return ExitCode::FAILURE;
    }

    let Some(extension_type) = to_extension_type(input_path) else {
        return ExitCode::FAILURE;
    };

    let binding = if args.len() == 3 {
        PathBuf::from(&args[2])
    } else {
        input_path
            .parent()
            .unwrap_or(Path::new("."))
            .join(Path::new("merged-project.txt"))
    };
    let output_path = binding.as_path();

    let projects = if extension_type == ExtensionType::Csproj {
        vec![input_path]
    } else {
        find_projects_in_solution(input_path)
    };

    let projects_count = projects.len();
    if projects_count == 0 {
        eprintln!("No C# projects found.");
        return ExitCode::FAILURE;
    }
    println!("Projects found {}", &projects_count);
    for project in &projects {
        println!("{}", project.display());
    }

    let mut content  = String::new();
    build_merged_file(&mut content, projects, output_path);

    let Some(out_dir) = output_path.parent() else {
        eprintln!("Cannot get output directory for {}", output_path.display());
        return ExitCode::FAILURE; // todo
    };

    if !out_dir.exists() {
        let Ok(()) = create_dir_all(out_dir) else {
            eprintln!("Cannot create output directory {}", out_dir.display());
            return ExitCode::FAILURE;
        };
    }

    let Ok(()) = fs::write(output_path, &content) else {
        eprintln!("Cannot create output file {}", output_path.display());
        return ExitCode::FAILURE;
    };

    println!();
    println!("Created: {}", output_path.display());
    println!("Size: {} characters", format_with_commas(content.len()));

    ExitCode::SUCCESS
}

fn format_with_commas(n:usize) -> String {
    let s = n.to_string();
    let mut r = String::new();

    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            r.push(',');
        }
        r.push(c);
    }

    r.chars().rev().collect()
}

fn to_extension_type(input_path: &Path) -> Option<ExtensionType> {
    let ext = match input_path.extension() {
        Some(ext) => ext,
        None => {
            println!("Cannot get extension of {}", input_path.display());
            return None;
        }
    };

    let ext_str = match ext.to_str() {
        Some(ext) => ext,
        None => {
            println!("Cannot parse extension of {}", input_path.display());
            return None;
        }
    };

    match ext_str {
        ext_str if ext_str.eq_ignore_ascii_case("csproj") => Some(ExtensionType::Csproj),
        ext_str if ext_str.eq_ignore_ascii_case("sln") => Some(ExtensionType::Sln),
        _ => None,
    }
}

fn find_projects_in_solution(input_path: &Path) -> Vec<&Path> {
    todo!()
}

fn build_merged_file(builder:&mut String, projects:Vec<&Path>, output_path:&Path) -> () {

    for project_path in projects  {
        append_project(builder, project_path, output_path);
    }
}

fn append_project(builder:&mut String, project_path: &Path, output_full_path: &Path) {

}

fn append_header(builder:&mut String, file_path: &Path) {
    writeln!(builder, "// {}", file_path.display()).expect("could not append to builder");
}

fn is_ignored_directory(file_path: &Path) -> bool {

    let mut directory = file_path.parent();

    while let Some(dir) = directory {
        if let Some(dir_name) = dir.file_name().and_then(|s| s.to_str()) { // get last folder segment
            if IGNORED_DIRECTORIES
                .iter()
                .any(|ignored| dir_name.eq_ignore_ascii_case(ignored)) // case insensitive (ascii)
            {
                return true;
            }
        }

        directory = dir.parent();
    }

    false
}

fn print_usage() {
    println!(
        "
             C# Project Merger

            Usage:
              RsProjectMerger <project.csproj|solution.sln> [output-file]

            Examples:
              RsProjectMerger MyProject.csproj
              RsProjectMerger MyProject.csproj merged.txt
              RsProjectMerger MySolution.sln
              RsProjectMerger MySolution.sln all-source.txt

            If output-file is omitted:
              merged-project.txt    
    "
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_format_with_commas_success() {
        let result = format_with_commas(2026);
        assert_eq!(result, "2,026");
    }

    #[rstest]
    #[case("c:\\src\\project\\bin\\temp\\", true)]
    #[case("C:\\SRC\\PROJECT\\BIN\\TEMP\\", true)]
    #[case("c:\\src\\project\\", false)]
    fn test_is_ignored_directory_success(
        #[case] path : &str,
        #[case] expected : bool
    ) {
        let file_path = Path::new(path);
        let result = is_ignored_directory(file_path);
        assert_eq!(result, expected);
    }
}