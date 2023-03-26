use crate::tlv::TlvType;
use bytes::{Buf, BufMut};
use std::fmt::Display;

/// Time To Live TLV
///
/// The Time To Live TLV indicates the number of seconds that the recipient LLDP agent is to regard the information
/// associated with the transmitting LLDP agent as valid.
///
/// The Time To Live TLV is mandatory and MUST be the third TLV in the LLDPDU.
/// Each LLDPDU MUST contain one, and only one, TTL TLV.
///
/// # TLV Format:
///
///      0                   1                   2                   3
///      0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
///     +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
///     |             |                 |                               |
///     |      3      |      Length     |               TTL             |
///     |             |                 |                               |
///     +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
#[derive(Debug, Clone)]
pub struct TtlTLV {
    /// The type of the TLV
    pub tlv_type: TlvType,
    /// The TTL in seconds
    pub value: u16,
}

impl Display for TtlTLV {
    /// Write a printable representation of the TLV object.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Implement
        write!(f, "TtlTLV({})",self.value)
    }
}

impl TtlTLV {
    /// Constructor
    pub fn new(ttl: u16) -> TtlTLV {
        // TODO: Implement
        TtlTLV {
            tlv_type: TlvType::Ttl,
            value: ttl,
        }
    }

    /// Create a TLV instance from raw bytes.
    ///
    /// Panics if the provided TLV contains errors (e.g. has the wrong type).
    pub fn new_from_bytes(bytes: &[u8]) -> TtlTLV {
        // TODO: Implement
        let mut type_value = bytes[0];
        type_value = bytes[0] & 0b11111110;

        let last_bit = bytes[0] & 0b00000001;

        type_value = type_value >> 1;

        let mut length_value = bytes[1] as u16;

        if last_bit != 0{
            length_value= length_value + 256;
        }

        let mut v = 0 as u16;

        v = (( (bytes[2] as u16) << 8) as u16) | v;
        v = (bytes[3] as u16) | v;

        TtlTLV::new(v)
    }

    /// Return the length of the TLV value
    pub fn len(&self) -> usize {
        // TODO: Implement
        2
    }

    /// Return the byte representation of the TLV.
    pub fn bytes(&self) -> Vec<u8> {
        // TODO: Implement
        let mut type_rep = self.tlv_type as u8;
        
        type_rep = type_rep << 1;

        let last_bit_set = self.len() & 0b100000000;

        if last_bit_set !=0 {
            type_rep = type_rep | 0b000000001;
        }

        let len_rep = (self.len() & 0xFF) as u8;

        let byte1 = (self.value & 0xFF) as u8;
        let byte2 = ((self.value & 0xFF00) >> 8) as u8;

        vec![type_rep,len_rep,byte1,byte2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set_up() -> (TtlTLV, u16) {
        let r = 36575;
        (TtlTLV::new(r), r)
    }

    #[test]
    fn test_type() {
        let (ttltlv, _) = set_up();
        assert_eq!(ttltlv.tlv_type as u8, 3);
    }

    #[test]
    fn test_length() {
        let (ttltlv, _) = set_up();
        assert_eq!(ttltlv.len(), 2);
    }

    #[test]
    fn test_value() {
        let (ttltlv, r) = set_up();
        assert_eq!(ttltlv.value, r);
    }

    #[test]
    fn test_dump() {
        let (ttltlv, r) = set_up();
        let mut b = vec![6, 2];
        b.put_u16(r);
        assert_eq!(ttltlv.bytes(), b);
    }

    #[test]
    fn test_load() {
        let ttltlv = TtlTLV::new_from_bytes(b"\x06\x02\x00\x78".as_ref());
        assert_eq!(ttltlv.value, 120);
    }

    #[test]
    #[should_panic]
    fn test_load_invalid_length() {
        TtlTLV::new_from_bytes(b"\x06\x03\x00\x78\x00".as_ref());
    }

    #[test]
    #[should_panic]
    fn test_load_incorrect_length() {
        TtlTLV::new_from_bytes(b"\x06\x01\x00\x78".as_ref());
    }

    #[test]
    fn test_display() {
        let (tlv, _) = set_up();
        assert_eq!(format!("{}", tlv), "TtlTLV(36575)");
    }
}
