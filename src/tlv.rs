use std::convert::TryFrom;
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
        write!(f, "{}", todo!())
    }
}

impl Tlv {
    pub fn get_type(&self) -> TlvType {
        // TODO: Implement
        todo!()
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
        todo!()
    }

    /// Get the length of a packed TLV.
    ///
    /// Extracts the relevant bytes from "data" and returns them.
    pub fn get_length(bytes: &[u8]) -> u16 {
        // TODO: Implement
        todo!()
    }

    ///Create a Tlv instance from raw bytes.
    ///
    /// Reads the TLV Type of "bytes" and calls the from_bytes() method of the corresponding TLV subclass.
    ///
    /// Panics if the provided TLV is of unknown type. Apart from that validity checks are left to the
    /// subclass.
    pub fn from_bytes(bytes: &[u8]) -> Tlv {
        // TODO: Implement
        todo!()
    }
}
