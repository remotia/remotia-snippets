use remotia::pipeline::ascode::AscodePipeline;
use remotia::pipeline::ascode::component::Component;
use remotia::processors::containers::sequential::Sequential;
use remotia::processors::debug::random_dropper::RandomFrameDropper;
use remotia::processors::frame_drop::threshold::ThresholdBasedFrameDropper;
use remotia::processors::functional::Function;
use remotia::processors::ticker::Ticker;
use remotia::time::add::TimestampAdder;

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
        .append(ThresholdBasedFrameDropper::new("generation_time", 800))
        .append(Function::new(|frame_data| {
            println!("[Delay Checker] This frame DTO has passed the check: {:?}", frame_data);
            Some(frame_data)
        }));

    let dropper = Sequential::new()
        .append(RandomFrameDropper::new(0.5))
        .append(Function::new(|frame_data| {
            println!("[Dropper] This frame DTO has not been dropped: {:?}", frame_data);
            Some(frame_data)
        }));

    let handles = AscodePipeline::new()
        .link(Component::new()
            .append(generator)
            .append(dropper)
            .append(delay_check)
        )
        .bind()
        .run();

    for handle in handles {
        handle.await.unwrap();
    }
}

