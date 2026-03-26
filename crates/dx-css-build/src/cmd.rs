use std::{path::PathBuf, process::Command};

use derive_new::new;

use crate::Error;

pub type CmdResult<T> = Result<T, Error>;

pub trait Cmd {
    fn cli(&self) -> CmdResult<(String, Vec<String>)>;

    fn execute(&self) -> CmdResult<String> {
        let (exe, args) = self.cli()?;
        let exe = which::which(exe)?;

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
            Err(Error::Stderr(err.into_owned()))
        }
    }
}

#[derive(new)]
pub struct TailwindCssCmd {
    #[new(into)]
    pub input: PathBuf,
    #[new(into)]
    pub output: PathBuf,
}

impl Cmd for TailwindCssCmd {
    fn cli(&self) -> CmdResult<(String, Vec<String>)> {
        let input = self.input.to_string_lossy().to_string();
        let output = self.output.to_string_lossy().to_string();
        Ok((
            "tailwindcss".into(),
            vec!["-i".into(), input, "-o".into(), output],
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::cmd::Cmd;

    #[test]
    fn ls_la() {
        struct Ls;
        impl Cmd for Ls {
            fn cli(&self) -> super::CmdResult<(String, Vec<String>)> {
                Ok((String::from("ls"), vec!["-l".to_string(), "-a".to_string()]))
            }
        }
        let out = Ls.execute().expect("failed to execute ls");
        println!("out: {out}");
    }
}
