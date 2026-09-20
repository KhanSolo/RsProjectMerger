mod utils;
mod merger;

use crate::utils::*;
use crate::merger::*;

use std::env;
use std::fs::{self, create_dir_all};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    // args[0] - executable
    if args.len() < 2 || args.len() > 3 {
        print_usage();
        return ExitCode::FAILURE;
    }

    let input_path = PathBuf::from(&args[1]);
    if !input_path.exists() {
        eprintln!("Input file not found {}", input_path.display());
        return ExitCode::FAILURE;
    }

    let extension_type = match to_extension_type(input_path.as_path()){
        Ok(et) => et,
        Err(msg) => {
            eprintln!("{msg}");
            return ExitCode::FAILURE;
        }
    };

    let output_path_owned = if args.len() == 3 {
        PathBuf::from(&args[2])
    } else {
        input_path
            .parent()
            .unwrap_or(Path::new("."))
            .join(Path::new("merged-project.txt"))
    };
    let output_path = output_path_owned.as_path();

    let projects_owned = match extension_type {
        ExtensionType::Csproj => vec![input_path],
        ExtensionType::Sln => find_projects_in_solution(input_path.as_path()).unwrap_or(vec![]),
    };
    let projects:Vec<&Path> = projects_owned.iter().map(|p| p.as_path()).collect();

    let projects_count = projects.len();
    if projects_count == 0 {
        eprintln!("No C# projects found.");
        return ExitCode::FAILURE;
    }
    println!("Projects found {}", &projects_count);
    for project in &projects {
        println!("{}", project.display());
    }

    let content = build_merged_file(projects, output_path);

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
