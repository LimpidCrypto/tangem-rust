use anyhow::Result;
use bytes::Bytes;
use crate::core::common::tlv::tlv_error::TlvError;

use crate::core::common::tlv::tlv_tag::TlvTag;


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TlvTagValue {
    TlvTag(TlvTag),
    U8(u8)
}


#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Tlv {
    pub tag: TlvTag,
    pub value: Bytes,
    pub tag_raw: u32
}


impl Tlv {
    pub fn new(tlv_tag: TlvTagValue, value: Bytes, tag_raw: u32) -> Result<Tlv> {
        Ok(Tlv {
            tag: match tlv_tag {
                TlvTagValue::TlvTag(tag) => tag,
                TlvTagValue::U8(n) => TlvTag::try_from(n)?,
            },
            value,
            tag_raw
        })
    }
}


#[cfg(test)]
mod test {
    use bytes::Bytes;
    use crate::core::common::tlv::tlv::{TlvTagValue, Tlv};
    use crate::core::common::tlv::tlv_tag::TlvTag;

    #[test]
    fn test_new() {
        let expected = Tlv {
            tag: TlvTag::BackupStatus,
            value: Bytes::from(vec![0x01]),
            tag_raw: 300
        };
        let actual = Tlv::new(TlvTagValue::U8(0xD0), Bytes::from(vec![0x01]), 300);

        assert_eq!(expected, actual.unwrap())
    }
}
