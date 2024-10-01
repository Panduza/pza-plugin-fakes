
use panduza_platform_core::Plugin;



#[no_mangle]
pub unsafe extern "C" fn plugin_entry_point() -> Plugin {
    Plugin::new("test", "v0.1")
}

