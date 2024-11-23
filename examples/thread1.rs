use anyhow::Result;
use std::time::Duration;
use std::{sync::mpsc, thread};

const N: usize = 4;

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    idx: usize,
    v: usize,
}

impl Msg {
    fn new(idx: usize, v: usize) -> Self {
        Msg { idx, v }
    }
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();
    for i in 0..N {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }
    drop(tx); // 线程中的tx都是由clone创建的，这里drop掉初始的tx。

    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("msg: {:?}", msg);
        }
        println!("consumer exit");
        42 // consumer闭包可以有返回值，并在join()后返回
    });

    let secret = consumer
        .join()
        .map_err(|e| anyhow::anyhow!("thread join error {:?}", e))?;
    println!("secret: {}", secret);

    Ok(())
}

fn producer(i: usize, tx: mpsc::Sender<Msg>) -> Result<()> {
    loop {
        tx.send(Msg::new(i, rand::random::<usize>()))?;
        let sleep_time = rand::random::<u8>() as u64 * 10;
        thread::sleep(Duration::from_millis(sleep_time));
        if rand::random::<u8>() % 3 == 0 {
            println!("producer {} exit", i);
            break;
        }
    }
    Ok(())
}
