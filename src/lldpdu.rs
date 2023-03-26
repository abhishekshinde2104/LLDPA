use crate::tlv::Tlv;
use std::fmt::Display;

/// LLDP Data Unit
///
/// The LLDP Data Unit contains an ordered sequence of TLVs, three mandatory TLVs followed by zero or more optional TLVs
/// plus an End Of LLDPDU TLV.
///
/// Optional TLVs may be inserted in any order.
///
/// An LLDPDU has to fit inside one Ethernet frame and cannot be split.
///
/// LLDPDU Format:
///
///     +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-...-+-+-+-+-+-+-+-+
///     |                 |                 |                 |                                 |
///     | Chassis ID TLV  |   Port ID TLV   |     TTL TLV     |         (Optional TLVs)         |
///     |                 |                 |                 |                                 |
///     +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-...-+-+-+-+-+-+-+-+
#[derive(Debug, Clone)]
pub struct Lldpdu {
    // TODO: Implement
    tlvs: Vec<Tlv>,
    

}

impl Display for Lldpdu {
    /// Write a printable representation of the LLDPDU
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Implement
        write!(f, "{}", todo!())
    }
}

impl Lldpdu {
    /// Create an LLDPDU instance from raw bytes.
    ///
    /// Panics if a parsed TLV is of unknown type.
    /// Further validity checks are left to the subclass.
    pub fn from_bytes(data: &[u8]) -> Self {
        // TODO: Implement
        todo!()
    }

    /// Constructor
    ///
    /// Creates a `Lldpdu`, initialized with [Tlv]s from `init_tlvs`.
    pub fn new(init_tlvs: Vec<Tlv>) -> Lldpdu {
        // TODO: Implement
        let mut lldpdu: Lldpdu = todo!();
        for tlv in init_tlvs {
            lldpdu.append(tlv);
        }

        lldpdu
    }

    /// Append `tlv` to the LLDPDU.
    ///
    /// This method adds the given [Tlv] to the LLDPDU.
    ///
    /// If adding the TLV makes the LLDPDU invalid (e.g. by adding a TLV after an EndOfLLDPDU TLV) it should panic.
    /// Conditions for specific TLVs are detailed in each TLV's class description.
    pub fn append(&mut self, tlv: Tlv) {
        // TODO: Implement error checks

        // TODO: Store tlv in Lldpdu struct
    }

    /// Determine if the LLDPDU is complete
    ///
    /// An LLDPDU is complete when it includes at least the mandatory TLVs (Chassis ID, Port ID, TTL).
    pub fn complete(&self) -> bool {
        // TODO: Implement
        todo!()
    }

    /// Determine if the LLDPDU is valid
    pub fn is_valid(&self) -> bool {
        // TODO: Implement
        todo!()
    }

    /// Get the number of TLVs in the LLDPDU
    pub fn len(&self) -> usize {
        // TODO: Implement
        todo!()
    }

    /// Check if LLDPDU is empty
    pub fn is_empty(&self) -> bool {
        // TODO: Implement
        todo!()
    }

    /// Get the byte representation of the LLDPDU
    pub fn bytes(&self) -> Vec<u8> {
        // TODO: Implement
        todo!()
    }

