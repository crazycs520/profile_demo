#![feature(bench_black_box)]

use std::collections::HashMap;
use std::hint::black_box;
use std::os::raw::c_int;
use std::ptr::null_mut;
use std::thread::ThreadId;
use std::{thread, time};

use nix::sys::signal;
use parking_lot::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref PROFILER: Mutex<HashMap<ThreadId, i64>> = Mutex::new(HashMap::new());
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Timeval {
    pub tv_sec: i64,
    pub tv_usec: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Itimerval {
    pub it_interval: Timeval,
    pub it_value: Timeval,
}

extern "C" {
    fn setitimer(which: c_int, new_value: *mut Itimerval, old_value: *mut Itimerval) -> c_int;
}

const ITIMER_PROF: c_int = 2;

fn main() {
    let freq = 99;
    let interval = 1_000_000 / freq;
    let timeval = Timeval {
        tv_sec: interval / 1_000_000,
        tv_usec: interval % 1_000_000,
    };
    unsafe {
        setitimer(
            ITIMER_PROF,
            &mut Itimerval {
                it_interval: timeval,
                it_value: timeval,
            },
            null_mut(),
        )
    };

    let handler = signal::SigHandler::Handler(perf_signal_handler);
    unsafe { signal::signal(signal::SIGPROF, handler).expect("setup profile handler failed") };

    for _thread in 0..2 {
        thread::spawn(move || {
            println!("thread {:?} started", thread::current().id());

            // heavy cpu workload
            loop {
                let mut i = 0;
                while i < 100000000 {
                    i += 1;
                }
                black_box(i);
            }
        });
    }

    for _thread in 0..2 {
        thread::spawn(move || {
            println!("thread {:?} started", thread::current().id());
            loop {
                let duration = time::Duration::from_millis(1);
                thread::sleep(duration);
            }
        });
    }

    loop {
        let duration = time::Duration::from_millis(1000);
        thread::sleep(duration);

        println!("-------------------");
        if let Some(guard) = PROFILER.try_lock() {
            for (id, cnt) in guard.iter() {
                println!("id {:?}, {:?}", id, cnt);
            }
        }
    }
}

#[no_mangle]
#[allow(clippy::uninit_assumed_init)]
extern "C" fn perf_signal_handler(_signal: c_int) {
    let thread_id = thread::current().id();
    if let Some(mut guard) = PROFILER.try_lock() {
        let cnt = guard.entry(thread_id).or_insert(0);
        *cnt += 1;
    }
}
