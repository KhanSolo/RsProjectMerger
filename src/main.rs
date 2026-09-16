use std::env;
use std::fs::create_dir_all;
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

    let output_path = (if args.len() == 3 {
        PathBuf::from(&args[2])
    } else {
        input_path
            .parent()
            .unwrap_or(Path::new("."))
            .join(Path::new("merged-project.txt"))
    })
    .as_path();

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
    for project in projects {
        println!("{}", project.display());
    }

    let content: &String = build_merged_file(projects, output_path);

    let Some(out_dir) = output_path.parent() else {
        eprintln!("Cannot get output directory for {}", output_path.display());
        return ExitCode::FAILURE; // todo
    };

    let Ok(()) = create_dir_all(out_dir) else {
        eprintln!("Cannot create output directory {}", out_dir.display());
        return ExitCode::FAILURE;
    };

    // File.WriteAllText(
    //     outputPath,
    //     content,
    //     new UTF8Encoding(encoderShouldEmitUTF8Identifier: false));

    // Console.WriteLine();
    // Console.WriteLine($"Created: {outputPath}");
    // Console.WriteLine($"Size: {content.Length:N0} characters");

    ExitCode::SUCCESS
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

fn build_merged_file(projects:Vec<&Path>, output_path:&Path) -> &String {
    todo!()
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
