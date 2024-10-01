use std::{sync::{Arc, Mutex}, thread::{self, JoinHandle}, time::Duration};

use panduza_platform_core::Plugin;
use tokio::time::sleep;


static mut tesst : Option<Arc<Mutex<u32>>> = None;
static mut handle : Option<JoinHandle<()>> = None;


async fn counter() {
    loop {
        println!("I am getting called by Tokio every 2 seconds");
        // Sleep for 1 second
        sleep(Duration::from_secs(2)).await;
    }
}

#[tokio::main]
async fn runForever() {
    let counterTask = tokio::spawn(
        counter()
    );
    tokio::try_join!(counterTask).unwrap();

}


pub extern "C" fn pok() {
    println!("pooook");

    let _handle: JoinHandle<()> = thread::spawn(move || {
        println!("Trying to create new thread for Tokio runtime");
        runForever();
    }
        // counter()
        // || {
        // some work here
        // println!("Trying to create new thread for Tokio runtime");
        // runForever();

    // }
    );

    unsafe  {
        handle = Some( _handle );
    }
    

    // handle.join().unwrap();
}


pub extern "C" fn join() {
    unsafe {
        handle.take().unwrap().join().unwrap();        
    }
}


#[no_mangle]
pub unsafe extern "C" fn plugin_entry_point() -> Plugin {


    
    let p = Plugin::new("tok", "v0.1", pok, join);

    // println!("pp {:?}", *(p.name) as u8);

    return p;
}

