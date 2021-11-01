#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Reset Status"]
    pub rst_stat: crate::Reg<rst_stat::RST_STAT_SPEC>,
    #[doc = "0x04 - ROM Reset Control"]
    pub rst_cntl_rom: crate::Reg<rst_cntl_rom::RST_CNTL_ROM_SPEC>,
    #[doc = "0x08 - RAM Reset Control"]
    pub rst_cntl_ram: crate::Reg<rst_cntl_ram::RST_CNTL_RAM_SPEC>,
    #[doc = "0x0c - ROM Protection Configuration"]
    pub rom_prot: crate::Reg<rom_prot::ROM_PROT_SPEC>,
    #[doc = "0x10 - ROM Scrub Period Configuration"]
    pub rom_scrub: crate::Reg<rom_scrub::ROM_SCRUB_SPEC>,
    #[doc = "0x14 - RAM Scrub Period Configuration"]
    pub ram_scrub: crate::Reg<ram_scrub::RAM_SCRUB_SPEC>,
    #[doc = "0x18 - ROM Trap Address"]
    pub rom_trap_addr: crate::Reg<rom_trap_addr::ROM_TRAP_ADDR_SPEC>,
    #[doc = "0x1c - ROM Trap Syndrome"]
    pub rom_trap_synd: crate::Reg<rom_trap_synd::ROM_TRAP_SYND_SPEC>,
    #[doc = "0x20 - RAM Trap Address"]
    pub ram_trap_addr: crate::Reg<ram_trap_addr::RAM_TRAP_ADDR_SPEC>,
    #[doc = "0x24 - RAM Trap Syndrome"]
    pub ram_trap_synd: crate::Reg<ram_trap_synd::RAM_TRAP_SYND_SPEC>,
    #[doc = "0x28 - Enable EDAC Error Interrupt Register"]
    pub irq_enb: crate::Reg<irq_enb::IRQ_ENB_SPEC>,
    #[doc = "0x2c - Raw EDAC Error Interrupt Status"]
    pub irq_raw: crate::Reg<irq_raw::IRQ_RAW_SPEC>,
    #[doc = "0x30 - Enabled EDAC Error Interrupt Status"]
    pub irq_end: crate::Reg<irq_end::IRQ_END_SPEC>,
    #[doc = "0x34 - Clear EDAC Error Interrupt Status"]
    pub irq_clr: crate::Reg<irq_clr::IRQ_CLR_SPEC>,
    #[doc = "0x38 - Count of RAM EDAC Single Bit Errors"]
    pub ram_sbe: crate::Reg<ram_sbe::RAM_SBE_SPEC>,
    #[doc = "0x3c - Count of RAM EDAC Multi Bit Errors"]
    pub ram_mbe: crate::Reg<ram_mbe::RAM_MBE_SPEC>,
    #[doc = "0x40 - Count of ROM EDAC Single Bit Errors"]
    pub rom_sbe: crate::Reg<rom_sbe::ROM_SBE_SPEC>,
    #[doc = "0x44 - Count of ROM EDAC Multi Bit Errors"]
    pub rom_mbe: crate::Reg<rom_mbe::ROM_MBE_SPEC>,
    #[doc = "0x48 - IO Configuration Clock Divider Register"]
    pub ioconfig_clkdiv0: crate::Reg<ioconfig_clkdiv0::IOCONFIG_CLKDIV0_SPEC>,
    #[doc = "0x4c - IO Configuration Clock Divider Register"]
    pub ioconfig_clkdiv1: crate::Reg<ioconfig_clkdiv::IOCONFIG_CLKDIV_SPEC>,
    #[doc = "0x50 - IO Configuration Clock Divider Register"]
    pub ioconfig_clkdiv2: crate::Reg<ioconfig_clkdiv::IOCONFIG_CLKDIV_SPEC>,
    #[doc = "0x54 - IO Configuration Clock Divider Register"]
    pub ioconfig_clkdiv3: crate::Reg<ioconfig_clkdiv::IOCONFIG_CLKDIV_SPEC>,
    #[doc = "0x58 - IO Configuration Clock Divider Register"]
    pub ioconfig_clkdiv4: crate::Reg<ioconfig_clkdiv::IOCONFIG_CLKDIV_SPEC>,
    #[doc = "0x5c - IO Configuration Clock Divider Register"]
    pub ioconfig_clkdiv5: crate::Reg<ioconfig_clkdiv::IOCONFIG_CLKDIV_SPEC>,
    #[doc = "0x60 - IO Configuration Clock Divider Register"]
    pub ioconfig_clkdiv6: crate::Reg<ioconfig_clkdiv::IOCONFIG_CLKDIV_SPEC>,
    #[doc = "0x64 - IO Configuration Clock Divider Register"]
    pub ioconfig_clkdiv7: crate::Reg<ioconfig_clkdiv::IOCONFIG_CLKDIV_SPEC>,
    #[doc = "0x68 - ROM BOOT Retry count"]
    pub rom_retries: crate::Reg<rom_retries::ROM_RETRIES_SPEC>,
    #[doc = "0x6c - Register Refresh Control"]
    pub refresh_config: crate::Reg<refresh_config::REFRESH_CONFIG_SPEC>,
    #[doc = "0x70 - TIM Reset Control"]
    pub tim_reset: crate::Reg<tim_reset::TIM_RESET_SPEC>,
    #[doc = "0x74 - TIM Enable Control"]
    pub tim_clk_enable: crate::Reg<tim_clk_enable::TIM_CLK_ENABLE_SPEC>,
    #[doc = "0x78 - Peripheral Reset Control"]
    pub peripheral_reset: crate::Reg<peripheral_reset::PERIPHERAL_RESET_SPEC>,
    #[doc = "0x7c - Peripheral Enable Control"]
    pub peripheral_clk_enable: crate::Reg<peripheral_clk_enable::PERIPHERAL_CLK_ENABLE_SPEC>,
    #[doc = "0x80 - Lockup Reset Configuration"]
    pub lockup_reset: crate::Reg<lockup_reset::LOCKUP_RESET_SPEC>,
    _reserved33: [u8; 0x0f6c],
    #[doc = "0xff0 - EFuse Config Register"]
    pub ef_config: crate::Reg<ef_config::EF_CONFIG_SPEC>,
    #[doc = "0xff4 - EFuse ID Register"]
    pub ef_id: crate::Reg<ef_id::EF_ID_SPEC>,
    #[doc = "0xff8 - Processor ID Register"]
    pub procid: crate::Reg<procid::PROCID_SPEC>,
    #[doc = "0xffc - Peripheral ID Register"]
    pub perid: crate::Reg<perid::PERID_SPEC>,
}
#[doc = "RST_STAT register accessor: an alias for `Reg<RST_STAT_SPEC>`"]
pub type RST_STAT = crate::Reg<rst_stat::RST_STAT_SPEC>;
#[doc = "System Reset Status"]
pub mod rst_stat;
#[doc = "RST_CNTL_ROM register accessor: an alias for `Reg<RST_CNTL_ROM_SPEC>`"]
pub type RST_CNTL_ROM = crate::Reg<rst_cntl_rom::RST_CNTL_ROM_SPEC>;
#[doc = "ROM Reset Control"]
pub mod rst_cntl_rom;
#[doc = "RST_CNTL_RAM register accessor: an alias for `Reg<RST_CNTL_RAM_SPEC>`"]
pub type RST_CNTL_RAM = crate::Reg<rst_cntl_ram::RST_CNTL_RAM_SPEC>;
#[doc = "RAM Reset Control"]
pub mod rst_cntl_ram;
#[doc = "ROM_PROT register accessor: an alias for `Reg<ROM_PROT_SPEC>`"]
pub type ROM_PROT = crate::Reg<rom_prot::ROM_PROT_SPEC>;
#[doc = "ROM Protection Configuration"]
pub mod rom_prot;
#[doc = "ROM_SCRUB register accessor: an alias for `Reg<ROM_SCRUB_SPEC>`"]
pub type ROM_SCRUB = crate::Reg<rom_scrub::ROM_SCRUB_SPEC>;
#[doc = "ROM Scrub Period Configuration"]
pub mod rom_scrub;
#[doc = "RAM_SCRUB register accessor: an alias for `Reg<RAM_SCRUB_SPEC>`"]
pub type RAM_SCRUB = crate::Reg<ram_scrub::RAM_SCRUB_SPEC>;
#[doc = "RAM Scrub Period Configuration"]
pub mod ram_scrub;
#[doc = "ROM_TRAP_ADDR register accessor: an alias for `Reg<ROM_TRAP_ADDR_SPEC>`"]
pub type ROM_TRAP_ADDR = crate::Reg<rom_trap_addr::ROM_TRAP_ADDR_SPEC>;
#[doc = "ROM Trap Address"]
pub mod rom_trap_addr;
#[doc = "ROM_TRAP_SYND register accessor: an alias for `Reg<ROM_TRAP_SYND_SPEC>`"]
pub type ROM_TRAP_SYND = crate::Reg<rom_trap_synd::ROM_TRAP_SYND_SPEC>;
#[doc = "ROM Trap Syndrome"]
pub mod rom_trap_synd;
#[doc = "RAM_TRAP_ADDR register accessor: an alias for `Reg<RAM_TRAP_ADDR_SPEC>`"]
pub type RAM_TRAP_ADDR = crate::Reg<ram_trap_addr::RAM_TRAP_ADDR_SPEC>;
#[doc = "RAM Trap Address"]
pub mod ram_trap_addr;
#[doc = "RAM_TRAP_SYND register accessor: an alias for `Reg<RAM_TRAP_SYND_SPEC>`"]
pub type RAM_TRAP_SYND = crate::Reg<ram_trap_synd::RAM_TRAP_SYND_SPEC>;
#[doc = "RAM Trap Syndrome"]
pub mod ram_trap_synd;
#[doc = "IRQ_ENB register accessor: an alias for `Reg<IRQ_ENB_SPEC>`"]
pub type IRQ_ENB = crate::Reg<irq_enb::IRQ_ENB_SPEC>;
#[doc = "Enable EDAC Error Interrupt Register"]
pub mod irq_enb;
#[doc = "IRQ_RAW register accessor: an alias for `Reg<IRQ_RAW_SPEC>`"]
pub type IRQ_RAW = crate::Reg<irq_raw::IRQ_RAW_SPEC>;
#[doc = "Raw EDAC Error Interrupt Status"]
pub mod irq_raw;
#[doc = "IRQ_END register accessor: an alias for `Reg<IRQ_END_SPEC>`"]
pub type IRQ_END = crate::Reg<irq_end::IRQ_END_SPEC>;
#[doc = "Enabled EDAC Error Interrupt Status"]
pub mod irq_end;
#[doc = "IRQ_CLR register accessor: an alias for `Reg<IRQ_CLR_SPEC>`"]
pub type IRQ_CLR = crate::Reg<irq_clr::IRQ_CLR_SPEC>;
#[doc = "Clear EDAC Error Interrupt Status"]
pub mod irq_clr;
#[doc = "RAM_SBE register accessor: an alias for `Reg<RAM_SBE_SPEC>`"]
pub type RAM_SBE = crate::Reg<ram_sbe::RAM_SBE_SPEC>;
#[doc = "Count of RAM EDAC Single Bit Errors"]
pub mod ram_sbe;
#[doc = "RAM_MBE register accessor: an alias for `Reg<RAM_MBE_SPEC>`"]
pub type RAM_MBE = crate::Reg<ram_mbe::RAM_MBE_SPEC>;
#[doc = "Count of RAM EDAC Multi Bit Errors"]
pub mod ram_mbe;
#[doc = "ROM_SBE register accessor: an alias for `Reg<ROM_SBE_SPEC>`"]
pub type ROM_SBE = crate::Reg<rom_sbe::ROM_SBE_SPEC>;
#[doc = "Count of ROM EDAC Single Bit Errors"]
pub mod rom_sbe;
#[doc = "ROM_MBE register accessor: an alias for `Reg<ROM_MBE_SPEC>`"]
pub type ROM_MBE = crate::Reg<rom_mbe::ROM_MBE_SPEC>;
#[doc = "Count of ROM EDAC Multi Bit Errors"]
pub mod rom_mbe;
#[doc = "IOCONFIG_CLKDIV0 register accessor: an alias for `Reg<IOCONFIG_CLKDIV0_SPEC>`"]
pub type IOCONFIG_CLKDIV0 = crate::Reg<ioconfig_clkdiv0::IOCONFIG_CLKDIV0_SPEC>;
#[doc = "IO Configuration Clock Divider Register"]
pub mod ioconfig_clkdiv0;
#[doc = "IOCONFIG_CLKDIV register accessor: an alias for `Reg<IOCONFIG_CLKDIV_SPEC>`"]
pub type IOCONFIG_CLKDIV = crate::Reg<ioconfig_clkdiv::IOCONFIG_CLKDIV_SPEC>;
#[doc = "IO Configuration Clock Divider Register"]
pub mod ioconfig_clkdiv;
#[doc = "ROM_RETRIES register accessor: an alias for `Reg<ROM_RETRIES_SPEC>`"]
pub type ROM_RETRIES = crate::Reg<rom_retries::ROM_RETRIES_SPEC>;
#[doc = "ROM BOOT Retry count"]
pub mod rom_retries;
#[doc = "REFRESH_CONFIG register accessor: an alias for `Reg<REFRESH_CONFIG_SPEC>`"]
pub type REFRESH_CONFIG = crate::Reg<refresh_config::REFRESH_CONFIG_SPEC>;
#[doc = "Register Refresh Control"]
pub mod refresh_config;
#[doc = "TIM_RESET register accessor: an alias for `Reg<TIM_RESET_SPEC>`"]
pub type TIM_RESET = crate::Reg<tim_reset::TIM_RESET_SPEC>;
#[doc = "TIM Reset Control"]
pub mod tim_reset;
#[doc = "TIM_CLK_ENABLE register accessor: an alias for `Reg<TIM_CLK_ENABLE_SPEC>`"]
pub type TIM_CLK_ENABLE = crate::Reg<tim_clk_enable::TIM_CLK_ENABLE_SPEC>;
#[doc = "TIM Enable Control"]
pub mod tim_clk_enable;
#[doc = "PERIPHERAL_RESET register accessor: an alias for `Reg<PERIPHERAL_RESET_SPEC>`"]
pub type PERIPHERAL_RESET = crate::Reg<peripheral_reset::PERIPHERAL_RESET_SPEC>;
#[doc = "Peripheral Reset Control"]
pub mod peripheral_reset;
#[doc = "PERIPHERAL_CLK_ENABLE register accessor: an alias for `Reg<PERIPHERAL_CLK_ENABLE_SPEC>`"]
pub type PERIPHERAL_CLK_ENABLE = crate::Reg<peripheral_clk_enable::PERIPHERAL_CLK_ENABLE_SPEC>;
#[doc = "Peripheral Enable Control"]
pub mod peripheral_clk_enable;
#[doc = "LOCKUP_RESET register accessor: an alias for `Reg<LOCKUP_RESET_SPEC>`"]
pub type LOCKUP_RESET = crate::Reg<lockup_reset::LOCKUP_RESET_SPEC>;
#[doc = "Lockup Reset Configuration"]
pub mod lockup_reset;
#[doc = "EF_CONFIG register accessor: an alias for `Reg<EF_CONFIG_SPEC>`"]
pub type EF_CONFIG = crate::Reg<ef_config::EF_CONFIG_SPEC>;
#[doc = "EFuse Config Register"]
pub mod ef_config;
#[doc = "EF_ID register accessor: an alias for `Reg<EF_ID_SPEC>`"]
pub type EF_ID = crate::Reg<ef_id::EF_ID_SPEC>;
#[doc = "EFuse ID Register"]
pub mod ef_id;
#[doc = "PROCID register accessor: an alias for `Reg<PROCID_SPEC>`"]
pub type PROCID = crate::Reg<procid::PROCID_SPEC>;
#[doc = "Processor ID Register"]
pub mod procid;
#[doc = "PERID register accessor: an alias for `Reg<PERID_SPEC>`"]
pub type PERID = crate::Reg<perid::PERID_SPEC>;
#[doc = "Peripheral ID Register"]
pub mod perid;
