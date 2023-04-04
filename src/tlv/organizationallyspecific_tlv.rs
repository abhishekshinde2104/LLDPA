use std::fmt::Display;

use crate::tlv::TlvType;
use bytes::BufMut;

/// Organizationally Specific TLV
///
/// This TLV type is provided to allow organizations, software developers and equipment vendors to define TLVs
/// to advertise information to remote devices which can not be included in other TLV types.
///
/// It is an optional TLV and as such may be included in an LLDPDU zero or more times between the TTL TLV and the
/// End of LLDPDU TLV.
///
/// # TLV Format:
///
///      0               1               2               5               6
///     +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-...-+-+-+-+-+-+-+-+-+-+-+-+-+-+-...-+-+-+-+-+-|
///     |             |                 |    Organiz.   |    Organiz.   |   Organizationally  |
///     |     127     |      Length     |   Unique ID   |    Defined    | Defined Information |
///     |             |                 |     (OUI)     |    Subtype    |       (Value)       |
///     +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-...-+-+-+-+-+-+-+-+-+-+-+-+-+-+-...-+-+-+-+-+-|
///
///                                                                          0 - 507 byte
///
/// The OUI is a 24 bit number uniquely identifying a vendor, manufacturer or organization.
///
/// The subtype should be a unique subtype value assigned by the defining organization.
#[derive(Debug, Clone)]
pub struct OrganizationallySpecificTLV {
    /// The type of the TLV
    pub tlv_type: TlvType,
    /// Organizationally unique identifier
    pub oui: Vec<u8>,
    /// Organizationally defined subtype
    pub subtype: u8,
    /// Organizationally defined information
    pub value: Vec<u8>,
}

impl Display for OrganizationallySpecificTLV {
    /// Write a printable representation of the TLV object.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Implement
        write!(f, "{}", todo!())
    }
}

impl OrganizationallySpecificTLV {
    /// Constructor
    pub fn new(oui: Vec<u8>, subtype: u8, value: Vec<u8>) -> OrganizationallySpecificTLV {
        // TODO: Implement
        OrganizationallySpecificTLV {
            tlv_type: TlvType::OrganizationallySpecific,
            oui: oui,
            subtype: subtype,
            value: value,
        }
    }

    /// Create a TLV instance from raw bytes.
    ///
    /// Panics if the provided TLV contains errors (e.g. has the wrong type).
    pub fn new_from_bytes(bytes: &[u8]) -> OrganizationallySpecificTLV {
        // TODO: Implement
        let mut type_value: u8 = bytes[0];
        type_value = bytes[0] & 0b11111110;

        let last_bit = bytes[0] & 0b00000001;

        type_value = type_value >> 1;

        let mut length_value = bytes[1] as u16;

        if last_bit != 0{
            length_value= length_value + 256;
        }

        let b1 = bytes[2] as u8;
        let b2 = bytes[3]  as u8;
        let b3 = bytes[4] as u8;

        let org_uni_id_vec = vec![b1,b2,b3];

        let org_def_subtype = bytes[5] as u8;

        let org_def_info = bytes[6..].to_vec();


        OrganizationallySpecificTLV::new(org_uni_id_vec, org_def_subtype, org_def_info)
    }

    /// Return the length of the TLV value
    pub fn len(&self) -> usize {
        // TODO: Implement
        let mut total_len = 4 as usize;

        let value_len = self.value.len();

        total_len = total_len + value_len;

        total_len
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

        let mut org_spec_tlv =  vec![type_rep,len_rep];

        let mut oui_rep = self.oui;

        org_spec_tlv.append(&mut oui_rep);

        let subtype_rep = self.subtype.clone();

        org_spec_tlv.push(subtype_rep);

        let mut org_info_rep = self.value;

        org_spec_tlv.append(&mut org_info_rep);

        org_spec_tlv
        
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set_up() -> (OrganizationallySpecificTLV, Vec<u8>, u8, String) {
        let oui = b"\xAA\xBB\xCC".to_vec();
        let subtype = 5;
        let data = String::from("HURZ!");
        let tlv = OrganizationallySpecificTLV::new(oui.clone(), subtype, data.as_bytes().to_vec());
        (tlv, oui, subtype, data)
    }

    #[test]
    fn test_type() {
        let (tlv, _, _, _) = set_up();
        assert_eq!(tlv.tlv_type as u8, TlvType::OrganizationallySpecific as u8);
    }

    #[test]
    fn test_length() {
        let (tlv, _, _, data) = set_up();
        assert_eq!(tlv.len(), data.len() + 4);
    }

    #[test]
    fn test_value() {
        let (tlv, _, _, data) = set_up();
        assert_eq!(tlv.value, data.as_bytes().to_vec());
    }

    #[test]
    fn test_subtype() {
        let (tlv, _, subtype, _) = set_up();
        assert_eq!(tlv.subtype, subtype);
    }

    #[test]
    fn test_dump() {
        let (tlv, oui, subtype, data) = set_up();
        let mut bytes = b"\xFE".to_vec();
        bytes.put_u8(data.as_bytes().len() as u8 + 4);
        bytes.put(oui.as_slice());
        bytes.put_u8(subtype);
        bytes.put(data.as_bytes());

        assert_eq!(tlv.bytes(), bytes);
    }

    #[test]
    fn test_load() {
        let tlv = OrganizationallySpecificTLV::new_from_bytes(
            b"\xFE\x1D\xAA\xBB\xCC\x1A0118 999 88199 9119 725 3".as_ref(),
        );
        assert_eq!(tlv.len(), 29);
        assert_eq!(tlv.value, b"0118 999 88199 9119 725 3".to_vec());
        assert_eq!(tlv.oui, b"\xAA\xBB\xCC".to_vec());
        assert_eq!(tlv.subtype, 0x1A);
    }

    #[test]
    fn test_display() {
        let (tlv, _, _, _) = set_up();
        assert_eq!(
            format!("{}", tlv),
            "OrganizationallySpecificTLV(\"AABBCC\", 5, \"4855525A21\")"
        );
    }
}
