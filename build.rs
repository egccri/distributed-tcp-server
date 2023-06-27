fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .out_dir("./src/storage/raft")
        .compile(
            &["./proto/raft/raft.proto", "./proto/raft/raft_client.proto"],
            &["./proto/raft"],
        )?;
    tonic_build::configure()
        .out_dir("./src/router")
        .compile(&["./proto/router/router.proto"], &["./proto/router"])?;
    Ok(())
}
