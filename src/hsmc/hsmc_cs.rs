#[repr(C)]
#[doc = "HSMC_CS\\[%s\\]"]
#[doc(alias = "HSMC_CS")]
pub struct HsmcCs {
    setup: Setup,
    pulse: Pulse,
    cycle: Cycle,
    mode: Mode,
}
impl HsmcCs {
    #[doc = "0x00 - HSMC Setup Register"]
    #[inline(always)]
    pub const fn setup(&self) -> &Setup {
        &self.setup
    }
    #[doc = "0x04 - HSMC Pulse Register"]
    #[inline(always)]
    pub const fn pulse(&self) -> &Pulse {
        &self.pulse
    }
    #[doc = "0x08 - HSMC Cycle Register"]
    #[inline(always)]
    pub const fn cycle(&self) -> &Cycle {
        &self.cycle
    }
    #[doc = "0x0c - HSMC Mode Register"]
    #[inline(always)]
    pub const fn mode(&self) -> &Mode {
        &self.mode
    }
}
#[doc = "SETUP (rw) register accessor: HSMC Setup Register\n\nYou can [`read`](crate::Reg::read) this register and get [`setup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`setup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setup`] module"]
#[doc(alias = "SETUP")]
pub type Setup = crate::Reg<setup::SetupSpec>;
#[doc = "HSMC Setup Register"]
pub mod setup;
#[doc = "PULSE (rw) register accessor: HSMC Pulse Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pulse::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pulse::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pulse`] module"]
#[doc(alias = "PULSE")]
pub type Pulse = crate::Reg<pulse::PulseSpec>;
#[doc = "HSMC Pulse Register"]
pub mod pulse;
#[doc = "CYCLE (rw) register accessor: HSMC Cycle Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cycle::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cycle::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cycle`] module"]
#[doc(alias = "CYCLE")]
pub type Cycle = crate::Reg<cycle::CycleSpec>;
#[doc = "HSMC Cycle Register"]
pub mod cycle;
#[doc = "MODE (rw) register accessor: HSMC Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode`] module"]
#[doc(alias = "MODE")]
pub type Mode = crate::Reg<mode::ModeSpec>;
#[doc = "HSMC Mode Register"]
pub mod mode;
