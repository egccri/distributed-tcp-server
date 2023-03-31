pub trait Encoder: tokio_util::codec::Encoder<String> {
    fn encode();
}
