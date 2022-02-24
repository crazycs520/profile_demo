#![feature(bench_black_box)]

use std::cell::Cell;
use std::collections::HashMap;
use std::hint::black_box;
use std::mem;
use std::os::raw::c_int;
use std::ptr::null_mut;
use std::{thread, time};

use lazy_static::lazy_static;
use nix::sys::signal;
use parking_lot::Mutex;

lazy_static! {
    static ref PROFILER: Mutex<HashMap<i64, i64>> = Mutex::new(HashMap::new());
}

thread_local! {
    static REQUEST_TAG: Cell<i64> = Cell::new(0);
}

fn set_thread_tag(id: i64) {
    REQUEST_TAG.with(|tag| tag.set(id));
}

fn get_thread_tag() -> i64 {
    REQUEST_TAG.with(|tag| tag.get())
}

fn main() {
    setup_timer();

    for _thread in 0..2 {
        thread::spawn(move || loop {
            for request_tag_id in 1..=4 {
                set_thread_tag(request_tag_id);

                handle_request(request_tag_id);

                set_thread_tag(0);
            }
        });
    }

    loop {
        let duration = time::Duration::from_millis(1000);
        thread::sleep(duration);

        let summary = PROFILER.try_lock().map(|mut guard| mem::take(&mut *guard));
        if summary.is_none() {
            continue;
        }
        let summary = summary.unwrap();

        println!("---------------------------------------");
        println!("request cpu usage:");
        let total = summary.values().sum::<i64>();
        for (id, cnt) in summary.iter() {
            println!(
                "request_id {:?}, count: {:?}, cpu usage: {}%",
                id,
                cnt,
                cnt * 100 / total
            );
        }
    }
}

const CYCLE: i64 = 1000000;
fn handle_request(id: i64) {
    // if id == 4 {
    //     thread::sleep(time::Duration::from_millis(10));
    //     return;
    // }
    let n = id * CYCLE;
    let mut sum = 0;
    for i in 0..n {
        sum += i * 2;
    }
    black_box(sum);
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

// Linux setitimer API doc: https://www.gnu.org/software/libc/manual/html_node/Setting-an-Alarm.html
fn setup_timer() {
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

    // Linux signal doc: https://man7.org/linux/man-pages/man7/signal.7.html
    let handler = signal::SigHandler::Handler(perf_signal_handler);
    unsafe { signal::signal(signal::SIGPROF, handler).expect("setup profile handler failed") };
}

#[no_mangle]
#[allow(clippy::uninit_assumed_init)]
extern "C" fn perf_signal_handler(_signal: c_int) {
    let tag = get_thread_tag();
    if let Some(mut guard) = PROFILER.try_lock() {
        let cnt = guard.entry(tag).or_insert(0);
        *cnt += 1;
    }
}
