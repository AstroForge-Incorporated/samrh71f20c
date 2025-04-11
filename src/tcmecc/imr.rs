#[doc = "Register `IMR` reader"]
pub type R = crate::R<ImrSpec>;
#[doc = "Field `MEM_FIX_I` reader - fixable error on instruction"]
pub type MemFixIR = crate::BitReader;
#[doc = "Field `MEM_NOFIX_I` reader - un-fixable error on instruction"]
pub type MemNofixIR = crate::BitReader;
#[doc = "Field `MEM_FIX_D` reader - fixable error on data"]
pub type MemFixDR = crate::BitReader;
#[doc = "Field `MEM_NOFIX_D` reader - un-fixable error on data"]
pub type MemNofixDR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - fixable error on instruction"]
    #[inline(always)]
    pub fn mem_fix_i(&self) -> MemFixIR {
        MemFixIR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - un-fixable error on instruction"]
    #[inline(always)]
    pub fn mem_nofix_i(&self) -> MemNofixIR {
        MemNofixIR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - fixable error on data"]
    #[inline(always)]
    pub fn mem_fix_d(&self) -> MemFixDR {
        MemFixDR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - un-fixable error on data"]
    #[inline(always)]
    pub fn mem_nofix_d(&self) -> MemNofixDR {
        MemNofixDR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "TCMECC Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImrSpec;
impl crate::RegisterSpec for ImrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for ImrSpec {}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for ImrSpec {}
