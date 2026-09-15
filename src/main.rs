use std::env;
use std::path::Path;
use std::process::ExitCode;

static IGNORED_DIRECTORIES : &'static [&str] = &[
        "bin",
        "obj",
        ".git",
        ".vs"
];

fn main() -> ExitCode {
    let args : Vec<String> = env::args().collect();

    // args[0] - executable
    if args.len() < 2 || args.len() > 3 {
        print_usage();
        return ExitCode::FAILURE;
    }

    let input_file_path = Path::new(&args[1]);
    if !input_file_path.exists() {
        println!("Input file not found {}", input_file_path.display());
        return ExitCode::FAILURE;
    }

    if let Some(extension ) =  input_file_path.extension() {
        if extension == ".csproj" || extension == ".sln" { // todo: case insensitive
        } else {
            println!("Extension {} is not supported", extension.display());
            return ExitCode::FAILURE;
        }
    } else {
        println!("Cannot get extension of the {}", input_file_path.display());
        return ExitCode::FAILURE;
    }

    let output_path = if args.len() == 3 {
        "" // todo: from 3rd arg
    } else {
        "merged-project.txt" // todo: add parent folder from input path
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