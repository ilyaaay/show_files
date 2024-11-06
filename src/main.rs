use colored::{ColoredString, Colorize};
use std::{
    env,
    fs::{self, FileType},
    io::{self, Write},
};

fn main() -> io::Result<()> {
    let args = env::args().skip(1).collect::<Vec<String>>();
    let mut handle = io::stdout().lock();

    match args.len() {
        0 => {
            let current_dir = env::current_dir()?;
            let content = fs::read_dir(current_dir)?
                .filter_map(|x| x.ok())
                .map(|x| (x.file_name().into_string().ok(), x.file_type().ok()))
                .collect::<Vec<(Option<String>, Option<FileType>)>>();

            let name_iter = content.iter().filter_map(|(name, _)| name.as_ref());
            let type_iter = content.iter().filter_map(|(_, r#type)| r#type.as_ref());

            let iter_consumer = name_iter
                .zip(type_iter)
                .collect::<Vec<(&String, &FileType)>>();

            let colored_vec = iter_consumer
                .into_iter()
                .map(|(name, r#type)| {
                    if r#type.is_dir() {
                        return name.green();
                    }
                    if r#type.is_file() {
                        return name.purple();
                    }
                    if r#type.is_symlink() {
                        return name.yellow();
                    }

                    name.red()
                })
                .collect::<Vec<ColoredString>>();

            for i in colored_vec {}
        }
        _ => {}
    }

    Ok(())
}
