#[doc = "Register `IDR` writer"]
pub type W = crate::W<IdrSpec>;
#[doc = "Field `MEM_FIX` writer - fixable error"]
pub type MemFixW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_NOFIX` writer - un-fixable error"]
pub type MemNofixW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - fixable error"]
    #[inline(always)]
    pub fn mem_fix(&mut self) -> MemFixW<IdrSpec> {
        MemFixW::new(self, 0)
    }
    #[doc = "Bit 1 - un-fixable error"]
    #[inline(always)]
    pub fn mem_nofix(&mut self) -> MemNofixW<IdrSpec> {
        MemNofixW::new(self, 1)
    }
}
#[doc = "FLEXRAMECC Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdrSpec;
impl crate::RegisterSpec for IdrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`idr::W`](W) writer structure"]
impl crate::Writable for IdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDR to value 0"]
impl crate::Resettable for IdrSpec {}
