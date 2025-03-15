use std::{
    cmp::{self, Reverse},
    fs::{self, DirEntry},
    path::PathBuf,
};

struct FolderSize {
    name: String,
    bytes: u128,
    size: String,
}

const UNITS: &[&str; 5] = &["B", "KB", "MB", "GB", "TB"];

fn main() {
    let mut folder_sizes = fs::read_dir(".")
        .unwrap()
        .filter_map(|res| match res {
            Ok(dir) if matches!(dir.metadata(), Ok(metadata) if metadata.is_dir()) => Some(dir),
            _ => None,
        })
        .map(|dir| {
            let name = dir.file_name().to_str().unwrap().to_owned();
            let bytes = folder_size(dir.path()).unwrap_or(0_u128);

            let (index, unit) = UNITS
                .iter()
                .enumerate()
                .find(|(index, _)| bytes < 1024_u128.pow((1 + index) as u32))
                .unwrap();

            let size = format!("{} {}", bytes / 1024_u128.pow(index as u32), unit);

            FolderSize { name, bytes, size }
        })
        .collect::<Vec<FolderSize>>();

    folder_sizes.sort_by_key(|folder| Reverse(folder.bytes));

    let name_max_width = folder_sizes
        .iter()
        .fold(0, |a, b| cmp::max(a, b.name.len()));

    let size_max_width = folder_sizes
        .iter()
        .fold(0, |a, b| cmp::max(a, b.size.len()));

    println!(
        "| Name{0} | Size{1} |",
        " ".repeat(name_max_width - "name".len()),
        " ".repeat(size_max_width - "size".len())
    );

    println!(
        "|{0}|{1}|",
        "-".repeat(name_max_width + 2),
        "-".repeat(size_max_width + 2)
    );

    for folder in folder_sizes {
        println!(
            "| {0}{1} | {2}{3} |",
            folder.name,
            " ".repeat(name_max_width - folder.name.len()),
            folder.size,
            " ".repeat(size_max_width - folder.size.len())
        )
    }
}

fn folder_size(path: PathBuf) -> Option<u128> {
    Some(
        fs::read_dir(path)
            .ok()?
            .filter_map(|res| res.ok())
            .fold(0, |a: u128, entry: DirEntry| {
                let metadata = entry.metadata().unwrap();
                a + if metadata.is_dir() {
                    folder_size(entry.path()).unwrap_or(0_u128)
                } else {
                    metadata.len() as u128
                }
            }),
    )
}
