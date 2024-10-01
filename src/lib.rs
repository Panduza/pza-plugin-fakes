use std::{
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    time::Duration,
};

use panduza_platform_core::{Factory, Plugin, Producer};
use tokio::time::sleep;

static mut tesst: Option<Arc<Mutex<u32>>> = None;
static mut FACTORY: Option<Arc<Mutex<Factory>>> = None;

static mut handle: Option<JoinHandle<()>> = None;

async fn counter() {
    loop {
        println!("I am getting called by Tokio every 2 seconds");
        // Sleep for 1 second
        sleep(Duration::from_secs(2)).await;
    }
}

#[tokio::main]
async fn runForever() {
    let counterTask = tokio::spawn(counter());
    tokio::try_join!(counterTask).unwrap();
}

pub extern "C" fn pok() {
    println!("pooook");

    let _handle: JoinHandle<()> = thread::spawn(move || {
        println!("Trying to create new thread for Tokio runtime");
        runForever();
    });

    unsafe {
        handle = Some(_handle);
    }

    // handle.join().unwrap();
}

pub extern "C" fn join() {
    unsafe {
        handle.take().unwrap().join().unwrap();
    }
}

pub extern "C" fn produce(str_production_order: *const i8) {}

#[no_mangle]
pub unsafe extern "C" fn plugin_entry_point() -> Plugin {
    // if factory none
    // init factory
    let mut factory = Factory::new();
    factory.add_producers(plugin_producers());
    unsafe {
        FACTORY = Some(Arc::new(Mutex::new(factory)));
    }

    // if reactor none
    // init reactor

    // build runtine

    let p = Plugin::new(c"tok", c"v0.1", pok, join, produce);

    // println!("pp {:?}", *(p.name) as u8);

    return p;
}

//
// Import modules
mod register_map;

//
// Export the producers of the plugin
pub fn plugin_producers() -> Vec<Box<dyn Producer>> {
    let mut producers: Vec<Box<dyn Producer>> = vec![];
    producers.push(register_map::producer::RegisterMapProducer::new());
    return producers;
}
