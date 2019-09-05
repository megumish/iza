use futures::{future, prelude::*, sync::mpsc};
use kotonoha::channel::*;
use kotonoha::log::OneLanguageSimpleLog as Log;

fn main() {
    // let log_sender_future = {
    //     use kotonoha::channel::*;
    //     StdoutChannel::new()
    // }
    // .wait()
    // .unwrap();

    let (sender, mut receiver) = mpsc::channel(5);

    receiver
        .take_while(1)
        .for_each(|m| {
            println!("{}", m);
            future::ok(())
        })
        .map_err(|e| ())
        .join(sender.send("A").map_err(|e| ()))
        .wait();
    // let _ = log_sender.send(Log::new("Initialize")).wait();
}
