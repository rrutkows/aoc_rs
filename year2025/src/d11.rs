use std::collections::HashMap;

pub fn solve01(input: &str) -> i32 {
    let conn = parse(input);
    let mut sum = 0;
    let mut q = vec!["you"];
    while let Some(current) = q.pop() {
        if current == "out" {
            sum += 1;
            continue;
        }

        q.extend(conn.get(current).unwrap().iter());
    }

    sum
}

pub fn solve02(input: &str) -> u64 {
    let conn = parse(input);
    let mut cache = HashMap::new();
    dp(&conn, &mut cache, "svr", false, false)
}

fn dp<'a>(
    conn: &'a HashMap<&str, Vec<&str>>,
    cache: &mut HashMap<(&'a str, bool, bool), u64>,
    device: &'a str,
    has_dac: bool,
    has_fft: bool,
) -> u64 {
    let key = (device, has_dac, has_fft);
    if device == "out" {
        if has_dac && has_fft { 1 } else { 0 }
    } else if cache.contains_key(&key) {
        cache[&key]
    } else {
        let result = conn[device]
            .iter()
            .map(|next| {
                dp(
                    conn,
                    cache,
                    next,
                    has_dac || device == "dac",
                    has_fft || device == "fft",
                )
            })
            .sum();
        cache.insert(key, result);
        result
    }
}

fn parse(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut conn: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let (src, dst) = line.split_once(':').unwrap();
        let connected = conn.entry(src).or_default();
        connected.extend(dst.split_whitespace());
    }

    conn
}
