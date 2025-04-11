#[doc = "Register `HECC_IER` writer"]
pub type W = crate::W<HeccIerSpec>;
#[doc = "Field `MEM_FIX` writer - Fixable error"]
pub type MemFixW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_NOFIX` writer - Un-fixable error"]
pub type MemNofixW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Fixable error"]
    #[inline(always)]
    pub fn mem_fix(&mut self) -> MemFixW<HeccIerSpec> {
        MemFixW::new(self, 0)
    }
    #[doc = "Bit 1 - Un-fixable error"]
    #[inline(always)]
    pub fn mem_nofix(&mut self) -> MemNofixW<HeccIerSpec> {
        MemNofixW::new(self, 1)
    }
}
#[doc = "HECC Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hecc_ier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HeccIerSpec;
impl crate::RegisterSpec for HeccIerSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hecc_ier::W`](W) writer structure"]
impl crate::Writable for HeccIerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HECC_IER to value 0"]
impl crate::Resettable for HeccIerSpec {}
