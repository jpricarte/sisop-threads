use std::sync::{Arc, Mutex};
use std::thread;

pub fn mergesort(numbers: Arc<Mutex<Vec<isize>>>, beg: usize, end: usize) {
    if beg==end{
        return;
    }

    let mid: usize = (beg+end)/2;
    
    let numbers_clone = Arc::clone(&numbers);
    let left_thread = thread::spawn(move || {
        mergesort(numbers_clone, beg, mid)
    });

    let numbers_clone = Arc::clone(&numbers);
    let right_thread = thread::spawn(move || {
        mergesort(numbers_clone, mid+1, end);
    });
    left_thread.join().unwrap();
    right_thread.join().unwrap();
    merge(numbers, beg, end);
}

fn merge(numbers: Arc<Mutex<Vec<isize>>>, beg: usize, end: usize) {
    let mid: usize = (beg+end)/2;
    
    // Copia os valores da lista para não prender o Mutex
    let aux_list: Vec<isize> = { numbers.lock().unwrap().clone() };
    // Lista que terá os valores organizados
    let mut org_list: Vec<isize> = Vec::new();

    let mut i = beg;
    let mut j = mid+1;

    while i != mid && j != end {
        println!("i={}, j={}",i,j);
        if aux_list[i] <=aux_list[j] {
            org_list.push(aux_list[i]);
            i+=1;
        } else {
            org_list.push(aux_list[j]);
            j+=1;
        }
    }

    while i != mid {
        org_list.push(aux_list[i]);
        i+=1;
    }

    while j != end {
        org_list.push(aux_list[i]);
        j+=1;
    }

    {
        let mut original_list = numbers.lock().unwrap();
        
        let mut i = 0;
        for number in org_list.iter() {
            original_list[beg+i] = *number;
            i+=1;
        }
    }
}