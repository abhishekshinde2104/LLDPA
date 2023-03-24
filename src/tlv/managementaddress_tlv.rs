use crate::tlv::TlvType;

use bytes::{Buf, BufMut};
use std::convert::TryFrom;
use std::fmt::Display;
use std::net::IpAddr;

#[derive(Debug, Clone)]
pub enum IFNumberingSubtype {
    Unknown = 1,
    IfIndex = 2,
    SystemPort = 3,
}

impl TryFrom<u8> for IFNumberingSubtype {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == IFNumberingSubtype::Unknown as u8 => Ok(IFNumberingSubtype::Unknown),
            x if x == IFNumberingSubtype::IfIndex as u8 => Ok(IFNumberingSubtype::IfIndex),
            x if x == IFNumberingSubtype::SystemPort as u8 => Ok(IFNumberingSubtype::SystemPort),
            _ => Err(()),
        }
    }
}

/// Management Address TLV
///
/// The Management Address TLV identifies an address associated with the local LLDP agent that may be used to reach
/// higher layer entities to assist discovery by network management, e.g. a web interface for device configuration.
///
/// It is an optional TLV and as such may be included in an LLDPDU zero or more times between
/// the TTL TLV and the End of LLDPDU TLV.
///
/// # TLV Format:
///
///       0               1               2               3               4
///      +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-...-+-+-+-+~
///      |             |                 |  Management   |  Management   |   Management    |
///      |     0x1     |      Length     |    Address    |    Address    |     Address     |
///      |             |                 | String Length |    Subtype    | (m=1-31 octets) |
///      +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-...-+-+-+-+~
///
///       5+m             6+m              10+m           11+m
///     ~+-+-+-+-+-+-+-+-+-+-+-+...+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+.....+-+-+-+-+-+-+-+
///      |   Interface   |   Interface   |  OID String   |        Object identifier        |
///      |   Numbering   |    Number     |    Length     |         (0-128 octets)          |
///      |    Subtype    |   (4 octets)  |   (1 octet)   |                                 |
///     ~+-+-+-+-+-+-+-+-+-+-+-+...+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+.....+-+-+-+-+-+-+-+
///
/// # Management Address Subtype and Management Address String Length:
///
/// In practice there are many different network protocols, each with their own address format and length.
///
/// To identify the type of network protocol and length of the network address the TLV includes a management address
/// subtype and string length. Address lengths are given in bytes.
///
/// For this implementation we only consider IPv4 and IPv6.
///
/// | Protocol | Subtype |
/// | -------- | ------- |
/// |   IPv4   |       1 |
/// |   IPv6   |       2 |
///
///  Example:
///  134.96.86.110 is an IPv4 address, so it has a subtype of 1 and it has a length of 4 bytes.
///
/// The full list of registered protocol families is available at:
/// <https://www.iana.org/assignments/address-family-numbers/address-family-numbers.xhtml>
///
///
/// # Interface Number and Subtype:
///
/// The interface numbering subtype indicates the numbering method used to define the interface number.
///
/// From the view of the LLDP agent the interface number is not treated differently depending on the numbering
/// subtype. It is just a number.
///
/// The LLDP standard specifies three valid subtypes:
///
/// | Subtype |    Description     |
/// | ------- | ------------------ |
/// |       1 |      Unknown       |
/// |       2 |  Interface Index   |
/// |       3 | System Port Number |
///
/// # OID / OID Length:
///
/// An OID (Object IDentifier) is a globally unabiguous name for any type of object / thing.
/// It can be used to e.g. identify the kind of hardware component associated with the management address.
///
/// This implementation will not make use of the OID, but it nevertheless has to be handled properly if included in
/// a TLV. It does not have to be interpreted.
///
/// Example:
///
///     let tlv = ManagementAddressTLV::new( "192.2.0.1".parse().unwrap(), 4, IFNumberingSubtype::IF_INDEX, b"\x00\x08\x15".to_vec());
///     println!("{:?}", tlv.oid);
///     // Should print:
///     [0, 8, 21]
#[derive(Debug, Clone)]
pub struct ManagementAddressTLV {
    /// The type of the TLV
    pub tlv_type: TlvType,
    /// The interface numbering subtype
    pub subtype: IFNumberingSubtype,
    /// The management address
    pub value: IpAddr,
    /// The object identifier of the device sending the TLV
    pub oid: Vec<u8>,
}

impl Display for ManagementAddressTLV {
    /// Write a printable representation of the TLV object.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Implement
        write!(f, "{}", todo!())
    }
}

