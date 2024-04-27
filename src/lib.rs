use rand::Rng;

pub const FRUITS: [&str; 10] = [
    "Apple",
    "Banana",
    "Orange",
    "Pineapple",
    "Strawberry",
    "fdgfsd1","fdgsf2","fdgsdfg3","fdgsdfg4", "fdgsfgsdgf5"
];

pub fn random_fruit() -> &'static str {
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..FRUITS.len());
    FRUITS[random_index]
}
