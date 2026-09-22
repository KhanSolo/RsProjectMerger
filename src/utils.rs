use std::path::Path;

static IGNORED_DIRECTORIES: &'static [&str] = &["bin", "obj", ".git", ".vs"];

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ExtensionType {
    Csproj,
    Sln,
}

pub fn to_extension_type(input_path: &Path) -> Result<ExtensionType, String> {
    let ext = match input_path.extension() {
        Some(ext) => ext,
        None => return Err(format!("Cannot get extension of {}", input_path.display())),
    };

    let ext_str = match ext.to_str() {
        Some(ext) => ext,
        None => {
            return Err(format!(
                "Cannot parse extension of {}",
                input_path.display()
            ));
        }
    };

    match ext_str {
        ext_str if ext_str.eq_ignore_ascii_case("csproj") => Ok(ExtensionType::Csproj),
        ext_str if ext_str.eq_ignore_ascii_case("sln") => Ok(ExtensionType::Sln),
        _ => Err(format!("Unsupported extension of {}", ext_str)),
    }
}

pub fn format_with_commas(n: usize) -> String {
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

pub fn is_ignored_directory(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| {
            IGNORED_DIRECTORIES
                .iter()
                .any(|ignored| name.eq_ignore_ascii_case(ignored))
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(r"c:\src\project\project.csproj", ExtensionType::Csproj)]
    #[case(r"c:\src\project\project.sln", ExtensionType::Sln)]
    #[case(r"project.csproj", ExtensionType::Csproj)]
    #[case(r"project.sln", ExtensionType::Sln)]
    fn test_to_extension_type_success(
        #[case] path_str: &str,
        #[case] expected: ExtensionType,
    ) -> Result<(), String> {
        let path = Path::new(path_str);
        let Ok(result) = to_extension_type(path) else {
            return Err("Cannot get ExtensionType".into());
        };
        assert_eq!(result, expected);
        Ok(())
    }

    #[rstest]
    #[case(2026, "2,026")]
    #[case(202, "202")]
    #[case(20262026, "20,262,026")]
    fn test_format_with_commas_success(#[case] r: usize, #[case] expected: &str) {
        let result = format_with_commas(r);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(r"c:\src\project\bin", true)]
    #[case(r"c:\src\project\BIN", true)]
    #[case(r"c:\src\project\Bin", true)]
    #[case(r"c:\src\project\obj", true)]
    #[case(r"c:\src\project\.git", true)]
    #[case(r"c:\src\project\.vs", true)]
    #[case(r"c:\src\project\src", false)]
    #[case(r"c:\src\project\source", false)]
    #[case(r"c:\src\project\binary", false)]
    fn test_is_ignored_directory(#[case] path: &str, #[case] expected: bool) {
        assert_eq!(is_ignored_directory(Path::new(path)), expected);
    }
}
