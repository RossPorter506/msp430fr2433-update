#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CRC Data In Register"]
    pub crcdi: CRCDI,
    #[doc = "0x02 - CRC data in reverse byte Register"]
    pub crcdirb: CRCDIRB,
    #[doc = "0x04 - CRC Initialisation Register and Result Register"]
    pub crcinires: CRCINIRES,
    #[doc = "0x06 - CRC reverse result Register"]
    pub crcresr: CRCRESR,
}
#[doc = "CRCDI (rw) register accessor: an alias for `Reg<CRCDI_SPEC>`"]
pub type CRCDI = crate::Reg<crcdi::CRCDI_SPEC>;
#[doc = "CRC Data In Register"]
pub mod crcdi;
#[doc = "CRCDIRB (rw) register accessor: an alias for `Reg<CRCDIRB_SPEC>`"]
pub type CRCDIRB = crate::Reg<crcdirb::CRCDIRB_SPEC>;
#[doc = "CRC data in reverse byte Register"]
pub mod crcdirb;
#[doc = "CRCINIRES (rw) register accessor: an alias for `Reg<CRCINIRES_SPEC>`"]
pub type CRCINIRES = crate::Reg<crcinires::CRCINIRES_SPEC>;
#[doc = "CRC Initialisation Register and Result Register"]
pub mod crcinires;
#[doc = "CRCRESR (rw) register accessor: an alias for `Reg<CRCRESR_SPEC>`"]
pub type CRCRESR = crate::Reg<crcresr::CRCRESR_SPEC>;
#[doc = "CRC reverse result Register"]
pub mod crcresr;
