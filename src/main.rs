use colored::Colorize;
use std::{env, fs, io};

fn main() -> io::Result<()> {
    let option = env::args().skip(1).nth(0);
    let files = fs::read_dir(env::current_dir()?)?
        .filter_map(|x| x.ok())
        .map(|x| x.file_name().into_string().ok())
        .filter_map(|x| x)
        .collect::<Vec<String>>();

    match option.is_some() {
        true => {
            if option.is_some_and(|x| x.eq("-a")) {
                files.into_iter().for_each(|file_name| {
                    if !file_name.starts_with('.') {
                        println!("{file_name}");
                    };
                })
            }
        }
        false => {
            files.into_iter().for_each(|file_name| {
                println!("{file_name}");
            });
        }
    }

    Ok(())
}
