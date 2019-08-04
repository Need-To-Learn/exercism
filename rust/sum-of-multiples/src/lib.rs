pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
    .filter(|x| have_multiple(x, factors))
    .sum()
}

fn is_multiple(n: u32, factor: u32) -> bool {
    factor > 0 && n % factor == 0
}

pub fn have_multiple(n: &u32, factors: &[u32]) -> bool {
    factors.iter().any(|factor| is_multiple(*n, *factor))
}