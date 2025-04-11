#[doc = "Register `ROUTER_TIMEOUT` reader"]
pub type R = crate::R<RouterTimeoutSpec>;
#[doc = "Field `ADDR` reader - Physical Address"]
pub type AddrR = crate::FieldReader;
#[doc = "Field `LOCKED` reader - Locked"]
pub type LockedR = crate::BitReader;
impl R {
    #[doc = "Bits 0:4 - Physical Address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Locked"]
    #[inline(always)]
    pub fn locked(&self) -> LockedR {
        LockedR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "SpW Router Timeout\n\nYou can [`read`](crate::Reg::read) this register and get [`router_timeout::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RouterTimeoutSpec;
impl crate::RegisterSpec for RouterTimeoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`router_timeout::R`](R) reader structure"]
impl crate::Readable for RouterTimeoutSpec {}
#[doc = "`reset()` method sets ROUTER_TIMEOUT to value 0"]
impl crate::Resettable for RouterTimeoutSpec {}
