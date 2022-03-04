use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::{fs, io};

pub use std::env::set_current_dir as cd;
use std::ffi::OsStr;
use std::process::{Child, Command};

#[cfg(windows)]
pub use windows_console::enable_ansi_codes;

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

pub fn cmd<S: AsRef<OsStr>, const N: usize>(
	exec_mode: ExecMode,
	program: S,
	args: [&dyn AsRef<OsStr>; N],
) -> io::Result<Child> {
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
		}
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

#[cfg(windows)]
mod windows_console {
	use std::ffi::c_void;
	use std::io;

	extern "system" {
		fn GetStdHandle(nStdHandle: u32) -> *mut c_void;
		fn GetConsoleMode(hConsoleHandle: *mut c_void, lpMode: *mut u32) -> i32;
		fn SetConsoleMode(hConsoleHandle: *mut c_void, dwMode: u32) -> i32;
	}

	pub const ENABLE_VIRTUAL_TERMINAL_PROCESSING: u32 = 0x0004;
	pub const STD_OUTPUT_HANDLE: u32 = -11i32 as u32;
	pub const STD_ERROR_HANDLE: u32 = -12i32 as u32;
	pub const INVALID_HANDLE_VALUE: *mut c_void = -1isize as *mut c_void;

	pub fn enable_ansi_codes() {
		if let Err(err) = try_enable_ansi_codes() {
			eprintln!("WARN: Failed to enable ansi codes. Caused by: {:?}", err);
		}
	}

	fn try_enable_ansi_codes() -> Result<(), io::Error> {
		unsafe {
			let std_handle = GetStdHandle(STD_OUTPUT_HANDLE);
			if std_handle == INVALID_HANDLE_VALUE || std_handle.is_null() {
				return Err(io::Error::last_os_error());
			}

			let error_handle = GetStdHandle(STD_ERROR_HANDLE);
			if error_handle == INVALID_HANDLE_VALUE || error_handle.is_null() {
				return Err(io::Error::last_os_error());
			}

			let mut std_lp_mode = 0;
			let result = GetConsoleMode(std_handle, &mut std_lp_mode as *mut u32);
			if result == 0 {
				return Err(io::Error::last_os_error());
			}
			let mut error_lp_mode = 0;
			let result = GetConsoleMode(error_handle, &mut error_lp_mode as *mut u32);
			if result == 0 {
				return Err(io::Error::last_os_error());
			}

			let dw_mode = std_lp_mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING;
			let result = SetConsoleMode(std_handle, dw_mode);
			if result == 0 {
				return Err(io::Error::last_os_error());
			}

			let dw_mode = error_lp_mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING;
			let result = SetConsoleMode(error_handle, dw_mode);
			if result == 0 {
				return Err(io::Error::last_os_error());
			}

			Ok(())
		}
	}
}
