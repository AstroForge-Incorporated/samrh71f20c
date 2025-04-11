#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    hsmc_cs: [HsmcCs; 6],
    _reserved1: [u8; 0x10],
    wpmr: Wpmr,
    wpsr: Wpsr,
}
impl RegisterBlock {
    #[doc = "0x00..0x60 - HSMC_CS\\[%s\\]"]
    #[inline(always)]
    pub const fn hsmc_cs(&self, n: usize) -> &HsmcCs {
        &self.hsmc_cs[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x60 - HSMC_CS\\[%s\\]"]
    #[inline(always)]
    pub fn hsmc_cs_iter(&self) -> impl Iterator<Item = &HsmcCs> {
        self.hsmc_cs.iter()
    }
    #[doc = "0x70 - HSMC Write Protection Mode Register"]
    #[inline(always)]
    pub const fn wpmr(&self) -> &Wpmr {
        &self.wpmr
    }
    #[doc = "0x74 - HSMC Write Protection Status Register"]
    #[inline(always)]
    pub const fn wpsr(&self) -> &Wpsr {
        &self.wpsr
    }
}
#[doc = "HSMC_CS\\[%s\\]"]
pub use self::hsmc_cs::HsmcCs;
#[doc = r"Cluster"]
#[doc = "HSMC_CS\\[%s\\]"]
pub mod hsmc_cs;
#[doc = "WPMR (rw) register accessor: HSMC Write Protection Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpmr`] module"]
#[doc(alias = "WPMR")]
pub type Wpmr = crate::Reg<wpmr::WpmrSpec>;
#[doc = "HSMC Write Protection Mode Register"]
pub mod wpmr;
#[doc = "WPSR (r) register accessor: HSMC Write Protection Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpsr`] module"]
#[doc(alias = "WPSR")]
pub type Wpsr = crate::Reg<wpsr::WpsrSpec>;
#[doc = "HSMC Write Protection Status Register"]
pub mod wpsr;
