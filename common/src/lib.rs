use std::{fmt::Display, path::Path, time::Instant};

pub fn get_input(package_dir: impl AsRef<Path>, day: u8) -> String {
    let path = package_dir.as_ref().join(format!("src/d{day:02}.txt"));
    std::fs::read_to_string(path).unwrap_or_default()
}

pub fn run<T, F>(f: F)
where
    T: Display,
    F: Fn() -> T,
{
    let start = Instant::now();
    println!("{} {:?}", f(), start.elapsed());
}
