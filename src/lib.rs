panduza_platform_core::plugin_interface!();

//
// Import modules
mod register_map;

//
// Export the producers of the plugin
pub fn plugin_producers() -> Vec<Box<dyn Producer>> {
    let mut producers: Vec<Box<dyn Producer>> = vec![];
    producers.push(register_map::producer::RegisterMapProducer::new());
    // producers.push(register_map::producer::RegisterMapProducer2::new());
    return producers;
}
