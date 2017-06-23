use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let logger = Arc::new(Mutex::new(vec![]));

    let mut handlers = vec![];
    for i in 0..10 {
        let logger_cloned = logger.clone();
        handlers.push(
            thread::spawn(move || {
                // ... do something ...

                logger_cloned.lock().unwrap().push(i);
            }
        ));
    }

    for h in handlers {
        h.join().unwrap();
    }

    println!("{:?}", *logger.lock().unwrap());
}
