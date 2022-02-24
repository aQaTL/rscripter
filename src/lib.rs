use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::{fs, io};

pub use std::env::set_current_dir as cd;
use std::ffi::OsStr;
use std::process::{Child, Command};

#[macro_export]
macro_rules! log {
    ($($token:tt)*) => {{
        let tokens = vec![$(stringify!($token)),*];
        let mut final_cmd = String::new();
        for token in tokens {
            let token = token.trim();
            let token = token.strip_prefix("r").unwrap_or(token);
            let token = token.strip_prefix("\"").unwrap_or(token);
            let token = token.strip_suffix("\"").unwrap_or(token);
            if token == "," {
                continue;
            }
            final_cmd.push_str(token);
            final_cmd.push(' ');
        }

        const YELLOW: &str = "\x1b[33m";
        const RESET: &str = "\x1b[0m";
        println!("{YELLOW}{final_cmd}{RESET}");

        {
            $($token)*
        }
    }}
}

pub fn echo(s: impl Display) {
    const YELLOW: &str = "\x1b[33m";
    const RESET: &str = "\x1b[0m";
    // println!("{YELLOW}{s}{RESET}");
    println!("{}{}{}", YELLOW, s, RESET);
}

#[macro_export]
macro_rules! ls {
    () => {{
        crate::ls(".")
    }};

    ($path:expr) => {{
        crate::ls($path)
    }};
}

pub fn ls(path: impl AsRef<Path>) -> io::Result<()> {
    for entry in (fs::read_dir(path)?).flatten() {
        if let Some(filename) = entry.path().file_name() {
            println!("{}", filename.to_string_lossy());
        }
    }
    Ok(())
}

#[derive(Copy, Clone)]
pub enum ExecMode {
    Spawn,
    WaitForCompletion,
}

pub fn cmd<S: AsRef<OsStr>, const N: usize>(exec_mode: ExecMode, program: S, args: [&dyn AsRef<OsStr>; N]) -> io::Result<Child> {
    let mut command = Command::new(program);
    command.args(args.into_iter().map(|arg| arg.as_ref()));

    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        command.creation_flags(CREATE_NO_WINDOW);
    }

    let mut child = command.spawn()?;

    match exec_mode {
        ExecMode::Spawn => Ok(child),
        ExecMode::WaitForCompletion => {
            let _exit_status = child.wait()?;
            Ok(child)
        },
    }
}

#[macro_export]
macro_rules! cmd {
    ($($token:tt)*) => {{
        let tokens = vec![$(stringify!($token)),*]        ;
        let mut final_cmd = String::new();
        for token in tokens {
            let token = token.trim();
            let token = token.strip_prefix("r").unwrap_or(token);
            let token = token.strip_prefix("\"").unwrap_or(token);
            let token = token.strip_suffix("\"").unwrap_or(token);
            if token == "," {
                continue;
            }
            final_cmd.push_str(token);
            final_cmd.push(' ');
        }

        const YELLOW: &str = "\x1b[33m";
        const RESET: &str = "\x1b[0m";
        println!("{YELLOW}{final_cmd}{RESET}");

        {
            rscripter::cmd_inner!($($token)*)
        }
    }}
}

#[macro_export]
macro_rules! cmd_inner {
    (fork; $program:expr, $($segment:expr),*$(,)*) => {{
        rscripter::cmd(
            rscripter::ExecMode::Spawn,
            $program,
            [
                $(
                    &$segment,
                )*
            ],
        )
    }};
    ($program:expr, $($segment:expr),*$(,)*) => {{
        rscripter::cmd(
            rscripter::ExecMode::WaitForCompletion,
            $program,
            [
                $(
                    &$segment,
                )*
            ],
        )
    }};
}

pub fn path<const N: usize>(segments: [&dyn AsRef<Path>; N]) -> PathBuf {
    segments.into_iter().map(AsRef::as_ref).collect()
}

#[macro_export]
macro_rules! path {
    [$($segment:expr),*$(,)*] => {{
        rscripter::path(
            [
                $(
                    &$segment,
                )*
            ]
        )
    }}
}