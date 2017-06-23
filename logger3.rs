use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let logger = Arc::new(Mutex::new(vec![]));

    let mut handlers = vec![];
    for i in 0..10 {
        let cloned_logger = logger.clone();
        handlers.push(
            thread::spawn(move || {
                // ... do something ...

                cloned_logger.lock().unwrap().push(i);
            }
        ));
    }

    for h in handlers {
        h.join().unwrap();
    }

    println!("{:?}", *logger.lock().unwrap());
}
