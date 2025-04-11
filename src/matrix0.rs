#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mcfg: [Mcfg; 16],
    scfg: [Scfg; 16],
    matrix_pr: [MatrixPr; 16],
    mrcr: Mrcr,
    _reserved4: [u8; 0x4c],
    meier: Meier,
    meidr: Meidr,
    meimr: Meimr,
    mesr: Mesr,
    mear: [Mear; 16],
    _reserved9: [u8; 0x44],
    wpmr: Wpmr,
    wpsr: Wpsr,
    _reserved11: [u8; 0x14],
    psr: [Psr; 16],
    passr: [Passr; 16],
    prtsr: [Prtsr; 16],
    ppselr: [Ppselr; 3],
}
impl RegisterBlock {
    #[doc = "0x00..0x40 - Master Configuration Register"]
    #[inline(always)]
    pub const fn mcfg(&self, n: usize) -> &Mcfg {
        &self.mcfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x40 - Master Configuration Register"]
    #[inline(always)]
    pub fn mcfg_iter(&self) -> impl Iterator<Item = &Mcfg> {
        self.mcfg.iter()
    }
    #[doc = "0x40..0x80 - Slave Configuration Register"]
    #[inline(always)]
    pub const fn scfg(&self, n: usize) -> &Scfg {
        &self.scfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x80 - Slave Configuration Register"]
    #[inline(always)]
    pub fn scfg_iter(&self) -> impl Iterator<Item = &Scfg> {
        self.scfg.iter()
    }
    #[doc = "0x80..0x100 - MATRIX_PR\\[%s\\]"]
    #[inline(always)]
    pub const fn matrix_pr(&self, n: usize) -> &MatrixPr {
        &self.matrix_pr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x100 - MATRIX_PR\\[%s\\]"]
    #[inline(always)]
    pub fn matrix_pr_iter(&self) -> impl Iterator<Item = &MatrixPr> {
        self.matrix_pr.iter()
    }
    #[doc = "0x100 - Master Remap Control Register"]
    #[inline(always)]
    pub const fn mrcr(&self) -> &Mrcr {
        &self.mrcr
    }
    #[doc = "0x150 - Master Error Interrupt Enable Register"]
    #[inline(always)]
    pub const fn meier(&self) -> &Meier {
        &self.meier
    }
    #[doc = "0x154 - Master Error Interrupt Disable Register"]
    #[inline(always)]
    pub const fn meidr(&self) -> &Meidr {
        &self.meidr
    }
    #[doc = "0x158 - Master Error Interrupt Mask Register"]
    #[inline(always)]
    pub const fn meimr(&self) -> &Meimr {
        &self.meimr
    }
    #[doc = "0x15c - Master Error Status Register"]
    #[inline(always)]
    pub const fn mesr(&self) -> &Mesr {
        &self.mesr
    }
    #[doc = "0x160..0x1a0 - Master 0 Error Address Register"]
    #[inline(always)]
    pub const fn mear(&self, n: usize) -> &Mear {
        &self.mear[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x160..0x1a0 - Master 0 Error Address Register"]
    #[inline(always)]
    pub fn mear_iter(&self) -> impl Iterator<Item = &Mear> {
        self.mear.iter()
    }
    #[doc = "0x1e4 - Write Protect Mode Register"]
    #[inline(always)]
    pub const fn wpmr(&self) -> &Wpmr {
        &self.wpmr
    }
    #[doc = "0x1e8 - Write Protect Status Register"]
    #[inline(always)]
    pub const fn wpsr(&self) -> &Wpsr {
        &self.wpsr
    }
    #[doc = "0x200..0x240 - Protection Slave Register"]
    #[inline(always)]
    pub const fn psr(&self, n: usize) -> &Psr {
        &self.psr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x200..0x240 - Protection Slave Register"]
    #[inline(always)]
    pub fn psr_iter(&self) -> impl Iterator<Item = &Psr> {
        self.psr.iter()
    }
    #[doc = "0x240..0x280 - Protected Areas Split Slave Register"]
    #[inline(always)]
    pub const fn passr(&self, n: usize) -> &Passr {
        &self.passr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x240..0x280 - Protected Areas Split Slave Register"]
    #[inline(always)]
    pub fn passr_iter(&self) -> impl Iterator<Item = &Passr> {
        self.passr.iter()
    }
    #[doc = "0x280..0x2c0 - Protected Region Top Slave Register"]
    #[inline(always)]
    pub const fn prtsr(&self, n: usize) -> &Prtsr {
        &self.prtsr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x280..0x2c0 - Protected Region Top Slave Register"]
    #[inline(always)]
    pub fn prtsr_iter(&self) -> impl Iterator<Item = &Prtsr> {
        self.prtsr.iter()
    }
    #[doc = "0x2c0..0x2cc - Protected Peripheral Select Register"]
    #[inline(always)]
    pub const fn ppselr(&self, n: usize) -> &Ppselr {
        &self.ppselr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2c0..0x2cc - Protected Peripheral Select Register"]
    #[inline(always)]
    pub fn ppselr_iter(&self) -> impl Iterator<Item = &Ppselr> {
        self.ppselr.iter()
    }
}
#[doc = "MCFG (rw) register accessor: Master Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcfg`] module"]
#[doc(alias = "MCFG")]
pub type Mcfg = crate::Reg<mcfg::McfgSpec>;
#[doc = "Master Configuration Register"]
pub mod mcfg;
#[doc = "SCFG (rw) register accessor: Slave Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scfg`] module"]
#[doc(alias = "SCFG")]
pub type Scfg = crate::Reg<scfg::ScfgSpec>;
#[doc = "Slave Configuration Register"]
pub mod scfg;
#[doc = "MATRIX_PR\\[%s\\]"]
pub use self::matrix_pr::MatrixPr;
#[doc = r"Cluster"]
#[doc = "MATRIX_PR\\[%s\\]"]
pub mod matrix_pr;
#[doc = "MRCR (rw) register accessor: Master Remap Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mrcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mrcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrcr`] module"]
#[doc(alias = "MRCR")]
pub type Mrcr = crate::Reg<mrcr::MrcrSpec>;
#[doc = "Master Remap Control Register"]
pub mod mrcr;
#[doc = "MEIER (w) register accessor: Master Error Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`meier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@meier`] module"]
#[doc(alias = "MEIER")]
pub type Meier = crate::Reg<meier::MeierSpec>;
#[doc = "Master Error Interrupt Enable Register"]
pub mod meier;
#[doc = "MEIDR (w) register accessor: Master Error Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`meidr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@meidr`] module"]
#[doc(alias = "MEIDR")]
pub type Meidr = crate::Reg<meidr::MeidrSpec>;
#[doc = "Master Error Interrupt Disable Register"]
pub mod meidr;
#[doc = "MEIMR (r) register accessor: Master Error Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`meimr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@meimr`] module"]
#[doc(alias = "MEIMR")]
pub type Meimr = crate::Reg<meimr::MeimrSpec>;
#[doc = "Master Error Interrupt Mask Register"]
pub mod meimr;
#[doc = "MESR (r) register accessor: Master Error Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mesr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mesr`] module"]
#[doc(alias = "MESR")]
pub type Mesr = crate::Reg<mesr::MesrSpec>;
#[doc = "Master Error Status Register"]
pub mod mesr;
#[doc = "MEAR (r) register accessor: Master 0 Error Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mear::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mear`] module"]
#[doc(alias = "MEAR")]
pub type Mear = crate::Reg<mear::MearSpec>;
#[doc = "Master 0 Error Address Register"]
pub mod mear;
#[doc = "WPMR (rw) register accessor: Write Protect Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpmr`] module"]
#[doc(alias = "WPMR")]
pub type Wpmr = crate::Reg<wpmr::WpmrSpec>;
#[doc = "Write Protect Mode Register"]
pub mod wpmr;
#[doc = "WPSR (r) register accessor: Write Protect Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpsr`] module"]
#[doc(alias = "WPSR")]
pub type Wpsr = crate::Reg<wpsr::WpsrSpec>;
#[doc = "Write Protect Status Register"]
pub mod wpsr;
#[doc = "PSR (rw) register accessor: Protection Slave Register\n\nYou can [`read`](crate::Reg::read) this register and get [`psr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psr`] module"]
#[doc(alias = "PSR")]
pub type Psr = crate::Reg<psr::PsrSpec>;
#[doc = "Protection Slave Register"]
pub mod psr;
#[doc = "PASSR (rw) register accessor: Protected Areas Split Slave Register\n\nYou can [`read`](crate::Reg::read) this register and get [`passr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`passr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@passr`] module"]
#[doc(alias = "PASSR")]
pub type Passr = crate::Reg<passr::PassrSpec>;
#[doc = "Protected Areas Split Slave Register"]
pub mod passr;
#[doc = "PRTSR (rw) register accessor: Protected Region Top Slave Register\n\nYou can [`read`](crate::Reg::read) this register and get [`prtsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prtsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prtsr`] module"]
#[doc(alias = "PRTSR")]
pub type Prtsr = crate::Reg<prtsr::PrtsrSpec>;
#[doc = "Protected Region Top Slave Register"]
pub mod prtsr;
#[doc = "PPSELR (rw) register accessor: Protected Peripheral Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ppselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppselr`] module"]
#[doc(alias = "PPSELR")]
pub type Ppselr = crate::Reg<ppselr::PpselrSpec>;
#[doc = "Protected Peripheral Select Register"]
pub mod ppselr;
