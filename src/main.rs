use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

mod multi_thread;
mod mono_thread;

fn main() {
    // Number used to generate vector (size = GEN_THREADS * NUM_PER_THREADS)
    const GEN_THREADS: usize = 1 << 4;
    const NUM_PER_THREADS: usize = 1 << 10;
    const MIN_SIZE: usize = 1 << 11;
    let mono_numbers = Arc::new(Mutex::new(Vec::<isize>::new()));
    let multi_numbers = Arc::new(Mutex::new(Vec::<isize>::new()));
    
    //Generate vector using threads
    let mut threads = Vec::new();
    for _ in 1..=GEN_THREADS {
        let mono_numbers = Arc::clone(&mono_numbers);
        let multi_numbers = Arc::clone(&multi_numbers);
        threads.push(thread::spawn(move || { // Populating vector
            let mut rng = rand::thread_rng();
            let mut mono = mono_numbers.lock().unwrap();
            let mut multi = multi_numbers.lock().unwrap();
            for _ in 1..=NUM_PER_THREADS {
                let number = rng.gen::<isize>();
                mono.push(number);
                multi.push(number);
            }
        }));
    }
    for t in threads {
        t.join().unwrap();
    }
    let start_mono = Instant::now();
    mono_thread::mergesort(Arc::clone(&mono_numbers), 0, GEN_THREADS*NUM_PER_THREADS);
    let end_mono = start_mono.elapsed();

    let start_multi = Instant::now();
    multi_thread::mergesort(Arc::clone(&multi_numbers), 0, GEN_THREADS*NUM_PER_THREADS, MIN_SIZE);
    let end_multi = start_multi.elapsed();

    println!("Single Thread elapsed time: {}ms", end_mono.as_millis());
    println!("Multi Thread elapsed time: {}ms", end_multi.as_millis());
}