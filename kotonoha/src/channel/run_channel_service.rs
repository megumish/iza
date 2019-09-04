use crate::channel::*;
use futures::prelude::*;
use std::pin::Pin;
use std::sync::Arc;

pub trait RunChannelService {
    fn run(
        &'static self,
    ) -> Pin<Box<dyn Future<Output = Result<(Arc<Listener>, Arc<Sender>)>> + Send>> {
    }
}

pub trait RunChannelServiceComponent {
    type Service: RunChannelService;

    fn run_channel_service(&self) -> &Self::Service;
}
