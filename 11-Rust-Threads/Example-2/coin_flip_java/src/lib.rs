use std::time::Instant;

use robusta_jni::bridge;
use robusta_jni::convert::Signature;

use ::coin_flip::*;
use ::coin_flip::flip_task::FlipTask;


#[bridge]
mod jni {
    use super::*;

    #[derive(Signature)]
    #[package()]
    pub struct Simulate;

    impl Simulate {
        pub extern "jni" fn doFlips(num_threads: i64, num_flips: i64) {

            let num_flips: u64 = num_flips.try_into().unwrap();
            if num_threads == 1 {
                let start = Instant::now();

                let result = FlipTask::simulate_flips(num_flips);
                println!("{}", result);

                let duration = start.elapsed();
                println!("{}", "-".repeat(72));
                println!("Sequential Time: {duration:?}");
            } else {
                let start = Instant::now();

                run_parallel(num_threads.try_into().unwrap(), num_flips);

                let duration = start.elapsed();
                println!("Parallel Time: {duration:?}");
            }
        }
    }
}
