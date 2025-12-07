use std::{fmt::Display, path::Path, time::Instant};

pub fn get_input(package_dir: impl AsRef<Path>, year: u16, day: u8) -> String {
    let path = package_dir.as_ref().join(format!("src/d{day:02}.txt"));
    if path.exists() {
        std::fs::read_to_string(&path).unwrap()
    } else {
        let session_path = package_dir.as_ref().parent().unwrap().join("session");
        let session = std::fs::read_to_string(session_path).unwrap();
        ureq::get(format!("https://adventofcode.com/{year}/day/{day}/input"))
            .header(
                "User-Agent",
                "https://github.com/rrutkows/aoc_rs by rrutkowski (at) gmail.com",
            )
            .header("Cookie", &format!("session={}", session.trim()))
            .call()
            .and_then(|mut response| response.body_mut().read_to_string())
            .inspect(|input| {
                std::fs::write(&path, input).unwrap();
            })
            .unwrap()
    }
}

pub fn run<T, F>(f: F)
where
    T: Display,
    F: Fn() -> T,
{
    let start = Instant::now();
    println!("{} {:?}", f(), start.elapsed());
}

pub fn run_both<T1, T2, F>(f: F)
where
    T1: Display,
    T2: Display,
    F: Fn() -> (T1, T2),
{
    let start = Instant::now();
    let (p1, p2) = f();
    println!("{}, {} {:?}", p1, p2, start.elapsed());
}
