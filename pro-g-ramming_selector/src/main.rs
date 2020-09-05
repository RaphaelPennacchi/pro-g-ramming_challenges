use rand::Rng;

fn main() {
    println!("Selected Project: {}", rand::thread_rng().gen_range(1, 145));
}
