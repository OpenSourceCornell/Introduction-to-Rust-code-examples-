use std::thread;

fn main() {
    let mut acc = Box::new(0);

    let threads: Vec<_> = (0..10).map(|_| {
        thread::scoped(move || {
            for _ in 0..100000 {
                *acc += 1;
            }
        })
    }).collect();

    for t in threads {
        t.join();
    }

    println!("{}", acc);
}


// use std::thread;
// use std::sync::{Arc,Mutex};

// fn main() {
    // let mut acc = Arc::new(Mutex::new(Box::new(0)));

    // let threads: Vec<_> = (0..10).map(|_| {
        // let acc = acc.clone();
        // thread::scoped(move || {
            // let mut acc = acc.lock().unwrap();
            // for _ in 0..100000 {
                // **acc = **acc + 1;
            // }
        // })
    // }).collect();

    // for t in threads {
        // t.join();
    // }

    // println!("{}", **acc.lock().unwrap());
// }
