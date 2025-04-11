#[repr(C)]
#[doc = "PIO_GROUP\\[%s\\]"]
#[doc(alias = "PIO_GROUP")]
pub struct PioGroup {
    mskr: Mskr,
    cfgr: Cfgr,
    pdsr: Pdsr,
    locksr: Locksr,
    sodr: Sodr,
    codr: Codr,
    odsr: Odsr,
    _reserved7: [u8; 0x04],
    ier: Ier,
    idr: Idr,
    imr: Imr,
    isr: Isr,
    _reserved11: [u8; 0x0c],
    iofr: Iofr,
}
impl PioGroup {
    #[doc = "0x00 - PIO Mask Register"]
    #[inline(always)]
    pub const fn mskr(&self) -> &Mskr {
        &self.mskr
    }
    #[doc = "0x04 - PIO Configuration Register"]
    #[inline(always)]
    pub const fn cfgr(&self) -> &Cfgr {
        &self.cfgr
    }
    #[doc = "0x08 - PIO Pin Data Status Register"]
    #[inline(always)]
    pub const fn pdsr(&self) -> &Pdsr {
        &self.pdsr
    }
    #[doc = "0x0c - PIO Lock Status Register"]
    #[inline(always)]
    pub const fn locksr(&self) -> &Locksr {
        &self.locksr
    }
    #[doc = "0x10 - PIO Set Output Data Register"]
    #[inline(always)]
    pub const fn sodr(&self) -> &Sodr {
        &self.sodr
    }
    #[doc = "0x14 - PIO Clear Output Data Register"]
    #[inline(always)]
    pub const fn codr(&self) -> &Codr {
        &self.codr
    }
    #[doc = "0x18 - PIO Output Data Status Register"]
    #[inline(always)]
    pub const fn odsr(&self) -> &Odsr {
        &self.odsr
    }
    #[doc = "0x20 - PIO Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x24 - PIO Interrupt Disable Register"]
    #[inline(always)]
    pub const fn idr(&self) -> &Idr {
        &self.idr
    }
    #[doc = "0x28 - PIO Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        &self.imr
    }
    #[doc = "0x2c - PIO Interrupt Status Register"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x3c - PIO I/O Freeze Configuration Register"]
    #[inline(always)]
    pub const fn iofr(&self) -> &Iofr {
        &self.iofr
    }
}
#[doc = "MSKR (rw) register accessor: PIO Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mskr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mskr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mskr`] module"]
#[doc(alias = "MSKR")]
pub type Mskr = crate::Reg<mskr::MskrSpec>;
#[doc = "PIO Mask Register"]
pub mod mskr;
#[doc = "CFGR (rw) register accessor: PIO Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr`] module"]
#[doc(alias = "CFGR")]
pub type Cfgr = crate::Reg<cfgr::CfgrSpec>;
#[doc = "PIO Configuration Register"]
pub mod cfgr;
#[doc = "PDSR (r) register accessor: PIO Pin Data Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pdsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdsr`] module"]
#[doc(alias = "PDSR")]
pub type Pdsr = crate::Reg<pdsr::PdsrSpec>;
#[doc = "PIO Pin Data Status Register"]
pub mod pdsr;
#[doc = "LOCKSR (r) register accessor: PIO Lock Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`locksr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@locksr`] module"]
#[doc(alias = "LOCKSR")]
pub type Locksr = crate::Reg<locksr::LocksrSpec>;
#[doc = "PIO Lock Status Register"]
pub mod locksr;
#[doc = "SODR (w) register accessor: PIO Set Output Data Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sodr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sodr`] module"]
#[doc(alias = "SODR")]
pub type Sodr = crate::Reg<sodr::SodrSpec>;
#[doc = "PIO Set Output Data Register"]
pub mod sodr;
#[doc = "CODR (w) register accessor: PIO Clear Output Data Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`codr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@codr`] module"]
#[doc(alias = "CODR")]
pub type Codr = crate::Reg<codr::CodrSpec>;
#[doc = "PIO Clear Output Data Register"]
pub mod codr;
#[doc = "ODSR (rw) register accessor: PIO Output Data Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`odsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@odsr`] module"]
#[doc(alias = "ODSR")]
pub type Odsr = crate::Reg<odsr::OdsrSpec>;
#[doc = "PIO Output Data Status Register"]
pub mod odsr;
#[doc = "IER (w) register accessor: PIO Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`] module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "PIO Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR (w) register accessor: PIO Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`] module"]
#[doc(alias = "IDR")]
pub type Idr = crate::Reg<idr::IdrSpec>;
#[doc = "PIO Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR (r) register accessor: PIO Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`] module"]
#[doc(alias = "IMR")]
pub type Imr = crate::Reg<imr::ImrSpec>;
#[doc = "PIO Interrupt Mask Register"]
pub mod imr;
#[doc = "ISR (r) register accessor: PIO Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`] module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "PIO Interrupt Status Register"]
pub mod isr;
#[doc = "IOFR (w) register accessor: PIO I/O Freeze Configuration Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iofr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iofr`] module"]
#[doc(alias = "IOFR")]
pub type Iofr = crate::Reg<iofr::IofrSpec>;
#[doc = "PIO I/O Freeze Configuration Register"]
pub mod iofr;
