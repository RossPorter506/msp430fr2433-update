#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Enable 1"]
    pub sfrie1: SFRIE1,
    #[doc = "0x02 - Interrupt Flag 1"]
    pub sfrifg1: SFRIFG1,
    #[doc = "0x04 - RESET Pin Control Register"]
    pub sfrrpcr: SFRRPCR,
}
#[doc = "SFRIE1 (rw) register accessor: an alias for `Reg<SFRIE1_SPEC>`"]
pub type SFRIE1 = crate::Reg<sfrie1::SFRIE1_SPEC>;
#[doc = "Interrupt Enable 1"]
pub mod sfrie1;
#[doc = "SFRIFG1 (rw) register accessor: an alias for `Reg<SFRIFG1_SPEC>`"]
pub type SFRIFG1 = crate::Reg<sfrifg1::SFRIFG1_SPEC>;
#[doc = "Interrupt Flag 1"]
pub mod sfrifg1;
#[doc = "SFRRPCR (rw) register accessor: an alias for `Reg<SFRRPCR_SPEC>`"]
pub type SFRRPCR = crate::Reg<sfrrpcr::SFRRPCR_SPEC>;
#[doc = "RESET Pin Control Register"]
pub mod sfrrpcr;
