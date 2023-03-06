use std::{sync::mpsc, thread};

fn main() {
    let (tx, rx) = mpsc::channel::<snorkel::Event>();
    let t = thread::spawn(|| snorkel::timerfd::run(rx));

    ctrlc::set_handler(move || {
        tx.send(snorkel::Event)
            .expect("Could not send signal on channel.")
    })
    .expect("Error setting Ctrl-C handler");

    t.join().expect("couldn't join the fun")
}
