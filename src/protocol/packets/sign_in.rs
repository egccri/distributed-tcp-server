use crate::protocol::packets::{Fire, Recv};
use crate::protocol::PacketError;
use crate::protocol::PacketError::ParsePacketError;

#[derive(Debug, Clone, PartialEq)]
pub struct SignInRecv {}

impl TryFrom<String> for SignInRecv {
    type Error = PacketError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(ParsePacketError { raw: value })
        } else {
            Ok(SignInRecv {})
        }
    }
}

impl Recv for SignInRecv {}

#[derive(Debug, Clone, PartialEq)]
pub struct SignInFire {
    code: String,
}

impl Into<String> for SignInFire {
    fn into(self) -> String {
        format!("{}", self.code)
    }
}

impl Fire for SignInFire {}
