use crate::tlv::TlvType;
use bytes::BufMut;

use std::convert::{TryFrom, TryInto};
use std::fmt::Display;
use std::net::IpAddr;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PortIdSubtype {
    InterfaceAlias = 1,
    PortComponent = 2,
    MacAddress = 3,
    NetworkAddress = 4,
    InterfaceName = 5,
    CircuitId = 6,
    Local = 7,
}

impl TryFrom<u8> for PortIdSubtype {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == PortIdSubtype::InterfaceAlias as u8 => Ok(PortIdSubtype::InterfaceAlias),
            x if x == PortIdSubtype::PortComponent as u8 => Ok(PortIdSubtype::PortComponent),
            x if x == PortIdSubtype::MacAddress as u8 => Ok(PortIdSubtype::MacAddress),
            x if x == PortIdSubtype::NetworkAddress as u8 => Ok(PortIdSubtype::NetworkAddress),
            x if x == PortIdSubtype::InterfaceName as u8 => Ok(PortIdSubtype::InterfaceName),
            x if x == PortIdSubtype::CircuitId as u8 => Ok(PortIdSubtype::CircuitId),
            x if x == PortIdSubtype::Local as u8 => Ok(PortIdSubtype::Local),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum PortIdValue {
    Mac(Vec<u8>),
    IpAddress(IpAddr),
    Other(String),
}

/// Port ID TLV
///
/// The port ID TLV identifies the sending port used by the LLDP agent.
///
/// The port ID TLV is mandatory and MUST be the second TLV in the LLDPDU.
/// Each LLDPDU MUST contain one, and only one, Port ID TLV.
///
/// # TLV Format:
///
///         0               1               2               3
///        +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-...-+-+-+-+
///        |             |                 |               |               |
///        |      2      |      Length     |    Subtype    |    Port ID    |
///        |             |                 |               |               |
///        +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-...-+-+-+-+
///
///                                                            1 - 255 byte
///
/// # Subtypes
///
/// There are several ways in which a port may be identified.
/// A port ID subtype is used to indicate how the port is being referenced in the Port ID field.
///
/// | Subtype | ID Basis          | Example                    |
/// | ------- | ----------------- | -------------------------- |
/// | 0       | Reserved          |                            |
/// | 1       | Interface Alias   | Ethernet Interface         |
/// | 2       | Port Component    | backplane1                 |
/// | 3       | MAC Address       | 02:04:df:88:a2:b4          |
/// | 4       | Network Address   | 134.96.86.110              |
/// | 5       | Interface Name    | eth0                       |
/// | 6       | Agent Circuit ID  |                            |
/// | 7       | Locally Assigned  | Frank's Computer           |
/// | 8 - 255 | Reserved          |                            |
///
/// With the exception of subtypes 3 (MAC Address) and 4 (Network Address) the subtype is a string
/// as far as the LLDP agent is concerned. A distinction between these types is only made by a human observer.
///
/// ## MAC Address Subtype:
///
/// MAC addresses are represented as raw bytes, e.g. the MAC address 02:04:df:88:a2:b4 corresponds to a value of
/// `b"\x02\x04\xDF\x88\xA2\xB4"`.
///
/// ## Network Address Subtype:
///
/// Network addresses are represented as raw bytes.
///
/// In practice there are many different network protocols, each with their own address format with e.g. a different
/// length.
///
/// To determine the type of network protocol and the appropriate length of the network address transmitted in the
/// port ID TLV, network addresses are prefixed with an extra byte identifying the address family.
///
/// For this implementation we only consider IPv4 and IPv6.
///
/// | Protocol | Family Number |
/// | -------- | ------------- |
/// |   IPv4   |             1 |
/// |   IPv6   |             2 |
///
///     Examples (Address -> Bytes -> Prefixed Bytes):
///         134.96.86.110  ->  b"\x86\x60\x56\x6E"  -> b"\x01\x86\x60\x56\x6E"
///
///         20db::1        ->  b"\x20\xdb\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01"
///                        ->  b"\x02\x20\xdb\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01"
///
/// The full list of registered protocol families is available at:
/// <https://www.iana.org/assignments/address-family-numbers/address-family-numbers.xhtml>
#[derive(Debug, Clone)]
pub struct PortIdTLV {
    /// The type of the TLV
    pub tlv_type: TlvType,
    /// The port ID subtype
    pub subtype: PortIdSubtype,
    /// The port ID.
    ///
    /// The type of the value field depends on the subtype
    /// * Mac Address -> `PortIdValue::Mac(Vec<u8>)`,
    /// * Network Address -> `PortIdValue::IpAddress(IpAddr)`,
    /// * Otherwise -> `PortIdValue::Other(String)`
    pub value: PortIdValue,
}

impl Display for PortIdTLV {
    /// Write a printable representation of the TLV object.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Implement
        // write!(f, "PortIdTLV({}, \"{}\")",self.subtype,self.value)
        todo!()
    }
}

