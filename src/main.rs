use rand::Rng;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

mod mergesort;


fn main() {
    // Number used to generate vector (size = GEN_THREADS * NUM_PER_THREADS)
    const GEN_THREADS: usize = 3;
    const NUM_PER_THREADS: usize = 10;
    let numbers = Arc::new(Mutex::new(Vec::<isize>::new()));

    //Generate vector using threads
    for _ in 0..GEN_THREADS {
        let numbers = Arc::clone(&numbers);
        thread::spawn(move || { // Populating vector
            let mut rng = rand::thread_rng();
            let mut v = numbers.lock().unwrap();
            for _ in 1..=NUM_PER_THREADS {
                v.push(rng.gen::<isize>());
            }
        });
    }

    {println!("{:?}", numbers.lock().unwrap())};
    mergesort::mergesort(Arc::clone(&numbers), 0, (GEN_THREADS*NUM_PER_THREADS) - 1);
    println!("{:?}", numbers.lock().unwrap());
}