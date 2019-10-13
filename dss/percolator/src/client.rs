use crate::service::{TSOClient, TransactionClient};
use futures::future;

use labrpc::*;

use crate::msg::*;

// BACKOFF_TIME_MS is the wait time before retrying to send the request.
// It should be exponential growth. e.g.
//|  retry time  |  backoff time  |
//|--------------|----------------|
//|      1       |       100      |
//|      2       |       200      |
//|      3       |       400      |
const BACKOFF_TIME_MS: u64 = 100;
// RETRY_TIMES is the maximum number of times a client attempts to send a request.
const RETRY_TIMES: usize = 3;

/// Client mainly has two purposes:
/// One is getting a monotonically increasing timestamp from TSO (Timestamp Oracle).
/// The other is do the transaction logic.
#[derive(Clone)]
pub struct Client { 
    tsoClient: TSOClient,
    txClient: TransactionClient
}

impl Client {
    pub fn new(tso_client: TSOClient, txn_client: TransactionClient) -> Client {
        Client {
            tsoClient: tso_client,
            txClient: txn_client
        }
    }

    pub fn get_timestamp(&self) -> Result<u64> {
        let ts_request = TimestampRequest {};

        let reply = self.tsoClient.get_timestamp(&ts_request).wait().unwrap();

        return Ok(reply.timestamp);
    }

    /// Begins a new transaction.
    pub fn begin(&mut self) {
        // Your code here.
        unimplemented!()
    }

    /// Gets the value for a given key.
    pub fn get(&self, key: Vec<u8>) -> Result<Vec<u8>> {
        // Your code here.
        unimplemented!()
    }

    /// Sets keys in a buffer until commit time.
    pub fn set(&mut self, key: Vec<u8>, value: Vec<u8>) {
        // Your code here.
        unimplemented!()
    }

    /// Commits a transaction.
    pub fn commit(&self) -> Result<bool> {
        // Your code here.
        unimplemented!()
    }
}
