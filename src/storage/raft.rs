use crate::router::RouterStorage;
use openraft::{BasicNode, Entry};
use std::io::Cursor;

use storage::Request;
use storage::Response;

mod client;
mod network;
mod server;
mod storage;

/// The unique id of the raft node.
pub type NodeId = u64;
/// Node is custom node data that can used by raft core.
pub type Node = BasicNode;

openraft::declare_raft_types!(
    /// Declare the type configuration for example K/V store.
    pub TypeConfig: D = Request, R = Response, NodeId = NodeId, Node = Node,
    Entry = Entry<TypeConfig>, SnapshotData = Cursor<Vec<u8>>
);

// Collect AppData and AppDateResponse here use protoc.
pub enum Messaging {}

pub struct RaftStore {}

impl RaftStore {
    // When main server start and before accept tcp connections, start the RaftStore.
    // Include start a raft server, check snapshot data to the router etc.
    pub fn start(addr: &str) {
        todo!()
    }
}

// Impl router operations here.
impl RouterStorage for RaftStore {}
