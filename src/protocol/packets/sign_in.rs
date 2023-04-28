use crate::protocol::packets::{Fire, Recv};
use crate::protocol::PacketError;
use crate::protocol::PacketError::ParsePacketError;

#[derive(Debug, Clone, PartialEq)]
pub struct SignInRecv {
    pub client_id: String,
    pub username: String,
    pub password: String,
}

impl TryFrom<String> for SignInRecv {
    type Error = PacketError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let attrs = value.split(',').collect::<Vec<&str>>();
        if attrs.len() != 4 {
            Err(ParsePacketError { raw: value })
        } else {
            Ok(SignInRecv {
                client_id: attrs[1].to_string(),
                username: attrs[2].to_string(),
                password: attrs[3].to_string(),
            })
        }
    }
}

impl Recv for SignInRecv {}

#[derive(Debug, Clone, PartialEq)]
pub struct SignInFire {
    pub code: String,
}

impl Into<String> for SignInFire {
    fn into(self) -> String {
        format!("{}", self.code)
    }
}

impl Fire for SignInFire {}
