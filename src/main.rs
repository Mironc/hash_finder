use std::sync::{Mutex, RwLock};

use base16ct::lower::{encode_str, encode_string};
use sha2::{Digest, Sha256};
use std::thread::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut f = 0;
    let mut n = 0;
    for i in 0..args.len() {
        if args[i] == "-F" {
            f = args
                .get(i + 1)
                .expect("Missing parameter F")
                .parse()
                .expect("Parameter F should be an integer");
        }
        if args[i] == "-N" {
            n = args
                .get(i + 1)
                .expect("Missing parameter N")
                .parse()
                .expect("Parameter F should be an integer");
        }
    }
    static FOUND: RwLock<i32> = RwLock::new(0);
    static THREAD_N: Mutex<i32> = Mutex::new(0);
    scope(move |scope| {
        let find_hashes = move || {
            let mut thread_n_lock = THREAD_N.lock().unwrap();
            let thread_n = *thread_n_lock;
            let mut iter = 0;
            *thread_n_lock += 1;
            while true {
                iter += 1;
                for i in (thread_n) * iter * 100000..(thread_n + 1) * iter * 100000 {
                    let hash = Sha256::digest(i.to_string());
                    let hex = encode_string(&hash);
                    if *FOUND.read().unwrap() >= f {
                        return;
                    }
                    if hex.chars().rev().take(n).all(|x| x.eq(&'0')) {
                        println!("{} {}", i, hex);
                        let mut write = FOUND.write().unwrap_or_else(|mut e| {
                            **e.get_mut() = 1;
                            FOUND.clear_poison();
                            e.into_inner()
                        });
                        *write += 1;
                    }
                }
            }
        };

        let t1 = spawn(find_hashes);
        let t2 = spawn(find_hashes);
        let t3 = spawn(find_hashes);
        let t4 = spawn(find_hashes);
        t1.join();
        t2.join();
        t3.join();
        t4.join();
    })
}
