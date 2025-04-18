#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    csr: Csr,
    rvr: Rvr,
    cvr: Cvr,
    calib: Calib,
}
impl RegisterBlock {
    #[doc = "0x00 - Control and Status Register"]
    #[inline(always)]
    pub const fn csr(&self) -> &Csr {
        &self.csr
    }
    #[doc = "0x04 - Reload Value Register"]
    #[inline(always)]
    pub const fn rvr(&self) -> &Rvr {
        &self.rvr
    }
    #[doc = "0x08 - Current Value Register"]
    #[inline(always)]
    pub const fn cvr(&self) -> &Cvr {
        &self.cvr
    }
    #[doc = "0x0c - Calibration Value Register"]
    #[inline(always)]
    pub const fn calib(&self) -> &Calib {
        &self.calib
    }
}
#[doc = "CSR (rw) register accessor: Control and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`] module"]
#[doc(alias = "CSR")]
pub type Csr = crate::Reg<csr::CsrSpec>;
#[doc = "Control and Status Register"]
pub mod csr;
#[doc = "RVR (rw) register accessor: Reload Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rvr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rvr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rvr`] module"]
#[doc(alias = "RVR")]
pub type Rvr = crate::Reg<rvr::RvrSpec>;
#[doc = "Reload Value Register"]
pub mod rvr;
#[doc = "CVR (rw) register accessor: Current Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cvr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cvr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cvr`] module"]
#[doc(alias = "CVR")]
pub type Cvr = crate::Reg<cvr::CvrSpec>;
#[doc = "Current Value Register"]
pub mod cvr;
#[doc = "CALIB (r) register accessor: Calibration Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`calib::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calib`] module"]
#[doc(alias = "CALIB")]
pub type Calib = crate::Reg<calib::CalibSpec>;
#[doc = "Calibration Value Register"]
pub mod calib;
