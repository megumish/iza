use crate::channel::*;
use crate::log::*;
use futures::prelude::*;
use std::pin::Pin;
use std::sync::Arc;

use crate::channel::{Error, Result};

pub trait ChannelApp: RunChannelServiceComponent {
    fn channel(
        &'static self,
    ) -> Pin<Box<dyn Future<Output = Result<(Arc<Listener>, Arc<Sender>)>> + Send>> {
        self.run_channel_service().run().boxed()
    }
}
