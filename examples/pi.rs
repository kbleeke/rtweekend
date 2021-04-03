use rand::{Rng, thread_rng};

fn main() {
    let mut rng = thread_rng();

    let sqrt_n = 10_000;
    let mut inside_circle = 0;
    let mut inside_stratified = 0;
    for i in 0..sqrt_n {
        for j in 0..sqrt_n {
            let x = rng.gen_range(-1.0..=1.0);
            let y = rng.gen_range(-1.0..=1.0);

            if x*x + y*y < 1. {
                inside_circle += 1;
            }

            let x = 2. * ((i as f64 + rng.gen::<f64>()) / sqrt_n as f64) - 1.;
            let y = 2. * ((j as f64 + rng.gen::<f64>()) / sqrt_n as f64) - 1.;
            if x*x + y*y < 1. {
                inside_stratified += 1;
            }
        }
    }

    println!("Regular estimate of Pi = {:.12}", 4.0 * inside_circle as f64 / (sqrt_n * sqrt_n) as f64);
    println!("Stratified estimate of Pi = {:.12}", 4.0 * inside_stratified as f64 / (sqrt_n * sqrt_n) as f64);

}