mod m1;

use m1::func;

use std::thread::spawn;//creating a thread

use std::sync::mpsc::channel;//creating sender and receiver;
use std::sync::Arc;// Arc is a rc for multithreads => preventing multiaccess to same memory;
use std::sync::Mutex;//Mutex is a refcell for multithreads => preventing multichanges to same data;


fn main() {
    func();
    let handle = spawn(||{

        println!("this is thread1");
    });
    let _ = handle.join();


    let x = Arc::new(Mutex::new(0));
    let y = x.clone();
    let t = x.clone();
    let (tx,rx) = channel();
    let tx1 = tx.clone();
    let _ = spawn(move ||{
        tx.send("1").unwrap();
        tx.send("2").unwrap();
        tx.send("3").unwrap();
        let mut val = x.try_lock();
        match val {
            Ok(mut val) =>{
                println!("value changed by thread 1");
                *val += 1;
            }
            Err(_) => {
                println!("failed to lock");
            }
        }
    });
    let _ = spawn(move || {
        tx1.send("hello").unwrap();
        let mut val = y.try_lock();
        match val {
            Ok(mut val) => {
                *val += 1;
                println!("value changed by thread 2");
            }
            Err(_) => println!("failed to lock from thread 2"),
        }
    });

    println!("{}",rx.recv().unwrap());
    println!("{}",rx.recv().unwrap());
    println!("{}",rx.recv().unwrap());
    println!("{}",rx.recv().unwrap());
    // the threads run simultaneously, so the received message order isn't determined
    println!("{}",t.lock().unwrap());
}
