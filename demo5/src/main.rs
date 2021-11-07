use nix::sys::signal;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::os::raw::c_int;
use std::ptr::null_mut;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;
use std::{thread, time};

lazy_static::lazy_static! {
    pub(crate) static ref PROFILER: RwLock<HashMap<i64,i64>> = RwLock::new(HashMap::new());
    pub static ref INT_COUNTER: Arc<AtomicI64> = Arc::new(AtomicI64::new(0));
}

thread_local! {
    pub static REQUEST_TAG: AtomicI64 = AtomicI64::new(0);
}

#[repr(C)]
#[derive(Clone)]
struct Timeval {
    pub tv_sec: i64,
    pub tv_usec: i64,
}

#[repr(C)]
#[derive(Clone)]
struct Itimerval {
    pub it_interval: Timeval,
    pub it_value: Timeval,
}

extern "C" {
    fn setitimer(which: c_int, new_value: *mut Itimerval, old_value: *mut Itimerval) -> c_int;
}

const ITIMER_PROF: c_int = 2;

fn set_thread_tag(id: i64) {
    REQUEST_TAG.with(|tag| {
        tag.store(id, Ordering::SeqCst);
    });
}

fn get_thread_tag() -> i64 {
    let mut id = 0;
    REQUEST_TAG.with(|tag| {
        id = tag.load(Ordering::SeqCst);
    });
    return id;
}

fn main() {
    setup_timer();

    for _n in 0..2 {
        thread::spawn(move || {
            loop {
                for id in 1..=4 {
                    set_thread_tag(id);

                    handle_request(id);

                    set_thread_tag(0);
                }
            }
        });
    }

    loop {
        let duration = time::Duration::from_millis(1000);
        thread::sleep(duration);

        let mut summary = HashMap::new();
        if let Some(mut guard) = PROFILER.try_write() {
            for (id, cnt) in guard.iter() {
                summary.insert(*id, *cnt);
            }
            *guard = HashMap::new();
        }

        println!("\n\nrequest cpu usage:");
        let mut total = 0;
        for (_id, cnt) in summary.iter() {
            total = total + cnt;
        }
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
    if id == 4 {
        thread::sleep(time::Duration::from_millis(1));
        return
    }
    let n = id * CYCLE;
    let mut sum = 0;
    for i in 0..n {
        sum = sum + i * 2;
    }
}

fn setup_timer() {
    let freq = 100;
    let interval = 1e6 as i64 / i64::from(freq);
    let it_interval = Timeval {
        tv_sec: interval / 1e6 as i64,
        tv_usec: interval % 1e6 as i64,
    };
    let it_value = it_interval.clone();
    unsafe {
        setitimer(
            ITIMER_PROF,
            &mut Itimerval {
                it_interval,
                it_value,
            },
            null_mut(),
        )
    };

    let handler = signal::SigHandler::Handler(perf_signal_handler);
    unsafe { signal::signal(signal::SIGPROF, handler) };
}

#[no_mangle]
#[allow(clippy::uninit_assumed_init)]
extern "C" fn perf_signal_handler(_signal: c_int) {
    let tag = get_thread_tag();
    if let Some(mut guard) = PROFILER.try_write() {
        let cnt = guard.entry(tag).or_insert(0);
        *cnt = *cnt + 1;
    }
}
