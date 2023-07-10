# iot-server-template
An iot server that bridge between the private tcp protocol device and egccri.

_[WIP] Distributed version use openraft._

#### Quick start

```shell
cargo build
./target/debug/iot-server-template -c ./config/cluster/cluster_1.toml
./target/debug/iot-server-template -c ./config/cluster/cluster_2.toml
./target/debug/iot-server-template -c ./config/cluster/cluster_3.toml
```

#### How tcp server worked.

![how it worked](asserts/imgs/img.png)

#### Cluster arch.

![cluster arch](asserts/imgs/cluster.png)

#### Raft storage

Raft storage is an optional embedded distribute storage for the cluster router component, and it based on openraft. 
 
The other way is redis.

##### Raft storage RPC calls

![RPC calls](asserts/imgs/rpcs.png)
