pub fn t1(rx: std::sync::mpsc::Receiver<std::option::Option<std::string::String>>, txx: std::sync::mpsc::SyncSender<bool>) {
    print("setting".to_string());
    for received in rx {
        match received {
            Some(x) => {
                if x.contains("s") {
                    txx.send(true).unwrap();
                    print(x);
                } else {
                    txx.send(false).unwrap();
                }
            }
            None => {
                println!("exit");
                break;
            }
        }
    }
}

fn print(x: String) {
    println!("Got: {}", x);
}