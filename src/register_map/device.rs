use async_trait::async_trait;
use panduza_platform_core::{
    spawn_on_command, DriverOperations, Error, Instance, InstanceLogger, MemoryCommandAttServer,
    MemoryCommandMode, NumberAttServer, TaskResult,
};
use std::{sync::Arc, time::Duration};
use tokio::time::sleep;

///
///
static mut COUNTER: u16 = 0;
///
/// This device is a simulation of a register map that you can access through commands
///
pub struct RegisterMapDevice {
    logger: Option<InstanceLogger>,
    array: Arc<Vec<NumberAttServer>>,
}

impl RegisterMapDevice {
    ///
    /// Constructor
    ///
    pub fn new() -> RegisterMapDevice {
        RegisterMapDevice {
            logger: None,
            array: Arc::new(Vec::new()),
        }
    }

    ///
    /// Triggered when a new command is received
    ///
    async fn on_command_action(
        logger: InstanceLogger,
        array: Arc<Vec<NumberAttServer>>,
        mut attr_command: MemoryCommandAttServer,
    ) -> TaskResult {
        while let Some(command) = attr_command.pop_cmd().await {
            logger.debug(format!("New command {:?}", command));

            match command.mode {
                MemoryCommandMode::Read => {
                    let idx = command.address;
                    unsafe {
                        COUNTER += 1;
                        array[idx as usize].set_from_i64(COUNTER.into()).await?;
                    }
                }
                MemoryCommandMode::Write => {}
                _ => {}
            }
        }

        Ok(())
    }

    ///
    /// Register map can be updated through memory command
    ///
    async fn create_memory_command_attribute(
        &mut self,
        mut instance: Instance,
    ) -> Result<(), Error> {
        //
        // Create the attribute
        let attr_command = instance
            .create_attribute("command")
            .with_rw()
            .finish_as_memory_command()
            .await?;

        //
        // Execute action on each command received
        let logger = self.logger.as_ref().unwrap().clone();
        let array = self.array.clone();
        spawn_on_command!(
            instance,
            attr_command,
            Self::on_command_action(logger.clone(), array.clone(), attr_command.clone())
        );

        Ok(())
    }

    ///
    ///
    ///
    async fn create_registers(&mut self, mut instance: Instance) -> Result<(), Error> {
        //
        // Get the logger
        self.logger = Some(instance.logger.clone());

        //
        // Register interface
        let mut interface = instance.create_class("registers").finish();

        //
        // Create 20 register
        let mut array = Vec::new();
        for n in 0..20 {
            let a = interface
                .create_attribute(format!("{}", n))
                .with_ro()
                .finish_as_number()
                .await?;
            a.set_from_i64(2).await.unwrap();
            array.push(a);
        }
        self.array = Arc::new(array);

        Ok(())
    }
}

#[async_trait]
impl DriverOperations for RegisterMapDevice {
    ///
    /// Mount the device
    ///
    async fn mount(&mut self, instance: Instance) -> Result<(), Error> {
        // return Err(Error::Wtf);

        //
        // First create registers because command will need them
        self.create_registers(instance.clone()).await?;
        //
        // Create command
        self.create_memory_command_attribute(instance.clone())
            .await?;
        Ok(())
    }

    ///
    /// Easiest way to implement the reboot event
    ///
    async fn wait_reboot_event(&mut self, _: Instance) {
        sleep(Duration::from_secs(5)).await;
    }
}