impl PortIdTLV {
    /// Constructor
    ///
    /// The type of the id argument depends on the subtype
    /// * Mac Address -> `PortIdValue::Mac(Vec<u8>)`,
    /// * Network Address -> `PortIdValue::IpAddress(IpAddr)`,
    /// * Otherwise -> `PortIdValue::Other(String)`
    pub fn new(subtype: PortIdSubtype, id: PortIdValue) -> PortIdTLV {
        // TODO: Implement
        PortIdTLV {
            tlv_type: TlvType::PortId,
            subtype: subtype,
            value: id,
        }
    }

    /// Create a TLV instance from raw bytes.
    ///
    /// Panics if the provided TLV contains errors (e.g. has the wrong type).
    pub fn new_from_bytes(bytes: &[u8]) -> PortIdTLV {
        // TODO: Implement
        let mut type_value: u8 = bytes[0];
        type_value = bytes[0] & 0b11111110;

        let last_bit = bytes[0] & 0b00000001;

        type_value = type_value >> 1;

        let mut length_value = bytes[1] as u16;

        if last_bit != 0{
            length_value= length_value + 256;
        }

        let subtype_value:PortIdSubtype = match bytes[2].try_into(){
            Ok(subtype) => subtype,
            Err(_) => panic!("Port Id subtype Panic"),
        };

        let mac_value;

        let ip_addr;

        let other_value:String;

        let port_id_value;

        if (subtype_value.clone() as u8) == 3{
            assert_eq!(bytes[3..].len(), 6);
            mac_value = bytes[3..].to_vec();
            port_id_value = PortIdValue::Mac(mac_value);
        }

        else if (subtype_value.clone() as u8) == 4{
            let ip_first_byte = bytes[3];

            if ip_first_byte == 1{
                assert_eq!(bytes[4..].len(), 4);
                let ip_addr_bytes:[u8;4] = bytes[4..8].try_into().unwrap();
                ip_addr = IpAddr::from(ip_addr_bytes);
                port_id_value = PortIdValue::IpAddress(ip_addr);
                
            }
            else if ip_first_byte == 2{
                assert_eq!(bytes[4..].len(), 16);
                let ip_addr_bytes:[u8;16] = bytes[4..].try_into().unwrap();
                ip_addr = IpAddr::from(ip_addr_bytes);
                port_id_value = PortIdValue::IpAddress(ip_addr);    
            
            } 
            else {
                panic!("Port Id IP Address Error!")
            }
        }

        else {
            other_value = String::from_utf8(bytes[3..].to_vec()).unwrap();
            port_id_value = PortIdValue::Other(other_value);
        }

        PortIdTLV::new(subtype_value,port_id_value)
    }

