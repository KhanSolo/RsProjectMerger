use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args : Vec<String> = env::args().collect();
    // for arg in &args {
    //     println!("{}", &arg);
    // }

    if args.len() < 1 || args.len() > 2 {
        print_usage();
        return ExitCode::FAILURE;
    }

    

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