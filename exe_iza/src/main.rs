use futures::{executor, prelude::*};
use kotonoha::channel::*;
use kotonoha::log::OneLanguage1337Log as Log;

fn main() {
    let channel = StdoutChannel::new();
    let future = {
        let sender = channel.get_sender();
        sender
            .send(Log::success("Initialize"))
            .join(channel.run().map_err(Into::into))
    };

    let _ = executor::spawn(future).wait_future();
}
