use std::sync::Arc;
use std::thread;

fn main() {
    let mut logger = Arc::new(vec![]);

    let mut handlers = vec![];
    for i in 0..10 {
        let logger_cloned = logger.clone();
        handlers.push(
            thread::spawn(move || {
                // ... do something ...

                // XXX cannot compile because here:
                logger_cloned.push(i);
            }
        ));
    }

    for h in handlers {
        h.join().unwrap();
    }
}
