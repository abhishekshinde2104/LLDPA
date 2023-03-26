use std::convert::{TryFrom, TryInto};
use std::fmt::Display;

pub mod chassisid_tlv;
pub mod eolldpdu_tlv;
pub mod managementaddress_tlv;
pub mod organizationallyspecific_tlv;
pub mod portdescription_tlv;
pub mod portid_tlv;
pub mod systemcapabilities_tlv;
pub mod systemdescription_tlv;
pub mod systemname_tlv;
pub mod ttl_tlv;

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum TlvType {
    EndOfLLDPDU = 0,
    ChassisId = 1,
    PortId = 2,
    Ttl = 3,
    PortDescription = 4,
    SystemName = 5,
    SystemDescription = 6,
    SystemCapabilities = 7,
    ManagementAddress = 8,
    OrganizationallySpecific = 127,
}

impl TryFrom<u8> for TlvType {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == TlvType::EndOfLLDPDU as u8 => Ok(TlvType::EndOfLLDPDU),
            x if x == TlvType::ChassisId as u8 => Ok(TlvType::ChassisId),
            x if x == TlvType::PortId as u8 => Ok(TlvType::PortId),
            x if x == TlvType::Ttl as u8 => Ok(TlvType::Ttl),
            x if x == TlvType::PortDescription as u8 => Ok(TlvType::PortDescription),
            x if x == TlvType::SystemName as u8 => Ok(TlvType::SystemName),
            x if x == TlvType::SystemDescription as u8 => Ok(TlvType::SystemDescription),
            x if x == TlvType::SystemCapabilities as u8 => Ok(TlvType::SystemCapabilities),
            x if x == TlvType::ManagementAddress as u8 => Ok(TlvType::ManagementAddress),
            x if x == TlvType::OrganizationallySpecific as u8 => {
                Ok(TlvType::OrganizationallySpecific)
            }
            _ => Err(()),
        }
    }
}

// create bare tlv class, this allows for calling default TLV::functions

use crate::tlv::chassisid_tlv::ChassisIdTLV;
use crate::tlv::eolldpdu_tlv::EndOfLLDPDUTLV;
use crate::tlv::managementaddress_tlv::ManagementAddressTLV;
use crate::tlv::organizationallyspecific_tlv::OrganizationallySpecificTLV;
use crate::tlv::portdescription_tlv::PortDescriptionTLV;
use crate::tlv::portid_tlv::PortIdTLV;
use crate::tlv::systemcapabilities_tlv::SystemCapabilitiesTLV;
use crate::tlv::systemdescription_tlv::SystemDescriptionTLV;
use crate::tlv::systemname_tlv::SystemNameTLV;
use crate::tlv::ttl_tlv::TtlTLV;

/// TLV Base class
///
/// This is the basic abstract TLV class. It provides some functionality common to all (or at least most) of the TLVs
/// defined by IEEE802.AB.
///
/// No instances of this class should ever be created. It is simply an ancestor for TLVs to inherit from.
///
/// You have to implement at least "TLV.get_length()" and parts of "TLV.get_type()".
///
/// Hint: Implementing the other methods in this class (or even adding some methods) can save you a lot of work in the
/// other TLVs. It might be worth checking out the formats of the other TLVs and implement a lowest common
/// denominator here. It is not required however.
#[derive(Debug, Clone)]
pub enum Tlv {
    ChassisId(ChassisIdTLV),
    EndOfLldpdu(EndOfLLDPDUTLV),
    ManagementAddress(ManagementAddressTLV),
    OrganizationallySpecific(OrganizationallySpecificTLV),
    PortId(PortIdTLV),
    PortDescription(PortDescriptionTLV),
    SystemDescription(SystemDescriptionTLV),
    SystemName(SystemNameTLV),
    SystemCapabilities(SystemCapabilitiesTLV),
    Ttl(TtlTLV),
}

impl Display for Tlv {
    /// Write a printable representation of the TLV object.
    ///
    /// The representation should have the following form:
    ///     StructName(arg1, arg2, arg3)
    ///
    /// (See also the test_display tests in the corresponding files)
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Implement
        match self{
            Tlv::ChassisId(tlv) => write!(f, "{}",tlv),
            Tlv::EndOfLldpdu(tlv) => write!(f, "{}",tlv),
            Tlv::ManagementAddress(tlv) => write!(f, "{}",tlv),
            Tlv::OrganizationallySpecific(tlv) => write!(f, "{}",tlv),
            Tlv::PortId(tlv) => write!(f, "{}",tlv),
            Tlv::PortDescription(tlv) => write!(f, "{}",tlv),
            Tlv::SystemDescription(tlv) => write!(f, "{}",tlv),
            Tlv::SystemName(tlv) => write!(f, "{}",tlv),
            Tlv::SystemCapabilities(tlv) => write!(f, "{}",tlv),
            Tlv::Ttl(tlv) => write!(f, "{}",tlv),
        }
    }
}

