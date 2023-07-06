use crate::router::{RouterId, Value};
use crate::server::channel::ChannelId;
use crate::storage::raft::{Node, NodeId, TypeConfig};
use openraft::async_trait::async_trait;
use openraft::{
    Entry, EntryPayload, LogId, LogState, RaftLogId, RaftLogReader, RaftSnapshotBuilder,
    RaftStorage, RaftTypeConfig, Snapshot, SnapshotMeta, StorageError, StorageIOError,
    StoredMembership, Vote,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::io::Cursor;
use std::ops::RangeBounds;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use tracing::debug;

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

impl Response {
    pub fn new(value: Option<String>) -> Response {
        Response { value }
    }
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

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct StateMachine {
    pub last_applied_log: Option<LogId<NodeId>>,

    pub last_membership: StoredMembership<NodeId, Node>,

    pub data: BTreeMap<String, String>,
}

pub struct StoreSnapshot {
    pub meta: SnapshotMeta<NodeId, Node>,

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
            last_membership: Default::default(),
            data: Default::default(),
        }
    }

    pub fn last_applied_log(&self) -> Option<LogId<NodeId>> {
        self.last_applied_log.clone()
    }

    pub fn last_membership(&self) -> StoredMembership<NodeId, Node> {
        self.last_membership.clone()
    }
}

#[async_trait]
impl RaftLogReader<TypeConfig> for Arc<Store> {
    async fn get_log_state(&mut self) -> Result<LogState<TypeConfig>, StorageError<NodeId>> {
        let log = self.log.read().await;
        let last_serialized = log.iter().rev().next().map(|(_, ent)| ent);

        let last = match last_serialized {
            None => None,
            Some(serialized) => {
                let ent: Entry<TypeConfig> =
                    serde_json::from_str(serialized).map_err(|e| StorageIOError::read_logs(&e))?;
                Some(*ent.get_log_id())
            }
        };

        let last_purged = *self.last_purged_log_id.read().await;

        let last = match last {
            None => last_purged,
            Some(x) => Some(x),
        };

        Ok(LogState {
            last_purged_log_id: last_purged,
            last_log_id: last,
        })
    }

    async fn try_get_log_entries<RB: RangeBounds<u64> + Clone + Debug + Send + Sync>(
        &mut self,
        range: RB,
    ) -> Result<Vec<Entry<TypeConfig>>, StorageError<NodeId>> {
        let mut entries = vec![];
        {
            let log = self.log.read().await;
            for (_, serialized) in log.range(range.clone()) {
                let ent =
                    serde_json::from_str(serialized).map_err(|e| StorageIOError::read_logs(&e))?;
                entries.push(ent);
            }
        };

        Ok(entries)
    }
}

#[async_trait]
impl RaftSnapshotBuilder<TypeConfig> for Arc<Store> {
    async fn build_snapshot(&mut self) -> Result<Snapshot<TypeConfig>, StorageError<NodeId>> {
        let data;
        let last_applied_log;
        let last_membership;

        {
            // Serialize the data of the state machine.
            let sm = self.state_machine.read().await;
            data = serde_json::to_vec(&*sm).map_err(|e| StorageIOError::read_state_machine(&e))?;

            last_applied_log = sm.last_applied_log;
            last_membership = sm.last_membership.clone();
        }

        let snapshot_size = data.len();

        let snapshot_idx = {
            let mut l = self.snapshot_idx.lock().await;
            *l += 1;
            *l
        };

        let snapshot_id = if let Some(last) = last_applied_log {
            format!("{}-{}-{}", last.leader_id, last.index, snapshot_idx)
        } else {
            format!("--{}", snapshot_idx)
        };

        let meta = SnapshotMeta {
            last_log_id: last_applied_log,
            last_membership,
            snapshot_id,
        };

        let snapshot = StoreSnapshot {
            meta: meta.clone(),
            data: data.clone(),
        };

        {
            let mut current_snapshot = self.current_snapshot.write().await;
            *current_snapshot = Some(snapshot);
        }

        tracing::info!(snapshot_size, "log compaction complete");

        Ok(Snapshot {
            meta,
            snapshot: Box::new(Cursor::new(data)),
        })
    }
}

#[async_trait]
impl RaftStorage<TypeConfig> for Arc<Store> {
    type LogReader = Self;
    type SnapshotBuilder = Self;

    async fn save_vote(&mut self, vote: &Vote<NodeId>) -> Result<(), StorageError<NodeId>> {
        debug!(?vote, "save vote");
        let mut h = self.voted.write().await;
        *h = Some(*vote);
        Ok(())
    }

    async fn read_vote(&mut self) -> Result<Option<Vote<NodeId>>, StorageError<NodeId>> {
        let h = self.voted.read().await;
        Ok(*h)
    }

