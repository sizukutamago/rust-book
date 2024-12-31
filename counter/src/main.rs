use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..4 {
        let counter = counter.clone();
        handles.push(thread::spawn(move || {
            let mut local_counter = 0;
            for _ in 0..2_5000_0000 {
                local_counter += 1;
            }
            let mut writer = counter.lock().unwrap();
            *writer += local_counter;
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("counter = {}", counter.as_ref().lock().unwrap());
}
