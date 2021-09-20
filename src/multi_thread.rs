use std::sync::{Arc,Mutex};
use std::thread;

pub fn mergesort(arr: Arc<Mutex<Vec<i16>>>, begin: usize, end: usize, min_size: usize)  {
    if end - 1 > begin {
        let mid = begin + (end - begin) / 2;
        
        let create_threads = if end-begin <= min_size {false} else {true};
        
        if create_threads {
            let arr_clone = Arc::clone(&arr);
            let left_thread = thread::spawn(move || {
                mergesort(arr_clone, begin, mid, min_size);
            }); 
            
            let arr_clone = Arc::clone(&arr);
            let right_thread = thread::spawn(move || {
                mergesort(arr_clone, mid, end, min_size);
            });
            left_thread.join().unwrap();
            right_thread.join().unwrap();
        } else {
            mergesort(Arc::clone(&arr), begin, mid, min_size);
            mergesort(Arc::clone(&arr), mid, end, min_size);
        }
        merge(Arc::clone(&arr), begin, mid, end);
    }
}

fn merge(arr: Arc<Mutex<Vec<i16>>>, begin: usize, mid: usize, end: usize) {
    let left_size = mid - begin;
    let right_size = end - mid;
    let mut numbers = arr.lock().unwrap();
    let left_slice = &numbers.clone()[begin..mid];
    let right_slice = &numbers.clone()[mid..end];

    let mut i = 0; // indice da parte esquerda
    let mut j = 0; // indice da parte direita
    let mut k = begin; // indice da lista final
    while i < left_size && j < right_size {
        if left_slice[i] < right_slice[j] {
            numbers[k] = left_slice[i];
            i = i + 1;
        } else {
            numbers[k] = right_slice[j];
            j = j + 1;
        }
        k = k + 1;
    }
    while i < left_size {
        numbers[k] = left_slice[i];
        i = i + 1;
        k = k + 1;
    }

    while j < right_size {
        numbers[k] = right_slice[j];
        j = j + 1;
        k = k + 1;
    }
}