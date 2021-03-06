use rand::{thread_rng, Rng};

fn pdf(_x: f64) -> f64 {
    0.5
}

fn main() {
    let mut rng = thread_rng();

    let n = 1_000_000;
    let sum: f64 = (0..n)
        .map(|_i| {
            let x: f64 = rng.gen_range::<f64, _>(0.0..=2.0);
            x * x / pdf(x)
        })
        .sum();

    println!("I = {:.12}", sum / n as f64);
}
