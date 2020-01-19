mod t;
use t::t1;
use std::io;
use std::thread;
use std::sync::mpsc::sync_channel;
use std::time::Duration;

fn main() {
    let (tx, rx) = sync_channel::<Option<String>>(0);
    let (txx, rxx) = sync_channel::<bool>(0);

    thread::spawn(move || {
        t1(rx, txx);    
    });

    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                println!("{}", input);
            }
            Err(error) => println!("error: {}", error),
        }
        tx.send(Some(input.clone())).unwrap();
        println!("{:?}", rxx.recv());
        if input == "ss\n" {
            tx.send(None).unwrap();
            //println!("{:?}", rxx.recv_timeout(Duration::from_secs(1)));
            thread::sleep(Duration::from_secs(1));
            break;
        }
    }
}
