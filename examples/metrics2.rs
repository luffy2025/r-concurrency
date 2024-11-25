use anyhow::Result;
use r_concurrency::Metrics2;
use rand::Rng;
use std::thread;
use std::time::Duration;

const N: usize = 2;
const M: usize = 4;

fn main() -> Result<()> {
    let metrics2 = Metrics2::new();
    metrics2.inc("requests.prepare")?;
    metrics2.inc("requests.prepare")?;
    println!("{}", metrics2);

    for idx in 0..N {
        task_worker(idx, metrics2.clone());
    }
    for _ in 0..M {
        request_worker(metrics2.clone());
    }

    loop {
        thread::sleep(Duration::from_secs(2));
        println!("{}", metrics2);
    }
}

fn task_worker(idx: usize, metrics2: Metrics2) {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_millis(rng.gen_range(0..1000)));
            metrics2.inc(format!("thread.worker.{}", idx))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
}

fn request_worker(metrics2: Metrics2) {
    thread::spawn(move || {
        for _ in 0..10 {
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_millis(rng.gen_range(500..800)));
            let page = rng.gen_range(6..8);
            metrics2.inc(format!("requests.page.{}", page))?;
        }
        Ok::<_, anyhow::Error>(())
    });
}
