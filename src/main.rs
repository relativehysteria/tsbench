use std::thread;
use std::time::Duration;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

const N_THREADS: usize = 30;
const DURATION: Duration = Duration::from_secs(5);
const YIELD_FREQ: usize = 1;

fn bench(yield_freq: Option<usize>) -> Vec<usize> {
    let stop_flag = Arc::new(AtomicBool::new(false));

    let handles: Vec<_> = (0..N_THREADS).map(|_| {
        let stop_flag = Arc::clone(&stop_flag);
        thread::spawn(move || {
            let mut runs = 0;

            while !stop_flag.load(Ordering::Relaxed) {
                runs += 1;
                if let Some(yield_freq) = yield_freq {
                    if runs % yield_freq == 0 { thread::yield_now(); }
                }
            }
            runs
        })
    }).collect();

    thread::sleep(DURATION);
    stop_flag.store(true, Ordering::Relaxed);

    handles.into_iter().map(|h| h.join().unwrap()).collect()
}

fn main() {
    let yields = thread::spawn(|| bench(Some(YIELD_FREQ)));
    let tots   = thread::spawn(|| bench(None));

    let yields = yields.join().unwrap();
    let tots   = tots.join().unwrap();

    let res = tots.into_iter()
        .zip(yields.into_iter())
        .map(|(t, y)| t / y)
        .collect::<Vec<usize>>();

    let min  = res.iter().min().unwrap();
    let max  = res.iter().max().unwrap();
    let mean = res.iter().sum::<usize>() / res.len();
    println!("{min} {max} {mean}");
}
