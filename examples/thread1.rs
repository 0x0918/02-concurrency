use std::{sync::mpsc, thread, time::Duration};

use anyhow::{anyhow, Ok, Result};
use rand::random;
const NUM_PRODUCER: usize = 4;

#[derive(Debug)]
#[allow(dead_code)]
struct Msg {
    idx: usize,
    value: usize,
}
fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();
    //创建producers
    for i in 0..NUM_PRODUCER {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }

    //创建consumer
    let consumer = thread::spawn(|| {
        for msg in rx {
            println!("Consumer: {:?}", msg);
        }
    });
    let _ = consumer
        .join()
        .map_err(|e| anyhow!("Thread join error: {:?}", e));
    Ok(())
}

fn producer(idx: usize, tx: mpsc::Sender<Msg>) -> Result<()> {
    loop {
        let value: usize = random::<usize>();
        tx.send(Msg::new(idx, value))?;
        let sleep_time: u64 = random::<u8>() as u64 * 10;
        thread::sleep(Duration::from_millis(sleep_time));
    }
}

impl Msg {
    fn new(idx: usize, value: usize) -> Self {
        Self { idx, value }
    }
}
