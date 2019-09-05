use futures::{executor, future, prelude::*, sync::mpsc};
use kotonoha::channel::*;
use kotonoha::log::OneLanguageSimpleLog as Log;

fn main() {
    let channel = StdoutChannel::new();
    let future = {
        let channel = channel.clone();
        channel
            .send(Log::new("Initialize"))
            .and_then(move |_| channel.finish())
    };

    let _ = executor::spawn(future).wait_future();
    let _ = executor::spawn(channel.run()).wait_future();

    // let (sender, mut receiver) = mpsc::channel(5);

    // receiver
    //     .take_while(1)
    //     .for_each(|m| {
    //         println!("{}", m);
    //         future::ok(())
    //     })
    //     .map_err(|e| ())
    //     .join(sender.send("A").map_err(|e| ()))
    //     .wait();
}
