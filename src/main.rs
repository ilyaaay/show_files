use colored::Colorize;
use std::{env, fs, io};

#[inline(always)]
fn main() -> io::Result<()> {
    let option = env::args().skip(1).nth(0);

    fs::read_dir(env::current_dir()?)?
        .filter_map(|x| x.ok())
        .for_each(|file| {
            if let Ok(file_type) = file.file_type() {
                if let Ok(name) = file.file_name().into_string() {
                    if file_type.is_file() {
                        if option.as_ref().is_some_and(|x| x.eq("-a") || x.eq("--a")) {
                            print!("{} ", name.bright_blue());
                        } else if !name.starts_with('.') {
                            print!("{} ", name.bright_blue());
                        }
                    }
                    if file_type.is_dir() {
                        if option.as_ref().is_some_and(|x| x.eq("-a") || x.eq("--a")) {
                            print!("{} ", name.green());
                        } else if !name.starts_with('.') {
                            print!("{} ", name.green());
                        }
                    }
                    if file_type.is_symlink() {
                        if option.as_ref().is_some_and(|x| x.eq("-a") || x.eq("--a")) {
                            print!("{} ", name.red());
                        } else if !name.starts_with('.') {
                            print!("{} ", name.red());
                        }
                    }
                }
            }
        });

    println!();

    Ok(())
}
