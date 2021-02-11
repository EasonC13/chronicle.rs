use super::*;
use async_trait::async_trait;

#[async_trait]
pub trait EventLoop<H>: Sized {
    async fn event_loop(&mut self, supervisor: &mut Option<H>) -> NeedResult;
}
