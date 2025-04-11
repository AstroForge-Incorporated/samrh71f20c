#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0xa0],
    can0: Can0,
    can1: Can1,
    _reserved2: [u8; 0x3c],
    wpmr: Wpmr,
}
impl RegisterBlock {
    #[doc = "0xa0 - CAN0 MSB Base Address"]
    #[inline(always)]
    pub const fn can0(&self) -> &Can0 {
        &self.can0
    }
    #[doc = "0xa4 - CAN1 MSB Base Address"]
    #[inline(always)]
    pub const fn can1(&self) -> &Can1 {
        &self.can1
    }
    #[doc = "0xe4 - Write Protection Mode Register"]
    #[inline(always)]
    pub const fn wpmr(&self) -> &Wpmr {
        &self.wpmr
    }
}
#[doc = "CAN0 (rw) register accessor: CAN0 MSB Base Address\n\nYou can [`read`](crate::Reg::read) this register and get [`can0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can0`] module"]
#[doc(alias = "CAN0")]
pub type Can0 = crate::Reg<can0::Can0Spec>;
#[doc = "CAN0 MSB Base Address"]
pub mod can0;
#[doc = "CAN1 (rw) register accessor: CAN1 MSB Base Address\n\nYou can [`read`](crate::Reg::read) this register and get [`can1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can1`] module"]
#[doc(alias = "CAN1")]
pub type Can1 = crate::Reg<can1::Can1Spec>;
#[doc = "CAN1 MSB Base Address"]
pub mod can1;
#[doc = "WPMR (rw) register accessor: Write Protection Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpmr`] module"]
#[doc(alias = "WPMR")]
pub type Wpmr = crate::Reg<wpmr::WpmrSpec>;
#[doc = "Write Protection Mode Register"]
pub mod wpmr;
