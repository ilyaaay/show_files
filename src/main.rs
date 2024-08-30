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
                    if option.as_ref().is_some_and(|x| x.eq("-a") || x.eq("--a")) {
                        if file_type.is_file() {
                            print!("{} ", name.bright_blue());
                        }
                        if file_type.is_dir() {
                            print!("{} ", name.green());
                        }
                        if file_type.is_symlink() {
                            print!("{} ", name.red());
                        }
                    } else {
                        if !name.starts_with('.') {
                            if file_type.is_file() {
                                print!("{} ", name.bright_blue());
                            }
                            if file_type.is_dir() {
                                print!("{} ", name.green());
                            }
                            if file_type.is_symlink() {
                                print!("{} ", name.red());
                            }
                        }
                    }
                }
            }
        });

    println!();

    Ok(())
}
