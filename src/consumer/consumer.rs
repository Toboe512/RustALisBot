use crate::events::processor::Processor;
use crate::events::types::Event;

use std::thread;
use std::time::Duration;
use log::{debug, error, info};

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
            let evn = self.processor.fetch(self.batch_size).await;
            match evn {
                Ok(..) => {
                    //TODO добавить дебаг
                    if evn.clone()?.is_empty() {
                        // Приостановить поток на UPDATE_PERIOD сек (период обновления)
                        thread::sleep(UPDATE_PERIOD);
                        continue;
                    }
                    self.handle_events(evn.clone().unwrap()).await?
                }
                Err(e) => {
                    error!("Consumer: {}", e);
                }
            }
        }
    }

    async fn handle_events(&mut self, events: Vec<Event>) -> Result<(), String> {
        for event in events {
            debug!("Handle event: {:?}", event);
            match self.processor.process(event).await {
                Err(e) => {
                    error!("Can't handle event: {}", e);
                    continue;
                }
                Ok(..) => {}
            }
        }
        Ok(())
    }
}

