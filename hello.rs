use std::thread;

fn main() {
    let mut x = 5;

    let h = thread::spawn(|| {
        x += 1;
    });

    h.join().unwrap();

    println!("{}", x);
}
