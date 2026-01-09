#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port 1 Input register"]
    pub p1in: P1IN,
    _reserved1: [u8; 0x01],
    #[doc = "0x02 - Port 1 Output register"]
    pub p1out: P1OUT,
    _reserved2: [u8; 0x01],
    #[doc = "0x04 - Port 1 Direction register"]
    pub p1dir: P1DIR,
    _reserved3: [u8; 0x01],
    #[doc = "0x06 - Port 1 Resistor Enable register"]
    pub p1ren: P1REN,
    _reserved4: [u8; 0x03],
    #[doc = "0x0a - Port 1 Selection register bit 0"]
    pub p1sel0: P1SEL0,
    _reserved5: [u8; 0x01],
    #[doc = "0x0c - Port 1 Selection register bit 1"]
    pub p1sel1: P1SEL1,
    _reserved6: [u8; 0x01],
    #[doc = "0x0e - Port 1 Interrupt Vector register"]
    pub p1iv: P1IV,
    _reserved7: [u8; 0x06],
    #[doc = "0x16 - Port 1 Complement Select register"]
    pub p1selc: P1SELC,
    _reserved8: [u8; 0x01],
    #[doc = "0x18 - Port 1 Interrupt Edge Select register"]
    pub p1ies: P1IES,
    _reserved9: [u8; 0x01],
    #[doc = "0x1a - Port 1 Interrupt Enable register"]
    pub p1ie: P1IE,
    _reserved10: [u8; 0x01],
    #[doc = "0x1c - Port 1 Interrupt Flag register"]
    pub p1ifg: P1IFG,
}
#[doc = "P1IN (rw) register accessor: an alias for `Reg<P1IN_SPEC>`"]
pub type P1IN = crate::Reg<p1in::P1IN_SPEC>;
#[doc = "Port 1 Input register"]
pub mod p1in;
#[doc = "P1OUT (rw) register accessor: an alias for `Reg<P1OUT_SPEC>`"]
pub type P1OUT = crate::Reg<p1out::P1OUT_SPEC>;
#[doc = "Port 1 Output register"]
pub mod p1out;
#[doc = "P1DIR (rw) register accessor: an alias for `Reg<P1DIR_SPEC>`"]
pub type P1DIR = crate::Reg<p1dir::P1DIR_SPEC>;
#[doc = "Port 1 Direction register"]
pub mod p1dir;
#[doc = "P1REN (rw) register accessor: an alias for `Reg<P1REN_SPEC>`"]
pub type P1REN = crate::Reg<p1ren::P1REN_SPEC>;
#[doc = "Port 1 Resistor Enable register"]
pub mod p1ren;
#[doc = "P1SEL0 (rw) register accessor: an alias for `Reg<P1SEL0_SPEC>`"]
pub type P1SEL0 = crate::Reg<p1sel0::P1SEL0_SPEC>;
#[doc = "Port 1 Selection register bit 0"]
pub mod p1sel0;
#[doc = "P1SEL1 (rw) register accessor: an alias for `Reg<P1SEL1_SPEC>`"]
pub type P1SEL1 = crate::Reg<p1sel1::P1SEL1_SPEC>;
#[doc = "Port 1 Selection register bit 1"]
pub mod p1sel1;
#[doc = "P1SELC (rw) register accessor: an alias for `Reg<P1SELC_SPEC>`"]
pub type P1SELC = crate::Reg<p1selc::P1SELC_SPEC>;
#[doc = "Port 1 Complement Select register"]
pub mod p1selc;
#[doc = "P1IES (rw) register accessor: an alias for `Reg<P1IES_SPEC>`"]
pub type P1IES = crate::Reg<p1ies::P1IES_SPEC>;
#[doc = "Port 1 Interrupt Edge Select register"]
pub mod p1ies;
#[doc = "P1IE (rw) register accessor: an alias for `Reg<P1IE_SPEC>`"]
pub type P1IE = crate::Reg<p1ie::P1IE_SPEC>;
#[doc = "Port 1 Interrupt Enable register"]
pub mod p1ie;
#[doc = "P1IFG (rw) register accessor: an alias for `Reg<P1IFG_SPEC>`"]
pub type P1IFG = crate::Reg<p1ifg::P1IFG_SPEC>;
#[doc = "Port 1 Interrupt Flag register"]
pub mod p1ifg;
#[doc = "P1IV (rw) register accessor: an alias for `Reg<P1IV_SPEC>`"]
pub type P1IV = crate::Reg<p1iv::P1IV_SPEC>;
#[doc = "Port 1 Interrupt Vector register"]
pub mod p1iv;
