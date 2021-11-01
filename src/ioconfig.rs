#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x80 - PORTA Pin Configuration Register"]
    pub porta: [crate::Reg<porta::PORTA_SPEC>; 32],
    #[doc = "0x80..0x100 - PORTB Pin Configuration Register"]
    pub portb: [crate::Reg<portb::PORTB_SPEC>; 32],
    _reserved2: [u8; 0x0efc],
    #[doc = "0xffc - Peripheral ID Register"]
    pub perid: crate::Reg<perid::PERID_SPEC>,
}
#[doc = "PORTA register accessor: an alias for `Reg<PORTA_SPEC>`"]
pub type PORTA = crate::Reg<porta::PORTA_SPEC>;
#[doc = "PORTA Pin Configuration Register"]
pub mod porta;
#[doc = "PORTB register accessor: an alias for `Reg<PORTB_SPEC>`"]
pub type PORTB = crate::Reg<portb::PORTB_SPEC>;
#[doc = "PORTB Pin Configuration Register"]
pub mod portb;
#[doc = "PERID register accessor: an alias for `Reg<PERID_SPEC>`"]
pub type PERID = crate::Reg<perid::PERID_SPEC>;
#[doc = "Peripheral ID Register"]
pub mod perid;
