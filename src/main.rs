use std::{
	env, process,
	process::{Command, Stdio},
};

fn main() {
	if let Ok("1") = env::var("CARGO_FETCH2_INTERNAL").as_deref() {
		let mut args = env::args().skip(1).collect::<Vec<_>>();
		let rustc = args.remove(0);
		// TODO: this is not the most resilient
		if !args.windows(3).any(|x| x == ["-", "--crate-name", "___"]) {
			process::exit(1);
		}
		let mut rustc = Command::new(rustc);
		rustc.env_remove("CARGO_FETCH2_INTERNAL");
		rustc.args(args);
		let exit = rustc.spawn().unwrap().wait().unwrap();
		if !exit.success() {
			process::exit(exit.code().unwrap_or(1));
		}
	} else {
		let cargo = env::var_os("CARGO").unwrap();
		let args = env::args().skip(2).collect::<Vec<_>>();
		let mut cargo = Command::new(cargo);
		cargo.env("CARGO_BUILD_RUSTC_WRAPPER", "cargo-fetch2");
		cargo.env("CARGO_FETCH2_INTERNAL", "1");
		cargo.args(args);
		cargo.stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null());
		let _exit = cargo.spawn().unwrap().wait().unwrap();
	}
}
