use remotia::pipeline::Pipeline;
use remotia::pipeline::component::Component;
use remotia::processors::functional::Function;
use remotia::processors::ticker::Ticker;
use remotia::time::add::TimestampAdder;

#[tokio::main]
async fn main() {
    println!("Hello functional processors!");

    let handles = Pipeline::new()
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
