use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let logger1 = Arc::new(Mutex::new(vec![]));
    let logger2 = Arc::new(Mutex::new(vec![]));

    let mut handlers = vec![];
    for i in 0..10 {
        let cloned_logger1 = logger1.clone();
        let cloned_logger2 = logger2.clone();
        handlers.push(
            thread::spawn(move || {
                // ... do something ...

                // XXX
                cloned_logger1.lock().unwrap().push(i);
                cloned_logger2.lock().unwrap().push(i);
            }
        ));
    }

    for h in handlers {
        h.join().unwrap();
    }
}
