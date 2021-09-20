use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;
use std::fs::File;
use std::io::prelude::*;

mod multi_thread;
mod mono_thread;

fn main() {
    // Number used to generate vector (size = GEN_THREADS * NUM_PER_THREADS)
    const GEN_THREADS: usize = 1 << 4;
    // const NUM_PER_THREADS: usize = 1 << 10;
    const MIN_SIZE: usize = 1 << 5;

    const SIZES: [usize; 8] = [1<<5, 1<<7, 1<<9, (1<<7)+(1<<9), 1<<10, (1<<9)+(1<<10), 1<<11, 1<<12];

    let mut file = File::create("foo.txt").unwrap();
    
    for vec_size in 0..7 {
        writeln!(&mut file, "Size vector = {}", SIZES[vec_size]).unwrap();
        println!("Size vector = {}",  SIZES[vec_size]);

        let mut count_single = 0.0;
        let mut count_multi = 0.0;
        for _ in 1..51 {
            let unordered_vec_a = Arc::new(Mutex::new(Vec::<isize>::new()));
            let unordered_vec_b = Arc::new(Mutex::new(Vec::<isize>::new()));
            
            let mut threads = Vec::new();
            for _ in 1..=GEN_THREADS {
                let unordered_vec_a = Arc::clone(&unordered_vec_a);
                let unordered_vec_b = Arc::clone(&unordered_vec_b);
                threads.push(thread::spawn(move || { // Populating vector
                    let mut rng = rand::thread_rng();
                    let mut single = unordered_vec_a.lock().unwrap();
                    let mut multi = unordered_vec_b.lock().unwrap();
                    for _ in 1..=SIZES[vec_size] {
                        let number = rng.gen::<isize>();
                        single.push(number);
                        multi.push(number);
                    }
                }));
            }
            for t in threads {
                t.join().unwrap();
            }
            let start_single = Instant::now();
            mono_thread::mergesort(Arc::clone(&unordered_vec_a), 0, GEN_THREADS*SIZES[vec_size]);
            let end_single = start_single.elapsed();
            
            let start_multi = Instant::now();
            multi_thread::mergesort(Arc::clone(&unordered_vec_b), 0, GEN_THREADS*SIZES[vec_size], MIN_SIZE);
            let end_multi = start_multi.elapsed();
    
            count_single += end_single.as_millis() as f32;
            count_multi += end_multi.as_millis() as f32;
            writeln!(&mut file, "{:?}\t{:?}",end_single, end_multi).unwrap();
            println!("Single Thread elapsed time: {}ms", end_single.as_millis());
            println!("Multi Thread elapsed time: {}ms", end_multi.as_millis());
        }
        count_single /= 50.0;
        count_multi /= 50.0;
        writeln!(&mut file, "M: {:?}\t{:?}\n",count_single, count_multi).unwrap();
        println!("Media Single: {}ms", count_single);
        println!("Media Multi: {}ms\n", count_multi);
    }
    
    // let mono_numbers = Arc::new(Mutex::new(Vec::<isize>::new()));
    // let multi_numbers = Arc::new(Mutex::new(Vec::<isize>::new()));
    
    // //Generate vector using threads
    // let mut threads = Vec::new();
    // for _ in 1..=GEN_THREADS {
    //     // let mono_numbers = Arc::clone(&mono_numbers);
    //     let multi_numbers = Arc::clone(&multi_numbers);
    //     threads.push(thread::spawn(move || { // Populating vector
    //         let mut rng = rand::thread_rng();
    //         // let mut mono = mono_numbers.lock().unwrap();
    //         let mut multi = multi_numbers.lock().unwrap();
    //         for _ in 1..=NUM_PER_THREADS {
    //             let number = rng.gen::<isize>();
    //             // mono.push(number);
    //             multi.push(number);
    //         }
    //     }));
    // }
    // for t in threads {
    //     t.join().unwrap();
    // }
    // let start_mono = Instant::now();
    // mono_thread::mergesort(Arc::clone(&multi_numbers), 0, GEN_THREADS*NUM_PER_THREADS);
    // let end_mono = start_mono.elapsed();

    // let start_multi = Instant::now();
    // multi_thread::mergesort(Arc::clone(&multi_numbers), 0, GEN_THREADS*NUM_PER_THREADS, MIN_SIZE);
    // let end_multi = start_multi.elapsed();

    // /* CUIDADO, CRIAR THREADS SEM LIMITAÇÃO PODE QUEBRAR O PC
    // let start_multi = Instant::now();
    // multi_thread::mergesort(Arc::clone(&multi_numbers), 0, GEN_THREADS*NUM_PER_THREADS, 1);
    // let end_multi = start_multi.elapsed();
    // */

    // println!("Single Thread elapsed time: {}ms", end_mono.as_millis());
    // println!("Multi Thread elapsed time: {}ms", end_multi.as_millis());
}