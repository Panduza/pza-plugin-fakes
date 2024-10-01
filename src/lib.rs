use panduza_platform_core::Plugin;




pub extern "C" fn pok() {
    println!("pooook");
}


#[no_mangle]
pub unsafe extern "C" fn plugin_entry_point() -> Plugin {


    
    let p = Plugin::new("tok", "v0.1", pok);

    // println!("pp {:?}", *(p.name) as u8);

    return p;
}

