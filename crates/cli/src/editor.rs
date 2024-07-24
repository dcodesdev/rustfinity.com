use std::process::Command;

/// Editors the user might be using
#[derive(Debug, PartialEq)]
pub enum Editor {
    VSCode,
    Zed,
}

impl Editor {
    pub fn find() -> Option<Self> {
        if Command::new("code").arg("--version").output().is_ok() {
            return Some(Editor::VSCode);
        }

        if Command::new("zed").arg("--version").output().is_ok() {
            return Some(Editor::Zed);
        }

        None
    }

    pub fn open(&self, path: &str) {
        Command::new(self.command()).arg(path).spawn().unwrap();
    }

    fn command(&self) -> &str {
        match self {
            Editor::VSCode => "code",
            Editor::Zed => "zed",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_editor() {
        let editor = Editor::find();
        assert_eq!(editor, None);
    }
}
