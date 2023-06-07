use std::collections::HashMap;

use remotia::pipeline::component::Component;
use remotia::pipeline::Pipeline;
use remotia::processors::containers::sequential::Sequential;
use remotia::processors::functional::Function;
use remotia::processors::ticker::Ticker;
use remotia::profilation::frame_drop::threshold::ThresholdBasedFrameDropper;
use remotia::profilation::time::add::TimestampAdder;
use remotia::traits::{FrameError, FrameProperties};

#[derive(Debug, Copy, Clone)]
enum Error {
    TooHighDelay,
}

#[derive(Debug)]
struct FrameData {
    properties: HashMap<String, u128>,
    error: Option<Error>,
}

impl Default for FrameData {
    fn default() -> Self {
        Self {
            properties: Default::default(),
            error: None
        }
    }
}

impl FrameProperties<u128> for FrameData {
    fn set(&mut self, key: &str, value: u128) {
        self.properties.insert(key.to_string(), value);
    }

    fn get(&mut self, key: &str) -> u128 {
        *self.properties.get(key).unwrap()
    }
}

impl FrameError<Error> for FrameData {
    fn report_error(&mut self, error: Error) {
        self.error = Some(error);
    }

    fn has_error(&mut self) -> bool {
        self.error.is_some()
    }

    fn get_error(&mut self) -> Error {
        self.error.unwrap()
    }
}

#[tokio::main]
async fn main() {
    println!("Hello Sequential!");

    let generator = Sequential::new()
        .append(Ticker::new(1000))
        .append(TimestampAdder::new("generation_time"))
        .append(Function::new(|frame_data| {
            println!("[Generator] A new frame DTO has been generated");
            Some(frame_data)
        }));

    let delay_check = Sequential::new()
        .append(ThresholdBasedFrameDropper::new("generation_time", 800, Error::TooHighDelay))
        .append(Function::new(|frame_data| {
            println!(
                "[Delay Checker] This frame DTO has passed the check: {:?}",
                frame_data
            );
            Some(frame_data)
        }));

    let dropper = Sequential::new()
        .append(Function::new(|frame_data| {
            use rand::Rng;
            let mut rng = rand::thread_rng();

            if rng.gen::<f32>() <= 0.5 {
                None
            } else {
                Some(frame_data)
            }
        }))
        .append(Function::new(|frame_data| {
            println!(
                "[Dropper] This frame DTO has not been dropped: {:?}",
                frame_data
            );
            Some(frame_data)
        }));

    let handles = Pipeline::<FrameData>::new()
        .link(
            Component::new()
                .append(generator)
                .append(dropper)
                .append(delay_check),
        )
        .run();

    for handle in handles {
        handle.await.unwrap();
    }
}
