use crate::events::processor::Processor;
use crate::events::types::Event;

use std::thread;
use std::time::Duration;

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
        loop {
            let evn = self.processor.fetch(self.batch_size).await;
            match evn {
                Ok(..) => {
                    //TODO добавить дебаг
                    if evn.clone()?.is_empty() {
                        thread::sleep(Duration::from_secs(1)); // Приостановить на 1 сек
                        continue;
                    }
                    self.handle_events(evn.clone().unwrap()).await?
                }
                Err(e) => {
                    println!("[ERR] consumer: {}", e)
                }
            }
        }
    }

    async fn handle_events(&mut self, events: Vec<Event>) -> Result<(), String> {
        for event in events {
            //println!("got new event: {}", event.text);//TODO добавить дебаг

            match self.processor.process(event).await {
                Err(e) => {
                    println!("can't handle event: {}", e);
                    continue;
                }
                Ok(..) => {}
            }
        }
        Ok(())
    }
}

