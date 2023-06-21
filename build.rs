fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        // .out_dir("./src/server")
        .compile(&["./proto/raft.proto"], &["./proto"])?;
    Ok(())
}
