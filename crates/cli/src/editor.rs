use std::{env, iter::once, process::Command};

/// User default editor.
///
/// The user default editor is searched for in the environment variables
/// `EDITOR` and `VISUAL`.
///
/// Any arguments to the program are extracted and stored separately so they can
/// be passed to the program along with the path to open.
#[derive(Debug, PartialEq)]
pub struct Editor {
    /// Default options and arguments to the program.
    args: Vec<String>,
    /// The program to launch.
    program: String,
}

impl Editor {
    /// Search the environment variables `EDITOR` and `VISUAL` for the user
    /// preferred editor.
    pub fn find() -> Option<Self> {
        env::var("EDITOR")
            .or_else(|_| env::var("VISUAL"))
            .ok()
            .filter(|value| !value.is_empty())
            .map(|value| {
                let mut parts = value.split_whitespace().map(String::from);
                Self {
                    program: parts.next().expect("there should be a program"),
                    args: parts.collect(),
                }
            })
    }

    /// Open the user default editor.
    pub fn open(&self, path: &str) {
        let args = self.args.iter().map(|s| s.as_str()).chain(once(path));
        Command::new(&self.program)
            .args(args)
            .spawn()
            .expect("default editor failed to start");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_user_editor() {
        env::set_var("EDITOR", "editor --editor-flag");
        env::set_var("VISUAL", "visual");
        let editor = Editor::find();
        let expected = Some(Editor {
            args: vec![String::from("--editor-flag")],
            program: String::from("editor"),
        });
        assert_eq!(editor, expected, "$EDITOR should be preferred editor");

        env::remove_var("EDITOR");
        env::set_var("VISUAL", "visual");
        let editor = Editor::find();
        let expected = Some(Editor {
            args: vec![],
            program: String::from("visual"),
        });
        assert_eq!(
            editor, expected,
            "$VISUAL should be used when $EDITOR is absent"
        );

        env::remove_var("VISUAL");
        let editor = Editor::find();
        assert_eq!(editor, None);
    }
}
