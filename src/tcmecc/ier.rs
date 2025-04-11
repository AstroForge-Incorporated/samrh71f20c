#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `MEM_FIX_I` writer - Fixable error on instruction"]
pub type MemFixIW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_NOFIX_I` writer - Un-fixable error on instruction"]
pub type MemNofixIW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_FIX_D` writer - Fixable error on data"]
pub type MemFixDW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MEM_NOFIX_D` writer - Un-fixable error on data"]
pub type MemNofixDW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Fixable error on instruction"]
    #[inline(always)]
    pub fn mem_fix_i(&mut self) -> MemFixIW<IerSpec> {
        MemFixIW::new(self, 0)
    }
    #[doc = "Bit 1 - Un-fixable error on instruction"]
    #[inline(always)]
    pub fn mem_nofix_i(&mut self) -> MemNofixIW<IerSpec> {
        MemNofixIW::new(self, 1)
    }
    #[doc = "Bit 2 - Fixable error on data"]
    #[inline(always)]
    pub fn mem_fix_d(&mut self) -> MemFixDW<IerSpec> {
        MemFixDW::new(self, 2)
    }
    #[doc = "Bit 3 - Un-fixable error on data"]
    #[inline(always)]
    pub fn mem_nofix_d(&mut self) -> MemNofixDW<IerSpec> {
        MemNofixDW::new(self, 3)
    }
}
#[doc = "TCMECC Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
