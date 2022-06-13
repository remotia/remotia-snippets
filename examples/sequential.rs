use std::time::Duration;

use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};
use remotia::async_func;
use remotia::pipeline::ascode::component::Component;
use remotia::pipeline::ascode::AscodePipeline;
use remotia::processors::async_functional::AsyncFunction;
use remotia::processors::containers::sequential::Sequential;
use remotia::processors::frame_drop::threshold::ThresholdBasedFrameDropper;
use remotia::processors::functional::Function;
use remotia::processors::ticker::Ticker;
use remotia::time::add::TimestampAdder;
use remotia::time::diff::TimestampDiffCalculator;

#[tokio::main]
async fn main() {
    println!("Hello Sequential!");

    let generator = Sequential::new()
        .append(Ticker::new(1000))
        .append(TimestampAdder::new("generation_time"))
        .append(Function::new(|mut frame_data| {
            let mut rng = rand::thread_rng();
            let id = rng.gen::<u128>() % 10000;
            frame_data.set("id", id);

            println!(
                "[Generator] A new frame DTO has been generated: #{}",
                frame_data.get("id")
            );
            Some(frame_data)
        }));

    let random_delayer = Sequential::new().append(AsyncFunction::new(|frame_data| {
        async_func!(async move {
            let mut rng: StdRng = SeedableRng::from_entropy();
            let sleep_duration = rng.gen::<u64>() % 1500;
            println!(
                "[Random delayer] Sleeping for {:?} milliseconds",
                sleep_duration
            );
            tokio::time::sleep(Duration::from_millis(sleep_duration)).await;

            Some(frame_data)
        })
    }));

    let delay_check = Sequential::new()
        .append(TimestampDiffCalculator::new("generation_time", "delay"))
        .append(ThresholdBasedFrameDropper::new("delay", 800))
        .append(Function::new(|frame_data| {
            println!(
                "[Delay Checker] Frame #{} check result: {:?} (delay = {})",
                frame_data.get("id"),
                frame_data.get_drop_reason(),
                frame_data.get("delay"),
            );
            Some(frame_data)
        }));

    let handles = AscodePipeline::new()
        .link(
            Component::new()
                .append(generator)
                .append(random_delayer)
                .append(delay_check),
        )
        .bind()
        .run();

    for handle in handles {
        handle.await.unwrap();
    }
}
