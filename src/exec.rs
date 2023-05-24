use super::Diff;
use command_extra::CommandExtra;
use derive_more::{Display, Error};
use pipe_trait::Pipe;
use std::{
    fs,
    io::{self, Write},
    process::{Command, Output},
};
use tempfile::tempdir;

/// Error returned by [`Diff::exec`].
#[derive(Debug, Display, Error)]
pub enum DiffExecError {
    #[display(fmt = "Failed to create temporary directory: {_0}")]
    Workspace(io::Error),
    #[display(fmt = "Failed to create temporary file the left value: {_0}")]
    Left(io::Error),
    #[display(fmt = "Failed to create temporary file the right value: {_0}")]
    Right(io::Error),
    #[display(fmt = "Failed to execute the 'diff' command: {_0}")]
    Exec(io::Error),
    #[display(fmt = "The 'diff' process exits with code {code:?}")]
    Status { code: Option<i32>, stderr: Vec<u8> },
}

impl<Left, Right> Diff<Left, Right>
where
    Left: AsRef<str>,
    Right: AsRef<str>,
{
    /// Use the GNU `diff` command to compare two strings.
    ///
    /// **Return:**
    /// * `Err(_)` means that the `diff` command was failed to executed or status > 1
    /// * `Ok(None)` means that no diff was found (status = 0).
    /// * `Ok(Some(diff))` means that there was diff (status = 1).
    pub fn exec(&self) -> Result<Option<String>, DiffExecError> {
        let Diff {
            left,
            right,
            color,
            unified,
        } = self;

        let workspace = tempdir().map_err(DiffExecError::Workspace)?;

        let write_file = |name: &str, text: &str| fs::write(workspace.path().join(name), text);
        write_file("left", left.as_ref()).map_err(DiffExecError::Left)?;
        write_file("right", right.as_ref()).map_err(DiffExecError::Right)?;

        let output = Command::new("diff")
            .with_current_dir(&workspace)
            .with_args(color.as_flag())
            .with_args(unified.then_some("--unified"))
            .with_arg("--label=left")
            .with_arg("--label=right")
            .with_arg("left")
            .with_arg("right")
            .output()
            .map_err(DiffExecError::Exec)?;
        let Output {
            status,
            stdout,
            stderr,
        } = output;

        io::stderr().write_all(&stderr).pipe(drop);

        if status.success() {
            return Ok(None);
        }

        if status.code() == Some(1) {
            return stdout
                .pipe(String::from_utf8)
                .expect("The stdout of the diff command should be valid UTF-8")
                .pipe(Some)
                .pipe(Ok);
        }

        Err(DiffExecError::Status {
            code: status.code(),
            stderr,
        })
    }

    /// Assert that two strings are equal.
    ///
    /// It will fallback to [`pretty_assertions`] if `diff` is failed to execute.
    pub fn assert_eq(&self) {
        match self.exec() {
            Ok(None) => { /* assertion passed, do nothing */ }
            Ok(Some(diff)) => panic!("assertion failed: `(left == right)`\n{diff}"),
            Err(_) => pretty_assertions::assert_str_eq!(self.left.as_ref(), self.right.as_ref()),
        }
    }
}
