use crate::events::processor::Processor;
use crate::events::types::Event;

use std::thread;
use std::time::Duration;
use log::{debug, error, info};
use crate::errors::errors::log_err;

const UPDATE_PERIOD: Duration = Duration::from_secs(1);

pub struct Consumer {
    processor: Processor,
    batch_size: i32,
}

impl Consumer {
    pub fn of(processor: Processor, batch_size: i32) -> Consumer {
        Consumer {
            processor,
            batch_size,
        }
    }

    pub async fn start(&mut self) -> Result<(), String> {
        info!("Application started");
        loop {

            match self.processor.fetch(self.batch_size).await {
                Ok(ev) => {
                    //TODO добавить дебаг?
                    if ev.is_empty() {
                        // Приостановить поток на UPDATE_PERIOD сек (период обновления)
                        thread::sleep(UPDATE_PERIOD);
                        continue;
                    }
                    self.handle_events(ev.clone()).await?
                }
                Err(e) => {
                    error!("Consumer: {}", e);
                }
            }
        }
    }

    async fn handle_events(&self, events: Vec<Event>) -> Result<(), String> {
        let log = String::from("Can't handle event");

        for event in events {
            debug!("Handle event: {:?}", event);
            self.processor.process(event).await.map_err(|e|{
                log_err(&log, e)
            })?;
        }
        Ok(())
    }
}