impl Tlv {
    pub fn get_type(&self) -> TlvType {
        // TODO: Implement
        match self{
            Tlv::ChassisId(tlv) => tlv.tlv_type,
            Tlv::EndOfLldpdu(tlv) => tlv.tlv_type,
            Tlv::ManagementAddress(tlv) => tlv.tlv_type,
            Tlv::OrganizationallySpecific(tlv) => tlv.tlv_type,
            Tlv::PortId(tlv) => tlv.tlv_type,
            Tlv::PortDescription(tlv) => tlv.tlv_type,
            Tlv::SystemDescription(tlv) => tlv.tlv_type,
            Tlv::SystemName(tlv) => tlv.tlv_type,
            Tlv::SystemCapabilities(tlv) => tlv.tlv_type,
            Tlv::Ttl(tlv) => tlv.tlv_type,
        }
    }

    /// Return the byte representation of the TLV.
    ///
    /// Consider the following TLV:
    ///
    ///      0                   1                   2                   3
    ///      0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
    ///     +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    ///     |             |                 |                               |
    ///     |     0x3     |       0x2       |            0x003c             |
    ///     |             |                 |                               |
    ///     +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    ///
    /// When called on this TLV, this method should return `b"\x06\x02\x00\x3c".to_vec()`.
    pub fn bytes(&self) -> Vec<u8> {
        // TODO: Implement
        match self{
            Tlv::ChassisId(tlv) => tlv.bytes(),
            Tlv::EndOfLldpdu(tlv) => tlv.bytes(),
            Tlv::ManagementAddress(tlv) => tlv.bytes(),
            Tlv::OrganizationallySpecific(tlv) => tlv.bytes(),
            Tlv::PortId(tlv) => tlv.bytes(),
            Tlv::PortDescription(tlv) => tlv.bytes(),
            Tlv::SystemDescription(tlv) => tlv.bytes(),
            Tlv::SystemName(tlv) => tlv.bytes(),
            Tlv::SystemCapabilities(tlv) => tlv.bytes(),
            Tlv::Ttl(tlv) => tlv.bytes(),
        }
    }

    /// Get the length of a packed TLV.
    ///
    /// Extracts the relevant bytes from "data" and returns them.
    pub fn get_length(bytes: &[u8]) -> u16 {
        // TODO: Implement
        bytes[..].len() as u16
    }

    ///Create a Tlv instance from raw bytes.
    ///
    /// Reads the TLV Type of "bytes" and calls the from_bytes() method of the corresponding TLV subclass.
    ///
    /// Panics if the provided TLV is of unknown type. Apart from that validity checks are left to the
    /// subclass.
    pub fn from_bytes(bytes: &[u8]) -> Tlv {
        // TODO: Implement
        let mut type_value: u8 = bytes[0];
        type_value = bytes[0] & 0b11111110;

        type_value = type_value >> 1;

        let type_value: TlvType = type_value.try_into().unwrap();

        match type_value{
            TlvType::ChassisId => Tlv::ChassisId((ChassisIdTLV::new_from_bytes(bytes))),
            TlvType::PortId=> Tlv::PortId((PortIdTLV::new_from_bytes(bytes))),
            TlvType::Ttl => Tlv::Ttl((TtlTLV::new_from_bytes(bytes))),
            TlvType::EndOfLLDPDU => Tlv::EndOfLldpdu((EndOfLLDPDUTLV::new_from_bytes(bytes))),
            TlvType::PortDescription => Tlv::PortDescription((PortDescriptionTLV::new_from_bytes(bytes))),
            TlvType::SystemName => Tlv::SystemName((SystemNameTLV::new_from_bytes(bytes))),
            TlvType::SystemDescription => Tlv::SystemDescription((SystemDescriptionTLV::new_from_bytes(bytes))),
            TlvType::SystemCapabilities => Tlv::SystemCapabilities((SystemCapabilitiesTLV::new_from_bytes(bytes))),
            TlvType::ManagementAddress => Tlv::ManagementAddress((ManagementAddressTLV::new_from_bytes(bytes))),
            TlvType::OrganizationallySpecific => Tlv::OrganizationallySpecific((OrganizationallySpecificTLV::new_from_bytes(bytes))),
        }
    }
}
