#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    scer: Scer,
    scdr: Scdr,
    scsr: Scsr,
    _reserved3: [u8; 0x14],
    ckgr_mor: CkgrMor,
    ckgr_mcfr: CkgrMcfr,
    ckgr_pllar: CkgrPllar,
    ckgr_pllbr: CkgrPllbr,
    mckr: Mckr,
    _reserved8: [u8; 0x0c],
    pck: [Pck; 4],
    _reserved9: [u8; 0x10],
    ier: Ier,
    idr: Idr,
    sr: Sr,
    imr: Imr,
    _reserved13: [u8; 0x08],
    focr: Focr,
    _reserved14: [u8; 0x04],
    pll_cfg: PllCfg,
    _reserved15: [u8; 0x60],
    wpmr: Wpmr,
    wpsr: Wpsr,
    _reserved17: [u8; 0x20],
    pcr: Pcr,
    ocr1: Ocr1,
    _reserved19: [u8; 0x1c],
    pmmr: Pmmr,
    _reserved20: [u8; 0x2c],
    cpulim: Cpulim,
    _reserved21: [u8; 0x0c],
    csr0: Csr0,
    csr1: Csr1,
    csr2: Csr2,
    csr3: Csr3,
    _reserved25: [u8; 0x10],
    gcsr0: Gcsr0,
    gcsr1: Gcsr1,
    gcsr2: Gcsr2,
    gcsr3: Gcsr3,
    _reserved29: [u8; 0x10],
    osc2: Osc2,
    ocr2: Ocr2,
}
impl RegisterBlock {
    #[doc = "0x00 - System Clock Enable Register"]
    #[inline(always)]
    pub const fn scer(&self) -> &Scer {
        &self.scer
    }
    #[doc = "0x04 - System Clock Disable Register"]
    #[inline(always)]
    pub const fn scdr(&self) -> &Scdr {
        &self.scdr
    }
    #[doc = "0x08 - System Clock Status Register"]
    #[inline(always)]
    pub const fn scsr(&self) -> &Scsr {
        &self.scsr
    }
    #[doc = "0x20 - Main Oscillator Register"]
    #[inline(always)]
    pub const fn ckgr_mor(&self) -> &CkgrMor {
        &self.ckgr_mor
    }
    #[doc = "0x24 - Main Clock Frequency Register"]
    #[inline(always)]
    pub const fn ckgr_mcfr(&self) -> &CkgrMcfr {
        &self.ckgr_mcfr
    }
    #[doc = "0x28 - PLLA Register"]
    #[inline(always)]
    pub const fn ckgr_pllar(&self) -> &CkgrPllar {
        &self.ckgr_pllar
    }
    #[doc = "0x2c - PLLB Register"]
    #[inline(always)]
    pub const fn ckgr_pllbr(&self) -> &CkgrPllbr {
        &self.ckgr_pllbr
    }
    #[doc = "0x30 - Master Clock Register"]
    #[inline(always)]
    pub const fn mckr(&self) -> &Mckr {
        &self.mckr
    }
    #[doc = "0x40..0x50 - Programmable Clock Register"]
    #[inline(always)]
    pub const fn pck(&self, n: usize) -> &Pck {
        &self.pck[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x50 - Programmable Clock Register"]
    #[inline(always)]
    pub fn pck_iter(&self) -> impl Iterator<Item = &Pck> {
        self.pck.iter()
    }
    #[doc = "0x60 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x64 - Interrupt Disable Register"]
    #[inline(always)]
    pub const fn idr(&self) -> &Idr {
        &self.idr
    }
    #[doc = "0x68 - Status Register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x6c - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        &self.imr
    }
    #[doc = "0x78 - Fault Output Clear Register"]
    #[inline(always)]
    pub const fn focr(&self) -> &Focr {
        &self.focr
    }
    #[doc = "0x80 - PLL Configuration Register"]
    #[inline(always)]
    pub const fn pll_cfg(&self) -> &PllCfg {
        &self.pll_cfg
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
    #[doc = "0x10c - Peripheral Control Register"]
    #[inline(always)]
    pub const fn pcr(&self) -> &Pcr {
        &self.pcr
    }
    #[doc = "0x110 - Oscillator Calibration Register"]
    #[inline(always)]
    pub const fn ocr1(&self) -> &Ocr1 {
        &self.ocr1
    }
    #[doc = "0x130 - PLL Maximum Multiplier Value Register"]
    #[inline(always)]
    pub const fn pmmr(&self) -> &Pmmr {
        &self.pmmr
    }
    #[doc = "0x160 - CPU Monitor Limits Register"]
    #[inline(always)]
    pub const fn cpulim(&self) -> &Cpulim {
        &self.cpulim
    }
    #[doc = "0x170 - Peripheral Clock Status Register 0"]
    #[inline(always)]
    pub const fn csr0(&self) -> &Csr0 {
        &self.csr0
    }
    #[doc = "0x174 - Peripheral Clock Status Register 1"]
    #[inline(always)]
    pub const fn csr1(&self) -> &Csr1 {
        &self.csr1
    }
    #[doc = "0x178 - Peripheral Clock Status Register 2"]
    #[inline(always)]
    pub const fn csr2(&self) -> &Csr2 {
        &self.csr2
    }
    #[doc = "0x17c - Peripheral Clock Status Register 3"]
    #[inline(always)]
    pub const fn csr3(&self) -> &Csr3 {
        &self.csr3
    }
    #[doc = "0x190 - Generic Clock Status Register 0"]
    #[inline(always)]
    pub const fn gcsr0(&self) -> &Gcsr0 {
        &self.gcsr0
    }
    #[doc = "0x194 - Generic Clock Status Register 1"]
    #[inline(always)]
    pub const fn gcsr1(&self) -> &Gcsr1 {
        &self.gcsr1
    }
    #[doc = "0x198 - Generic Clock Status Register 2"]
    #[inline(always)]
    pub const fn gcsr2(&self) -> &Gcsr2 {
        &self.gcsr2
    }
    #[doc = "0x19c - Generic Clock Status Register 3"]
    #[inline(always)]
    pub const fn gcsr3(&self) -> &Gcsr3 {
        &self.gcsr3
    }
    #[doc = "0x1b0 - Oscillator Control Register 2"]
    #[inline(always)]
    pub const fn osc2(&self) -> &Osc2 {
        &self.osc2
    }
    #[doc = "0x1b4 - Oscillator Calibration Register 2"]
    #[inline(always)]
    pub const fn ocr2(&self) -> &Ocr2 {
        &self.ocr2
    }
}
#[doc = "SCER (w) register accessor: System Clock Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scer::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scer`] module"]
#[doc(alias = "SCER")]
pub type Scer = crate::Reg<scer::ScerSpec>;
#[doc = "System Clock Enable Register"]
pub mod scer;
#[doc = "SCDR (w) register accessor: System Clock Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scdr`] module"]
#[doc(alias = "SCDR")]
pub type Scdr = crate::Reg<scdr::ScdrSpec>;
#[doc = "System Clock Disable Register"]
pub mod scdr;
#[doc = "SCSR (r) register accessor: System Clock Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scsr`] module"]
#[doc(alias = "SCSR")]
pub type Scsr = crate::Reg<scsr::ScsrSpec>;
#[doc = "System Clock Status Register"]
pub mod scsr;
#[doc = "CKGR_MOR (rw) register accessor: Main Oscillator Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ckgr_mor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckgr_mor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ckgr_mor`] module"]
#[doc(alias = "CKGR_MOR")]
pub type CkgrMor = crate::Reg<ckgr_mor::CkgrMorSpec>;
#[doc = "Main Oscillator Register"]
pub mod ckgr_mor;
#[doc = "CKGR_MCFR (rw) register accessor: Main Clock Frequency Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ckgr_mcfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckgr_mcfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ckgr_mcfr`] module"]
#[doc(alias = "CKGR_MCFR")]
pub type CkgrMcfr = crate::Reg<ckgr_mcfr::CkgrMcfrSpec>;
#[doc = "Main Clock Frequency Register"]
pub mod ckgr_mcfr;
#[doc = "CKGR_PLLAR (rw) register accessor: PLLA Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ckgr_pllar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckgr_pllar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ckgr_pllar`] module"]
#[doc(alias = "CKGR_PLLAR")]
pub type CkgrPllar = crate::Reg<ckgr_pllar::CkgrPllarSpec>;
#[doc = "PLLA Register"]
pub mod ckgr_pllar;
#[doc = "CKGR_PLLBR (rw) register accessor: PLLB Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ckgr_pllbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckgr_pllbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ckgr_pllbr`] module"]
#[doc(alias = "CKGR_PLLBR")]
pub type CkgrPllbr = crate::Reg<ckgr_pllbr::CkgrPllbrSpec>;
#[doc = "PLLB Register"]
pub mod ckgr_pllbr;
#[doc = "MCKR (rw) register accessor: Master Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mckr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mckr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mckr`] module"]
#[doc(alias = "MCKR")]
pub type Mckr = crate::Reg<mckr::MckrSpec>;
#[doc = "Master Clock Register"]
pub mod mckr;
#[doc = "PCK (rw) register accessor: Programmable Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pck::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pck::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pck`] module"]
#[doc(alias = "PCK")]
pub type Pck = crate::Reg<pck::PckSpec>;
#[doc = "Programmable Clock Register"]
pub mod pck;
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
#[doc = "SR (r) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "Status Register"]
pub mod sr;
#[doc = "IMR (r) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`] module"]
#[doc(alias = "IMR")]
pub type Imr = crate::Reg<imr::ImrSpec>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "FOCR (w) register accessor: Fault Output Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`focr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@focr`] module"]
#[doc(alias = "FOCR")]
pub type Focr = crate::Reg<focr::FocrSpec>;
#[doc = "Fault Output Clear Register"]
pub mod focr;
#[doc = "PLL_CFG (rw) register accessor: PLL Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll_cfg`] module"]
#[doc(alias = "PLL_CFG")]
pub type PllCfg = crate::Reg<pll_cfg::PllCfgSpec>;
#[doc = "PLL Configuration Register"]
pub mod pll_cfg;
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
#[doc = "PCR (rw) register accessor: Peripheral Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcr`] module"]
#[doc(alias = "PCR")]
pub type Pcr = crate::Reg<pcr::PcrSpec>;
#[doc = "Peripheral Control Register"]
pub mod pcr;
#[doc = "OCR1 (rw) register accessor: Oscillator Calibration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ocr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocr1`] module"]
#[doc(alias = "OCR1")]
pub type Ocr1 = crate::Reg<ocr1::Ocr1Spec>;
#[doc = "Oscillator Calibration Register"]
pub mod ocr1;
#[doc = "PMMR (rw) register accessor: PLL Maximum Multiplier Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmmr`] module"]
#[doc(alias = "PMMR")]
pub type Pmmr = crate::Reg<pmmr::PmmrSpec>;
#[doc = "PLL Maximum Multiplier Value Register"]
pub mod pmmr;
#[doc = "CPULIM (rw) register accessor: CPU Monitor Limits Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpulim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpulim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpulim`] module"]
#[doc(alias = "CPULIM")]
pub type Cpulim = crate::Reg<cpulim::CpulimSpec>;
#[doc = "CPU Monitor Limits Register"]
pub mod cpulim;
#[doc = "CSR0 (r) register accessor: Peripheral Clock Status Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`csr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr0`] module"]
#[doc(alias = "CSR0")]
pub type Csr0 = crate::Reg<csr0::Csr0Spec>;
#[doc = "Peripheral Clock Status Register 0"]
pub mod csr0;
#[doc = "CSR1 (r) register accessor: Peripheral Clock Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`csr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr1`] module"]
#[doc(alias = "CSR1")]
pub type Csr1 = crate::Reg<csr1::Csr1Spec>;
#[doc = "Peripheral Clock Status Register 1"]
pub mod csr1;
#[doc = "CSR2 (r) register accessor: Peripheral Clock Status Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`csr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr2`] module"]
#[doc(alias = "CSR2")]
pub type Csr2 = crate::Reg<csr2::Csr2Spec>;
#[doc = "Peripheral Clock Status Register 2"]
pub mod csr2;
#[doc = "CSR3 (r) register accessor: Peripheral Clock Status Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`csr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr3`] module"]
#[doc(alias = "CSR3")]
pub type Csr3 = crate::Reg<csr3::Csr3Spec>;
#[doc = "Peripheral Clock Status Register 3"]
pub mod csr3;
#[doc = "GCSR0 (r) register accessor: Generic Clock Status Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gcsr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcsr0`] module"]
#[doc(alias = "GCSR0")]
pub type Gcsr0 = crate::Reg<gcsr0::Gcsr0Spec>;
#[doc = "Generic Clock Status Register 0"]
pub mod gcsr0;
#[doc = "GCSR1 (r) register accessor: Generic Clock Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gcsr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcsr1`] module"]
#[doc(alias = "GCSR1")]
pub type Gcsr1 = crate::Reg<gcsr1::Gcsr1Spec>;
#[doc = "Generic Clock Status Register 1"]
pub mod gcsr1;
#[doc = "GCSR2 (r) register accessor: Generic Clock Status Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`gcsr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcsr2`] module"]
#[doc(alias = "GCSR2")]
pub type Gcsr2 = crate::Reg<gcsr2::Gcsr2Spec>;
#[doc = "Generic Clock Status Register 2"]
pub mod gcsr2;
#[doc = "GCSR3 (r) register accessor: Generic Clock Status Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`gcsr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcsr3`] module"]
#[doc(alias = "GCSR3")]
pub type Gcsr3 = crate::Reg<gcsr3::Gcsr3Spec>;
#[doc = "Generic Clock Status Register 3"]
pub mod gcsr3;
#[doc = "OSC2 (rw) register accessor: Oscillator Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`osc2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osc2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osc2`] module"]
#[doc(alias = "OSC2")]
pub type Osc2 = crate::Reg<osc2::Osc2Spec>;
#[doc = "Oscillator Control Register 2"]
pub mod osc2;
#[doc = "OCR2 (rw) register accessor: Oscillator Calibration Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ocr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocr2`] module"]
#[doc(alias = "OCR2")]
pub type Ocr2 = crate::Reg<ocr2::Ocr2Spec>;
#[doc = "Oscillator Calibration Register 2"]
pub mod ocr2;
