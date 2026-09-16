use std::env;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

static IGNORED_DIRECTORIES : &'static [&str] = &[
        "bin",
        "obj",
        ".git",
        ".vs"
];

enum ExtensionType {
    Csproj,
    Sln,
}

fn main() -> ExitCode {
    let args : Vec<String> = env::args().collect();

    // args[0] - executable
    if args.len() < 2 || args.len() > 3 {
        print_usage();
        return ExitCode::FAILURE;
    }

    let input_path = Path::new(&args[1]);
    if !input_path.exists() {
        println!("Input file not found {}", input_path.display());
        return ExitCode::FAILURE;
    }

    let mut extension : Option<&OsStr> = None;
    if let Some(ext ) =  input_path.extension() {
        if !check_extension(ext) {
            println!("Extension {} is not supported", ext.display());
            return ExitCode::FAILURE;
        }
        extension = Some(ext);
    } else {
        println!("Cannot get extension of the {}", input_path.display());
        return ExitCode::FAILURE;
    }

    let output_path = (
         if args.len() == 3 {
            PathBuf::from(&args[2])
        } else {
            input_path
            .parent()
            .unwrap_or(Path::new("."))
            .join(Path::new("merged-project.txt"))
        }
    ).as_path();

    let projects = if extension.unwrap().to_str().unwrap().to_lowercase() == "csproj" {

    } else {

    };

    println!("todo: implement logic");
    ExitCode::SUCCESS
}

fn print_usage() {
    println!("
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
    ");    
}

fn check_extension(extension:&OsStr) -> bool {
    if let Some(ext) = extension.to_str() {
        let lower = ext.to_lowercase();
        lower == "csproj" || lower == "sln"
    } else {
        false
    }
}

