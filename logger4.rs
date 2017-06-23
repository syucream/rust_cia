use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let logger = Arc::new(Mutex::new(vec![0]));

    let mut handlers = vec![];
    for _ in 0..10 {
        let cloned_logger = logger.clone();
        handlers.push(
            thread::spawn(move || {
                // ... do something ...

                // XXX deadlock
                let g1 = cloned_logger.lock().unwrap();
                let g2 = cloned_logger.lock().unwrap();
            }
        ));
    }

    for h in handlers {
        h.join().unwrap();
    }

    println!("{:?}", *logger.lock().unwrap());
}
