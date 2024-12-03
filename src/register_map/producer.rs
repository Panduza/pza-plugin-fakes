use super::device::RegisterMapDevice;
use panduza_platform_core::{DriverOperations, Producer};

pub struct RegisterMapProducer {}

impl RegisterMapProducer {
    pub fn new() -> Box<RegisterMapProducer> {
        Box::new(RegisterMapProducer {})
    }
}

impl Producer for RegisterMapProducer {
    fn manufacturer(&self) -> String {
        "panduza".to_string()
    }

    fn model(&self) -> String {
        "fake_register_map".to_string()
    }

    fn description(&self) -> String {
        "todo".to_string()
    }

    fn props(&self) -> panduza_platform_core::Props {
        panduza_platform_core::Props::default()
    }

    fn produce(&self) -> Result<Box<dyn DriverOperations>, panduza_platform_core::Error> {
        return Ok(Box::new(RegisterMapDevice::new()));
    }
}
