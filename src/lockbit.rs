#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    lockbit: Lockbit,
}
impl RegisterBlock {
    #[doc = "0x00 - Lock Bits"]
    #[inline(always)]
    pub const fn lockbit(&self) -> &Lockbit {
        &self.lockbit
    }
}
#[doc = "LOCKBIT (rw) register accessor: Lock Bits\n\nYou can [`read`](crate::Reg::read) this register and get [`lockbit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lockbit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lockbit`] module"]
#[doc(alias = "LOCKBIT")]
pub type Lockbit = crate::Reg<lockbit::LockbitSpec>;
#[doc = "Lock Bits"]
pub mod lockbit;