    /// Return the length of the TLV value
    pub fn len(&self) -> usize {
        // TODO: Implement
        let mut total_len = 1 as usize;

        let value_len = match &self.value{
                PortIdValue::Mac(_) => 6,
                PortIdValue::IpAddress(ip_addr) => match ip_addr{
                    IpAddr::V4(_) => 5,
                    IpAddr::V6(_) => 17,
                },
                PortIdValue::Other(other) => other.len(),
        };

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

        let subtype_rep = self.subtype.clone() as u8;

        //let value_rep = self.len() as u8;

        let mut value_rep = match &self.value{
            PortIdValue::Mac(mac_addr) => mac_addr.clone(),
            PortIdValue::IpAddress(ip_addr) => match ip_addr {
                IpAddr::V4(ip_addr) => ip_addr.octets().to_vec(),
                IpAddr::V6(ip_addr) => ip_addr.octets().to_vec(),
            } ,
            PortIdValue::Other(other) => other.as_bytes().to_vec(),
        };

        if let PortIdValue::IpAddress(IpAddr::V4(_)) = self.value{
            value_rep.insert(0, 1)
        } 
            
        if let PortIdValue::IpAddress(IpAddr::V6(_)) = self.value {
            value_rep.insert(0, 2);
        }

        let mut port_id_rep = vec![type_rep,len_rep,subtype_rep];
        port_id_rep.append(&mut value_rep);

        port_id_rep



    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, Ipv6Addr};
    fn set_up() -> (PortIdTLV, String, PortIdSubtype) {
        let s: String = String::from("Bla bla bla, Mr.Freeman.");
        let st: PortIdSubtype = PortIdSubtype::Local;
        (PortIdTLV::new(st, PortIdValue::Other(s.clone())), s, st)
    }

    #[test]
    fn test_type() {
        let (pidtlv, _, _) = set_up();
        assert_eq!(pidtlv.tlv_type as u8, 2);
    }

    #[test]
    fn test_length() {
        let (pidtlv, value, _) = set_up();
        assert_eq!(pidtlv.len(), value.len() + 1);
    }

    #[test]
    fn test_value() {
        let (pidtlv, value, _) = set_up();
        match pidtlv.value {
            PortIdValue::Other(s) => assert_eq!(s, value),
            v => panic!("Expected OTHER, got {:?}", v),
        }
    }

    #[test]
    fn test_subtype() {
        let (pidtlv, _, subtype) = set_up();
        assert_eq!(pidtlv.subtype, subtype);
    }

    #[test]
    fn test_generic_subtypes() {
        for subtype in [
            PortIdSubtype::InterfaceAlias,
            PortIdSubtype::PortComponent,
            PortIdSubtype::InterfaceName,
            PortIdSubtype::CircuitId,
            PortIdSubtype::Local,
        ] {
            let pidtlv = PortIdTLV::new(
                subtype,
                PortIdValue::Other(String::from("Bla bla bla, Mr.Freeman.")),
            );
            match pidtlv.value {
                PortIdValue::Other(s) => assert_eq!(s, "Bla bla bla, Mr.Freeman."),
                v => panic!("Expected OTHER, got {:?}", v),
            }
            assert_eq!(pidtlv.subtype, subtype);
        }
    }

    #[test]
    fn test_generic_subtypes_dump() {
        for subtype in [
            PortIdSubtype::InterfaceAlias,
            PortIdSubtype::PortComponent,
            PortIdSubtype::InterfaceName,
            PortIdSubtype::CircuitId,
            PortIdSubtype::Local,
        ] {
            let value = String::from("Bla bla bla, Mr.Freeman.");
            let mut buf = b"\x04".to_vec();
            buf.put_u8(value.len() as u8 + 1);
            buf.put_u8(subtype as u8);
            buf.put(&value.as_bytes().to_vec()[..]);
            let pidtlv = PortIdTLV::new(subtype, PortIdValue::Other(value));
            assert_eq!(pidtlv.bytes(), buf);
        }
    }

    #[test]
    fn test_address_subtype_ipv4_dump() {
        let value: Ipv4Addr = "192.0.2.100".parse().unwrap();

        let pidtlv = PortIdTLV::new(
            PortIdSubtype::NetworkAddress,
            PortIdValue::IpAddress(IpAddr::V4(value)),
        );
        assert_eq!(pidtlv.bytes(), b"\x04\x06\x04\x01\xC0\x00\x02\x64".to_vec());
    }

