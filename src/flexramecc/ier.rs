#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `MEM_FIX` writer - Fixable error"]
pub type MemFixW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_NOFIX` writer - Un-fixable error"]
pub type MemNofixW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Fixable error"]
    #[inline(always)]
    pub fn mem_fix(&mut self) -> MemFixW<IerSpec> {
        MemFixW::new(self, 0)
    }
    #[doc = "Bit 1 - Un-fixable error"]
    #[inline(always)]
    pub fn mem_nofix(&mut self) -> MemNofixW<IerSpec> {
        MemNofixW::new(self, 1)
    }
}
#[doc = "FLEXRAMECC Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IerSpec {}
