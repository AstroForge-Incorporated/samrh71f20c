#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ier: Ier,
    idr: Idr,
    iar: Iar,
    isr: Isr,
    gfcs: Gfcs,
    scfg0r: Scfg0r,
    scfg1r: Scfg1r,
    scfg2r: Scfg2r,
    scfg3r: Scfg3r,
    scfg4r: Scfg4r,
    scfg5r: Scfg5r,
    scfg6r: Scfg6r,
    scfg7r: Scfg7r,
    scfg8r: Scfg8r,
    _reserved14: [u8; 0xac],
    wpmr: Wpmr,
    wpsr: Wpsr,
}
impl RegisterBlock {
    #[doc = "0x00 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x04 - Interrupt Disable Register"]
    #[inline(always)]
    pub const fn idr(&self) -> &Idr {
        &self.idr
    }
    #[doc = "0x08 - Interrupt Active Register"]
    #[inline(always)]
    pub const fn iar(&self) -> &Iar {
        &self.iar
    }
    #[doc = "0x0c - Interrupt Status Register"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x10 - Glitch Filter Configuration Status Register"]
    #[inline(always)]
    pub const fn gfcs(&self) -> &Gfcs {
        &self.gfcs
    }
    #[doc = "0x14 - Source Configuration Register 0"]
    #[inline(always)]
    pub const fn scfg0r(&self) -> &Scfg0r {
        &self.scfg0r
    }
    #[doc = "0x18 - Source Configuration Register 1"]
    #[inline(always)]
    pub const fn scfg1r(&self) -> &Scfg1r {
        &self.scfg1r
    }
    #[doc = "0x1c - Source Configuration Register 2"]
    #[inline(always)]
    pub const fn scfg2r(&self) -> &Scfg2r {
        &self.scfg2r
    }
    #[doc = "0x20 - Source Configuration Register 3"]
    #[inline(always)]
    pub const fn scfg3r(&self) -> &Scfg3r {
        &self.scfg3r
    }
    #[doc = "0x24 - Source Configuration Register 4"]
    #[inline(always)]
    pub const fn scfg4r(&self) -> &Scfg4r {
        &self.scfg4r
    }
    #[doc = "0x28 - Source Configuration Register 5"]
    #[inline(always)]
    pub const fn scfg5r(&self) -> &Scfg5r {
        &self.scfg5r
    }
    #[doc = "0x2c - Source Configuration Register 6"]
    #[inline(always)]
    pub const fn scfg6r(&self) -> &Scfg6r {
        &self.scfg6r
    }
    #[doc = "0x30 - Source Configuration Register 7"]
    #[inline(always)]
    pub const fn scfg7r(&self) -> &Scfg7r {
        &self.scfg7r
    }
    #[doc = "0x34 - Source Configuration Register 8"]
    #[inline(always)]
    pub const fn scfg8r(&self) -> &Scfg8r {
        &self.scfg8r
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
#[doc = "IAR (r) register accessor: Interrupt Active Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iar`] module"]
#[doc(alias = "IAR")]
pub type Iar = crate::Reg<iar::IarSpec>;
#[doc = "Interrupt Active Register"]
pub mod iar;
#[doc = "ISR (r) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`] module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "GFCS (r) register accessor: Glitch Filter Configuration Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gfcs::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gfcs`] module"]
#[doc(alias = "GFCS")]
pub type Gfcs = crate::Reg<gfcs::GfcsSpec>;
#[doc = "Glitch Filter Configuration Status Register"]
pub mod gfcs;
#[doc = "SCFG0R (rw) register accessor: Source Configuration Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`scfg0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scfg0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scfg0r`] module"]
#[doc(alias = "SCFG0R")]
pub type Scfg0r = crate::Reg<scfg0r::Scfg0rSpec>;
#[doc = "Source Configuration Register 0"]
pub mod scfg0r;
#[doc = "SCFG1R (rw) register accessor: Source Configuration Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scfg1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scfg1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scfg1r`] module"]
#[doc(alias = "SCFG1R")]
pub type Scfg1r = crate::Reg<scfg1r::Scfg1rSpec>;
#[doc = "Source Configuration Register 1"]
pub mod scfg1r;
#[doc = "SCFG2R (rw) register accessor: Source Configuration Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scfg2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scfg2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scfg2r`] module"]
#[doc(alias = "SCFG2R")]
pub type Scfg2r = crate::Reg<scfg2r::Scfg2rSpec>;
#[doc = "Source Configuration Register 2"]
pub mod scfg2r;
#[doc = "SCFG3R (rw) register accessor: Source Configuration Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`scfg3r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scfg3r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scfg3r`] module"]
#[doc(alias = "SCFG3R")]
pub type Scfg3r = crate::Reg<scfg3r::Scfg3rSpec>;
#[doc = "Source Configuration Register 3"]
pub mod scfg3r;
#[doc = "SCFG4R (rw) register accessor: Source Configuration Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`scfg4r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scfg4r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scfg4r`] module"]
#[doc(alias = "SCFG4R")]
pub type Scfg4r = crate::Reg<scfg4r::Scfg4rSpec>;
#[doc = "Source Configuration Register 4"]
pub mod scfg4r;
#[doc = "SCFG5R (rw) register accessor: Source Configuration Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`scfg5r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scfg5r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scfg5r`] module"]
#[doc(alias = "SCFG5R")]
pub type Scfg5r = crate::Reg<scfg5r::Scfg5rSpec>;
#[doc = "Source Configuration Register 5"]
pub mod scfg5r;
#[doc = "SCFG6R (rw) register accessor: Source Configuration Register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`scfg6r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scfg6r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scfg6r`] module"]
#[doc(alias = "SCFG6R")]
pub type Scfg6r = crate::Reg<scfg6r::Scfg6rSpec>;
#[doc = "Source Configuration Register 6"]
pub mod scfg6r;
#[doc = "SCFG7R (rw) register accessor: Source Configuration Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`scfg7r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scfg7r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scfg7r`] module"]
#[doc(alias = "SCFG7R")]
pub type Scfg7r = crate::Reg<scfg7r::Scfg7rSpec>;
#[doc = "Source Configuration Register 7"]
pub mod scfg7r;
#[doc = "SCFG8R (rw) register accessor: Source Configuration Register 8\n\nYou can [`read`](crate::Reg::read) this register and get [`scfg8r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scfg8r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scfg8r`] module"]
#[doc(alias = "SCFG8R")]
pub type Scfg8r = crate::Reg<scfg8r::Scfg8rSpec>;
#[doc = "Source Configuration Register 8"]
pub mod scfg8r;
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
