use std::path::Path;

static IGNORED_DIRECTORIES: &'static [&str] = &["bin", "obj", ".git", ".vs"];

#[derive(Copy, Clone, PartialEq)]
pub enum ExtensionType {
    Csproj,
    Sln,
}

pub fn to_extension_type(input_path: &Path) -> Option<ExtensionType> {
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

pub fn format_with_commas(n:usize) -> String {
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

pub fn is_ignored_directory(file_path: &Path) -> bool {

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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(2026, "2,026")]
    #[case(202, "202")]
    #[case(20262026, "20,262,026")]
    fn test_format_with_commas_success(
        #[case] r:usize,
        #[case] expected:&str
    ) {
        let result = format_with_commas(r);
        assert_eq!(result, expected);
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