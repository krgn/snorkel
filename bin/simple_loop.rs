use ndhistogram::{axis::Uniform, ndhistogram, Histogram};
use portmidi as pm;
use std::{
    io::BufRead,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

fn run(rx: mpsc::Receiver<Event>) {
    snorkel::setup_rt_priority();

    let pm = pm::PortMidi::new().expect("could not initialize PortMidi");
    let device = pm.device(0i32).expect("Could not get device 0");
    let mut output = pm.output_port(device, 1024).unwrap();

    // snorkel::print_devices(&pm);

    let bpm = 120;
    let tpb = 24 * bpm;
    let dur = Duration::from_micros(60_000_000 / tpb);

    snorkel::midi::send_ssp(&mut output);
    snorkel::midi::send_start(&mut output);

    let mut hist = ndhistogram!(Uniform::new(20, 0.0, 1000.0));

    // ░█░░░█▀█░█▀█░█▀█
    // ░█░░░█░█░█░█░█▀▀
    // ░▀▀▀░▀▀▀░▀▀▀░▀░░

    let sleeper = spin_sleep::SpinSleeper::new(100_000);

    let mut iter = 0u64;
    'inner: loop {
        let start = Instant::now();

        snorkel::midi::send_clock_tick(&mut output);

        if iter % 24 == 0 {
            snorkel::midi::send_note_on(&mut output, 34);
        }

        iter += 1;

        sleeper.sleep(dur);

        // record how long we slept
        hist.fill(&((start.elapsed().as_micros() - dur.as_micros()) as f32));

        snorkel::midi::send_note_off(&mut output, 34);

        if let Ok(_) = rx.try_recv() {
            break 'inner;
        }
    }

    snorkel::midi::send_stop(&mut output);

    println!("histogram: {}", hist);
}

struct Event;

fn main() {
    let (tx, rx) = mpsc::channel::<Event>();
    let t = thread::spawn(|| run(rx));

    let mut stdin = std::io::stdin().lock().lines();
    'main: loop {
        let line = stdin.next().unwrap().unwrap();
        if line == "q" {
            tx.send(Event).unwrap();
            break 'main;
        }
    }

    t.join().expect("couldn't join the fun")
}
