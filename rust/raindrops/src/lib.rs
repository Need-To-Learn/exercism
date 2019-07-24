
enum RainNoise {
    Pling,
    Plang,
    Plong
}

pub fn raindrops(n: u32) -> String {
    let rainDrops = [
        (3, RainNoise::Pling),
        (5, RainNoise::Plang),
        (7, RainNoise::Plong)
    ]
    let mut res = String::from("");

    for (x, v) in rainDrops.iter() {
        if n % x == 0 {
            res.push_str(&v.to_string())
        }
    }

    if res.is_empty() {
        n.to_string()
    } else {
        res
    }
}
