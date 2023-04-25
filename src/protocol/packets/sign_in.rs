use crate::protocol::packets::{Fire, Recv};
use crate::protocol::PacketError;
use crate::protocol::PacketError::ParsePacketError;

pub struct SignInRecv {}

impl TryFrom<String> for SignInRecv {
    type Error = PacketError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(ParsePacketError { packet: value })
        } else {
            Ok(SignInRecv {})
        }
    }
}

impl Recv for SignInRecv {}

pub struct SignInFire {
    code: String,
}

impl Into<String> for SignInFire {
    fn into(self) -> String {
        format!("{}", self.code)
    }
}

impl Fire for SignInFire {}
