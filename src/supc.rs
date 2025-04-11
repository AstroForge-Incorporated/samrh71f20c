#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    smmr: Smmr,
    mr: Mr,
    _reserved3: [u8; 0x08],
    sr: Sr,
    _reserved4: [u8; 0x04],
    pwr: Pwr,
    _reserved5: [u8; 0xb4],
    sysc_wpmr: SyscWpmr,
}
impl RegisterBlock {
    #[doc = "0x00 - Supply Controller Control Register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - Supply Controller Supply Monitor Mode Register"]
    #[inline(always)]
    pub const fn smmr(&self) -> &Smmr {
        &self.smmr
    }
    #[doc = "0x08 - Supply Controller Mode Register"]
    #[inline(always)]
    pub const fn mr(&self) -> &Mr {
        &self.mr
    }
    #[doc = "0x14 - Supply Controller Status Register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x1c - Supply Controller Power Register"]
    #[inline(always)]
    pub const fn pwr(&self) -> &Pwr {
        &self.pwr
    }
    #[doc = "0xd4 - Write Protection Mode Register"]
    #[inline(always)]
    pub const fn sysc_wpmr(&self) -> &SyscWpmr {
        &self.sysc_wpmr
    }
}
#[doc = "CR (w) register accessor: Supply Controller Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Supply Controller Control Register"]
pub mod cr;
#[doc = "SMMR (rw) register accessor: Supply Controller Supply Monitor Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smmr`] module"]
#[doc(alias = "SMMR")]
pub type Smmr = crate::Reg<smmr::SmmrSpec>;
#[doc = "Supply Controller Supply Monitor Mode Register"]
pub mod smmr;
#[doc = "MR (rw) register accessor: Supply Controller Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mr`] module"]
#[doc(alias = "MR")]
pub type Mr = crate::Reg<mr::MrSpec>;
#[doc = "Supply Controller Mode Register"]
pub mod mr;
#[doc = "SR (r) register accessor: Supply Controller Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "Supply Controller Status Register"]
pub mod sr;
#[doc = "PWR (rw) register accessor: Supply Controller Power Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr`] module"]
#[doc(alias = "PWR")]
pub type Pwr = crate::Reg<pwr::PwrSpec>;
#[doc = "Supply Controller Power Register"]
pub mod pwr;
#[doc = "SYSC_WPMR (rw) register accessor: Write Protection Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysc_wpmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysc_wpmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysc_wpmr`] module"]
#[doc(alias = "SYSC_WPMR")]
pub type SyscWpmr = crate::Reg<sysc_wpmr::SyscWpmrSpec>;
#[doc = "Write Protection Mode Register"]
pub mod sysc_wpmr;
