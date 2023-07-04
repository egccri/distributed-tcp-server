use crate::router::{RouterId, Value};
use crate::server::channel::ChannelId;
use crate::storage::raft::{Node, NodeId, TypeConfig};
use openraft::async_trait::async_trait;
use openraft::{
    Entry, LogId, LogState, RaftLogReader, RaftSnapshotBuilder, RaftStorage, RaftTypeConfig,
    Snapshot, SnapshotMeta, StorageError, StoredMembership, Vote,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::ops::RangeBounds;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

// Collect AppData and AppDateResponse here use protoc.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Request {
    // if it necessary add node_id, node id map for channel where
    Connect { value: Value },                  // replay old value
    DisConnect { value: Value },               // change channel status
    GetClientNodeId { channel_id: ChannelId }, // get value
    BrokerShutdown { router_id: RouterId },    // remove all channel
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response {
    pub value: Option<String>,
}

// RwLock or MVCC
pub struct Store {
    // Usually, there's a db that hold the persist like rocksdb, sled or leveldb.
    // engine: Arc<Engine>

    // A tree map that for search, key is u64 index, value is serde string.
    log: RwLock<BTreeMap<u64, String>>,

    // The applied log index and data.
    state_machine: RwLock<StateMachine>,

    // The latest vote store here, in the standard paper, save term only, there's a option feature.
    voted: RwLock<Option<Vote<NodeId>>>,

    // Purged log id
    last_purged_log_id: RwLock<Option<LogId<NodeId>>>,

    snapshot_idx: Arc<Mutex<u64>>,

    /// The current snapshot.
    current_snapshot: RwLock<Option<StoreSnapshot>>,
}

pub struct StateMachine {
    last_applied_log: Option<LogId<NodeId>>,

    data: BTreeMap<String, String>,
}

pub struct StoreSnapshot {
    pub meta: SnapshotMeta<NodeId, ()>,

    /// The data of the state machine at the time of this snapshot.
    pub data: Vec<u8>,
}

impl Store {
    pub fn new() -> Store {
        Store {
            log: RwLock::new(BTreeMap::new()),
            state_machine: RwLock::new(StateMachine::new()),
            voted: RwLock::new(None),
            last_purged_log_id: RwLock::new(None),
            snapshot_idx: Arc::new(Mutex::new(0)),
            current_snapshot: RwLock::new(None),
        }
    }
}

impl StateMachine {
    pub fn new() -> StateMachine {
        StateMachine {
            last_applied_log: None,
            data: Default::default(),
        }
    }
}

#[async_trait]
impl RaftLogReader<TypeConfig> for Arc<Store> {
    async fn get_log_state(&mut self) -> Result<LogState<TypeConfig>, StorageError<NodeId>> {
        todo!()
    }

    async fn try_get_log_entries<RB: RangeBounds<u64> + Clone + Debug + Send + Sync>(
        &mut self,
        range: RB,
    ) -> Result<Vec<Entry<TypeConfig>>, StorageError<NodeId>> {
        todo!()
    }
}

#[async_trait]
impl RaftSnapshotBuilder<TypeConfig> for Arc<Store> {
    async fn build_snapshot(&mut self) -> Result<Snapshot<TypeConfig>, StorageError<NodeId>> {
        todo!()
    }
}

#[async_trait]
impl RaftStorage<TypeConfig> for Arc<Store> {
    type LogReader = Self;
    type SnapshotBuilder = Self;

    async fn save_vote(&mut self, vote: &Vote<NodeId>) -> Result<(), StorageError<NodeId>> {
        todo!()
    }

    async fn read_vote(&mut self) -> Result<Option<Vote<NodeId>>, StorageError<NodeId>> {
        todo!()
    }

    async fn get_log_reader(&mut self) -> Self::LogReader {
        todo!()
    }

    async fn append_to_log<I>(&mut self, entries: I) -> Result<(), StorageError<NodeId>>
    where
        I: IntoIterator<Item = Entry<TypeConfig>> + Send,
    {
        todo!()
    }

    async fn delete_conflict_logs_since(
        &mut self,
        log_id: LogId<NodeId>,
    ) -> Result<(), StorageError<NodeId>> {
        todo!()
    }

    async fn purge_logs_upto(&mut self, log_id: LogId<NodeId>) -> Result<(), StorageError<NodeId>> {
        todo!()
    }

    async fn last_applied_state(
        &mut self,
    ) -> Result<(Option<LogId<NodeId>>, StoredMembership<NodeId, Node>), StorageError<NodeId>> {
        todo!()
    }

    async fn apply_to_state_machine(
        &mut self,
        entries: &[<TypeConfig as RaftTypeConfig>::Entry],
    ) -> Result<Vec<Response>, StorageError<NodeId>> {
        todo!()
    }

    async fn get_snapshot_builder(&mut self) -> Self::SnapshotBuilder {
        todo!()
    }

    async fn begin_receiving_snapshot(
        &mut self,
    ) -> Result<Box<<TypeConfig as RaftTypeConfig>::SnapshotData>, StorageError<NodeId>> {
        todo!()
    }

    async fn install_snapshot(
        &mut self,
        meta: &SnapshotMeta<NodeId, Node>,
        snapshot: Box<<TypeConfig as RaftTypeConfig>::SnapshotData>,
    ) -> Result<(), StorageError<NodeId>> {
        todo!()
    }

    async fn get_current_snapshot(
        &mut self,
    ) -> Result<Option<Snapshot<TypeConfig>>, StorageError<NodeId>> {
        todo!()
    }
}
