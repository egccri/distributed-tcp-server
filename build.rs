fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .out_dir("./src/storage/raft")
        .compile(
            &["./proto/raft.proto", "./proto/raft_client.proto"],
            &["./proto"],
        )?;
    Ok(())
}
