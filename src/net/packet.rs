use super::Serialise;

pub(super) struct Packet {}

impl Packet {}

impl Serialise for Packet {
    fn deserialise(buffer: &[u8]) -> Self {
        todo!()
    }

    fn serialise(&self, buffer: &mut [u8]) {
        todo!()
    }
}
