#[doc = "Register `IDR` writer"]
pub type W = crate::W<IdrSpec>;
#[doc = "Field `MEM_FIX_I` writer - fixable error on instruction"]
pub type MemFixIW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_NOFIX_I` writer - un-fixable error on instruction"]
pub type MemNofixIW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_FIX_D` writer - fixable error on data"]
pub type MemFixDW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_NOFIX_D` writer - un-fixable error on data"]
pub type MemNofixDW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - fixable error on instruction"]
    #[inline(always)]
    pub fn mem_fix_i(&mut self) -> MemFixIW<IdrSpec> {
        MemFixIW::new(self, 0)
    }
    #[doc = "Bit 1 - un-fixable error on instruction"]
    #[inline(always)]
    pub fn mem_nofix_i(&mut self) -> MemNofixIW<IdrSpec> {
        MemNofixIW::new(self, 1)
    }
    #[doc = "Bit 2 - fixable error on data"]
    #[inline(always)]
    pub fn mem_fix_d(&mut self) -> MemFixDW<IdrSpec> {
        MemFixDW::new(self, 2)
    }
    #[doc = "Bit 3 - un-fixable error on data"]
    #[inline(always)]
    pub fn mem_nofix_d(&mut self) -> MemNofixDW<IdrSpec> {
        MemNofixDW::new(self, 3)
    }
}
#[doc = "TCMECC Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
