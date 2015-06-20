use std::thread;
use std::sync::{Arc, Mutex};

fn main() {
    let x1 = Arc::new(Mutex::new(5));

    let x2 = x1.clone();
    let h = thread::spawn(move || {
        let mut x3 = x2.lock().unwrap();
        *x3 += 1;
    });

    h.join().unwrap();

    println!("{:?}", *(x1.lock().unwrap()));
}