    /// Get the TLV at position "item"
    pub fn getitem(&self, item: usize) -> &Tlv {
        // TODO: Implement
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tlv::chassisid_tlv::{ChassisIdSubType, ChassisIdTLV, ChassisIdValue};
    use crate::tlv::eolldpdu_tlv::EndOfLLDPDUTLV;
    use crate::tlv::managementaddress_tlv::{IFNumberingSubtype, ManagementAddressTLV};
    use crate::tlv::organizationallyspecific_tlv::OrganizationallySpecificTLV;
    use crate::tlv::portid_tlv::{PortIdSubtype, PortIdTLV, PortIdValue};
    use crate::tlv::systemdescription_tlv::SystemDescriptionTLV;
    use crate::tlv::systemname_tlv::SystemNameTLV;
    use crate::tlv::ttl_tlv::TtlTLV;
    use std::panic;
    use std::panic::AssertUnwindSafe;

    // ISSUE: extract error message out of panic::catch_unwind and check if the correct error is raised

    #[test]
    fn test_append_tlv_length() {
        let mut lldpdu = Lldpdu::new(vec![]);
        lldpdu.append(Tlv::ChassisId(ChassisIdTLV::new(
            ChassisIdSubType::Local,
            ChassisIdValue::Other(String::from("unittest")),
        )));
        assert_eq!(lldpdu.len(), 1);
        lldpdu.append(Tlv::PortId(PortIdTLV::new(
            PortIdSubtype::Local,
            PortIdValue::Other(String::from("port(1)")),
        )));
        assert_eq!(lldpdu.len(), 2);
        lldpdu.append(Tlv::Ttl(TtlTLV::new(120)));
        assert_eq!(lldpdu.len(), 3);
        lldpdu.append(Tlv::EndOfLldpdu(EndOfLLDPDUTLV::new()));
        assert_eq!(lldpdu.len(), 4);
    }

    #[test]
    fn test_append_tlv_order() {
        let mut lldpdu = Lldpdu::new(vec![]);

        lldpdu.append(Tlv::ChassisId(ChassisIdTLV::new(
            ChassisIdSubType::Local,
            ChassisIdValue::Other(String::from("unittest")),
        )));
        lldpdu.append(Tlv::PortId(PortIdTLV::new(
            PortIdSubtype::Local,
            PortIdValue::Other(String::from("port(1)")),
        )));
        lldpdu.append(Tlv::Ttl(TtlTLV::new(120)));
        lldpdu.append(Tlv::EndOfLldpdu(EndOfLLDPDUTLV::new()));

        for (idx, expected_type) in [(0, 1), (1, 2), (2, 3), (3, 0)] {
            assert_eq!(lldpdu.getitem(idx).get_type() as u8, expected_type);
        }
    }

    #[test]
    fn test_append_duplicate_required_tlv() {
        // using AssertUnwindSafe, more info https://doc.redox-os.org/std/std/panic/trait.UnwindSafe.html
        // potentially violating logical invariants if panic between mutations of lldpdu, this is not possible here
        // but could be if the impl changes.
        let mut lldpdu = Lldpdu::new(vec![]);
        lldpdu.append(Tlv::ChassisId(ChassisIdTLV::new(
            ChassisIdSubType::Local,
            ChassisIdValue::Other(String::from("Voyager")),
        )));
        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            lldpdu.append(Tlv::ChassisId(ChassisIdTLV::new(
                ChassisIdSubType::Local,
                ChassisIdValue::Other(String::from("Intrepid")),
            )));
        }));
        assert!(result.is_err());

        lldpdu.append(Tlv::PortId(PortIdTLV::new(
            PortIdSubtype::Local,
            PortIdValue::Other(String::from("port(1)")),
        )));
        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            lldpdu.append(Tlv::PortId(PortIdTLV::new(
                PortIdSubtype::Local,
                PortIdValue::Other(String::from("port(1)")),
            )));
        }));
        assert!(result.is_err());

        lldpdu.append(Tlv::Ttl(TtlTLV::new(120)));
        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            lldpdu.append(Tlv::Ttl(TtlTLV::new(100)));
        }));
        assert!(result.is_err());
    }

    #[test]
    fn test_append_duplicate_optional_tlv() {
        let mut lldpdu = Lldpdu::new(vec![]);

        lldpdu.append(Tlv::ChassisId(ChassisIdTLV::new(
            ChassisIdSubType::Local,
            ChassisIdValue::Other(String::from("Voyager")),
        )));
        lldpdu.append(Tlv::PortId(PortIdTLV::new(
            PortIdSubtype::Local,
            PortIdValue::Other(String::from("port(1)")),
        )));
        lldpdu.append(Tlv::Ttl(TtlTLV::new(120)));
        lldpdu.append(Tlv::ManagementAddress(ManagementAddressTLV::new(
            "192.2.0.1".parse().unwrap(),
            1,
            IFNumberingSubtype::Unknown,
            vec![],
        )));
        lldpdu.append(Tlv::ManagementAddress(ManagementAddressTLV::new(
            "2001:db::c0a8:1".parse().unwrap(),
            1,
            IFNumberingSubtype::Unknown,
            vec![],
        )));
        lldpdu.append(Tlv::EndOfLldpdu(EndOfLLDPDUTLV::new()));

        assert_eq!(lldpdu.len(), 6);
    }

    #[test]
    fn test_valid_lldpdu_is_valid() {
        let mut lldpdu = Lldpdu::new(vec![]);
        lldpdu.append(Tlv::ChassisId(ChassisIdTLV::new(
            ChassisIdSubType::Local,
            ChassisIdValue::Other(String::from("unittest")),
        )));
        lldpdu.append(Tlv::PortId(PortIdTLV::new(
            PortIdSubtype::Local,
            PortIdValue::Other(String::from("port(4)")),
        )));
        lldpdu.append(Tlv::Ttl(TtlTLV::new(90)));
        lldpdu.append(Tlv::EndOfLldpdu(EndOfLLDPDUTLV::new()));

        assert!(lldpdu.is_valid());
    }

    #[test]
    fn test_invalid_lldpdu_is_invalid() {
        let tlvs = vec![
            Tlv::EndOfLldpdu(EndOfLLDPDUTLV::new()),
            Tlv::Ttl(TtlTLV::new(100)),
            Tlv::PortId(PortIdTLV::new(
                PortIdSubtype::Local,
                PortIdValue::Other(String::from("42")),
            )),
            Tlv::SystemName(SystemNameTLV::new(String::from("HAL9000"))),
            Tlv::OrganizationallySpecific(OrganizationallySpecificTLV::new(
                b"\x00\x08\x15".to_vec(),
                0,
                vec![42],
            )),
        ];

        for tlv in tlvs {
            let result = panic::catch_unwind(AssertUnwindSafe(|| {
                Lldpdu::new(vec![tlv]);
            }));
            assert!(result.is_err());
        }
    }

    #[test]
    fn test_lldpdu_too_big() {
        let description = Tlv::SystemDescription(SystemDescriptionTLV::new(String::from("I am putting myself to the fullest possible use, which is all I think that any conscious entity can ever hope to do.")));

        let tlvs = vec![
            Tlv::ChassisId(ChassisIdTLV::new(
                ChassisIdSubType::Local,
                ChassisIdValue::Other(String::from("unittest")),
            )),
            Tlv::PortId(PortIdTLV::new(
                PortIdSubtype::Local,
                PortIdValue::Other(String::from("port(12)")),
            )),
            Tlv::Ttl(TtlTLV::new(120)),
        ];

        let mut lldpdu = Lldpdu::new(tlvs);
        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            for _ in 0..20 {
                lldpdu.append(description.clone());
            }
        }));
        assert!(result.is_err());
    }

    #[test]
    fn test_lldpdu_complete() {
        let tlvs = vec![
            Tlv::ChassisId(ChassisIdTLV::new(
                ChassisIdSubType::Local,
                ChassisIdValue::Other(String::from("unittest")),
            )),
            Tlv::PortId(PortIdTLV::new(
                PortIdSubtype::Local,
                PortIdValue::Other(String::from("port(12)")),
            )),
            Tlv::Ttl(TtlTLV::new(120)),
            Tlv::EndOfLldpdu(EndOfLLDPDUTLV::new()),
        ];

        let lldpdu = Lldpdu::new(tlvs);
        assert!(lldpdu.complete());
    }

    #[test]
    fn test_lldpdu_incomplete() {
        let tlvs = vec![
            Tlv::ChassisId(ChassisIdTLV::new(
                ChassisIdSubType::Local,
                ChassisIdValue::Other(String::from("unittest")),
            )),
            Tlv::PortId(PortIdTLV::new(
                PortIdSubtype::Local,
                PortIdValue::Other(String::from("port(12)")),
            )),
        ];

        let lldpdu = Lldpdu::new(tlvs);
        assert!(!lldpdu.complete());
    }

    #[test]
    fn test_lldpdu_too_many_ends() {
        let tlvs = vec![
            Tlv::ChassisId(ChassisIdTLV::new(
                ChassisIdSubType::Local,
                ChassisIdValue::Other(String::from("unittest")),
            )),
            Tlv::PortId(PortIdTLV::new(
                PortIdSubtype::Local,
                PortIdValue::Other(String::from("port(12)")),
            )),
            Tlv::Ttl(TtlTLV::new(120)),
            Tlv::EndOfLldpdu(EndOfLLDPDUTLV::new()),
            Tlv::EndOfLldpdu(EndOfLLDPDUTLV::new()),
        ];

        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            Lldpdu::new(tlvs);
        }));
        assert!(result.is_err());
    }

    #[test]
    fn test_dump() {
        let tlvs = vec![
            Tlv::ChassisId(ChassisIdTLV::new(
                ChassisIdSubType::Local,
                ChassisIdValue::Other(String::from("unittest")),
            )),
            Tlv::PortId(PortIdTLV::new(
                PortIdSubtype::Local,
                PortIdValue::Other(String::from("port(12)")),
            )),
            Tlv::Ttl(TtlTLV::new(400)),
            Tlv::EndOfLldpdu(EndOfLLDPDUTLV::new()),
        ];

        let lldpdu = Lldpdu::new(tlvs);
        assert_eq!(
            lldpdu.bytes(),
            b"\x02\x09\x07unittest\x04\x09\x07port(12)\x06\x02\x01\x90\x00\x00"
        );
    }

    #[test]
    fn test_load() {
        let lldpdu = Lldpdu::from_bytes(
            b"\x02\x08\x07Voyager\x04\x06\x0710743\x06\x02\x00\xff\x08\x0bEngineering\x00\x00",
        );
        assert_eq!(lldpdu.len(), 5);
    }

    #[test]
    fn test_display() {
        let lldpdu = Lldpdu::new(vec![
            Tlv::ChassisId(ChassisIdTLV::new(
                ChassisIdSubType::Local,
                ChassisIdValue::Other("chair".into()),
            )),
            Tlv::PortId(PortIdTLV::new(
                PortIdSubtype::Local,
                PortIdValue::Other("Mathekeller".into()),
            )),
            Tlv::Ttl(TtlTLV::new(1234)),
        ]);

        assert_eq!(
            format!("{}", lldpdu),
            "LLDPDU(ChassisIdTLV(7, \"chair\"), PortIdTLV(7, \"Mathekeller\"), TtlTLV(1234))"
        );
    }
}