impl ManagementAddressTLV {
    /// Constructor
    pub fn new(
        address: IpAddr,
        interface_number: u32,
        ifsubtype: IFNumberingSubtype,
        oid: Vec<u8>,
    ) -> ManagementAddressTLV {
        // TODO: Implement
        ManagementAddressTLV {
            tlv_type: TlvType::ManagementAddress,
            subtype: ifsubtype,
            value: address,
            oid: oid,
        }
    }

    /// Create a TLV instance from raw bytes.
    ///
    /// Panics if the provided TLV contains errors (e.g. has the wrong type).
    pub fn new_from_bytes(bytes: &[u8]) -> ManagementAddressTLV {
        // TODO: Implement
        let mut type_value = bytes[0];
        type_value = bytes[0] & 0b11111110;

        let last_bit = bytes[0] & 0b00000001;

        type_value = type_value >> 1;

        let mut length_value = bytes[1] as u16;

        if last_bit != 0{
            length_value= length_value + 256;
        }

        ManagementAddressTLV::new(address, interface_number, ifsubtype, oid)
    }

    /// Return the length of the TLV value
    pub fn len(&self) -> usize {
        // TODO: Implement
        todo!()
    }

    /// Return the byte representation of the TLV.
    pub fn bytes(&self) -> Vec<u8> {
        // TODO: Implement
        todo!()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, Ipv6Addr};

    fn set_up() -> (ManagementAddressTLV, ManagementAddressTLV) {
        let ipv4: Ipv4Addr = "192.0.2.100".parse().unwrap();
        let ipv6: Ipv6Addr = "2001:db::4".parse().unwrap();
        let ifnum = 5;
        let oid = b"\x2b\x06\x01\x04\x01\x82\x37\x15\x14".to_vec();
        let tlv4 = ManagementAddressTLV::new(
            IpAddr::V4(ipv4),
            ifnum,
            IFNumberingSubtype::Unknown,
            oid.clone(),
        );
        let tlv6 =
            ManagementAddressTLV::new(IpAddr::V6(ipv6), ifnum, IFNumberingSubtype::Unknown, oid);
        (tlv4, tlv6)
    }

    #[test]
    fn test_chassisid_type() {
        let (tlv4, tlv6) = set_up();
        assert_eq!(tlv4.tlv_type as u8, TlvType::ManagementAddress as u8);
        assert_eq!(tlv6.tlv_type as u8, TlvType::ManagementAddress as u8);
    }

    #[test]
    fn test_length_v4() {
        let (tlv4, _) = set_up();
        assert_eq!(
            tlv4.len(),
            12 + b"\x2b\x06\x01\x04\x01\x82\x37\x15\x14".to_vec().len()
        );
    }

    #[test]
    fn test_length_v6() {
        let (_, tlv6) = set_up();
        assert_eq!(
            tlv6.len(),
            24 + b"\x2b\x06\x01\x04\x01\x82\x37\x15\x14".to_vec().len()
        );
    }

    #[test]
    fn test_value() {
        let (tlv4, tlv6) = set_up();
        match tlv4.value {
            IpAddr::V4(ip) => {
                assert_eq!(ip.octets(), [192, 0, 2, 100]);
            }
            IpAddr::V6(_) => {
                panic!("Expected IPv4, got IPv6 address");
            }
        }
        match tlv6.value {
            IpAddr::V4(_) => {
                panic!("Expected IPv6, got IPv4 address");
            }
            IpAddr::V6(ip) => {
                let parsed: Ipv6Addr = "2001:db::4".parse().unwrap();
                assert_eq!(ip.octets(), parsed.octets());
            }
        }
    }

    #[test]
    fn test_oid() {
        let (tlv4, tlv6) = set_up();
        assert_eq!(tlv4.oid, b"\x2b\x06\x01\x04\x01\x82\x37\x15\x14".to_vec());
        assert_eq!(tlv6.oid, b"\x2b\x06\x01\x04\x01\x82\x37\x15\x14".to_vec());
    }

    #[test]
    fn test_none_oid() {
        let (tlv4, tlv6) = set_up();
        let t1 = ManagementAddressTLV::new(tlv4.value, 5, IFNumberingSubtype::Unknown, vec![]);
        let t2 = ManagementAddressTLV::new(tlv6.value, 5, IFNumberingSubtype::Unknown, vec![]);
        assert_eq!(t1.oid, vec![]);
        assert_eq!(t2.oid, vec![]);
    }

