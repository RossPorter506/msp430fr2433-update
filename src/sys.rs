#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System control"]
    pub sysctl: SYSCTL,
    #[doc = "0x02 - Boot strap configuration area"]
    pub sysbslc: SYSBSLC,
    _reserved2: [u8; 0x02],
    #[doc = "0x06 - JTAG mailbox control"]
    pub sysjmbc: SYSJMBC,
    #[doc = "0x08 - JTAG mailbox input 0"]
    pub sysjmbi0: SYSJMBI0,
    #[doc = "0x0a - JTAG mailbox input 1"]
    pub sysjmbi1: SYSJMBI1,
    #[doc = "0x0c - JTAG mailbox output 0"]
    pub sysjmbo0: SYSJMBO0,
    #[doc = "0x0e - JTAG mailbox output 1"]
    pub sysjmbo1: SYSJMBO1,
    _reserved7: [u8; 0x08],
    #[doc = "0x18 - Bus Error vector generator"]
    pub sysberriv: SYSBERRIV,
    #[doc = "0x1a - User NMI vector generator"]
    pub sysuniv: SYSUNIV,
    #[doc = "0x1c - System NMI vector generator"]
    pub syssniv: SYSSNIV,
    #[doc = "0x1e - Reset vector generator"]
    pub sysrstiv: SYSRSTIV,
    #[doc = "0x20 - System Configuration 0"]
    pub syscfg0: SYSCFG0,
    #[doc = "0x22 - System Configuration 1"]
    pub syscfg1: SYSCFG1,
    #[doc = "0x24 - System Configuration 2"]
    pub syscfg2: SYSCFG2,
}
#[doc = "SYSCTL (rw) register accessor: an alias for `Reg<SYSCTL_SPEC>`"]
pub type SYSCTL = crate::Reg<sysctl::SYSCTL_SPEC>;
#[doc = "System control"]
pub mod sysctl;
#[doc = "SYSBSLC (rw) register accessor: an alias for `Reg<SYSBSLC_SPEC>`"]
pub type SYSBSLC = crate::Reg<sysbslc::SYSBSLC_SPEC>;
#[doc = "Boot strap configuration area"]
pub mod sysbslc;
#[doc = "SYSJMBC (rw) register accessor: an alias for `Reg<SYSJMBC_SPEC>`"]
pub type SYSJMBC = crate::Reg<sysjmbc::SYSJMBC_SPEC>;
#[doc = "JTAG mailbox control"]
pub mod sysjmbc;
#[doc = "SYSJMBI0 (rw) register accessor: an alias for `Reg<SYSJMBI0_SPEC>`"]
pub type SYSJMBI0 = crate::Reg<sysjmbi0::SYSJMBI0_SPEC>;
#[doc = "JTAG mailbox input 0"]
pub mod sysjmbi0;
#[doc = "SYSJMBI1 (rw) register accessor: an alias for `Reg<SYSJMBI1_SPEC>`"]
pub type SYSJMBI1 = crate::Reg<sysjmbi1::SYSJMBI1_SPEC>;
#[doc = "JTAG mailbox input 1"]
pub mod sysjmbi1;
#[doc = "SYSJMBO0 (rw) register accessor: an alias for `Reg<SYSJMBO0_SPEC>`"]
pub type SYSJMBO0 = crate::Reg<sysjmbo0::SYSJMBO0_SPEC>;
#[doc = "JTAG mailbox output 0"]
pub mod sysjmbo0;
#[doc = "SYSJMBO1 (rw) register accessor: an alias for `Reg<SYSJMBO1_SPEC>`"]
pub type SYSJMBO1 = crate::Reg<sysjmbo1::SYSJMBO1_SPEC>;
#[doc = "JTAG mailbox output 1"]
pub mod sysjmbo1;
#[doc = "SYSBERRIV (rw) register accessor: an alias for `Reg<SYSBERRIV_SPEC>`"]
pub type SYSBERRIV = crate::Reg<sysberriv::SYSBERRIV_SPEC>;
#[doc = "Bus Error vector generator"]
pub mod sysberriv;
#[doc = "SYSUNIV (rw) register accessor: an alias for `Reg<SYSUNIV_SPEC>`"]
pub type SYSUNIV = crate::Reg<sysuniv::SYSUNIV_SPEC>;
#[doc = "User NMI vector generator"]
pub mod sysuniv;
#[doc = "SYSSNIV (rw) register accessor: an alias for `Reg<SYSSNIV_SPEC>`"]
pub type SYSSNIV = crate::Reg<syssniv::SYSSNIV_SPEC>;
#[doc = "System NMI vector generator"]
pub mod syssniv;
#[doc = "SYSRSTIV (rw) register accessor: an alias for `Reg<SYSRSTIV_SPEC>`"]
pub type SYSRSTIV = crate::Reg<sysrstiv::SYSRSTIV_SPEC>;
#[doc = "Reset vector generator"]
pub mod sysrstiv;
#[doc = "SYSCFG0 (rw) register accessor: an alias for `Reg<SYSCFG0_SPEC>`"]
pub type SYSCFG0 = crate::Reg<syscfg0::SYSCFG0_SPEC>;
#[doc = "System Configuration 0"]
pub mod syscfg0;
#[doc = "SYSCFG1 (rw) register accessor: an alias for `Reg<SYSCFG1_SPEC>`"]
pub type SYSCFG1 = crate::Reg<syscfg1::SYSCFG1_SPEC>;
#[doc = "System Configuration 1"]
pub mod syscfg1;
#[doc = "SYSCFG2 (rw) register accessor: an alias for `Reg<SYSCFG2_SPEC>`"]
pub type SYSCFG2 = crate::Reg<syscfg2::SYSCFG2_SPEC>;
#[doc = "System Configuration 2"]
pub mod syscfg2;
