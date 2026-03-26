use std::{error::Error, ffi::OsStr, path::PathBuf, process::Command};

use derive_new::new;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("{self:?}")]
pub enum CommandError {
    Stderr(String),
    Io(
        #[from]
        #[source]
        std::io::Error,
    ),
    Generic {
        message: String,
        source: Option<Box<dyn std::error::Error>>,
    },
}

impl CommandError {
    fn message<S: Into<String>>(message: S) -> Self {
        Self::Generic {
            message: message.into(),
            source: None,
        }
    }

    fn generic<E: std::error::Error + 'static>(source: E) -> Self {
        Self::Generic {
            message: source.to_string(),
            source: Some(Box::new(source)),
        }
    }
}

pub trait Cmd {
    fn execute(&self) -> Result<String, CommandError>;
}

#[derive(new)]
pub struct TailwindCssCmd {
    #[new(into)]
    input: PathBuf,
    #[new(into)]
    output: PathBuf,
}

impl Cmd for TailwindCssCmd {
    fn execute(&self) -> Result<String, CommandError> {
        let input = self.input.to_string_lossy();
        let output = self.output.to_string_lossy();
        run(&["tailwindcss", "-i", &input, "-o", &output])
    }
}

fn run<S: AsRef<OsStr>>(cmd: &[S]) -> Result<String, CommandError> {
    let mut args = cmd.into_iter();

    let exe = args.next().ok_or(CommandError::message(
        "args must have at least one item (the exe)",
    ))?;

    let mut cmd = Command::new(exe);
    for arg in args {
        cmd.arg(arg);
    }

    let output = cmd.output()?;
    if output.status.success() {
        let out = String::from_utf8_lossy(&output.stdout);
        Ok(out.into_owned())
    } else {
        let err = String::from_utf8_lossy(&output.stdout);
        Err(CommandError::Stderr(err.into_owned()))
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd::run;

    #[test]
    fn ls_la() {
        let out = run(&["ls", "-l", "-a"]).unwrap();
        println!("out: {out}");
    }
}
