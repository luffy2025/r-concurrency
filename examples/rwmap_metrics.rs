use anyhow::Result;
use r_concurrency::RwMetrics;
use rand::Rng;
use std::thread;
use std::time::Duration;

const N: usize = 2;
const M: usize = 4;

fn main() -> Result<()> {
    let metrics = RwMetrics::new();
    metrics.inc("requests.prepare")?;
    metrics.inc("requests.prepare")?;
    println!("{:?}", metrics.snapshot());

    for idx in 0..N {
        task_worker(idx, metrics.clone());
    }
    for _ in 0..M {
        request_worker(metrics.clone());
    }

    loop {
        thread::sleep(Duration::from_secs(2));
        println!("{}", metrics);
    }
}

fn task_worker(idx: usize, metrics: RwMetrics) {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_millis(rng.gen_range(0..1000)));
            metrics.inc(format!("thread.worker.{}", idx))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
}

fn request_worker(metrics: RwMetrics) {
    thread::spawn(move || {
        for _ in 0..10 {
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_millis(rng.gen_range(500..800)));
            let page = rng.gen_range(6..8);
            metrics.inc(format!("requests.page.{}", page))?;
        }
        Ok::<_, anyhow::Error>(())
    });
}
