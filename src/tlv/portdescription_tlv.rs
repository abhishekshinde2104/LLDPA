use std::fmt::Display;

use crate::tlv::TlvType;

/// Port Description TLV
///
/// The Port Description TLV allows network management to advertise the device's port description.
///
/// It is an optional TLV and as such may be included in an LLDPDU zero or more times between
/// the TTL TLV and the End of LLDPDU TLV.
///
/// # TLV Format:
///
///      0                   1                   2
///      0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3
///     +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-...-+-+-+-+
///     |             |                 |                           |
///     |      4      |      Length     |     Port Description      |
///     |             |                 |                           |
///     +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-...-+-+-+-+
///
///                                             0 - 255 byte
#[derive(Debug, Clone)]
pub struct PortDescriptionTLV {
    /// The type of the TLV
    pub tlv_type: TlvType,
    /// The port description
    pub value: String,
}

impl Display for PortDescriptionTLV {
    /// Write a printable representation of the TLV object.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Implement
        write!(f, "PortDescriptionTLV(\"{}\")",self.value)
    }
}

impl PortDescriptionTLV {
    /// Constructor
    pub fn new(value: String) -> PortDescriptionTLV {
        // TODO: Implement
        let value_bytes: Vec<u8> = value.as_bytes().to_vec();
        PortDescriptionTLV {
            tlv_type: TlvType::PortDescription,
            value: value,
        }
    }

    /// Create a TLV instance from raw bytes.
    ///
    /// Panics if the provided TLV contains errors (e.g. has the wrong type).
    pub fn new_from_bytes(bytes: &[u8]) -> PortDescriptionTLV {
        // TODO: Implement
        let mut type_value = bytes[0];
        type_value = bytes[0] & 0b11111110;

        let last_bit = bytes[0] & 0b00000001;

        type_value = type_value >> 1;

        let mut length_value = bytes[1] as u16;

        if last_bit != 0{
            length_value= length_value + 256;
        }

        let port_desc =  String::from_utf8(bytes[2..].to_vec()).unwrap();
        
        if type_value!=(TlvType::PortDescription as u8) || length_value==0{
            panic!(" SystemName error! ")
        }

        PortDescriptionTLV::new(port_desc)
    }

    /// Return the length of the TLV value
    pub fn len(&self) -> usize {
        // TODO: Implement
        self.value.len()
    }

    /// Return the byte representation of the TLV.
    pub fn bytes(&self) -> Vec<u8> {
        // TODO: Implement
        let mut type_rep = self.tlv_type as u8;

        type_rep = type_rep << 1;

        let bit_9_set = self.len() & 0b100000000;

        if bit_9_set  == 1{
            type_rep = type_rep | 0b000000001;
        }

        let len_rep = (self.len() & 0xFF) as u8;
        
        let mut value_rep = self.value.as_bytes().to_vec();

        let mut port_desc_rep = vec![type_rep,len_rep];
        port_desc_rep.append(&mut value_rep);

        port_desc_rep
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set_up() -> (PortDescriptionTLV, String) {
        let string = String::from("Unittest");
        (PortDescriptionTLV::new(string.clone()), string)
    }

    #[test]
    fn test_type() {
        let (tlv, _) = set_up();
        assert_eq!(tlv.tlv_type as u8, TlvType::PortDescription as u8);
        assert_eq!(tlv.tlv_type as u8, 4);
    }

    #[test]
    fn test_length() {
        let (tlv, _) = set_up();
        assert_eq!(tlv.len(), 8);
    }

    #[test]
    fn test_value() {
        let (tlv, s) = set_up();
        assert_eq!(tlv.value, s);
    }

    #[test]
    fn test_dump() {
        let (tlv, _) = set_up();
        assert_eq!(tlv.bytes(), b"\x08\x08Unittest".to_vec());
    }

    #[test]
    fn test_load() {
        let tlv = PortDescriptionTLV::new_from_bytes(b"\x08\x0FAnotherUnittest".as_ref());
        assert_eq!(tlv.len(), 15);
        assert_eq!(tlv.value, String::from("AnotherUnittest"));
    }

    #[test]
    fn test_display() {
        let (tlv, _) = set_up();
        assert_eq!(format!("{}", tlv), "PortDescriptionTLV(\"Unittest\")");
    }
}
