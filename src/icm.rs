#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfg: Cfg,
    ctrl: Ctrl,
    sr: Sr,
    _reserved3: [u8; 0x04],
    ier: Ier,
    idr: Idr,
    imr: Imr,
    isr: Isr,
    uasr: Uasr,
    _reserved8: [u8; 0x0c],
    dscr: Dscr,
    hash: Hash,
    uihval0: Uihval0,
    uihval1: Uihval1,
    uihval2: Uihval2,
    uihval3: Uihval3,
    uihval4: Uihval4,
    uihval5: Uihval5,
    uihval6: Uihval6,
    uihval7: Uihval7,
    _reserved18: [u8; 0x8c],
    wpmr: Wpmr,
    wpsr: Wpsr,
}
impl RegisterBlock {
    #[doc = "0x00 - Configuration Register"]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x04 - Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x08 - Status Register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x10 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x14 - Interrupt Disable Register"]
    #[inline(always)]
    pub const fn idr(&self) -> &Idr {
        &self.idr
    }
    #[doc = "0x18 - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        &self.imr
    }
    #[doc = "0x1c - Interrupt Status Register"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x20 - Undefined Access Status Register"]
    #[inline(always)]
    pub const fn uasr(&self) -> &Uasr {
        &self.uasr
    }
    #[doc = "0x30 - Region Descriptor Area Start Address Register"]
    #[inline(always)]
    pub const fn dscr(&self) -> &Dscr {
        &self.dscr
    }
    #[doc = "0x34 - Region Hash Area Start Address Register"]
    #[inline(always)]
    pub const fn hash(&self) -> &Hash {
        &self.hash
    }
    #[doc = "0x38 - User Initial Hash Value 0 Register 0"]
    #[inline(always)]
    pub const fn uihval0(&self) -> &Uihval0 {
        &self.uihval0
    }
    #[doc = "0x3c - User Initial Hash Value 0 Register 1"]
    #[inline(always)]
    pub const fn uihval1(&self) -> &Uihval1 {
        &self.uihval1
    }
    #[doc = "0x40 - User Initial Hash Value 0 Register 2"]
    #[inline(always)]
    pub const fn uihval2(&self) -> &Uihval2 {
        &self.uihval2
    }
    #[doc = "0x44 - User Initial Hash Value 0 Register 3"]
    #[inline(always)]
    pub const fn uihval3(&self) -> &Uihval3 {
        &self.uihval3
    }
    #[doc = "0x48 - User Initial Hash Value 0 Register 4"]
    #[inline(always)]
    pub const fn uihval4(&self) -> &Uihval4 {
        &self.uihval4
    }
    #[doc = "0x4c - User Initial Hash Value 0 Register 5"]
    #[inline(always)]
    pub const fn uihval5(&self) -> &Uihval5 {
        &self.uihval5
    }
    #[doc = "0x50 - User Initial Hash Value 0 Register 6"]
    #[inline(always)]
    pub const fn uihval6(&self) -> &Uihval6 {
        &self.uihval6
    }
    #[doc = "0x54 - User Initial Hash Value 0 Register 7"]
    #[inline(always)]
    pub const fn uihval7(&self) -> &Uihval7 {
        &self.uihval7
    }
    #[doc = "0xe4 - ICM Write Protection Mode Register"]
    #[inline(always)]
    pub const fn wpmr(&self) -> &Wpmr {
        &self.wpmr
    }
    #[doc = "0xe8 - ICM Write Protection Status Register"]
    #[inline(always)]
    pub const fn wpsr(&self) -> &Wpsr {
        &self.wpsr
    }
}
#[doc = "CFG (rw) register accessor: Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg`] module"]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "Configuration Register"]
pub mod cfg;
#[doc = "CTRL (w) register accessor: Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "SR (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "IER (w) register accessor: Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`] module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR (w) register accessor: Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`] module"]
#[doc(alias = "IDR")]
pub type Idr = crate::Reg<idr::IdrSpec>;
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR (r) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`] module"]
#[doc(alias = "IMR")]
pub type Imr = crate::Reg<imr::ImrSpec>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "ISR (r) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`] module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "UASR (r) register accessor: Undefined Access Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uasr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uasr`] module"]
#[doc(alias = "UASR")]
pub type Uasr = crate::Reg<uasr::UasrSpec>;
#[doc = "Undefined Access Status Register"]
pub mod uasr;
#[doc = "DSCR (rw) register accessor: Region Descriptor Area Start Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dscr`] module"]
#[doc(alias = "DSCR")]
pub type Dscr = crate::Reg<dscr::DscrSpec>;
#[doc = "Region Descriptor Area Start Address Register"]
pub mod dscr;
#[doc = "HASH (rw) register accessor: Region Hash Area Start Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hash::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash`] module"]
#[doc(alias = "HASH")]
pub type Hash = crate::Reg<hash::HashSpec>;
#[doc = "Region Hash Area Start Address Register"]
pub mod hash;
#[doc = "UIHVAL0 (w) register accessor: User Initial Hash Value 0 Register 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uihval0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uihval0`] module"]
#[doc(alias = "UIHVAL0")]
pub type Uihval0 = crate::Reg<uihval0::Uihval0Spec>;
#[doc = "User Initial Hash Value 0 Register 0"]
pub mod uihval0;
#[doc = "UIHVAL1 (w) register accessor: User Initial Hash Value 0 Register 1\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uihval1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uihval1`] module"]
#[doc(alias = "UIHVAL1")]
pub type Uihval1 = crate::Reg<uihval1::Uihval1Spec>;
#[doc = "User Initial Hash Value 0 Register 1"]
pub mod uihval1;
#[doc = "UIHVAL2 (w) register accessor: User Initial Hash Value 0 Register 2\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uihval2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uihval2`] module"]
#[doc(alias = "UIHVAL2")]
pub type Uihval2 = crate::Reg<uihval2::Uihval2Spec>;
#[doc = "User Initial Hash Value 0 Register 2"]
pub mod uihval2;
#[doc = "UIHVAL3 (w) register accessor: User Initial Hash Value 0 Register 3\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uihval3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uihval3`] module"]
#[doc(alias = "UIHVAL3")]
pub type Uihval3 = crate::Reg<uihval3::Uihval3Spec>;
#[doc = "User Initial Hash Value 0 Register 3"]
pub mod uihval3;
#[doc = "UIHVAL4 (w) register accessor: User Initial Hash Value 0 Register 4\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uihval4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uihval4`] module"]
#[doc(alias = "UIHVAL4")]
pub type Uihval4 = crate::Reg<uihval4::Uihval4Spec>;
#[doc = "User Initial Hash Value 0 Register 4"]
pub mod uihval4;
#[doc = "UIHVAL5 (w) register accessor: User Initial Hash Value 0 Register 5\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uihval5::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uihval5`] module"]
#[doc(alias = "UIHVAL5")]
pub type Uihval5 = crate::Reg<uihval5::Uihval5Spec>;
#[doc = "User Initial Hash Value 0 Register 5"]
pub mod uihval5;
#[doc = "UIHVAL6 (w) register accessor: User Initial Hash Value 0 Register 6\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uihval6::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uihval6`] module"]
#[doc(alias = "UIHVAL6")]
pub type Uihval6 = crate::Reg<uihval6::Uihval6Spec>;
#[doc = "User Initial Hash Value 0 Register 6"]
pub mod uihval6;
#[doc = "UIHVAL7 (w) register accessor: User Initial Hash Value 0 Register 7\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uihval7::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uihval7`] module"]
#[doc(alias = "UIHVAL7")]
pub type Uihval7 = crate::Reg<uihval7::Uihval7Spec>;
#[doc = "User Initial Hash Value 0 Register 7"]
pub mod uihval7;
#[doc = "WPMR (rw) register accessor: ICM Write Protection Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpmr`] module"]
#[doc(alias = "WPMR")]
pub type Wpmr = crate::Reg<wpmr::WpmrSpec>;
#[doc = "ICM Write Protection Mode Register"]
pub mod wpmr;
#[doc = "WPSR (r) register accessor: ICM Write Protection Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpsr`] module"]
#[doc(alias = "WPSR")]
pub type Wpsr = crate::Reg<wpsr::WpsrSpec>;
#[doc = "ICM Write Protection Status Register"]
pub mod wpsr;