    async fn get_log_reader(&mut self) -> Self::LogReader {
        self.clone()
    }

    async fn append_to_log<I>(&mut self, entries: I) -> Result<(), StorageError<NodeId>>
    where
        I: IntoIterator<Item = Entry<TypeConfig>> + Send,
    {
        let mut log = self.log.write().await;
        for entry in entries {
            let s = serde_json::to_string(&entry)
                .map_err(|e| StorageIOError::write_log_entry(*entry.get_log_id(), &e))?;
            log.insert(entry.log_id.index, s);
        }
        Ok(())
    }

    async fn delete_conflict_logs_since(
        &mut self,
        log_id: LogId<NodeId>,
    ) -> Result<(), StorageError<NodeId>> {
        debug!("delete_log: [{:?}, +oo)", log_id);

        {
            let mut log = self.log.write().await;

            let keys = log
                .range(log_id.index..)
                .map(|(k, _v)| *k)
                .collect::<Vec<_>>();
            for key in keys {
                log.remove(&key);
            }
        }
        Ok(())
    }

    async fn purge_logs_upto(&mut self, log_id: LogId<NodeId>) -> Result<(), StorageError<NodeId>> {
        {
            let mut ld = self.last_purged_log_id.write().await;
            assert!(*ld <= Some(log_id));
            *ld = Some(log_id);
        }

        {
            let mut log = self.log.write().await;

            let keys = log
                .range(..=log_id.index)
                .map(|(k, _v)| *k)
                .collect::<Vec<_>>();
            for key in keys {
                log.remove(&key);
            }
        }
        Ok(())
    }

    async fn last_applied_state(
        &mut self,
    ) -> Result<(Option<LogId<NodeId>>, StoredMembership<NodeId, Node>), StorageError<NodeId>> {
        let state_machine = self.state_machine.read().await;
        let last_applied_log = state_machine.last_applied_log();
        let last_membership = state_machine.last_membership();
        Ok((last_applied_log, last_membership))
    }

    async fn apply_to_state_machine(
        &mut self,
        entries: &[<TypeConfig as RaftTypeConfig>::Entry],
    ) -> Result<Vec<Response>, StorageError<NodeId>> {
        let mut res = Vec::with_capacity(entries.len());
        let mut sm = self.state_machine.write().await;

        for entry in entries {
            debug!(%entry.log_id, "replicate to sm");
            sm.last_applied_log = Some(entry.log_id);

            match &entry.payload {
                EntryPayload::Blank => res.push(Response::new(None)),
                EntryPayload::Normal(ref data) => match data {
                    Request::Connect { value } => {
                        let json = serde_json::to_string(&value).map_err(|e| {
                            StorageIOError::write_log_entry(*entry.get_log_id(), &e)
                        })?;

                        sm.data.insert(value.channel_id().into(), json.clone());
                        res.push(Response::new(Some(json)));
                    }
                    _ => {}
                },
                EntryPayload::Membership(ref mem) => {
                    sm.last_membership = StoredMembership::new(Some(entry.log_id), mem.clone());
                    res.push(Response::new(None));
                }
            }
        }
        Ok(res)
    }

    async fn get_snapshot_builder(&mut self) -> Self::SnapshotBuilder {
        self.clone()
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
        tracing::info!(
            { snapshot_size = snapshot.get_ref().len() },
            "decoding snapshot for installation"
        );

        let new_snapshot = StoreSnapshot {
            meta: meta.clone(),
            data: snapshot.into_inner(),
        };

        {
            let t = &new_snapshot.data;
            let y = std::str::from_utf8(t).unwrap();
            debug!("SNAP META:{:?}", meta);
            debug!("JSON SNAP DATA:{}", y);
        }

        // Update the state machine.
        {
            let new_sm: StateMachine = serde_json::from_slice(&new_snapshot.data).map_err(|e| {
                StorageIOError::read_snapshot(Some(new_snapshot.meta.signature()), &e)
            })?;
            let mut sm = self.state_machine.write().await;
            *sm = new_sm;
        }

        // Update current snapshot.
        let mut current_snapshot = self.current_snapshot.write().await;
        *current_snapshot = Some(new_snapshot);
        Ok(())
    }

    async fn get_current_snapshot(
        &mut self,
    ) -> Result<Option<Snapshot<TypeConfig>>, StorageError<NodeId>> {
        match &*self.current_snapshot.read().await {
            Some(snapshot) => {
                let data = snapshot.data.clone();
                Ok(Some(Snapshot {
                    meta: snapshot.meta.clone(),
                    snapshot: Box::new(Cursor::new(data)),
                }))
            }
            None => Ok(None),
        }
    }
}