    #[test]
    fn test_address_subtype_ipv6_dump() {
        let value: Ipv6Addr = "20db::1".parse().unwrap();
        let pidtlv = PortIdTLV::new(
            PortIdSubtype::NetworkAddress,
            PortIdValue::IpAddress(IpAddr::V6(value)),
        );
        assert_eq!(
            pidtlv.bytes(),
            b"\x04\x12\x04\x02\x20\xDB\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01"
                .to_vec()
        );
    }

    #[test]
    fn test_load() {
        let tlv = PortIdTLV::new_from_bytes(b"\x04\x0C\x07Abracadabra".as_ref());
        assert_eq!(tlv.len(), 12);
        assert_eq!(tlv.subtype, PortIdSubtype::Local);
        match tlv.value {
            PortIdValue::Other(s) => assert_eq!(s, "Abracadabra"),
            v => panic!("Expected OTHER, got {:?}", v),
        };
    }

    #[test]
    fn test_load_ipv4() {
        let tlv = PortIdTLV::new_from_bytes(b"\x04\x06\x04\x01\xC0\x02\x00\x01".as_ref());
        assert_eq!(tlv.len(), 6);
        assert_eq!(tlv.subtype, PortIdSubtype::NetworkAddress);
        match tlv.value {
            PortIdValue::IpAddress(ip) => match ip {
                IpAddr::V4(ip) => {
                    assert_eq!(ip.octets(), [192, 2, 0, 1]);
                }
                addr => panic!("Expected IPv4 address, got {:?}", addr),
            },
            v => panic!("Expected IPADDRESS, got {:?}", v),
        };
    }

    #[test]
    #[should_panic]
    fn test_load_invalid_ipv4() {
        PortIdTLV::new_from_bytes(b"\x04\x07\x04\x01\xC0\x02\x00\x01\x99".as_ref());
    }

    #[test]
    fn test_load_ipv6() {
        let tlv = PortIdTLV::new_from_bytes(
            b"\x04\x12\x04\x02\x20\x01\x00\xdb\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x78"
                .as_ref(),
        );
        assert_eq!(tlv.len(), 18);
        assert_eq!(tlv.subtype, PortIdSubtype::NetworkAddress);
        match tlv.value {
            PortIdValue::IpAddress(ip) => match ip {
                IpAddr::V6(ip) => {
                    let parsed: Ipv6Addr = "2001:db::78".parse().unwrap();
                    assert_eq!(ip.octets(), parsed.octets());
                }
                addr => panic!("Expected IPv6 address, got {:?}", addr),
            },
            v => panic!("Expected IPADDRESS, got {:?}", v),
        };
    }

    #[test]
    #[should_panic]
    fn test_load_invalid_ipv6() {
        PortIdTLV::new_from_bytes(b"\x04\x06\x04\x02\xC0\x02\x00\x01".as_ref());
    }

    #[test]
    fn test_display1() {
        let tlv = PortIdTLV::new(
            PortIdSubtype::Local,
            PortIdValue::Other("Bla bla bla, Mr. Freeman".into()),
        );

        assert_eq!(
            format!("{}", tlv),
            "PortIdTLV(7, \"Bla bla bla, Mr. Freeman\")"
        )
    }

    #[test]
    fn test_chassisid_display2() {
        let tlv = PortIdTLV::new(
            PortIdSubtype::MacAddress,
            PortIdValue::Mac(vec![0x66, 0x6F, 0x6F, 0x62, 0x61, 0x72]),
        );

        assert_eq!(
            format!("{}", tlv).to_lowercase(),
            "portidtlv(3, \"66:6f:6f:62:61:72\")"
        )
    }

    #[test]
    fn test_chassisid_display3() {
        let tlv = PortIdTLV::new(
            PortIdSubtype::NetworkAddress,
            PortIdValue::IpAddress(Ipv4Addr::new(127, 0, 0, 1).into()),
        );

        assert_eq!(format!("{}", tlv), "PortIdTLV(4, \"127.0.0.1\")")
    }
}
