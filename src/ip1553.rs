#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    _reserved1: [u8; 0x08],
    cmdr1: Cmdr1,
    cmdr2: Cmdr2,
    cmdr3: Cmdr3,
    bitr: Bitr,
    vwr: Vwr,
    ier: Ier,
    idr: Idr,
    imr: Imr,
    isr: Isr,
    ctrl1: Ctrl1,
    ctrl2: Ctrl2,
    ctrl3: Ctrl3,
    arw: Arw,
    arr: Arr,
    rxbsr: Rxbsr,
    rxbaer: Rxbaer,
    txbsr: Txbsr,
    txbaer: Txbaer,
}
impl RegisterBlock {
    #[doc = "0x00 - Configuration Register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x0c - Command Register 1"]
    #[inline(always)]
    pub const fn cmdr1(&self) -> &Cmdr1 {
        &self.cmdr1
    }
    #[doc = "0x10 - Command Register 2"]
    #[inline(always)]
    pub const fn cmdr2(&self) -> &Cmdr2 {
        &self.cmdr2
    }
    #[doc = "0x14 - Command Register 3"]
    #[inline(always)]
    pub const fn cmdr3(&self) -> &Cmdr3 {
        &self.cmdr3
    }
    #[doc = "0x18 - BIT Register"]
    #[inline(always)]
    pub const fn bitr(&self) -> &Bitr {
        &self.bitr
    }
    #[doc = "0x1c - Vector Word Register"]
    #[inline(always)]
    pub const fn vwr(&self) -> &Vwr {
        &self.vwr
    }
    #[doc = "0x20 - IRQ Mask Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x24 - IRQ Mask Disable Register"]
    #[inline(always)]
    pub const fn idr(&self) -> &Idr {
        &self.idr
    }
    #[doc = "0x28 - IRQ Mask Register"]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        &self.imr
    }
    #[doc = "0x2c - IRQ Status Register"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x30 - Control Register 1"]
    #[inline(always)]
    pub const fn ctrl1(&self) -> &Ctrl1 {
        &self.ctrl1
    }
    #[doc = "0x34 - Control Register 2"]
    #[inline(always)]
    pub const fn ctrl2(&self) -> &Ctrl2 {
        &self.ctrl2
    }
    #[doc = "0x38 - Control Register 3"]
    #[inline(always)]
    pub const fn ctrl3(&self) -> &Ctrl3 {
        &self.ctrl3
    }
    #[doc = "0x3c - Address Register Write"]
    #[inline(always)]
    pub const fn arw(&self) -> &Arw {
        &self.arw
    }
    #[doc = "0x40 - Address Register Read"]
    #[inline(always)]
    pub const fn arr(&self) -> &Arr {
        &self.arr
    }
    #[doc = "0x44 - Rx Buffer Status Register"]
    #[inline(always)]
    pub const fn rxbsr(&self) -> &Rxbsr {
        &self.rxbsr
    }
    #[doc = "0x48 - Rx Buffer Access Error Register"]
    #[inline(always)]
    pub const fn rxbaer(&self) -> &Rxbaer {
        &self.rxbaer
    }
    #[doc = "0x4c - Tx Buffer Status Register"]
    #[inline(always)]
    pub const fn txbsr(&self) -> &Txbsr {
        &self.txbsr
    }
    #[doc = "0x50 - Tx Buffer Access Error Register"]
    #[inline(always)]
    pub const fn txbaer(&self) -> &Txbaer {
        &self.txbaer
    }
}
#[doc = "CR (rw) register accessor: Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Configuration Register"]
pub mod cr;
#[doc = "CMDR1 (w) register accessor: Command Register 1\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdr1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdr1`] module"]
#[doc(alias = "CMDR1")]
pub type Cmdr1 = crate::Reg<cmdr1::Cmdr1Spec>;
#[doc = "Command Register 1"]
pub mod cmdr1;
#[doc = "CMDR2 (w) register accessor: Command Register 2\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdr2`] module"]
#[doc(alias = "CMDR2")]
pub type Cmdr2 = crate::Reg<cmdr2::Cmdr2Spec>;
#[doc = "Command Register 2"]
pub mod cmdr2;
#[doc = "CMDR3 (w) register accessor: Command Register 3\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdr3`] module"]
#[doc(alias = "CMDR3")]
pub type Cmdr3 = crate::Reg<cmdr3::Cmdr3Spec>;
#[doc = "Command Register 3"]
pub mod cmdr3;
#[doc = "BITR (w) register accessor: BIT Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bitr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bitr`] module"]
#[doc(alias = "BITR")]
pub type Bitr = crate::Reg<bitr::BitrSpec>;
#[doc = "BIT Register"]
pub mod bitr;
#[doc = "VWR (w) register accessor: Vector Word Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwr`] module"]
#[doc(alias = "VWR")]
pub type Vwr = crate::Reg<vwr::VwrSpec>;
#[doc = "Vector Word Register"]
pub mod vwr;
#[doc = "IER (w) register accessor: IRQ Mask Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`] module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "IRQ Mask Enable Register"]
pub mod ier;
#[doc = "IDR (w) register accessor: IRQ Mask Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`] module"]
#[doc(alias = "IDR")]
pub type Idr = crate::Reg<idr::IdrSpec>;
#[doc = "IRQ Mask Disable Register"]
pub mod idr;
#[doc = "IMR (r) register accessor: IRQ Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`] module"]
#[doc(alias = "IMR")]
pub type Imr = crate::Reg<imr::ImrSpec>;
#[doc = "IRQ Mask Register"]
pub mod imr;
#[doc = "ISR (r) register accessor: IRQ Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`] module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "IRQ Status Register"]
pub mod isr;
#[doc = "CTRL1 (r) register accessor: Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl1`] module"]
#[doc(alias = "CTRL1")]
pub type Ctrl1 = crate::Reg<ctrl1::Ctrl1Spec>;
#[doc = "Control Register 1"]
pub mod ctrl1;
#[doc = "CTRL2 (r) register accessor: Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl2`] module"]
#[doc(alias = "CTRL2")]
pub type Ctrl2 = crate::Reg<ctrl2::Ctrl2Spec>;
#[doc = "Control Register 2"]
pub mod ctrl2;
#[doc = "CTRL3 (r) register accessor: Control Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl3`] module"]
#[doc(alias = "CTRL3")]
pub type Ctrl3 = crate::Reg<ctrl3::Ctrl3Spec>;
#[doc = "Control Register 3"]
pub mod ctrl3;
#[doc = "ARW (rw) register accessor: Address Register Write\n\nYou can [`read`](crate::Reg::read) this register and get [`arw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arw`] module"]
#[doc(alias = "ARW")]
pub type Arw = crate::Reg<arw::ArwSpec>;
#[doc = "Address Register Write"]
pub mod arw;
#[doc = "ARR (rw) register accessor: Address Register Read\n\nYou can [`read`](crate::Reg::read) this register and get [`arr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arr`] module"]
#[doc(alias = "ARR")]
pub type Arr = crate::Reg<arr::ArrSpec>;
#[doc = "Address Register Read"]
pub mod arr;
#[doc = "RXBSR (rw) register accessor: Rx Buffer Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxbsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxbsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxbsr`] module"]
#[doc(alias = "RXBSR")]
pub type Rxbsr = crate::Reg<rxbsr::RxbsrSpec>;
#[doc = "Rx Buffer Status Register"]
pub mod rxbsr;
#[doc = "RXBAER (rw) register accessor: Rx Buffer Access Error Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxbaer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxbaer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxbaer`] module"]
#[doc(alias = "RXBAER")]
pub type Rxbaer = crate::Reg<rxbaer::RxbaerSpec>;
#[doc = "Rx Buffer Access Error Register"]
pub mod rxbaer;
#[doc = "TXBSR (rw) register accessor: Tx Buffer Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txbsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbsr`] module"]
#[doc(alias = "TXBSR")]
pub type Txbsr = crate::Reg<txbsr::TxbsrSpec>;
#[doc = "Tx Buffer Status Register"]
pub mod txbsr;
#[doc = "TXBAER (rw) register accessor: Tx Buffer Access Error Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txbaer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbaer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbaer`] module"]
#[doc(alias = "TXBAER")]
pub type Txbaer = crate::Reg<txbaer::TxbaerSpec>;
#[doc = "Tx Buffer Access Error Register"]
pub mod txbaer;
