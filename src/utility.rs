#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Synd Data 0 Register"]
    pub synd_data0: crate::Reg<synd_data0::SYND_DATA0_SPEC>,
    #[doc = "0x04 - Synd Data 1 Register"]
    pub synd_data1: crate::Reg<synd_data1::SYND_DATA1_SPEC>,
    #[doc = "0x08 - Synd Parity Register"]
    pub synd_synd: crate::Reg<synd_synd::SYND_SYND_SPEC>,
    #[doc = "0x0c - Synd 32 bit Encoded Syndrome"]
    pub synd_enc_32: crate::Reg<synd_enc_32::SYND_ENC_32_SPEC>,
    #[doc = "0x10 - Synd 32 bit Corrected Data"]
    pub synd_check_32_data: crate::Reg<synd_check_32_data::SYND_CHECK_32_DATA_SPEC>,
    #[doc = "0x14 - Synd 32 bit Corrected Syndrome and Status"]
    pub synd_check_32_synd: crate::Reg<synd_check_32_synd::SYND_CHECK_32_SYND_SPEC>,
    #[doc = "0x18 - Synd 64 bit Encoded Syndrome"]
    pub synd_enc_64: crate::Reg<synd_enc_64::SYND_ENC_64_SPEC>,
    #[doc = "0x1c - Synd 64 bit Corrected Data 0"]
    pub synd_check_64_data0: crate::Reg<synd_check_64_data0::SYND_CHECK_64_DATA0_SPEC>,
    #[doc = "0x20 - Synd 64 bit Corrected Data 1"]
    pub synd_check_64_data1: crate::Reg<synd_check_64_data1::SYND_CHECK_64_DATA1_SPEC>,
    #[doc = "0x24 - Synd 64 bit Corrected Parity and Status"]
    pub synd_check_64_synd: crate::Reg<synd_check_64_synd::SYND_CHECK_64_SYND_SPEC>,
    #[doc = "0x28 - Synd 32/52 bit Encoded Syndrome"]
    pub synd_enc_32_52: crate::Reg<synd_enc_32_52::SYND_ENC_32_52_SPEC>,
    #[doc = "0x2c - Synd 32/52 bit Corrected Data"]
    pub synd_check_32_52_data: crate::Reg<synd_check_32_52_data::SYND_CHECK_32_52_DATA_SPEC>,
    #[doc = "0x30 - Synd 32/52 bit Corrected Syndrome and Status"]
    pub synd_check_32_52_synd: crate::Reg<synd_check_32_52_synd::SYND_CHECK_32_52_SYND_SPEC>,
    _reserved13: [u8; 0x0fc8],
    #[doc = "0xffc - Peripheral ID Register"]
    pub perid: crate::Reg<perid::PERID_SPEC>,
}
#[doc = "SYND_DATA0 register accessor: an alias for `Reg<SYND_DATA0_SPEC>`"]
pub type SYND_DATA0 = crate::Reg<synd_data0::SYND_DATA0_SPEC>;
#[doc = "Synd Data 0 Register"]
pub mod synd_data0;
#[doc = "SYND_DATA1 register accessor: an alias for `Reg<SYND_DATA1_SPEC>`"]
pub type SYND_DATA1 = crate::Reg<synd_data1::SYND_DATA1_SPEC>;
#[doc = "Synd Data 1 Register"]
pub mod synd_data1;
#[doc = "SYND_SYND register accessor: an alias for `Reg<SYND_SYND_SPEC>`"]
pub type SYND_SYND = crate::Reg<synd_synd::SYND_SYND_SPEC>;
#[doc = "Synd Parity Register"]
pub mod synd_synd;
#[doc = "SYND_ENC_32 register accessor: an alias for `Reg<SYND_ENC_32_SPEC>`"]
pub type SYND_ENC_32 = crate::Reg<synd_enc_32::SYND_ENC_32_SPEC>;
#[doc = "Synd 32 bit Encoded Syndrome"]
pub mod synd_enc_32;
#[doc = "SYND_CHECK_32_DATA register accessor: an alias for `Reg<SYND_CHECK_32_DATA_SPEC>`"]
pub type SYND_CHECK_32_DATA = crate::Reg<synd_check_32_data::SYND_CHECK_32_DATA_SPEC>;
#[doc = "Synd 32 bit Corrected Data"]
pub mod synd_check_32_data;
#[doc = "SYND_CHECK_32_SYND register accessor: an alias for `Reg<SYND_CHECK_32_SYND_SPEC>`"]
pub type SYND_CHECK_32_SYND = crate::Reg<synd_check_32_synd::SYND_CHECK_32_SYND_SPEC>;
#[doc = "Synd 32 bit Corrected Syndrome and Status"]
pub mod synd_check_32_synd;
#[doc = "SYND_ENC_64 register accessor: an alias for `Reg<SYND_ENC_64_SPEC>`"]
pub type SYND_ENC_64 = crate::Reg<synd_enc_64::SYND_ENC_64_SPEC>;
#[doc = "Synd 64 bit Encoded Syndrome"]
pub mod synd_enc_64;
#[doc = "SYND_CHECK_64_DATA0 register accessor: an alias for `Reg<SYND_CHECK_64_DATA0_SPEC>`"]
pub type SYND_CHECK_64_DATA0 = crate::Reg<synd_check_64_data0::SYND_CHECK_64_DATA0_SPEC>;
#[doc = "Synd 64 bit Corrected Data 0"]
pub mod synd_check_64_data0;
#[doc = "SYND_CHECK_64_DATA1 register accessor: an alias for `Reg<SYND_CHECK_64_DATA1_SPEC>`"]
pub type SYND_CHECK_64_DATA1 = crate::Reg<synd_check_64_data1::SYND_CHECK_64_DATA1_SPEC>;
#[doc = "Synd 64 bit Corrected Data 1"]
pub mod synd_check_64_data1;
#[doc = "SYND_CHECK_64_SYND register accessor: an alias for `Reg<SYND_CHECK_64_SYND_SPEC>`"]
pub type SYND_CHECK_64_SYND = crate::Reg<synd_check_64_synd::SYND_CHECK_64_SYND_SPEC>;
#[doc = "Synd 64 bit Corrected Parity and Status"]
pub mod synd_check_64_synd;
#[doc = "SYND_ENC_32_52 register accessor: an alias for `Reg<SYND_ENC_32_52_SPEC>`"]
pub type SYND_ENC_32_52 = crate::Reg<synd_enc_32_52::SYND_ENC_32_52_SPEC>;
#[doc = "Synd 32/52 bit Encoded Syndrome"]
pub mod synd_enc_32_52;
#[doc = "SYND_CHECK_32_52_DATA register accessor: an alias for `Reg<SYND_CHECK_32_52_DATA_SPEC>`"]
pub type SYND_CHECK_32_52_DATA = crate::Reg<synd_check_32_52_data::SYND_CHECK_32_52_DATA_SPEC>;
#[doc = "Synd 32/52 bit Corrected Data"]
pub mod synd_check_32_52_data;
#[doc = "SYND_CHECK_32_52_SYND register accessor: an alias for `Reg<SYND_CHECK_32_52_SYND_SPEC>`"]
pub type SYND_CHECK_32_52_SYND = crate::Reg<synd_check_32_52_synd::SYND_CHECK_32_52_SYND_SPEC>;
#[doc = "Synd 32/52 bit Corrected Syndrome and Status"]
pub mod synd_check_32_52_synd;
#[doc = "PERID register accessor: an alias for `Reg<PERID_SPEC>`"]
pub type PERID = crate::Reg<perid::PERID_SPEC>;
#[doc = "Peripheral ID Register"]
pub mod perid;
