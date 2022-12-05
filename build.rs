use std::fs;
use std::fs::FileType;
use std::io;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let input_folder = Path::new("..").join("input");

    for dir_entry in fs::read_dir(&input_folder)? {
        let year = dir_entry?;
        if !year.file_type()?.is_dir() {
            continue;
        }

        let year_path = input_folder.join(year.file_name());

        for day in (1..=25).map(|d| format!("day{}", d)) {
            write_import(
                &(year.file_name().into()),
                &day,
                year_path.join(format!("{}.txt", day)),
            );
        }
    }

    Ok(())
}

fn write_import<T: AsRef<Path>>(year: &str, day: &str, pth: T) {}
