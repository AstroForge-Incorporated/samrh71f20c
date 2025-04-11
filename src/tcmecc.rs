#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    testcb1: Testcb1,
    _reserved2: [u8; 0x04],
    sr: Sr,
    ier: Ier,
    idr: Idr,
    imr: Imr,
    failar: Failar,
    failard: Failard,
}
impl RegisterBlock {
    #[doc = "0x00 - TCMECC Control Register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - TCMECC Test mode register 1"]
    #[inline(always)]
    pub const fn testcb1(&self) -> &Testcb1 {
        &self.testcb1
    }
    #[doc = "0x0c - TCMECC Status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x10 - TCMECC Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x14 - TCMECC Interrupt Disable Register"]
    #[inline(always)]
    pub const fn idr(&self) -> &Idr {
        &self.idr
    }
    #[doc = "0x18 - TCMECC Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        &self.imr
    }
    #[doc = "0x1c - TCMECC Fail address register"]
    #[inline(always)]
    pub const fn failar(&self) -> &Failar {
        &self.failar
    }
    #[doc = "0x20 - TCMECC Fail address register data"]
    #[inline(always)]
    pub const fn failard(&self) -> &Failard {
        &self.failard
    }
}
#[doc = "CR (rw) register accessor: TCMECC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "TCMECC Control Register"]
pub mod cr;
#[doc = "TESTCB1 (rw) register accessor: TCMECC Test mode register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`testcb1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`testcb1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@testcb1`] module"]
#[doc(alias = "TESTCB1")]
pub type Testcb1 = crate::Reg<testcb1::Testcb1Spec>;
#[doc = "TCMECC Test mode register 1"]
pub mod testcb1;
#[doc = "SR (r) register accessor: TCMECC Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "TCMECC Status register"]
pub mod sr;
#[doc = "IER (w) register accessor: TCMECC Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`] module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "TCMECC Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR (w) register accessor: TCMECC Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`] module"]
#[doc(alias = "IDR")]
pub type Idr = crate::Reg<idr::IdrSpec>;
#[doc = "TCMECC Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR (r) register accessor: TCMECC Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`] module"]
#[doc(alias = "IMR")]
pub type Imr = crate::Reg<imr::ImrSpec>;
#[doc = "TCMECC Interrupt Mask Register"]
pub mod imr;
#[doc = "FAILAR (r) register accessor: TCMECC Fail address register\n\nYou can [`read`](crate::Reg::read) this register and get [`failar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@failar`] module"]
#[doc(alias = "FAILAR")]
pub type Failar = crate::Reg<failar::FailarSpec>;
#[doc = "TCMECC Fail address register"]
pub mod failar;
#[doc = "FAILARD (r) register accessor: TCMECC Fail address register data\n\nYou can [`read`](crate::Reg::read) this register and get [`failard::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@failard`] module"]
#[doc(alias = "FAILARD")]
pub type Failard = crate::Reg<failard::FailardSpec>;
#[doc = "TCMECC Fail address register data"]
pub mod failard;
