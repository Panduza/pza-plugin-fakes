
use std::ffi::{c_char, CString};

use panduza_platform_core::Plugin;


// Create Rc references to the CString objects to ensure they outlive the Plugin structure
// let name_rc = CString::new(name).unwrap();
// let version_rc = CString::new(version).unwrap();

// unsafe {
//     println!("pp {:?}", (*name_rc.as_ptr()) as u8);
// }

// const fn initialize_name_rc() -> CString {
//     // Your initialization logic here
//     CString::new("Hello from DLL").unwrap()
// }

// static mut NAME_RC: CString = initialize_name_rc();

// ;= CString::new("Hello from DLL").unwrap();

pub extern "C" fn pok() {
    println!("pooook");
}


#[no_mangle]
pub unsafe extern "C" fn plugin_entry_point() -> Plugin {


    
    let p = Plugin::new("tok", "v0.1", pok);

    // println!("pp {:?}", *(p.name) as u8);

    return p;
}

