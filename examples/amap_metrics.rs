use anyhow::Result;
use r_concurrency::AmapMetrics;
use rand::Rng;
use std::thread;
use std::time::Duration;

const N: usize = 2;
const M: usize = 4;

fn main() -> Result<()> {
    let worker_names = ["thread.worker.0", "thread.worker.1"];
    let request_names = [
        "page.request.0",
        "page.request.1",
        "page.request.2",
        "page.request.3",
    ];

    let mut metrics_names = Vec::with_capacity(request_names.len() + worker_names.len());
    worker_names
        .iter()
        .for_each(|&name| metrics_names.push(name));
    request_names
        .iter()
        .for_each(|&name| metrics_names.push(name));
    let metrics = AmapMetrics::new(metrics_names.as_slice());

    let len = metrics_names.len();
    for idx in 0..N {
        task_worker(metrics.clone(), worker_names[idx % len]);
    }
    for idx in 0..M {
        request_worker(metrics.clone(), request_names[idx % len]);
    }

    loop {
        thread::sleep(Duration::from_secs(2));
        println!("{}", metrics);
    }
}

fn task_worker(metrics: AmapMetrics, key: &'static str) {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_millis(rng.gen_range(0..1000)));
            metrics.inc(key)?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
}

fn request_worker(metrics: AmapMetrics, key: &'static str) {
    thread::spawn(move || {
        for _ in 0..10 {
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_millis(rng.gen_range(500..800)));
            metrics.inc(key)?;
        }
        Ok::<_, anyhow::Error>(())
    });
}
