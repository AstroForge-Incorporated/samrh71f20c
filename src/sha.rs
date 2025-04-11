#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    mr: Mr,
    _reserved2: [u8; 0x08],
    ier: Ier,
    idr: Idr,
    imr: Imr,
    isr: Isr,
    msr: Msr,
    _reserved7: [u8; 0x0c],
    bcr: Bcr,
    _reserved8: [u8; 0x0c],
    idatar0: Idatar0,
    idatar1: Idatar1,
    idatar2: Idatar2,
    idatar3: Idatar3,
    idatar4: Idatar4,
    idatar5: Idatar5,
    idatar6: Idatar6,
    idatar7: Idatar7,
    idatar8: Idatar8,
    idatar9: Idatar9,
    idatar10: Idatar10,
    idatar11: Idatar11,
    idatar12: Idatar12,
    idatar13: Idatar13,
    idatar14: Idatar14,
    idatar15: Idatar15,
    iodatar0: Iodatar0,
    iodatar1: Iodatar1,
    iodatar2: Iodatar2,
    iodatar3: Iodatar3,
    iodatar4: Iodatar4,
    iodatar5: Iodatar5,
    iodatar6: Iodatar6,
    iodatar7: Iodatar7,
    iodatar8: Iodatar8,
    iodatar9: Iodatar9,
    iodatar10: Iodatar10,
    iodatar11: Iodatar11,
    iodatar12: Iodatar12,
    iodatar13: Iodatar13,
    iodatar14: Iodatar14,
    iodatar15: Iodatar15,
    _reserved40: [u8; 0x24],
    wpmr: Wpmr,
    wpsr: Wpsr,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - Mode Register"]
    #[inline(always)]
    pub const fn mr(&self) -> &Mr {
        &self.mr
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
    #[doc = "0x20 - Message Size Register"]
    #[inline(always)]
    pub const fn msr(&self) -> &Msr {
        &self.msr
    }
    #[doc = "0x30 - Bytes Count Register"]
    #[inline(always)]
    pub const fn bcr(&self) -> &Bcr {
        &self.bcr
    }
    #[doc = "0x40 - Input Data 0 Register 0"]
    #[inline(always)]
    pub const fn idatar0(&self) -> &Idatar0 {
        &self.idatar0
    }
    #[doc = "0x44 - Input Data 0 Register 1"]
    #[inline(always)]
    pub const fn idatar1(&self) -> &Idatar1 {
        &self.idatar1
    }
    #[doc = "0x48 - Input Data 0 Register 2"]
    #[inline(always)]
    pub const fn idatar2(&self) -> &Idatar2 {
        &self.idatar2
    }
    #[doc = "0x4c - Input Data 0 Register 3"]
    #[inline(always)]
    pub const fn idatar3(&self) -> &Idatar3 {
        &self.idatar3
    }
    #[doc = "0x50 - Input Data 0 Register 4"]
    #[inline(always)]
    pub const fn idatar4(&self) -> &Idatar4 {
        &self.idatar4
    }
    #[doc = "0x54 - Input Data 0 Register 5"]
    #[inline(always)]
    pub const fn idatar5(&self) -> &Idatar5 {
        &self.idatar5
    }
    #[doc = "0x58 - Input Data 0 Register 6"]
    #[inline(always)]
    pub const fn idatar6(&self) -> &Idatar6 {
        &self.idatar6
    }
    #[doc = "0x5c - Input Data 0 Register 7"]
    #[inline(always)]
    pub const fn idatar7(&self) -> &Idatar7 {
        &self.idatar7
    }
    #[doc = "0x60 - Input Data 0 Register 8"]
    #[inline(always)]
    pub const fn idatar8(&self) -> &Idatar8 {
        &self.idatar8
    }
    #[doc = "0x64 - Input Data 0 Register 9"]
    #[inline(always)]
    pub const fn idatar9(&self) -> &Idatar9 {
        &self.idatar9
    }
    #[doc = "0x68 - Input Data 0 Register 10"]
    #[inline(always)]
    pub const fn idatar10(&self) -> &Idatar10 {
        &self.idatar10
    }
    #[doc = "0x6c - Input Data 0 Register 11"]
    #[inline(always)]
    pub const fn idatar11(&self) -> &Idatar11 {
        &self.idatar11
    }
    #[doc = "0x70 - Input Data 0 Register 12"]
    #[inline(always)]
    pub const fn idatar12(&self) -> &Idatar12 {
        &self.idatar12
    }
    #[doc = "0x74 - Input Data 0 Register 13"]
    #[inline(always)]
    pub const fn idatar13(&self) -> &Idatar13 {
        &self.idatar13
    }
    #[doc = "0x78 - Input Data 0 Register 14"]
    #[inline(always)]
    pub const fn idatar14(&self) -> &Idatar14 {
        &self.idatar14
    }
    #[doc = "0x7c - Input Data 0 Register 15"]
    #[inline(always)]
    pub const fn idatar15(&self) -> &Idatar15 {
        &self.idatar15
    }
    #[doc = "0x80 - Input/Output Data 0 Register 0"]
    #[inline(always)]
    pub const fn iodatar0(&self) -> &Iodatar0 {
        &self.iodatar0
    }
    #[doc = "0x84 - Input/Output Data 0 Register 1"]
    #[inline(always)]
    pub const fn iodatar1(&self) -> &Iodatar1 {
        &self.iodatar1
    }
    #[doc = "0x88 - Input/Output Data 0 Register 2"]
    #[inline(always)]
    pub const fn iodatar2(&self) -> &Iodatar2 {
        &self.iodatar2
    }
    #[doc = "0x8c - Input/Output Data 0 Register 3"]
    #[inline(always)]
    pub const fn iodatar3(&self) -> &Iodatar3 {
        &self.iodatar3
    }
    #[doc = "0x90 - Input/Output Data 0 Register 4"]
    #[inline(always)]
    pub const fn iodatar4(&self) -> &Iodatar4 {
        &self.iodatar4
    }
    #[doc = "0x94 - Input/Output Data 0 Register 5"]
    #[inline(always)]
    pub const fn iodatar5(&self) -> &Iodatar5 {
        &self.iodatar5
    }
    #[doc = "0x98 - Input/Output Data 0 Register 6"]
    #[inline(always)]
    pub const fn iodatar6(&self) -> &Iodatar6 {
        &self.iodatar6
    }
    #[doc = "0x9c - Input/Output Data 0 Register 7"]
    #[inline(always)]
    pub const fn iodatar7(&self) -> &Iodatar7 {
        &self.iodatar7
    }
    #[doc = "0xa0 - Input/Output Data 0 Register 8"]
    #[inline(always)]
    pub const fn iodatar8(&self) -> &Iodatar8 {
        &self.iodatar8
    }
    #[doc = "0xa4 - Input/Output Data 0 Register 9"]
    #[inline(always)]
    pub const fn iodatar9(&self) -> &Iodatar9 {
        &self.iodatar9
    }
    #[doc = "0xa8 - Input/Output Data 0 Register 10"]
    #[inline(always)]
    pub const fn iodatar10(&self) -> &Iodatar10 {
        &self.iodatar10
    }
    #[doc = "0xac - Input/Output Data 0 Register 11"]
    #[inline(always)]
    pub const fn iodatar11(&self) -> &Iodatar11 {
        &self.iodatar11
    }
    #[doc = "0xb0 - Input/Output Data 0 Register 12"]
    #[inline(always)]
    pub const fn iodatar12(&self) -> &Iodatar12 {
        &self.iodatar12
    }
    #[doc = "0xb4 - Input/Output Data 0 Register 13"]
    #[inline(always)]
    pub const fn iodatar13(&self) -> &Iodatar13 {
        &self.iodatar13
    }
    #[doc = "0xb8 - Input/Output Data 0 Register 14"]
    #[inline(always)]
    pub const fn iodatar14(&self) -> &Iodatar14 {
        &self.iodatar14
    }
    #[doc = "0xbc - Input/Output Data 0 Register 15"]
    #[inline(always)]
    pub const fn iodatar15(&self) -> &Iodatar15 {
        &self.iodatar15
    }
    #[doc = "0xe4 - Write Protection Mode Register"]
    #[inline(always)]
    pub const fn wpmr(&self) -> &Wpmr {
        &self.wpmr
    }
    #[doc = "0xe8 - Write Protection Status Register"]
    #[inline(always)]
    pub const fn wpsr(&self) -> &Wpsr {
        &self.wpsr
    }
}
#[doc = "CR (w) register accessor: Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "MR (rw) register accessor: Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mr`] module"]
#[doc(alias = "MR")]
pub type Mr = crate::Reg<mr::MrSpec>;
#[doc = "Mode Register"]
pub mod mr;
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
#[doc = "MSR (rw) register accessor: Message Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`msr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msr`] module"]
#[doc(alias = "MSR")]
pub type Msr = crate::Reg<msr::MsrSpec>;
#[doc = "Message Size Register"]
pub mod msr;
#[doc = "BCR (rw) register accessor: Bytes Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcr`] module"]
#[doc(alias = "BCR")]
pub type Bcr = crate::Reg<bcr::BcrSpec>;
#[doc = "Bytes Count Register"]
pub mod bcr;
#[doc = "IDATAR0 (w) register accessor: Input Data 0 Register 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idatar0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idatar0`] module"]
#[doc(alias = "IDATAR0")]
pub type Idatar0 = crate::Reg<idatar0::Idatar0Spec>;
#[doc = "Input Data 0 Register 0"]
pub mod idatar0;
#[doc = "IDATAR1 (w) register accessor: Input Data 0 Register 1\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idatar1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idatar1`] module"]
#[doc(alias = "IDATAR1")]
pub type Idatar1 = crate::Reg<idatar1::Idatar1Spec>;
#[doc = "Input Data 0 Register 1"]
pub mod idatar1;
#[doc = "IDATAR2 (w) register accessor: Input Data 0 Register 2\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idatar2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idatar2`] module"]
#[doc(alias = "IDATAR2")]
pub type Idatar2 = crate::Reg<idatar2::Idatar2Spec>;
#[doc = "Input Data 0 Register 2"]
pub mod idatar2;
#[doc = "IDATAR3 (w) register accessor: Input Data 0 Register 3\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idatar3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idatar3`] module"]
#[doc(alias = "IDATAR3")]
pub type Idatar3 = crate::Reg<idatar3::Idatar3Spec>;
#[doc = "Input Data 0 Register 3"]
pub mod idatar3;
#[doc = "IDATAR4 (w) register accessor: Input Data 0 Register 4\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idatar4::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idatar4`] module"]
#[doc(alias = "IDATAR4")]
pub type Idatar4 = crate::Reg<idatar4::Idatar4Spec>;
#[doc = "Input Data 0 Register 4"]
pub mod idatar4;
#[doc = "IDATAR5 (w) register accessor: Input Data 0 Register 5\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idatar5::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idatar5`] module"]
#[doc(alias = "IDATAR5")]
pub type Idatar5 = crate::Reg<idatar5::Idatar5Spec>;
#[doc = "Input Data 0 Register 5"]
pub mod idatar5;
#[doc = "IDATAR6 (w) register accessor: Input Data 0 Register 6\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idatar6::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idatar6`] module"]
#[doc(alias = "IDATAR6")]
pub type Idatar6 = crate::Reg<idatar6::Idatar6Spec>;
#[doc = "Input Data 0 Register 6"]
pub mod idatar6;
#[doc = "IDATAR7 (w) register accessor: Input Data 0 Register 7\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idatar7::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idatar7`] module"]
#[doc(alias = "IDATAR7")]
pub type Idatar7 = crate::Reg<idatar7::Idatar7Spec>;
#[doc = "Input Data 0 Register 7"]
pub mod idatar7;
#[doc = "IDATAR8 (w) register accessor: Input Data 0 Register 8\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idatar8::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idatar8`] module"]
#[doc(alias = "IDATAR8")]
pub type Idatar8 = crate::Reg<idatar8::Idatar8Spec>;
#[doc = "Input Data 0 Register 8"]
pub mod idatar8;
#[doc = "IDATAR9 (w) register accessor: Input Data 0 Register 9\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idatar9::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idatar9`] module"]
#[doc(alias = "IDATAR9")]
pub type Idatar9 = crate::Reg<idatar9::Idatar9Spec>;
#[doc = "Input Data 0 Register 9"]
pub mod idatar9;
#[doc = "IDATAR10 (w) register accessor: Input Data 0 Register 10\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idatar10::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idatar10`] module"]
#[doc(alias = "IDATAR10")]
pub type Idatar10 = crate::Reg<idatar10::Idatar10Spec>;
#[doc = "Input Data 0 Register 10"]
pub mod idatar10;
#[doc = "IDATAR11 (w) register accessor: Input Data 0 Register 11\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idatar11::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idatar11`] module"]
#[doc(alias = "IDATAR11")]
pub type Idatar11 = crate::Reg<idatar11::Idatar11Spec>;
#[doc = "Input Data 0 Register 11"]
pub mod idatar11;
#[doc = "IDATAR12 (w) register accessor: Input Data 0 Register 12\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idatar12::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idatar12`] module"]
#[doc(alias = "IDATAR12")]
pub type Idatar12 = crate::Reg<idatar12::Idatar12Spec>;
#[doc = "Input Data 0 Register 12"]
pub mod idatar12;
#[doc = "IDATAR13 (w) register accessor: Input Data 0 Register 13\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idatar13::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idatar13`] module"]
#[doc(alias = "IDATAR13")]
pub type Idatar13 = crate::Reg<idatar13::Idatar13Spec>;
#[doc = "Input Data 0 Register 13"]
pub mod idatar13;
#[doc = "IDATAR14 (w) register accessor: Input Data 0 Register 14\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idatar14::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idatar14`] module"]
#[doc(alias = "IDATAR14")]
pub type Idatar14 = crate::Reg<idatar14::Idatar14Spec>;
#[doc = "Input Data 0 Register 14"]
pub mod idatar14;
#[doc = "IDATAR15 (w) register accessor: Input Data 0 Register 15\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idatar15::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idatar15`] module"]
#[doc(alias = "IDATAR15")]
pub type Idatar15 = crate::Reg<idatar15::Idatar15Spec>;
#[doc = "Input Data 0 Register 15"]
pub mod idatar15;
#[doc = "IODATAR0 (rw) register accessor: Input/Output Data 0 Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`iodatar0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iodatar0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iodatar0`] module"]
#[doc(alias = "IODATAR0")]
pub type Iodatar0 = crate::Reg<iodatar0::Iodatar0Spec>;
#[doc = "Input/Output Data 0 Register 0"]
pub mod iodatar0;
#[doc = "IODATAR1 (rw) register accessor: Input/Output Data 0 Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`iodatar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iodatar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iodatar1`] module"]
#[doc(alias = "IODATAR1")]
pub type Iodatar1 = crate::Reg<iodatar1::Iodatar1Spec>;
#[doc = "Input/Output Data 0 Register 1"]
pub mod iodatar1;
#[doc = "IODATAR2 (rw) register accessor: Input/Output Data 0 Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`iodatar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iodatar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iodatar2`] module"]
#[doc(alias = "IODATAR2")]
pub type Iodatar2 = crate::Reg<iodatar2::Iodatar2Spec>;
#[doc = "Input/Output Data 0 Register 2"]
pub mod iodatar2;
#[doc = "IODATAR3 (rw) register accessor: Input/Output Data 0 Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`iodatar3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iodatar3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iodatar3`] module"]
#[doc(alias = "IODATAR3")]
pub type Iodatar3 = crate::Reg<iodatar3::Iodatar3Spec>;
#[doc = "Input/Output Data 0 Register 3"]
pub mod iodatar3;
#[doc = "IODATAR4 (rw) register accessor: Input/Output Data 0 Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`iodatar4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iodatar4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iodatar4`] module"]
#[doc(alias = "IODATAR4")]
pub type Iodatar4 = crate::Reg<iodatar4::Iodatar4Spec>;
#[doc = "Input/Output Data 0 Register 4"]
pub mod iodatar4;
#[doc = "IODATAR5 (rw) register accessor: Input/Output Data 0 Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`iodatar5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iodatar5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iodatar5`] module"]
#[doc(alias = "IODATAR5")]
pub type Iodatar5 = crate::Reg<iodatar5::Iodatar5Spec>;
#[doc = "Input/Output Data 0 Register 5"]
pub mod iodatar5;
#[doc = "IODATAR6 (rw) register accessor: Input/Output Data 0 Register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`iodatar6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iodatar6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iodatar6`] module"]
#[doc(alias = "IODATAR6")]
pub type Iodatar6 = crate::Reg<iodatar6::Iodatar6Spec>;
#[doc = "Input/Output Data 0 Register 6"]
pub mod iodatar6;
#[doc = "IODATAR7 (rw) register accessor: Input/Output Data 0 Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`iodatar7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iodatar7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iodatar7`] module"]
#[doc(alias = "IODATAR7")]
pub type Iodatar7 = crate::Reg<iodatar7::Iodatar7Spec>;
#[doc = "Input/Output Data 0 Register 7"]
pub mod iodatar7;
#[doc = "IODATAR8 (rw) register accessor: Input/Output Data 0 Register 8\n\nYou can [`read`](crate::Reg::read) this register and get [`iodatar8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iodatar8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iodatar8`] module"]
#[doc(alias = "IODATAR8")]
pub type Iodatar8 = crate::Reg<iodatar8::Iodatar8Spec>;
#[doc = "Input/Output Data 0 Register 8"]
pub mod iodatar8;
#[doc = "IODATAR9 (rw) register accessor: Input/Output Data 0 Register 9\n\nYou can [`read`](crate::Reg::read) this register and get [`iodatar9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iodatar9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iodatar9`] module"]
#[doc(alias = "IODATAR9")]
pub type Iodatar9 = crate::Reg<iodatar9::Iodatar9Spec>;
#[doc = "Input/Output Data 0 Register 9"]
pub mod iodatar9;
#[doc = "IODATAR10 (rw) register accessor: Input/Output Data 0 Register 10\n\nYou can [`read`](crate::Reg::read) this register and get [`iodatar10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iodatar10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iodatar10`] module"]
#[doc(alias = "IODATAR10")]
pub type Iodatar10 = crate::Reg<iodatar10::Iodatar10Spec>;
#[doc = "Input/Output Data 0 Register 10"]
pub mod iodatar10;
#[doc = "IODATAR11 (rw) register accessor: Input/Output Data 0 Register 11\n\nYou can [`read`](crate::Reg::read) this register and get [`iodatar11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iodatar11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iodatar11`] module"]
#[doc(alias = "IODATAR11")]
pub type Iodatar11 = crate::Reg<iodatar11::Iodatar11Spec>;
#[doc = "Input/Output Data 0 Register 11"]
pub mod iodatar11;
#[doc = "IODATAR12 (rw) register accessor: Input/Output Data 0 Register 12\n\nYou can [`read`](crate::Reg::read) this register and get [`iodatar12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iodatar12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iodatar12`] module"]
#[doc(alias = "IODATAR12")]
pub type Iodatar12 = crate::Reg<iodatar12::Iodatar12Spec>;
#[doc = "Input/Output Data 0 Register 12"]
pub mod iodatar12;
#[doc = "IODATAR13 (rw) register accessor: Input/Output Data 0 Register 13\n\nYou can [`read`](crate::Reg::read) this register and get [`iodatar13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iodatar13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iodatar13`] module"]
#[doc(alias = "IODATAR13")]
pub type Iodatar13 = crate::Reg<iodatar13::Iodatar13Spec>;
#[doc = "Input/Output Data 0 Register 13"]
pub mod iodatar13;
#[doc = "IODATAR14 (rw) register accessor: Input/Output Data 0 Register 14\n\nYou can [`read`](crate::Reg::read) this register and get [`iodatar14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iodatar14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iodatar14`] module"]
#[doc(alias = "IODATAR14")]
pub type Iodatar14 = crate::Reg<iodatar14::Iodatar14Spec>;
#[doc = "Input/Output Data 0 Register 14"]
pub mod iodatar14;
#[doc = "IODATAR15 (rw) register accessor: Input/Output Data 0 Register 15\n\nYou can [`read`](crate::Reg::read) this register and get [`iodatar15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iodatar15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iodatar15`] module"]
#[doc(alias = "IODATAR15")]
pub type Iodatar15 = crate::Reg<iodatar15::Iodatar15Spec>;
#[doc = "Input/Output Data 0 Register 15"]
pub mod iodatar15;
#[doc = "WPMR (rw) register accessor: Write Protection Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpmr`] module"]
#[doc(alias = "WPMR")]
pub type Wpmr = crate::Reg<wpmr::WpmrSpec>;
#[doc = "Write Protection Mode Register"]
pub mod wpmr;
#[doc = "WPSR (r) register accessor: Write Protection Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpsr`] module"]
#[doc(alias = "WPSR")]
pub type Wpsr = crate::Reg<wpsr::WpsrSpec>;
#[doc = "Write Protection Status Register"]
pub mod wpsr;
