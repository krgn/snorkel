use ndhistogram::{axis::Uniform, ndhistogram, Histogram};
use portmidi as pm;
use std::{
    sync::mpsc,
    time::{Duration, Instant},
};

pub fn run(rx: mpsc::Receiver<crate::Event>) {
    crate::linux::setup_rt_priority();

    let pm = pm::PortMidi::new().expect("could not initialize PortMidi");
    let device = pm.device(0i32).expect("Could not get device 0");
    let mut output = pm.output_port(device, 1024).unwrap();

    // snorkel::print_devices(&pm);

    let bpm = 120;
    let tpb = 24 * bpm;
    let dur = Duration::from_micros(60_000_000 / tpb);

    crate::midi::send_ssp(&mut output);
    crate::midi::send_start(&mut output);

    let mut hist = ndhistogram!(Uniform::new(20, 0.0, 1000.0));

    let sleeper = spin_sleep::SpinSleeper::new(100_000);
    sleeper.sleep(dur);

    // ░█░░░█▀█░█▀█░█▀█
    // ░█░░░█░█░█░█░█▀▀
    // ░▀▀▀░▀▀▀░▀▀▀░▀░░

    let mut start = Instant::now();
    let mut iter = 0u64;
    'inner: loop {
        crate::midi::send_clock_tick(&mut output);

        if iter % 24 == 0 {
            crate::midi::send_note_on(&mut output, 34);
        }

        iter += 1;

        sleeper.sleep(dur);

        // record how long we slept
        hist.fill(&((start.elapsed().as_micros() - dur.as_micros()) as f32));
        start = Instant::now();

        crate::midi::send_note_off(&mut output, 34);

        if let Ok(_) = rx.try_recv() {
            break 'inner;
        }
    }

    crate::midi::send_stop(&mut output);

    println!("histogram: {}", hist);
}
