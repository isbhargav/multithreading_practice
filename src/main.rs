use core::time;
use crossbeam::{scope, thread::ScopedJoinHandle};
use std::thread::sleep;
fn main() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let ans = scope(|s| {
        // println!("{:#?}", v);
        let mut results: Vec<ScopedJoinHandle<i32>> = Vec::new();
        for i in v.iter() {
            let res = s.spawn(move |_| {
                sleep(time::Duration::from_millis(5 * 1000));
                println!("{}", *i);
                return (*i) * 2;
            });
            results.push(res);
        }
        let mut res: Vec<i32> = Vec::new();
        for r in results {
            let val = r.join().unwrap();
            res.push(val);
        }
        return res;
    })
    .unwrap();
    println!("{:#?}", ans);
}
