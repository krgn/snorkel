use ndhistogram::{axis::Uniform, ndhistogram, Histogram};
use portmidi as pm;
use std::{
    sync::mpsc::Receiver,
    time::{Duration, Instant},
};
use timerfd::TimerFd;

pub fn run(rx: Receiver<crate::Event>) {
    crate::linux::setup_rt_priority();

    let pm = pm::PortMidi::new().expect("could not initialize PortMidi");
    let device = pm.device(0i32).expect("Could not get device 0");
    let mut output = pm.output_port(device, 1024).unwrap();

    let bpm = 120;
    let tpb = 24 * bpm;
    let dur = Duration::from_micros(60_000_000 / tpb);

    let mut tfd = TimerFd::new().unwrap();
    let state = timerfd::TimerState::Periodic {
        current: Duration::from_secs(1),
        interval: dur,
    };

    let mut hist = ndhistogram!(Uniform::new(20, -100.0, 100.0));

    tfd.set_state(state, timerfd::SetTimeFlags::Default);

    crate::midi::send_ssp(&mut output);
    crate::midi::send_start(&mut output);

    let mut iter = 0u64;
    'main: loop {
        let begin = Instant::now();
        crate::midi::send_clock_tick(&mut output);

        if iter % 24 == 0 {
            crate::midi::send_note_on(&mut output, 59);
        }

        iter += 1;

        tfd.read();

        let jitter = (dur.as_micros() as i64) - (begin.elapsed().as_micros() as i64);
        hist.fill(&(jitter as f32));

        if let Ok(_) = rx.try_recv() {
            break 'main;
        }
    }

    crate::midi::send_stop(&mut output);

    println!("histogram: {}", hist);
}
