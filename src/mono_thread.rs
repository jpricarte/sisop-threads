use std::sync::{Arc,Mutex};

pub fn mergesort(arr: Arc<Mutex<Vec<isize>>>, begin: usize, end: usize) {
    if end - 1 > begin {
        let mid = begin + (end - begin) / 2;
        mergesort(Arc::clone(&arr), begin, mid);
        mergesort(Arc::clone(&arr), mid, end);
        merge(Arc::clone(&arr), begin, mid, end);
    }
}

fn merge(arr: Arc<Mutex<Vec<isize>>>, begin: usize, mid: usize, end: usize) {
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