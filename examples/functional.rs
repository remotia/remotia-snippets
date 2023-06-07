use std::collections::HashMap;

use remotia::pipeline::Pipeline;
use remotia::pipeline::component::Component;
use remotia::processors::functional::Function;
use remotia::processors::ticker::Ticker;
use remotia::profilation::time::add::TimestampAdder;
use remotia::traits::FrameProperties;

#[derive(Debug, Default)]
struct FrameData {
    properties: HashMap<String, u128>
}

impl FrameProperties<u128> for FrameData {
    fn set(&mut self, key: &str, value: u128) {
        self.properties.insert(key.to_string(), value);
    }

    fn get(&self, key: &str) -> Option<u128> {
        self.properties.get(key).copied()
    }
}

#[tokio::main]
async fn main() {
    println!("Hello functional processors!");

    let handles = Pipeline::<FrameData>::new()
        .link(
            Component::new()
                .append(Ticker::new(1000))
                .append(Function::new(|frame_data| {
                    println!(
                        "This is a functional processor. Received frame data: {:?}",
                        frame_data
                    );
                    Some(frame_data)
                }))
                .append(TimestampAdder::new("timestamp")),
        )
        .link(
            Component::new()
                .append(Ticker::new(1500))
                .append(Function::new(|frame_data| {
                    println!(
                        "This is another functional processor. Received frame data: {:?}",
                        frame_data
                    );
                    Some(frame_data)
                })),
        )
        .run();

    for handle in handles {
        handle.await.unwrap();
    }
}
