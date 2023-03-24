use std::fmt::Display;

use crate::tlv::TlvType;

/// System Description TLV
///
/// The System Description TLV allows network management to advertise the system's description.
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
///     |      6      |      Length     |     System Description    |
///     |             |                 |                           |
///     +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-...-+-+-+-+
///
///                                             0 - 255 byte
#[derive(Debug, Clone)]
pub struct SystemDescriptionTLV {
    /// The type of the TLV
    pub tlv_type: TlvType,
    /// The system description
    pub value: String,
}

impl Display for SystemDescriptionTLV {
    /// Write a printable representation of the TLV object.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Implement
        write!(f, "SystemDescriptionTLV(\"{}\")",self.value)
    }
}

impl SystemDescriptionTLV {
    /// Constructor
    pub fn new(description: String) -> SystemDescriptionTLV {
        // TODO: Implement
        SystemDescriptionTLV {
            tlv_type: TlvType::SystemDescription,
            value: description,
        }
    }

    /// Create a TLV instance from raw bytes.
    ///
    /// Panics if the provided TLV contains errors (e.g. has the wrong type).
    pub fn new_from_bytes(bytes: &[u8]) -> SystemDescriptionTLV {
        // TODO: Implement
        let mut type_value = bytes[0];
        type_value = bytes[0] & 0b11111110;

        let last_bit = bytes[0] & 0b00000001;

        type_value = type_value >> 1;

        let mut length_value = bytes[1] as u16;

        if last_bit != 0{
            length_value= length_value + 256;
        }

        let sys_desc =  String::from_utf8(bytes[2..].to_vec()).unwrap();
        
        if type_value!=(TlvType::SystemDescription as u8) || length_value==0{
            panic!(" SystemName error! ")
        }

        SystemDescriptionTLV::new(sys_desc)
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

        let mut system_desc_rep = vec![type_rep,len_rep];
        system_desc_rep.append(&mut value_rep);

        system_desc_rep
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set_up() -> (SystemDescriptionTLV, String) {
        let string = String::from("Unittest");
        (SystemDescriptionTLV::new(string.clone()), string)
    }

    #[test]
    fn test_portdescription_type() {
        let (tlv, _) = set_up();
        assert_eq!(tlv.tlv_type as u8, TlvType::SystemDescription as u8);
        assert_eq!(tlv.tlv_type as u8, 6);
    }

    #[test]
    fn test_portdescription_length() {
        let (tlv, _) = set_up();
        assert_eq!(tlv.len(), 8);
    }

    #[test]
    fn test_portdescription_value() {
        let (tlv, s) = set_up();
        assert_eq!(tlv.value, s);
    }

    #[test]
    fn test_portdescription_dump() {
        let (tlv, _) = set_up();
        assert_eq!(tlv.bytes(), b"\x0C\x08Unittest".to_vec());
    }

    #[test]
    fn test_portdescription_load() {
        let tlv = SystemDescriptionTLV::new_from_bytes(b"\x0C\x12YetAnotherUnittest".as_ref());
        assert_eq!(tlv.len(), 18);
        assert_eq!(tlv.value, String::from("YetAnotherUnittest"));
    }

    #[test]
    fn test_display() {
        let (tlv, _) = set_up();
        assert_eq!(format!("{}", tlv), "SystemDescriptionTLV(\"Unittest\")");
    }
}
