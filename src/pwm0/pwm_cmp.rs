#[repr(C)]
#[doc = "PWM Comparison 0 Value Register"]
#[doc(alias = "PWM_CMP")]
pub struct PwmCmp {
    cmpv: Cmpv,
    cmpvupd: Cmpvupd,
    cmpm: Cmpm,
    cmpmupd: Cmpmupd,
}
impl PwmCmp {
    #[doc = "0x00 - PWM Comparison x Value Register"]
    #[inline(always)]
    pub const fn cmpv(&self) -> &Cmpv {
        &self.cmpv
    }
    #[doc = "0x04 - PWM Comparison x Value Update Register"]
    #[inline(always)]
    pub const fn cmpvupd(&self) -> &Cmpvupd {
        &self.cmpvupd
    }
    #[doc = "0x08 - PWM Comparison x Mode Register"]
    #[inline(always)]
    pub const fn cmpm(&self) -> &Cmpm {
        &self.cmpm
    }
    #[doc = "0x0c - PWM Comparison x Mode Update Register"]
    #[inline(always)]
    pub const fn cmpmupd(&self) -> &Cmpmupd {
        &self.cmpmupd
    }
}
#[doc = "CMPV (rw) register accessor: PWM Comparison x Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpv`] module"]
#[doc(alias = "CMPV")]
pub type Cmpv = crate::Reg<cmpv::CmpvSpec>;
#[doc = "PWM Comparison x Value Register"]
pub mod cmpv;
#[doc = "CMPVUPD (w) register accessor: PWM Comparison x Value Update Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpvupd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpvupd`] module"]
#[doc(alias = "CMPVUPD")]
pub type Cmpvupd = crate::Reg<cmpvupd::CmpvupdSpec>;
#[doc = "PWM Comparison x Value Update Register"]
pub mod cmpvupd;
#[doc = "CMPM (rw) register accessor: PWM Comparison x Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpm`] module"]
#[doc(alias = "CMPM")]
pub type Cmpm = crate::Reg<cmpm::CmpmSpec>;
#[doc = "PWM Comparison x Mode Register"]
pub mod cmpm;
#[doc = "CMPMUPD (w) register accessor: PWM Comparison x Mode Update Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpmupd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpmupd`] module"]
#[doc(alias = "CMPMUPD")]
pub type Cmpmupd = crate::Reg<cmpmupd::CmpmupdSpec>;
#[doc = "PWM Comparison x Mode Update Register"]
pub mod cmpmupd;
