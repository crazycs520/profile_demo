use std::sync::Arc;
use std::os::raw::c_int;
use std::ptr::null_mut;
use std::sync::atomic::{AtomicI64,Ordering};
use nix::sys::signal;
use std::{thread, time};
use std::time::{Duration, SystemTime};


lazy_static::lazy_static! {
    pub static ref INT_COUNTER: Arc<AtomicI64> = Arc::new(AtomicI64::new(0));
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

fn main() {
    let interval = 1e6 as i64 / i64::from(1);
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
    println!("Hello, world!");
    
    for n in 0..2 {
        thread::spawn(move || {
            println!("thread {:?} started", thread::current().id());
            while true {
                let mut i =0;
                while i < 100000000 {
                    i = i + 1;
                }
            }
        });
    }

    for n in 0..2 {
        thread::spawn(move || {
            println!("thread {:?} started", thread::current().id());
            while true {
                let duration = time::Duration::from_millis(1000);
                thread::sleep(duration);
            }
        });
    }

    while true {
        // let mut i =0;
        // while i < 100000000 {
        //     i = i + 1;
        // }
        let duration = time::Duration::from_millis(1000);
        thread::sleep(duration);
        let cnt = INT_COUNTER.load(Ordering::SeqCst);
        print!("{}\n", cnt);
    }
}


#[no_mangle]
#[allow(clippy::uninit_assumed_init)]
extern "C" fn perf_signal_handler(_signal: c_int) {
    INT_COUNTER.fetch_add(1, Ordering::SeqCst);
    let now = SystemTime::now();
    println!("id {:?}, {:?}", thread::current().id(), now);
}