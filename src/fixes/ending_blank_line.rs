use super::Fix;
use crate::common::*;

pub(crate) struct EndingBlankLineFixer<'a> {
    name: &'a str,
}

impl Default for EndingBlankLineFixer<'_> {
    fn default() -> Self {
        Self {
            name: "EndingBlankLine",
        }
    }
}

impl Fix for EndingBlankLineFixer<'_> {
    fn name(&self) -> &str {
        self.name
    }

    fn fix_line(&self, line: &mut LineEntry) -> Option<()> {
        if line.is_last_line() && !line.raw_string.ends_with(LF) {
            line.raw_string = format!("{}\n", line.raw_string);
        }
        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn fix_line_test() {
        let fixer = EndingBlankLineFixer::default();
        let mut line = LineEntry {
            number: 1,
            file: FileEntry {
                path: PathBuf::from(".env"),
                file_name: ".env".to_string(),
                total_lines: 1,
            },
            raw_string: String::from("FOO=BAR"),
        };
        assert_eq!(Some(()), fixer.fix_line(&mut line));
        assert_eq!("FOO=BAR\n", line.raw_string);
    }

    #[test]
    fn fix_warnings_test() {
        let fixer = EndingBlankLineFixer::default();
        let mut lines = vec![
            LineEntry {
                number: 1,
                file: FileEntry {
                    path: PathBuf::from(".env"),
                    file_name: ".env".to_string(),
                    total_lines: 2,
                },
                raw_string: String::from("FOO=BAR"),
            },
            LineEntry {
                number: 2,
                file: FileEntry {
                    path: PathBuf::from(".env"),
                    file_name: ".env".to_string(),
                    total_lines: 2,
                },
                raw_string: String::from("Z=Y"),
            },
        ];
        let mut warning = Warning::new(
            lines[1].clone(),
            "EndingBlankLine",
            String::from("No blank line at the end of the file"),
        );

        assert_eq!(Some(1), fixer.fix_warnings(vec![&mut warning], &mut lines));
        assert_eq!("Z=Y\n", lines[1].raw_string);
        assert!(warning.is_fixed);
    }
}