    #[test]
    fn test_dump_v4() {
        let (tlv4, _) = set_up();
        let oid = b"\x2b\x06\x01\x04\x01\x82\x37\x15\x14";
        let ipv4: Ipv4Addr = "192.0.2.100".parse().unwrap();

        let mut bytes = b"\x10".to_vec();
        bytes.put_u8(12 + oid.len() as u8);
        bytes.put(&b"\x05\x01"[..]);
        bytes.put(&ipv4.octets()[..]);
        bytes.put(&b"\x01"[..]);
        bytes.put_u32(5);
        bytes.put_u8(oid.len() as u8);
        bytes.put(&oid[..]);
        assert_eq!(tlv4.bytes(), bytes);

        assert_eq!(
            tlv4.bytes(),
            [16, 21, 5, 1, 192, 0, 2, 100, 1, 0, 0, 0, 5, 9, 43, 6, 1, 4, 1, 130, 55, 21, 20]
                .to_vec()
        );
    }

    #[test]
    fn test_dump_v6() {
        let (_, tlv6) = set_up();
        let oid = b"\x2b\x06\x01\x04\x01\x82\x37\x15\x14";
        let ipv6: Ipv6Addr = "2001:db::4".parse().unwrap();

        let mut bytes = b"\x10".to_vec();
        bytes.put_u8(24 + oid.len() as u8);
        bytes.put(&b"\x11\x02"[..]);
        bytes.put(&ipv6.octets()[..]);
        bytes.put(&b"\x01"[..]);
        bytes.put_u32(5);
        bytes.put_u8(oid.len() as u8);
        bytes.put(&oid[..]);
        assert_eq!(tlv6.bytes(), bytes);

        assert_eq!(
            tlv6.bytes(),
            [
                16, 33, 17, 2, 32, 1, 0, 219, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 1, 0, 0, 0, 5, 9,
                43, 6, 1, 4, 1, 130, 55, 21, 20
            ]
            .to_vec()
        );
    }

    #[test]
    fn test_dump_zero_oid() {
        let ipv4: Ipv4Addr = "192.0.2.42".parse().unwrap();
        let tlv =
            ManagementAddressTLV::new(IpAddr::V4(ipv4), 1, IFNumberingSubtype::SystemPort, vec![]);
        assert_eq!(
            tlv.bytes(),
            b"\x10\x0C\x05\x01\xC0\x00\x02*\x03\x00\x00\x00\x01\x00".to_vec()
        );
    }

    #[test]
    fn test_load_v4() {
        let ipv4: Ipv4Addr = "192.0.2.42".parse().unwrap();

        let tlv = ManagementAddressTLV::new_from_bytes(
            b"\x10\x0D\x05\x01\xC0\x00\x02*\x02\x00\x00\x00\x01\x01\x0A",
        );
        assert_eq!(tlv.tlv_type as u8, TlvType::ManagementAddress as u8);
        assert_eq!(tlv.subtype as u8, IFNumberingSubtype::IfIndex as u8);
        match tlv.value {
            IpAddr::V4(ip) => {
                assert_eq!(ip.octets(), ipv4.octets());
            }
            IpAddr::V6(_) => {
                panic!("Expected IPv4, got IPv6 address");
            }
        };
        assert_eq!(tlv.oid, b"\x0A".to_vec());
    }

    #[test]
    fn test_load_v6() {
        let ipv6: Ipv6Addr = "2001:db::42".parse().unwrap();

        let tlv = ManagementAddressTLV::new_from_bytes(
            b"\x10\x19\x11\x02 \x01\x00\xdb\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00B\x02\x00\x00\x00\x01\x01\x0A"
        );
        assert_eq!(tlv.tlv_type as u8, TlvType::ManagementAddress as u8);
        assert_eq!(tlv.subtype as u8, IFNumberingSubtype::IfIndex as u8);
        match tlv.value {
            IpAddr::V4(_) => {
                panic!("Expected IPv6, got IPv4 address");
            }
            IpAddr::V6(ip) => {
                assert_eq!(ip.octets(), ipv6.octets());
            }
        };
        assert_eq!(tlv.oid, b"\x0A".to_vec());
    }

    #[test]
    fn test_load_zero_oid() {
        let tlv = ManagementAddressTLV::new_from_bytes(
            b"\x10\x0C\x05\x01\xC0\x00\x02*\x03\x00\x00\x00\x01\x00",
        );
        assert_eq!(tlv.oid, vec![]);
    }

    #[test]
    fn test_display_v4() {
        let (tlv, _) = set_up();
        assert_eq!(
            format!("{}", tlv),
            "ManagementAddressTLV(\"192.0.2.100\", 5, \"2B0601040182371514\")"
        )
    }

    #[test]
    fn test_display_v6() {
        let (_, tlv) = set_up();
        assert_eq!(
            format!("{}", tlv),
            "ManagementAddressTLV(\"2001:db::4\", 5, \"2B0601040182371514\")"
        )
    }
}
