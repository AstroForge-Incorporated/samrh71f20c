#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pio_group: [PioGroup; 7],
    _reserved1: [u8; 0x0340],
    scdr: Scdr,
    _reserved2: [u8; 0xdc],
    wpmr: Wpmr,
    wpsr: Wpsr,
}
impl RegisterBlock {
    #[doc = "0x00..0x1c0 - PIO_GROUP\\[%s\\]"]
    #[inline(always)]
    pub const fn pio_group(&self, n: usize) -> &PioGroup {
        &self.pio_group[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x1c0 - PIO_GROUP\\[%s\\]"]
    #[inline(always)]
    pub fn pio_group_iter(&self) -> impl Iterator<Item = &PioGroup> {
        self.pio_group.iter()
    }
    #[doc = "0x500 - PIO Slow Clock Divider Debouncing Register"]
    #[inline(always)]
    pub const fn scdr(&self) -> &Scdr {
        &self.scdr
    }
    #[doc = "0x5e0 - PIO Write Protection Mode Register"]
    #[inline(always)]
    pub const fn wpmr(&self) -> &Wpmr {
        &self.wpmr
    }
    #[doc = "0x5e4 - PIO Write Protection Status Register"]
    #[inline(always)]
    pub const fn wpsr(&self) -> &Wpsr {
        &self.wpsr
    }
}
#[doc = "PIO_GROUP\\[%s\\]"]
pub use self::pio_group::PioGroup;
#[doc = r"Cluster"]
#[doc = "PIO_GROUP\\[%s\\]"]
pub mod pio_group;
#[doc = "SCDR (rw) register accessor: PIO Slow Clock Divider Debouncing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scdr`] module"]
#[doc(alias = "SCDR")]
pub type Scdr = crate::Reg<scdr::ScdrSpec>;
#[doc = "PIO Slow Clock Divider Debouncing Register"]
pub mod scdr;
#[doc = "WPMR (rw) register accessor: PIO Write Protection Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpmr`] module"]
#[doc(alias = "WPMR")]
pub type Wpmr = crate::Reg<wpmr::WpmrSpec>;
#[doc = "PIO Write Protection Mode Register"]
pub mod wpmr;
#[doc = "WPSR (r) register accessor: PIO Write Protection Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpsr`] module"]
#[doc(alias = "WPSR")]
pub type Wpsr = crate::Reg<wpsr::WpsrSpec>;
#[doc = "PIO Write Protection Status Register"]
pub mod wpsr;
