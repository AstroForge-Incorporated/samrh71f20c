#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    fmr: Fmr,
    fcr: Fcr,
    fsr: Fsr,
    frr: Frr,
    _reserved4: [u8; 0x30],
    fpmr: Fpmr,
    _reserved5: [u8; 0xa0],
    wpmr: Wpmr,
    _reserved6: [u8; 0x18],
    hecc_cr0: HeccCr0,
    hecc_cr1: HeccCr1,
    hecc_cr2: HeccCr2,
    hecc_cr3: HeccCr3,
    hecc_cr4: HeccCr4,
    hecc_cr5: HeccCr5,
    hecc_cr6: HeccCr6,
    hecc_cr7: HeccCr7,
    hecc_cr8: HeccCr8,
    hecc_cr9: HeccCr9,
    hecc_cr10: HeccCr10,
    hecc_cr11: HeccCr11,
    hecc_cr12: HeccCr12,
    hecc_cr13: HeccCr13,
    hecc_cr14: HeccCr14,
    hecc_cr15: HeccCr15,
    hecc_testcb0: HeccTestcb0,
    hecc_testcb1: HeccTestcb1,
    hecc_testcb2: HeccTestcb2,
    hecc_testcb3: HeccTestcb3,
    hecc_testcb4: HeccTestcb4,
    hecc_testcb5: HeccTestcb5,
    hecc_testcb6: HeccTestcb6,
    hecc_testcb7: HeccTestcb7,
    hecc_testcb8: HeccTestcb8,
    hecc_testcb9: HeccTestcb9,
    hecc_testcb10: HeccTestcb10,
    hecc_testcb11: HeccTestcb11,
    hecc_testcb12: HeccTestcb12,
    hecc_testcb13: HeccTestcb13,
    hecc_testcb14: HeccTestcb14,
    hecc_testcb15: HeccTestcb15,
    hecc_sr: HeccSr,
    hecc_ier: HeccIer,
    hecc_idr: HeccIdr,
    hecc_imr: HeccImr,
    hecc_failar: HeccFailar,
}
impl RegisterBlock {
    #[doc = "0x00 - HEFC Flash Mode Register"]
    #[inline(always)]
    pub const fn fmr(&self) -> &Fmr {
        &self.fmr
    }
    #[doc = "0x04 - HEFC Flash Command Register"]
    #[inline(always)]
    pub const fn fcr(&self) -> &Fcr {
        &self.fcr
    }
    #[doc = "0x08 - HEFC Flash Status Register"]
    #[inline(always)]
    pub const fn fsr(&self) -> &Fsr {
        &self.fsr
    }
    #[doc = "0x0c - HEFC Flash Result Register"]
    #[inline(always)]
    pub const fn frr(&self) -> &Frr {
        &self.frr
    }
    #[doc = "0x40 - HEFC Flash Power Management Register"]
    #[inline(always)]
    pub const fn fpmr(&self) -> &Fpmr {
        &self.fpmr
    }
    #[doc = "0xe4 - Write Protection Mode Register"]
    #[inline(always)]
    pub const fn wpmr(&self) -> &Wpmr {
        &self.wpmr
    }
    #[doc = "0x100 - HECC Control Register ChannelNumbers (ChannelNumbers = 0) 0"]
    #[inline(always)]
    pub const fn hecc_cr0(&self) -> &HeccCr0 {
        &self.hecc_cr0
    }
    #[doc = "0x104 - HECC Control Register ChannelNumbers (ChannelNumbers = 0) 1"]
    #[inline(always)]
    pub const fn hecc_cr1(&self) -> &HeccCr1 {
        &self.hecc_cr1
    }
    #[doc = "0x108 - HECC Control Register ChannelNumbers (ChannelNumbers = 0) 2"]
    #[inline(always)]
    pub const fn hecc_cr2(&self) -> &HeccCr2 {
        &self.hecc_cr2
    }
    #[doc = "0x10c - HECC Control Register ChannelNumbers (ChannelNumbers = 0) 3"]
    #[inline(always)]
    pub const fn hecc_cr3(&self) -> &HeccCr3 {
        &self.hecc_cr3
    }
    #[doc = "0x110 - HECC Control Register ChannelNumbers (ChannelNumbers = 0) 4"]
    #[inline(always)]
    pub const fn hecc_cr4(&self) -> &HeccCr4 {
        &self.hecc_cr4
    }
    #[doc = "0x114 - HECC Control Register ChannelNumbers (ChannelNumbers = 0) 5"]
    #[inline(always)]
    pub const fn hecc_cr5(&self) -> &HeccCr5 {
        &self.hecc_cr5
    }
    #[doc = "0x118 - HECC Control Register ChannelNumbers (ChannelNumbers = 0) 6"]
    #[inline(always)]
    pub const fn hecc_cr6(&self) -> &HeccCr6 {
        &self.hecc_cr6
    }
    #[doc = "0x11c - HECC Control Register ChannelNumbers (ChannelNumbers = 0) 7"]
    #[inline(always)]
    pub const fn hecc_cr7(&self) -> &HeccCr7 {
        &self.hecc_cr7
    }
    #[doc = "0x120 - HECC Control Register ChannelNumbers (ChannelNumbers = 0) 8"]
    #[inline(always)]
    pub const fn hecc_cr8(&self) -> &HeccCr8 {
        &self.hecc_cr8
    }
    #[doc = "0x124 - HECC Control Register ChannelNumbers (ChannelNumbers = 0) 9"]
    #[inline(always)]
    pub const fn hecc_cr9(&self) -> &HeccCr9 {
        &self.hecc_cr9
    }
    #[doc = "0x128 - HECC Control Register ChannelNumbers (ChannelNumbers = 0) 10"]
    #[inline(always)]
    pub const fn hecc_cr10(&self) -> &HeccCr10 {
        &self.hecc_cr10
    }
    #[doc = "0x12c - HECC Control Register ChannelNumbers (ChannelNumbers = 0) 11"]
    #[inline(always)]
    pub const fn hecc_cr11(&self) -> &HeccCr11 {
        &self.hecc_cr11
    }
    #[doc = "0x130 - HECC Control Register ChannelNumbers (ChannelNumbers = 0) 12"]
    #[inline(always)]
    pub const fn hecc_cr12(&self) -> &HeccCr12 {
        &self.hecc_cr12
    }
    #[doc = "0x134 - HECC Control Register ChannelNumbers (ChannelNumbers = 0) 13"]
    #[inline(always)]
    pub const fn hecc_cr13(&self) -> &HeccCr13 {
        &self.hecc_cr13
    }
    #[doc = "0x138 - HECC Control Register ChannelNumbers (ChannelNumbers = 0) 14"]
    #[inline(always)]
    pub const fn hecc_cr14(&self) -> &HeccCr14 {
        &self.hecc_cr14
    }
    #[doc = "0x13c - HECC Control Register ChannelNumbers (ChannelNumbers = 0) 15"]
    #[inline(always)]
    pub const fn hecc_cr15(&self) -> &HeccCr15 {
        &self.hecc_cr15
    }
    #[doc = "0x140 - HECC Test mode ChannelNumbers (ChannelNumbers = 0) 0"]
    #[inline(always)]
    pub const fn hecc_testcb0(&self) -> &HeccTestcb0 {
        &self.hecc_testcb0
    }
    #[doc = "0x144 - HECC Test mode ChannelNumbers (ChannelNumbers = 0) 1"]
    #[inline(always)]
    pub const fn hecc_testcb1(&self) -> &HeccTestcb1 {
        &self.hecc_testcb1
    }
    #[doc = "0x148 - HECC Test mode ChannelNumbers (ChannelNumbers = 0) 2"]
    #[inline(always)]
    pub const fn hecc_testcb2(&self) -> &HeccTestcb2 {
        &self.hecc_testcb2
    }
    #[doc = "0x14c - HECC Test mode ChannelNumbers (ChannelNumbers = 0) 3"]
    #[inline(always)]
    pub const fn hecc_testcb3(&self) -> &HeccTestcb3 {
        &self.hecc_testcb3
    }
    #[doc = "0x150 - HECC Test mode ChannelNumbers (ChannelNumbers = 0) 4"]
    #[inline(always)]
    pub const fn hecc_testcb4(&self) -> &HeccTestcb4 {
        &self.hecc_testcb4
    }
    #[doc = "0x154 - HECC Test mode ChannelNumbers (ChannelNumbers = 0) 5"]
    #[inline(always)]
    pub const fn hecc_testcb5(&self) -> &HeccTestcb5 {
        &self.hecc_testcb5
    }
    #[doc = "0x158 - HECC Test mode ChannelNumbers (ChannelNumbers = 0) 6"]
    #[inline(always)]
    pub const fn hecc_testcb6(&self) -> &HeccTestcb6 {
        &self.hecc_testcb6
    }
    #[doc = "0x15c - HECC Test mode ChannelNumbers (ChannelNumbers = 0) 7"]
    #[inline(always)]
    pub const fn hecc_testcb7(&self) -> &HeccTestcb7 {
        &self.hecc_testcb7
    }
    #[doc = "0x160 - HECC Test mode ChannelNumbers (ChannelNumbers = 0) 8"]
    #[inline(always)]
    pub const fn hecc_testcb8(&self) -> &HeccTestcb8 {
        &self.hecc_testcb8
    }
    #[doc = "0x164 - HECC Test mode ChannelNumbers (ChannelNumbers = 0) 9"]
    #[inline(always)]
    pub const fn hecc_testcb9(&self) -> &HeccTestcb9 {
        &self.hecc_testcb9
    }
    #[doc = "0x168 - HECC Test mode ChannelNumbers (ChannelNumbers = 0) 10"]
    #[inline(always)]
    pub const fn hecc_testcb10(&self) -> &HeccTestcb10 {
        &self.hecc_testcb10
    }
    #[doc = "0x16c - HECC Test mode ChannelNumbers (ChannelNumbers = 0) 11"]
    #[inline(always)]
    pub const fn hecc_testcb11(&self) -> &HeccTestcb11 {
        &self.hecc_testcb11
    }
    #[doc = "0x170 - HECC Test mode ChannelNumbers (ChannelNumbers = 0) 12"]
    #[inline(always)]
    pub const fn hecc_testcb12(&self) -> &HeccTestcb12 {
        &self.hecc_testcb12
    }
    #[doc = "0x174 - HECC Test mode ChannelNumbers (ChannelNumbers = 0) 13"]
    #[inline(always)]
    pub const fn hecc_testcb13(&self) -> &HeccTestcb13 {
        &self.hecc_testcb13
    }
    #[doc = "0x178 - HECC Test mode ChannelNumbers (ChannelNumbers = 0) 14"]
    #[inline(always)]
    pub const fn hecc_testcb14(&self) -> &HeccTestcb14 {
        &self.hecc_testcb14
    }
    #[doc = "0x17c - HECC Test mode ChannelNumbers (ChannelNumbers = 0) 15"]
    #[inline(always)]
    pub const fn hecc_testcb15(&self) -> &HeccTestcb15 {
        &self.hecc_testcb15
    }
    #[doc = "0x180 - HECC Status register"]
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
#[doc = "FMR (rw) register accessor: HEFC Flash Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmr`] module"]
#[doc(alias = "FMR")]
pub type Fmr = crate::Reg<fmr::FmrSpec>;
#[doc = "HEFC Flash Mode Register"]
pub mod fmr;
#[doc = "FCR (w) register accessor: HEFC Flash Command Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcr`] module"]
#[doc(alias = "FCR")]
pub type Fcr = crate::Reg<fcr::FcrSpec>;
#[doc = "HEFC Flash Command Register"]
pub mod fcr;
#[doc = "FSR (r) register accessor: HEFC Flash Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsr`] module"]
#[doc(alias = "FSR")]
pub type Fsr = crate::Reg<fsr::FsrSpec>;
#[doc = "HEFC Flash Status Register"]
pub mod fsr;
#[doc = "FRR (r) register accessor: HEFC Flash Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`frr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frr`] module"]
#[doc(alias = "FRR")]
pub type Frr = crate::Reg<frr::FrrSpec>;
#[doc = "HEFC Flash Result Register"]
pub mod frr;
#[doc = "FPMR (rw) register accessor: HEFC Flash Power Management Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fpmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpmr`] module"]
#[doc(alias = "FPMR")]
pub type Fpmr = crate::Reg<fpmr::FpmrSpec>;
#[doc = "HEFC Flash Power Management Register"]
pub mod fpmr;
#[doc = "WPMR (rw) register accessor: Write Protection Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpmr`] module"]
#[doc(alias = "WPMR")]
pub type Wpmr = crate::Reg<wpmr::WpmrSpec>;
#[doc = "Write Protection Mode Register"]
pub mod wpmr;
#[doc = "HECC_CR0 (rw) register accessor: HECC Control Register ChannelNumbers (ChannelNumbers = 0) 0\n\nYou can [`read`](crate::Reg::read) this register and get [`hecc_cr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hecc_cr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hecc_cr0`] module"]
#[doc(alias = "HECC_CR0")]
pub type HeccCr0 = crate::Reg<hecc_cr0::HeccCr0Spec>;
#[doc = "HECC Control Register ChannelNumbers (ChannelNumbers = 0) 0"]
pub mod hecc_cr0;
#[doc = "HECC_CR1 (rw) register accessor: HECC Control Register ChannelNumbers (ChannelNumbers = 0) 1\n\nYou can [`read`](crate::Reg::read) this register and get [`hecc_cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hecc_cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hecc_cr1`] module"]
#[doc(alias = "HECC_CR1")]
pub type HeccCr1 = crate::Reg<hecc_cr1::HeccCr1Spec>;
#[doc = "HECC Control Register ChannelNumbers (ChannelNumbers = 0) 1"]
pub mod hecc_cr1;
#[doc = "HECC_CR2 (rw) register accessor: HECC Control Register ChannelNumbers (ChannelNumbers = 0) 2\n\nYou can [`read`](crate::Reg::read) this register and get [`hecc_cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hecc_cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hecc_cr2`] module"]
#[doc(alias = "HECC_CR2")]
pub type HeccCr2 = crate::Reg<hecc_cr2::HeccCr2Spec>;
#[doc = "HECC Control Register ChannelNumbers (ChannelNumbers = 0) 2"]
pub mod hecc_cr2;
#[doc = "HECC_CR3 (rw) register accessor: HECC Control Register ChannelNumbers (ChannelNumbers = 0) 3\n\nYou can [`read`](crate::Reg::read) this register and get [`hecc_cr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hecc_cr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hecc_cr3`] module"]
#[doc(alias = "HECC_CR3")]
pub type HeccCr3 = crate::Reg<hecc_cr3::HeccCr3Spec>;
#[doc = "HECC Control Register ChannelNumbers (ChannelNumbers = 0) 3"]
pub mod hecc_cr3;
#[doc = "HECC_CR4 (rw) register accessor: HECC Control Register ChannelNumbers (ChannelNumbers = 0) 4\n\nYou can [`read`](crate::Reg::read) this register and get [`hecc_cr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hecc_cr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hecc_cr4`] module"]
#[doc(alias = "HECC_CR4")]
pub type HeccCr4 = crate::Reg<hecc_cr4::HeccCr4Spec>;
#[doc = "HECC Control Register ChannelNumbers (ChannelNumbers = 0) 4"]
pub mod hecc_cr4;
#[doc = "HECC_CR5 (rw) register accessor: HECC Control Register ChannelNumbers (ChannelNumbers = 0) 5\n\nYou can [`read`](crate::Reg::read) this register and get [`hecc_cr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hecc_cr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hecc_cr5`] module"]
#[doc(alias = "HECC_CR5")]
pub type HeccCr5 = crate::Reg<hecc_cr5::HeccCr5Spec>;
#[doc = "HECC Control Register ChannelNumbers (ChannelNumbers = 0) 5"]
pub mod hecc_cr5;
#[doc = "HECC_CR6 (rw) register accessor: HECC Control Register ChannelNumbers (ChannelNumbers = 0) 6\n\nYou can [`read`](crate::Reg::read) this register and get [`hecc_cr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hecc_cr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hecc_cr6`] module"]
#[doc(alias = "HECC_CR6")]
pub type HeccCr6 = crate::Reg<hecc_cr6::HeccCr6Spec>;
#[doc = "HECC Control Register ChannelNumbers (ChannelNumbers = 0) 6"]
pub mod hecc_cr6;
#[doc = "HECC_CR7 (rw) register accessor: HECC Control Register ChannelNumbers (ChannelNumbers = 0) 7\n\nYou can [`read`](crate::Reg::read) this register and get [`hecc_cr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hecc_cr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hecc_cr7`] module"]
#[doc(alias = "HECC_CR7")]
pub type HeccCr7 = crate::Reg<hecc_cr7::HeccCr7Spec>;
#[doc = "HECC Control Register ChannelNumbers (ChannelNumbers = 0) 7"]
pub mod hecc_cr7;
#[doc = "HECC_CR8 (rw) register accessor: HECC Control Register ChannelNumbers (ChannelNumbers = 0) 8\n\nYou can [`read`](crate::Reg::read) this register and get [`hecc_cr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hecc_cr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hecc_cr8`] module"]
#[doc(alias = "HECC_CR8")]
pub type HeccCr8 = crate::Reg<hecc_cr8::HeccCr8Spec>;
#[doc = "HECC Control Register ChannelNumbers (ChannelNumbers = 0) 8"]
pub mod hecc_cr8;
#[doc = "HECC_CR9 (rw) register accessor: HECC Control Register ChannelNumbers (ChannelNumbers = 0) 9\n\nYou can [`read`](crate::Reg::read) this register and get [`hecc_cr9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hecc_cr9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hecc_cr9`] module"]
#[doc(alias = "HECC_CR9")]
pub type HeccCr9 = crate::Reg<hecc_cr9::HeccCr9Spec>;
#[doc = "HECC Control Register ChannelNumbers (ChannelNumbers = 0) 9"]
pub mod hecc_cr9;
#[doc = "HECC_CR10 (rw) register accessor: HECC Control Register ChannelNumbers (ChannelNumbers = 0) 10\n\nYou can [`read`](crate::Reg::read) this register and get [`hecc_cr10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hecc_cr10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hecc_cr10`] module"]
#[doc(alias = "HECC_CR10")]
pub type HeccCr10 = crate::Reg<hecc_cr10::HeccCr10Spec>;
#[doc = "HECC Control Register ChannelNumbers (ChannelNumbers = 0) 10"]
pub mod hecc_cr10;
#[doc = "HECC_CR11 (rw) register accessor: HECC Control Register ChannelNumbers (ChannelNumbers = 0) 11\n\nYou can [`read`](crate::Reg::read) this register and get [`hecc_cr11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hecc_cr11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hecc_cr11`] module"]
#[doc(alias = "HECC_CR11")]
pub type HeccCr11 = crate::Reg<hecc_cr11::HeccCr11Spec>;
#[doc = "HECC Control Register ChannelNumbers (ChannelNumbers = 0) 11"]
pub mod hecc_cr11;
#[doc = "HECC_CR12 (rw) register accessor: HECC Control Register ChannelNumbers (ChannelNumbers = 0) 12\n\nYou can [`read`](crate::Reg::read) this register and get [`hecc_cr12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hecc_cr12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hecc_cr12`] module"]
#[doc(alias = "HECC_CR12")]
pub type HeccCr12 = crate::Reg<hecc_cr12::HeccCr12Spec>;
#[doc = "HECC Control Register ChannelNumbers (ChannelNumbers = 0) 12"]
pub mod hecc_cr12;
#[doc = "HECC_CR13 (rw) register accessor: HECC Control Register ChannelNumbers (ChannelNumbers = 0) 13\n\nYou can [`read`](crate::Reg::read) this register and get [`hecc_cr13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hecc_cr13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hecc_cr13`] module"]
#[doc(alias = "HECC_CR13")]
pub type HeccCr13 = crate::Reg<hecc_cr13::HeccCr13Spec>;
#[doc = "HECC Control Register ChannelNumbers (ChannelNumbers = 0) 13"]
pub mod hecc_cr13;
#[doc = "HECC_CR14 (rw) register accessor: HECC Control Register ChannelNumbers (ChannelNumbers = 0) 14\n\nYou can [`read`](crate::Reg::read) this register and get [`hecc_cr14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hecc_cr14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hecc_cr14`] module"]
#[doc(alias = "HECC_CR14")]
pub type HeccCr14 = crate::Reg<hecc_cr14::HeccCr14Spec>;
#[doc = "HECC Control Register ChannelNumbers (ChannelNumbers = 0) 14"]
pub mod hecc_cr14;
#[doc = "HECC_CR15 (rw) register accessor: HECC Control Register ChannelNumbers (ChannelNumbers = 0) 15\n\nYou can [`read`](crate::Reg::read) this register and get [`hecc_cr15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hecc_cr15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hecc_cr15`] module"]
#[doc(alias = "HECC_CR15")]
pub type HeccCr15 = crate::Reg<hecc_cr15::HeccCr15Spec>;
#[doc = "HECC Control Register ChannelNumbers (ChannelNumbers = 0) 15"]
pub mod hecc_cr15;
#[doc = "HECC_TESTCB0 (rw) register accessor: HECC Test mode ChannelNumbers (ChannelNumbers = 0) 0\n\nYou can [`read`](crate::Reg::read) this register and get [`hecc_testcb0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hecc_testcb0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hecc_testcb0`] module"]
#[doc(alias = "HECC_TESTCB0")]
pub type HeccTestcb0 = crate::Reg<hecc_testcb0::HeccTestcb0Spec>;
#[doc = "HECC Test mode ChannelNumbers (ChannelNumbers = 0) 0"]
pub mod hecc_testcb0;
#[doc = "HECC_TESTCB1 (rw) register accessor: HECC Test mode ChannelNumbers (ChannelNumbers = 0) 1\n\nYou can [`read`](crate::Reg::read) this register and get [`hecc_testcb1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hecc_testcb1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hecc_testcb1`] module"]
#[doc(alias = "HECC_TESTCB1")]
pub type HeccTestcb1 = crate::Reg<hecc_testcb1::HeccTestcb1Spec>;
#[doc = "HECC Test mode ChannelNumbers (ChannelNumbers = 0) 1"]
pub mod hecc_testcb1;
#[doc = "HECC_TESTCB2 (rw) register accessor: HECC Test mode ChannelNumbers (ChannelNumbers = 0) 2\n\nYou can [`read`](crate::Reg::read) this register and get [`hecc_testcb2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hecc_testcb2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hecc_testcb2`] module"]
#[doc(alias = "HECC_TESTCB2")]
pub type HeccTestcb2 = crate::Reg<hecc_testcb2::HeccTestcb2Spec>;
#[doc = "HECC Test mode ChannelNumbers (ChannelNumbers = 0) 2"]
pub mod hecc_testcb2;
#[doc = "HECC_TESTCB3 (rw) register accessor: HECC Test mode ChannelNumbers (ChannelNumbers = 0) 3\n\nYou can [`read`](crate::Reg::read) this register and get [`hecc_testcb3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hecc_testcb3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hecc_testcb3`] module"]
#[doc(alias = "HECC_TESTCB3")]
pub type HeccTestcb3 = crate::Reg<hecc_testcb3::HeccTestcb3Spec>;
#[doc = "HECC Test mode ChannelNumbers (ChannelNumbers = 0) 3"]
pub mod hecc_testcb3;
#[doc = "HECC_TESTCB4 (rw) register accessor: HECC Test mode ChannelNumbers (ChannelNumbers = 0) 4\n\nYou can [`read`](crate::Reg::read) this register and get [`hecc_testcb4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hecc_testcb4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hecc_testcb4`] module"]
#[doc(alias = "HECC_TESTCB4")]
pub type HeccTestcb4 = crate::Reg<hecc_testcb4::HeccTestcb4Spec>;
#[doc = "HECC Test mode ChannelNumbers (ChannelNumbers = 0) 4"]
pub mod hecc_testcb4;
#[doc = "HECC_TESTCB5 (rw) register accessor: HECC Test mode ChannelNumbers (ChannelNumbers = 0) 5\n\nYou can [`read`](crate::Reg::read) this register and get [`hecc_testcb5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hecc_testcb5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hecc_testcb5`] module"]
#[doc(alias = "HECC_TESTCB5")]
pub type HeccTestcb5 = crate::Reg<hecc_testcb5::HeccTestcb5Spec>;
#[doc = "HECC Test mode ChannelNumbers (ChannelNumbers = 0) 5"]
pub mod hecc_testcb5;
#[doc = "HECC_TESTCB6 (rw) register accessor: HECC Test mode ChannelNumbers (ChannelNumbers = 0) 6\n\nYou can [`read`](crate::Reg::read) this register and get [`hecc_testcb6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hecc_testcb6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hecc_testcb6`] module"]
#[doc(alias = "HECC_TESTCB6")]
pub type HeccTestcb6 = crate::Reg<hecc_testcb6::HeccTestcb6Spec>;
#[doc = "HECC Test mode ChannelNumbers (ChannelNumbers = 0) 6"]
pub mod hecc_testcb6;
#[doc = "HECC_TESTCB7 (rw) register accessor: HECC Test mode ChannelNumbers (ChannelNumbers = 0) 7\n\nYou can [`read`](crate::Reg::read) this register and get [`hecc_testcb7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hecc_testcb7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hecc_testcb7`] module"]
#[doc(alias = "HECC_TESTCB7")]
pub type HeccTestcb7 = crate::Reg<hecc_testcb7::HeccTestcb7Spec>;
#[doc = "HECC Test mode ChannelNumbers (ChannelNumbers = 0) 7"]
pub mod hecc_testcb7;
#[doc = "HECC_TESTCB8 (rw) register accessor: HECC Test mode ChannelNumbers (ChannelNumbers = 0) 8\n\nYou can [`read`](crate::Reg::read) this register and get [`hecc_testcb8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hecc_testcb8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hecc_testcb8`] module"]
#[doc(alias = "HECC_TESTCB8")]
pub type HeccTestcb8 = crate::Reg<hecc_testcb8::HeccTestcb8Spec>;
#[doc = "HECC Test mode ChannelNumbers (ChannelNumbers = 0) 8"]
pub mod hecc_testcb8;
#[doc = "HECC_TESTCB9 (rw) register accessor: HECC Test mode ChannelNumbers (ChannelNumbers = 0) 9\n\nYou can [`read`](crate::Reg::read) this register and get [`hecc_testcb9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hecc_testcb9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hecc_testcb9`] module"]
#[doc(alias = "HECC_TESTCB9")]
pub type HeccTestcb9 = crate::Reg<hecc_testcb9::HeccTestcb9Spec>;
#[doc = "HECC Test mode ChannelNumbers (ChannelNumbers = 0) 9"]
pub mod hecc_testcb9;
#[doc = "HECC_TESTCB10 (rw) register accessor: HECC Test mode ChannelNumbers (ChannelNumbers = 0) 10\n\nYou can [`read`](crate::Reg::read) this register and get [`hecc_testcb10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hecc_testcb10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hecc_testcb10`] module"]
#[doc(alias = "HECC_TESTCB10")]
pub type HeccTestcb10 = crate::Reg<hecc_testcb10::HeccTestcb10Spec>;
#[doc = "HECC Test mode ChannelNumbers (ChannelNumbers = 0) 10"]
pub mod hecc_testcb10;
#[doc = "HECC_TESTCB11 (rw) register accessor: HECC Test mode ChannelNumbers (ChannelNumbers = 0) 11\n\nYou can [`read`](crate::Reg::read) this register and get [`hecc_testcb11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hecc_testcb11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hecc_testcb11`] module"]
#[doc(alias = "HECC_TESTCB11")]
pub type HeccTestcb11 = crate::Reg<hecc_testcb11::HeccTestcb11Spec>;
#[doc = "HECC Test mode ChannelNumbers (ChannelNumbers = 0) 11"]
pub mod hecc_testcb11;
#[doc = "HECC_TESTCB12 (rw) register accessor: HECC Test mode ChannelNumbers (ChannelNumbers = 0) 12\n\nYou can [`read`](crate::Reg::read) this register and get [`hecc_testcb12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hecc_testcb12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hecc_testcb12`] module"]
#[doc(alias = "HECC_TESTCB12")]
pub type HeccTestcb12 = crate::Reg<hecc_testcb12::HeccTestcb12Spec>;
#[doc = "HECC Test mode ChannelNumbers (ChannelNumbers = 0) 12"]
pub mod hecc_testcb12;
#[doc = "HECC_TESTCB13 (rw) register accessor: HECC Test mode ChannelNumbers (ChannelNumbers = 0) 13\n\nYou can [`read`](crate::Reg::read) this register and get [`hecc_testcb13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hecc_testcb13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hecc_testcb13`] module"]
#[doc(alias = "HECC_TESTCB13")]
pub type HeccTestcb13 = crate::Reg<hecc_testcb13::HeccTestcb13Spec>;
#[doc = "HECC Test mode ChannelNumbers (ChannelNumbers = 0) 13"]
pub mod hecc_testcb13;
#[doc = "HECC_TESTCB14 (rw) register accessor: HECC Test mode ChannelNumbers (ChannelNumbers = 0) 14\n\nYou can [`read`](crate::Reg::read) this register and get [`hecc_testcb14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hecc_testcb14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hecc_testcb14`] module"]
#[doc(alias = "HECC_TESTCB14")]
pub type HeccTestcb14 = crate::Reg<hecc_testcb14::HeccTestcb14Spec>;
#[doc = "HECC Test mode ChannelNumbers (ChannelNumbers = 0) 14"]
pub mod hecc_testcb14;
#[doc = "HECC_TESTCB15 (rw) register accessor: HECC Test mode ChannelNumbers (ChannelNumbers = 0) 15\n\nYou can [`read`](crate::Reg::read) this register and get [`hecc_testcb15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hecc_testcb15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hecc_testcb15`] module"]
#[doc(alias = "HECC_TESTCB15")]
pub type HeccTestcb15 = crate::Reg<hecc_testcb15::HeccTestcb15Spec>;
#[doc = "HECC Test mode ChannelNumbers (ChannelNumbers = 0) 15"]
pub mod hecc_testcb15;
#[doc = "HECC_SR (r) register accessor: HECC Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`hecc_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hecc_sr`] module"]
#[doc(alias = "HECC_SR")]
pub type HeccSr = crate::Reg<hecc_sr::HeccSrSpec>;
#[doc = "HECC Status register"]
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
