use std::{
	fs,
	io::{Error, Write},
	path::{Path, PathBuf},
	process::{Command, Stdio},
};

use crate::cli::Kana;

use clap::CommandFactory;
use clap_complete::{self, Shell};

#[path = "src/cli.rs"]
mod cli;

fn main() -> Result<(), Error> {
	let base_path = project_root::get_project_root().unwrap();
	let target_dir = base_path.join("target");

	let doc_dir = base_path.join("doc");
	println!("cargo:rerun-if-changed={}", doc_dir.to_str().unwrap());

	let completions_dir = target_dir.join("completions");
	fs::create_dir_all(&completions_dir)?;

	let mut app = Kana::command();
	let app_name = app.get_name().to_string();

	for shell in &[Shell::Bash, Shell::Zsh, Shell::Fish] {
		clap_complete::generate_to(*shell, &mut app, &app_name, &completions_dir)?;
	}

	if Command::new("scdoc").spawn().is_err() {
		eprintln!("scdoc not found in PATH, skipping generating manpage templates from doc/");

		return Ok(());
	}

	build_manpages(&target_dir.join("doc"), &doc_dir)?;

	Ok(())
}

fn build_manpages(dst: &Path, src: &Path) -> Result<(), Error> {
	fs::create_dir_all(&dst)?;

	for doc in fs::read_dir(src)? {
		let doc = doc?;

		let cmd = Command::new("scdoc")
			.stdin(Stdio::piped())
			.stdout(Stdio::piped())
			.spawn();

		let mut cmd = cmd?;

		if let Some(mut stdin) = cmd.stdin.take() {
			let doc = fs::read(doc.path())?;
			stdin.write_all(&doc)?;
		}

		let output = cmd.wait_with_output()?;
		let doc = PathBuf::from(doc.file_name());
		match doc.extension() {
			Some(ext) if ext == "sdc" => {
				let doc = doc.file_stem().unwrap();

				fs::write(dst.join(doc), output.stdout)?;
			}
			_ => {}
		}
	}

	Ok(())
}
