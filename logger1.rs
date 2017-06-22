use std::io::prelude::*;
use std::thread;

fn main() {
    let mut logger = vec![];

    let mut handlers = vec![];
    for i in 0..10 {
        handlers.push(
            thread::spawn(move || {
                // ... do something ...

                // NOTE cannot compile because here:
                logger.push(i);
            }
        ));
    }

    for h in handlers {
        h.join().unwrap();
    }
}
