#[doc = "Register `CMDR3` writer"]
pub type W = crate::W<Cmdr3Spec>;
#[doc = "Field `BUS` writer - bus used"]
pub type BusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BCE` writer - 1553 emitter"]
pub type BceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BCR` writer - 1553 receiver"]
pub type BcrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ER` writer - data or command transfer"]
pub type ErW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - bus used"]
    #[inline(always)]
    pub fn bus(&mut self) -> BusW<Cmdr3Spec> {
        BusW::new(self, 0)
    }
    #[doc = "Bit 1 - 1553 emitter"]
    #[inline(always)]
    pub fn bce(&mut self) -> BceW<Cmdr3Spec> {
        BceW::new(self, 1)
    }
    #[doc = "Bit 2 - 1553 receiver"]
    #[inline(always)]
    pub fn bcr(&mut self) -> BcrW<Cmdr3Spec> {
        BcrW::new(self, 2)
    }
    #[doc = "Bit 3 - data or command transfer"]
    #[inline(always)]
    pub fn er(&mut self) -> ErW<Cmdr3Spec> {
        ErW::new(self, 3)
    }
}
#[doc = "Command Register 3\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdr3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmdr3Spec;
impl crate::RegisterSpec for Cmdr3Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmdr3::W`](W) writer structure"]
impl crate::Writable for Cmdr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMDR3 to value 0"]
impl crate::Resettable for Cmdr3Spec {}
