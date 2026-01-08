#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USCI A1 Control Register 1"]
    pub uca1ctl1_spi: UCA1CTL1_SPI,
    #[doc = "0x01 - USCI A1 Control Register 0"]
    pub uca1ctl0_spi: UCA1CTL0_SPI,
    _reserved2: [u8; 0x04],
    #[doc = "0x06 - USCI A1 Baud Rate 0"]
    pub uca1br0_spi: UCA1BR0_SPI,
    #[doc = "0x07 - USCI A1 Baud Rate 1"]
    pub uca1br1_spi: UCA1BR1_SPI,
    _reserved4: [u8; 0x02],
    #[doc = "0x0a - USCI A1 Status Register"]
    pub uca1statw_spi: UCA1STATW_SPI,
    _reserved5: [u8; 0x01],
    #[doc = "0x0c - USCI A1 Receive Buffer"]
    pub uca1rxbuf_spi: UCA1RXBUF_SPI,
    #[doc = "0x0e - USCI A1 Transmit Buffer"]
    pub uca1txbuf_spi: UCA1TXBUF_SPI,
    _reserved7: [u8; 0x0a],
    #[doc = "0x1a - USCI A1 Interrupt Enable Register"]
    pub uca1ie_spi: UCA1IE_SPI,
    _reserved8: [u8; 0x01],
    #[doc = "0x1c - USCI A1 Interrupt Flags Register"]
    pub uca1ifg_spi: UCA1IFG_SPI,
    _reserved9: [u8; 0x01],
    #[doc = "0x1e - USCI A1 Interrupt Vector Register"]
    pub uca1iv_spi: UCA1IV_SPI,
}
#[doc = "UCA1CTL1_SPI (rw) register accessor: an alias for `Reg<UCA1CTL1_SPI_SPEC>`"]
pub type UCA1CTL1_SPI = crate::Reg<uca1ctl1_spi::UCA1CTL1_SPI_SPEC>;
#[doc = "USCI A1 Control Register 1"]
pub mod uca1ctl1_spi;
#[doc = "UCA1CTL0_SPI (rw) register accessor: an alias for `Reg<UCA1CTL0_SPI_SPEC>`"]
pub type UCA1CTL0_SPI = crate::Reg<uca1ctl0_spi::UCA1CTL0_SPI_SPEC>;
#[doc = "USCI A1 Control Register 0"]
pub mod uca1ctl0_spi;
#[doc = "UCA1BR0_SPI (rw) register accessor: an alias for `Reg<UCA1BR0_SPI_SPEC>`"]
pub type UCA1BR0_SPI = crate::Reg<uca1br0_spi::UCA1BR0_SPI_SPEC>;
#[doc = "USCI A1 Baud Rate 0"]
pub mod uca1br0_spi;
#[doc = "UCA1BR1_SPI (rw) register accessor: an alias for `Reg<UCA1BR1_SPI_SPEC>`"]
pub type UCA1BR1_SPI = crate::Reg<uca1br1_spi::UCA1BR1_SPI_SPEC>;
#[doc = "USCI A1 Baud Rate 1"]
pub mod uca1br1_spi;
#[doc = "UCA1STATW_SPI (rw) register accessor: an alias for `Reg<UCA1STATW_SPI_SPEC>`"]
pub type UCA1STATW_SPI = crate::Reg<uca1statw_spi::UCA1STATW_SPI_SPEC>;
#[doc = "USCI A1 Status Register"]
pub mod uca1statw_spi;
#[doc = "UCA1IE_SPI (rw) register accessor: an alias for `Reg<UCA1IE_SPI_SPEC>`"]
pub type UCA1IE_SPI = crate::Reg<uca1ie_spi::UCA1IE_SPI_SPEC>;
#[doc = "USCI A1 Interrupt Enable Register"]
pub mod uca1ie_spi;
#[doc = "UCA1IFG_SPI (rw) register accessor: an alias for `Reg<UCA1IFG_SPI_SPEC>`"]
pub type UCA1IFG_SPI = crate::Reg<uca1ifg_spi::UCA1IFG_SPI_SPEC>;
#[doc = "USCI A1 Interrupt Flags Register"]
pub mod uca1ifg_spi;
#[doc = "UCA1RXBUF_SPI (rw) register accessor: an alias for `Reg<UCA1RXBUF_SPI_SPEC>`"]
pub type UCA1RXBUF_SPI = crate::Reg<uca1rxbuf_spi::UCA1RXBUF_SPI_SPEC>;
#[doc = "USCI A1 Receive Buffer"]
pub mod uca1rxbuf_spi;
#[doc = "UCA1TXBUF_SPI (rw) register accessor: an alias for `Reg<UCA1TXBUF_SPI_SPEC>`"]
pub type UCA1TXBUF_SPI = crate::Reg<uca1txbuf_spi::UCA1TXBUF_SPI_SPEC>;
#[doc = "USCI A1 Transmit Buffer"]
pub mod uca1txbuf_spi;
#[doc = "UCA1IV_SPI (rw) register accessor: an alias for `Reg<UCA1IV_SPI_SPEC>`"]
pub type UCA1IV_SPI = crate::Reg<uca1iv_spi::UCA1IV_SPI_SPEC>;
#[doc = "USCI A1 Interrupt Vector Register"]
pub mod uca1iv_spi;
