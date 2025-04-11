#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr_ncs0: CrNcs0,
    cr_ncs1: CrNcs1,
    cr_ncs2: CrNcs2,
    cr_ncs3: CrNcs3,
    cr_ncs4: CrNcs4,
    cr_ncs5: CrNcs5,
    ctrl: Ctrl,
    _reserved7: [u8; 0x04],
    crp_ncs: [CrpNcs; 6],
    ier: Ier,
    idr: Idr,
    imr: Imr,
    isr: Isr,
    sr: Sr,
    _reserved13: [u8; 0xb4],
    hecc_cr0: HeccCr0,
    hecc_cr1: HeccCr1,
    hecc_cr2: HeccCr2,
    _reserved16: [u8; 0x34],
    hecc_testcb0: HeccTestcb0,
    hecc_testcb1: HeccTestcb1,
    hecc_testcb2: HeccTestcb2,
    _reserved19: [u8; 0x34],
    hecc_sr: HeccSr,
    hecc_ier: HeccIer,
    hecc_idr: HeccIdr,
    hecc_imr: HeccImr,
    hecc_failar: HeccFailar,
}
impl RegisterBlock {
    #[doc = "0x00 - HEMC Control Register NCS 0"]
    #[inline(always)]
    pub const fn cr_ncs0(&self) -> &CrNcs0 {
        &self.cr_ncs0
    }
    #[doc = "0x04 - HEMC Control Register NCS 1"]
    #[inline(always)]
    pub const fn cr_ncs1(&self) -> &CrNcs1 {
        &self.cr_ncs1
    }
    #[doc = "0x08 - HEMC Control Register NCS 2"]
    #[inline(always)]
    pub const fn cr_ncs2(&self) -> &CrNcs2 {
        &self.cr_ncs2
    }
    #[doc = "0x0c - HEMC Control Register NCS 3"]
    #[inline(always)]
    pub const fn cr_ncs3(&self) -> &CrNcs3 {
        &self.cr_ncs3
    }
    #[doc = "0x10 - HEMC Control Register NCS 4"]
    #[inline(always)]
    pub const fn cr_ncs4(&self) -> &CrNcs4 {
        &self.cr_ncs4
    }
    #[doc = "0x14 - HEMC Control Register NCS 5"]
    #[inline(always)]
    pub const fn cr_ncs5(&self) -> &CrNcs5 {
        &self.cr_ncs5
    }
    #[doc = "0x18 - HEMC Polarity Control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x20..0x38 - HEMC Control Register Protection NCS"]
    #[inline(always)]
    pub const fn crp_ncs(&self, n: usize) -> &CrpNcs {
        &self.crp_ncs[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x38 - HEMC Control Register Protection NCS"]
    #[inline(always)]
    pub fn crp_ncs_iter(&self) -> impl Iterator<Item = &CrpNcs> {
        self.crp_ncs.iter()
    }
    #[doc = "0x38 - HEMC Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x3c - HEMC Interrupt Disable Register"]
    #[inline(always)]
    pub const fn idr(&self) -> &Idr {
        &self.idr
    }
    #[doc = "0x40 - HEMC Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        &self.imr
    }
    #[doc = "0x44 - HEMC Interrupt Status Register"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x48 - HEMC Status Register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x100 - HECC Control Register Channel 0 (HSMC)"]
    #[inline(always)]
    pub const fn hecc_cr0(&self) -> &HeccCr0 {
        &self.hecc_cr0
    }
    #[doc = "0x104 - HECC Control Register Channel 1 (HSDRAMC)"]
    #[inline(always)]
    pub const fn hecc_cr1(&self) -> &HeccCr1 {
        &self.hecc_cr1
    }
    #[doc = "0x108 - HECC Control Register Channel 2 (HSDRAMC)"]
    #[inline(always)]
    pub const fn hecc_cr2(&self) -> &HeccCr2 {
        &self.hecc_cr2
    }
    #[doc = "0x140 - HECC Test mode Register Channel 0 (HSMC)"]
    #[inline(always)]
    pub const fn hecc_testcb0(&self) -> &HeccTestcb0 {
        &self.hecc_testcb0
    }
    #[doc = "0x144 - HECC Test mode Register Channel 1 (HSDRAMC)"]
    #[inline(always)]
    pub const fn hecc_testcb1(&self) -> &HeccTestcb1 {
        &self.hecc_testcb1
    }
    #[doc = "0x148 - HECC Test mode Register Channel 2 (HSDRAMC)"]
    #[inline(always)]
    pub const fn hecc_testcb2(&self) -> &HeccTestcb2 {
        &self.hecc_testcb2
    }
    #[doc = "0x180 - HECC Status Register"]
    #[inline(always)]
    pub const fn hecc_sr(&self) -> &HeccSr {
        &self.hecc_sr
    }
    #[doc = "0x184 - HECC Interrupt Enable Register"]
    #[inline(always)]
    pub const fn hecc_ier(&self) -> &HeccIer {
        &self.hecc_ier
    }
    #[doc = "0x188 - HECC Interrupt Disable Register"]
    #[inline(always)]
    pub const fn hecc_idr(&self) -> &HeccIdr {
        &self.hecc_idr
    }
    #[doc = "0x18c - HECC Interrupt Mask Register"]
    #[inline(always)]
    pub const fn hecc_imr(&self) -> &HeccImr {
        &self.hecc_imr
    }
    #[doc = "0x190 - HECC Fail address register"]
    #[inline(always)]
    pub const fn hecc_failar(&self) -> &HeccFailar {
        &self.hecc_failar
    }
}
#[doc = "CR_NCS0 (rw) register accessor: HEMC Control Register NCS 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_ncs0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_ncs0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_ncs0`] module"]
#[doc(alias = "CR_NCS0")]
pub type CrNcs0 = crate::Reg<cr_ncs0::CrNcs0Spec>;
#[doc = "HEMC Control Register NCS 0"]
pub mod cr_ncs0;
#[doc = "CR_NCS1 (rw) register accessor: HEMC Control Register NCS 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_ncs1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_ncs1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_ncs1`] module"]
#[doc(alias = "CR_NCS1")]
pub type CrNcs1 = crate::Reg<cr_ncs1::CrNcs1Spec>;
#[doc = "HEMC Control Register NCS 1"]
pub mod cr_ncs1;
#[doc = "CR_NCS2 (rw) register accessor: HEMC Control Register NCS 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_ncs2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_ncs2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_ncs2`] module"]
#[doc(alias = "CR_NCS2")]
pub type CrNcs2 = crate::Reg<cr_ncs2::CrNcs2Spec>;
#[doc = "HEMC Control Register NCS 2"]
pub mod cr_ncs2;
#[doc = "CR_NCS3 (rw) register accessor: HEMC Control Register NCS 3\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_ncs3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_ncs3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_ncs3`] module"]
#[doc(alias = "CR_NCS3")]
pub type CrNcs3 = crate::Reg<cr_ncs3::CrNcs3Spec>;
#[doc = "HEMC Control Register NCS 3"]
pub mod cr_ncs3;
#[doc = "CR_NCS4 (rw) register accessor: HEMC Control Register NCS 4\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_ncs4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_ncs4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_ncs4`] module"]
#[doc(alias = "CR_NCS4")]
pub type CrNcs4 = crate::Reg<cr_ncs4::CrNcs4Spec>;
#[doc = "HEMC Control Register NCS 4"]
pub mod cr_ncs4;
#[doc = "CR_NCS5 (rw) register accessor: HEMC Control Register NCS 5\n\nYou can [`read`](crate::Reg::read) this register and get [`cr_ncs5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr_ncs5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr_ncs5`] module"]
#[doc(alias = "CR_NCS5")]
pub type CrNcs5 = crate::Reg<cr_ncs5::CrNcs5Spec>;
#[doc = "HEMC Control Register NCS 5"]
pub mod cr_ncs5;
#[doc = "CTRL (rw) register accessor: HEMC Polarity Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "HEMC Polarity Control register"]
pub mod ctrl;
#[doc = "CRP_NCS (rw) register accessor: HEMC Control Register Protection NCS\n\nYou can [`read`](crate::Reg::read) this register and get [`crp_ncs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crp_ncs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crp_ncs`] module"]
#[doc(alias = "CRP_NCS")]
pub type CrpNcs = crate::Reg<crp_ncs::CrpNcsSpec>;
#[doc = "HEMC Control Register Protection NCS"]
pub mod crp_ncs;
#[doc = "IER (w) register accessor: HEMC Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`] module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "HEMC Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR (w) register accessor: HEMC Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`] module"]
#[doc(alias = "IDR")]
pub type Idr = crate::Reg<idr::IdrSpec>;
#[doc = "HEMC Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR (r) register accessor: HEMC Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`] module"]
#[doc(alias = "IMR")]
pub type Imr = crate::Reg<imr::ImrSpec>;
#[doc = "HEMC Interrupt Mask Register"]
pub mod imr;
#[doc = "ISR (r) register accessor: HEMC Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`] module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "HEMC Interrupt Status Register"]
pub mod isr;
#[doc = "SR (r) register accessor: HEMC Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "HEMC Status Register"]
pub mod sr;
#[doc = "HECC_CR0 (rw) register accessor: HECC Control Register Channel 0 (HSMC)\n\nYou can [`read`](crate::Reg::read) this register and get [`hecc_cr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hecc_cr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hecc_cr0`] module"]
#[doc(alias = "HECC_CR0")]
pub type HeccCr0 = crate::Reg<hecc_cr0::HeccCr0Spec>;
#[doc = "HECC Control Register Channel 0 (HSMC)"]
pub mod hecc_cr0;
#[doc = "HECC_CR1 (rw) register accessor: HECC Control Register Channel 1 (HSDRAMC)\n\nYou can [`read`](crate::Reg::read) this register and get [`hecc_cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hecc_cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hecc_cr1`] module"]
#[doc(alias = "HECC_CR1")]
pub type HeccCr1 = crate::Reg<hecc_cr1::HeccCr1Spec>;
#[doc = "HECC Control Register Channel 1 (HSDRAMC)"]
pub mod hecc_cr1;
#[doc = "HECC_CR2 (rw) register accessor: HECC Control Register Channel 2 (HSDRAMC)\n\nYou can [`read`](crate::Reg::read) this register and get [`hecc_cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hecc_cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hecc_cr2`] module"]
#[doc(alias = "HECC_CR2")]
pub type HeccCr2 = crate::Reg<hecc_cr2::HeccCr2Spec>;
#[doc = "HECC Control Register Channel 2 (HSDRAMC)"]
pub mod hecc_cr2;
#[doc = "HECC_TESTCB0 (rw) register accessor: HECC Test mode Register Channel 0 (HSMC)\n\nYou can [`read`](crate::Reg::read) this register and get [`hecc_testcb0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hecc_testcb0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hecc_testcb0`] module"]
#[doc(alias = "HECC_TESTCB0")]
pub type HeccTestcb0 = crate::Reg<hecc_testcb0::HeccTestcb0Spec>;
#[doc = "HECC Test mode Register Channel 0 (HSMC)"]
pub mod hecc_testcb0;
#[doc = "HECC_TESTCB1 (rw) register accessor: HECC Test mode Register Channel 1 (HSDRAMC)\n\nYou can [`read`](crate::Reg::read) this register and get [`hecc_testcb1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hecc_testcb1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hecc_testcb1`] module"]
#[doc(alias = "HECC_TESTCB1")]
pub type HeccTestcb1 = crate::Reg<hecc_testcb1::HeccTestcb1Spec>;
#[doc = "HECC Test mode Register Channel 1 (HSDRAMC)"]
pub mod hecc_testcb1;
#[doc = "HECC_TESTCB2 (rw) register accessor: HECC Test mode Register Channel 2 (HSDRAMC)\n\nYou can [`read`](crate::Reg::read) this register and get [`hecc_testcb2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hecc_testcb2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hecc_testcb2`] module"]
#[doc(alias = "HECC_TESTCB2")]
pub type HeccTestcb2 = crate::Reg<hecc_testcb2::HeccTestcb2Spec>;
#[doc = "HECC Test mode Register Channel 2 (HSDRAMC)"]
pub mod hecc_testcb2;
#[doc = "HECC_SR (r) register accessor: HECC Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hecc_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hecc_sr`] module"]
#[doc(alias = "HECC_SR")]
pub type HeccSr = crate::Reg<hecc_sr::HeccSrSpec>;
#[doc = "HECC Status Register"]
pub mod hecc_sr;
#[doc = "HECC_IER (w) register accessor: HECC Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hecc_ier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hecc_ier`] module"]
#[doc(alias = "HECC_IER")]
pub type HeccIer = crate::Reg<hecc_ier::HeccIerSpec>;
#[doc = "HECC Interrupt Enable Register"]
pub mod hecc_ier;
#[doc = "HECC_IDR (w) register accessor: HECC Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hecc_idr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hecc_idr`] module"]
#[doc(alias = "HECC_IDR")]
pub type HeccIdr = crate::Reg<hecc_idr::HeccIdrSpec>;
#[doc = "HECC Interrupt Disable Register"]
pub mod hecc_idr;
#[doc = "HECC_IMR (r) register accessor: HECC Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hecc_imr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hecc_imr`] module"]
#[doc(alias = "HECC_IMR")]
pub type HeccImr = crate::Reg<hecc_imr::HeccImrSpec>;
#[doc = "HECC Interrupt Mask Register"]
pub mod hecc_imr;
#[doc = "HECC_FAILAR (r) register accessor: HECC Fail address register\n\nYou can [`read`](crate::Reg::read) this register and get [`hecc_failar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hecc_failar`] module"]
#[doc(alias = "HECC_FAILAR")]
pub type HeccFailar = crate::Reg<hecc_failar::HeccFailarSpec>;
#[doc = "HECC Fail address register"]
pub mod hecc_failar;
