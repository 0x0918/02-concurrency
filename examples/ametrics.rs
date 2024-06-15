use std::{thread, time::Duration};

use anyhow::{Error, Result};
use concurrency::AmapMetrics;
use rand::Rng;

const N: usize = 2;
const M: usize = 4;

fn main() -> Result<()> {
    let metrics = AmapMetrics::new(&[
        "call.thread.worker.0",
        "call.thread.worker.1",
        "req.page.2",
        "req.page.3",
        "req.page.4",
        "req.page.5",
    ]);
    println!("{}", metrics);
    for i in 0..N {
        task_worker(i, metrics.clone())?;
    }

    for _ in 0..M {
        request_worker(metrics.clone())?;
    }
    loop {
        {
            thread::sleep(Duration::from_secs(2));
            println!("{}", metrics);
        }
    }
}

fn task_worker(idx: usize, metric: AmapMetrics) -> Result<()> {
    thread::spawn(move || {
        let mut rng = rand::thread_rng();
        thread::sleep(Duration::from_millis(rng.gen_range(100..5000)));
        metric.inc(format!("call.thread.worker.{}", idx))?;
        Ok::<_, Error>(())
    });
    Ok(())
}

fn request_worker(metric: AmapMetrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_millis(rng.gen_range(50..800)));
            let page = rng.gen_range(1..5);
            metric.inc(format!("req.page.{}", page))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, Error>(())
    });

    Ok(())
}
