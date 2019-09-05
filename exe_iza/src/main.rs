use futures::{executor, future, prelude::*, sync::mpsc};
use kotonoha::channel::*;
use kotonoha::log::OneLanguageSimpleLog as Log;

fn main() {
    let channel = StdoutChannel::new();
    let future = {
        let sender = channel.get_sender();
        sender
            .send(Log::new("Initialize"))
            .join(channel.run().map_err(Into::into))
    };

    let _ = executor::spawn(future).wait_future();

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
