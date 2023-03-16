//! Snorkel!
//!
//! 🫧🫧
//!   🤿
//!

// TODO: turn this back on
// #![warn(missing_docs)]

pub mod config;
pub mod mode;
pub mod op;
pub mod simple_loop;
pub mod snrkl;
pub mod state;
pub mod timerfd;
pub mod ui;

pub struct Event;

pub mod util {
    use std::cmp;

    pub fn clip<T>(x: T, lo: T, hi: T) -> T
    where
        T: Ord,
    {
        std::cmp::max(std::cmp::min(x, hi), lo)
    }

    #[derive(Debug, Clone, Default, Eq, PartialEq, PartialOrd, Ord)]
    pub struct Selection {
        pub start_x: usize,
        pub start_y: usize,
        pub end_x: usize,
        pub end_y: usize,
    }

    impl Selection {
        pub fn from(a: &Coord, b: &Coord) -> Selection {
            let start_x = cmp::min(a.x, b.x);
            let start_y = cmp::min(a.y, b.y);
            let end_x = cmp::max(a.x, b.x);
            let end_y = cmp::max(a.y, b.y);
            Selection {
                start_x,
                start_y,
                end_x,
                end_y,
            }
        }

        pub fn contains(&self, loc: &Coord) -> bool {
            return self.start_x <= loc.x
                && loc.x <= self.end_x
                && self.start_y <= loc.y
                && loc.y <= self.end_y;
        }
    }

    #[cfg(test)]
    mod selection_tests {
        use super::{Coord, Selection};

        #[test]
        fn should_correctly_identify_position_inside_selection() {
            let start = Coord { x: 3, y: 3 };
            let end = Coord { x: 0, y: 0 };
            let selection = Selection::from(&start, &end);
            // all these points are inside
            for y in 0..4 {
                for x in 0..4 {
                    assert!(selection.contains(&Coord { x, y }))
                }
            }
            // all these points are outside
            for y in 4..20 {
                for x in 4..20 {
                    assert!(!selection.contains(&Coord { x, y }))
                }
            }
        }

        #[test]
        fn should_correctly_identify_position_inside_flipped_selection() {
            let start = Coord { x: 0, y: 0 };
            let end = Coord { x: 3, y: 3 };
            let selection = Selection::from(&start, &end);
            // all these points are inside
            for y in 0..4 {
                for x in 0..4 {
                    assert!(selection.contains(&Coord { x, y }))
                }
            }
            // all these points are outside
            for y in 4..20 {
                for x in 4..20 {
                    assert!(!selection.contains(&Coord { x, y }))
                }
            }
        }
    }

    #[derive(Debug, Clone, Default, Eq, PartialEq, PartialOrd, Ord)]
    pub struct Coord {
        pub x: usize,
        pub y: usize,
    }

    #[cfg(test)]
    mod coord_tests {
        use super::Coord;

        #[test]
        fn comparison_of_same_values_should_work_correctly() {
            let a = Coord { x: 1, y: 2 };
            let b = Coord { x: 1, y: 2 };
            assert_eq!(a, b)
        }

        #[test]
        fn comparison_of_struturally_different_values_should_work_correctly() {
            let a = Coord { x: 1, y: 8 };
            let b = Coord { x: 1, y: 2 };
            assert_ne!(a, b)
        }
    }
}

// ░█▄█░▀█▀░█▀▄░▀█▀
// ░█░█░░█░░█░█░░█░
// ░▀░▀░▀▀▀░▀▀░░▀▀▀

pub mod midi {
    use pm::MidiMessage;
    use portmidi as pm;

    const CLOCK_TICK: MidiMessage = MidiMessage {
        status: 0xF8,
        data1: 0x0,
        data2: 0x0,
        data3: 0x0,
    };

    const START: MidiMessage = MidiMessage {
        status: 0xFA,
        data1: 0x0,
        data2: 0x0,
        data3: 0x0,
    };

    const STOP: MidiMessage = MidiMessage {
        status: 0xFC,
        data1: 0x0,
        data2: 0x0,
        data3: 0x0,
    };

    const SSP: MidiMessage = MidiMessage {
        status: 0xF2,
        data1: 0x0,
        data2: 0x0,
        data3: 0x0,
    };

    pub fn send_ssp<'a>(output: &mut pm::OutputPort<'a>) {
        output.write_message(SSP).unwrap()
    }

    pub fn send_clock_tick<'a>(output: &mut pm::OutputPort<'a>) {
        output.write_message(CLOCK_TICK).unwrap()
    }

    pub fn send_start<'a>(output: &mut pm::OutputPort<'a>) {
        output.write_message(START).unwrap()
    }

    pub fn send_stop<'a>(output: &mut pm::OutputPort<'a>) {
        output.write_message(STOP).unwrap()
    }

    pub fn send_note_on<'a>(output: &mut pm::OutputPort<'a>, note: u8) {
        let note_on = MidiMessage {
            status: 0x90, // note-on, chan 0
            data1: note,  // middle c
            data2: 127,   // full blast
            data3: 0x0,
        };
        output.write_message(note_on).unwrap()
    }

    pub fn send_note_off<'a>(output: &mut pm::OutputPort<'a>, note: u8) {
        let note_off = MidiMessage {
            status: 0x90, // note-on, chan 0
            data1: note,  // middle c
            data2: 0x0,   // off
            data3: 0x0,
        };
        output.write_message(note_off).unwrap()
    }
}

// ░█▀█░█▀▄░▀█▀░█░█░█▀█░▀█▀░█▀▀
// ░█▀▀░█▀▄░░█░░▀▄▀░█▀█░░█░░█▀▀
// ░▀░░░▀░▀░▀▀▀░░▀░░▀░▀░░▀░░▀▀▀

pub mod linux {
    use portmidi as pm;

    pub fn setup_rt_priority() {
        let limit = 50;
        set_rt_prio_limit(limit);
        unsafe {
            let thread = libc::pthread_self();
            set_sched_param(thread, limit as libc::c_int);
        }
    }

    unsafe fn set_sched_param(thread: libc::pthread_t, priority: libc::c_int) {
        let mut policy: libc::c_int = 0x0;

        let mut param = libc::sched_param {
            sched_priority: 0x0,
        };

        let res = libc::pthread_getschedparam(thread, &mut policy, &mut param);

        if res != 0 {
            eprintln!("getting sched_param failed: {}", res);
        }

        // set up the desired policy before getting min/max values!
        policy = libc::SCHED_FIFO;

        let min = libc::sched_get_priority_min(policy);
        let max = libc::sched_get_priority_max(policy);

        param.sched_priority = crate::util::clip(priority, min, max);

        let res = libc::pthread_setschedparam(thread, policy, &param);
        if res != 0 {
            eprintln!(
                "could not set rt-prio {} on thread: {}",
                param.sched_priority, res
            );
        } else {
            println!(
                "rt-priorities set up successfully with priority: {}",
                param.sched_priority
            )
        }
    }

    /// Enables real time thread priorities in the current thread up to `limit`.
    fn set_rt_prio_limit(limit: u64) {
        let rt_limit_arg = libc::rlimit {
            rlim_cur: limit as libc::rlim_t,
            rlim_max: limit as libc::rlim_t,
        };
        // Safe because the kernel doesn't modify memory that is accessible to the process here.
        let res = unsafe { libc::setrlimit(libc::RLIMIT_RTPRIO, &rt_limit_arg) };
        if res != 0 {
            eprintln!("received {} from rlimit call", res)
        }
    }

    pub fn print_devices(pm: &pm::PortMidi) {
        for dev in pm.devices().unwrap() {
            println!("{}", dev);
        }
    }
}
