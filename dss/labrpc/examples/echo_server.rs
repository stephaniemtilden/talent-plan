#[macro_use]
extern crate prost_derive;

use futures::Future;
use labrpc::*;

/// A Hand-written protobuf messages
#[derive(Clone, PartialEq, Message)]
pub struct Echo {
    #[prost(int64, tag = "1")]
    pub x: i64,
}

service! {
    service echo {
        rpc ping(Echo) returns (Echo);
    }
}
use echo::{add_service, Client, Service};

#[derive(Clone)]
struct EchoService;

impl Service for EchoService {
    fn ping(&self, input: Echo) -> RpcFuture<Echo> {
        Box::new(futures::future::result(Ok(input.clone())))
    }
}