#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x80 - PORTA Interrupt Redirect Selection"]
    pub porta: [crate::Reg<porta::PORTA_SPEC>; 32],
    #[doc = "0x80..0x100 - PORTB Interrupt Redirect Selection"]
    pub portb: [crate::Reg<portb::PORTB_SPEC>; 32],
    #[doc = "0x100..0x180 - TIM Interrupt Redirect Selection"]
    pub tim: [crate::Reg<tim::TIM_SPEC>; 32],
    #[doc = "0x180..0x190 - UART Interrupt Redirect Selection"]
    pub uart: [crate::Reg<uart::UART_SPEC>; 4],
    #[doc = "0x190..0x1a0 - SPI Interrupt Redirect Selection"]
    pub spi: [crate::Reg<spi::SPI_SPEC>; 4],
    #[doc = "0x1a0..0x1b0 - Master I2C Interrupt Redirect Selection"]
    pub i2c_ms: [crate::Reg<i2c_ms::I2C_MS_SPEC>; 4],
    #[doc = "0x1b0..0x1c0 - Slave I2C Interrupt Redirect Selection"]
    pub i2c_sl: [crate::Reg<i2c_sl::I2C_SL_SPEC>; 4],
    #[doc = "0x1c0 - Internal Memory RAM SBE Interrupt Redirect Selection"]
    pub int_ram_sbe: crate::Reg<int_ram_sbe::INT_RAM_SBE_SPEC>,
    #[doc = "0x1c4 - Internal Memory RAM MBE Interrupt Redirect Selection"]
    pub int_ram_mbe: crate::Reg<int_ram_mbe::INT_RAM_MBE_SPEC>,
    #[doc = "0x1c8 - Internal Memory ROM SBE Interrupt Redirect Selection"]
    pub int_rom_sbe: crate::Reg<int_rom_sbe::INT_ROM_SBE_SPEC>,
    #[doc = "0x1cc - Internal Memory ROM MBE Interrupt Redirect Selection"]
    pub int_rom_mbe: crate::Reg<int_rom_mbe::INT_ROM_MBE_SPEC>,
    #[doc = "0x1d0 - Processor TXEV Interrupt Redirect Selection"]
    pub txev: crate::Reg<txev::TXEV_SPEC>,
    _reserved12: [u8; 0x062c],
    #[doc = "0x800..0x880 - Interrupt Status Register"]
    pub irqs: [crate::Reg<irqs::IRQS_SPEC>; 32],
    _reserved13: [u8; 0x68],
    #[doc = "0x8e8 - EDBGRQ Status Register"]
    pub edbgrq: crate::Reg<edbgrq::EDBGRQ_SPEC>,
    #[doc = "0x8ec - MERESET Status Register"]
    pub mereset: crate::Reg<mereset::MERESET_SPEC>,
    #[doc = "0x8f0 - WATCHDOG Status Register"]
    pub watchdog: crate::Reg<watchdog::WATCHDOG_SPEC>,
    #[doc = "0x8f4 - RXEV Status Register"]
    pub rxev: crate::Reg<rxev::RXEV_SPEC>,
    #[doc = "0x8f8 - NMI Status Register"]
    pub nmi: crate::Reg<nmi::NMI_SPEC>,
    _reserved18: [u8; 0x0700],
    #[doc = "0xffc - Peripheral ID Register"]
    pub perid: crate::Reg<perid::PERID_SPEC>,
}
#[doc = "INT_RAM_SBE register accessor: an alias for `Reg<INT_RAM_SBE_SPEC>`"]
pub type INT_RAM_SBE = crate::Reg<int_ram_sbe::INT_RAM_SBE_SPEC>;
#[doc = "Internal Memory RAM SBE Interrupt Redirect Selection"]
pub mod int_ram_sbe;
#[doc = "PORTA register accessor: an alias for `Reg<PORTA_SPEC>`"]
pub type PORTA = crate::Reg<porta::PORTA_SPEC>;
#[doc = "PORTA Interrupt Redirect Selection"]
pub mod porta;
#[doc = "PORTB register accessor: an alias for `Reg<PORTB_SPEC>`"]
pub type PORTB = crate::Reg<portb::PORTB_SPEC>;
#[doc = "PORTB Interrupt Redirect Selection"]
pub mod portb;
#[doc = "TIM register accessor: an alias for `Reg<TIM_SPEC>`"]
pub type TIM = crate::Reg<tim::TIM_SPEC>;
#[doc = "TIM Interrupt Redirect Selection"]
pub mod tim;
#[doc = "UART register accessor: an alias for `Reg<UART_SPEC>`"]
pub type UART = crate::Reg<uart::UART_SPEC>;
#[doc = "UART Interrupt Redirect Selection"]
pub mod uart;
#[doc = "SPI register accessor: an alias for `Reg<SPI_SPEC>`"]
pub type SPI = crate::Reg<spi::SPI_SPEC>;
#[doc = "SPI Interrupt Redirect Selection"]
pub mod spi;
#[doc = "I2C_MS register accessor: an alias for `Reg<I2C_MS_SPEC>`"]
pub type I2C_MS = crate::Reg<i2c_ms::I2C_MS_SPEC>;
#[doc = "Master I2C Interrupt Redirect Selection"]
pub mod i2c_ms;
#[doc = "I2C_SL register accessor: an alias for `Reg<I2C_SL_SPEC>`"]
pub type I2C_SL = crate::Reg<i2c_sl::I2C_SL_SPEC>;
#[doc = "Slave I2C Interrupt Redirect Selection"]
pub mod i2c_sl;
#[doc = "INT_RAM_MBE register accessor: an alias for `Reg<INT_RAM_MBE_SPEC>`"]
pub type INT_RAM_MBE = crate::Reg<int_ram_mbe::INT_RAM_MBE_SPEC>;
#[doc = "Internal Memory RAM MBE Interrupt Redirect Selection"]
pub mod int_ram_mbe;
#[doc = "INT_ROM_SBE register accessor: an alias for `Reg<INT_ROM_SBE_SPEC>`"]
pub type INT_ROM_SBE = crate::Reg<int_rom_sbe::INT_ROM_SBE_SPEC>;
#[doc = "Internal Memory ROM SBE Interrupt Redirect Selection"]
pub mod int_rom_sbe;
#[doc = "INT_ROM_MBE register accessor: an alias for `Reg<INT_ROM_MBE_SPEC>`"]
pub type INT_ROM_MBE = crate::Reg<int_rom_mbe::INT_ROM_MBE_SPEC>;
#[doc = "Internal Memory ROM MBE Interrupt Redirect Selection"]
pub mod int_rom_mbe;
#[doc = "TXEV register accessor: an alias for `Reg<TXEV_SPEC>`"]
pub type TXEV = crate::Reg<txev::TXEV_SPEC>;
#[doc = "Processor TXEV Interrupt Redirect Selection"]
pub mod txev;
#[doc = "NMI register accessor: an alias for `Reg<NMI_SPEC>`"]
pub type NMI = crate::Reg<nmi::NMI_SPEC>;
#[doc = "NMI Status Register"]
pub mod nmi;
#[doc = "RXEV register accessor: an alias for `Reg<RXEV_SPEC>`"]
pub type RXEV = crate::Reg<rxev::RXEV_SPEC>;
#[doc = "RXEV Status Register"]
pub mod rxev;
#[doc = "WATCHDOG register accessor: an alias for `Reg<WATCHDOG_SPEC>`"]
pub type WATCHDOG = crate::Reg<watchdog::WATCHDOG_SPEC>;
#[doc = "WATCHDOG Status Register"]
pub mod watchdog;
#[doc = "MERESET register accessor: an alias for `Reg<MERESET_SPEC>`"]
pub type MERESET = crate::Reg<mereset::MERESET_SPEC>;
#[doc = "MERESET Status Register"]
pub mod mereset;
#[doc = "EDBGRQ register accessor: an alias for `Reg<EDBGRQ_SPEC>`"]
pub type EDBGRQ = crate::Reg<edbgrq::EDBGRQ_SPEC>;
#[doc = "EDBGRQ Status Register"]
pub mod edbgrq;
#[doc = "IRQS register accessor: an alias for `Reg<IRQS_SPEC>`"]
pub type IRQS = crate::Reg<irqs::IRQS_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod irqs;
#[doc = "PERID register accessor: an alias for `Reg<PERID_SPEC>`"]
pub type PERID = crate::Reg<perid::PERID_SPEC>;
#[doc = "Peripheral ID Register"]
pub mod perid;
