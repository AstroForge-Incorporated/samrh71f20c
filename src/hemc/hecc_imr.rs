#[doc = "Register `HECC_IMR` reader"]
pub type R = crate::R<HeccImrSpec>;
#[doc = "Field `MEM_FIX` reader - fixable error"]
pub type MemFixR = crate::BitReader;
#[doc = "Field `MEM_NOFIX` reader - un-fixable error"]
pub type MemNofixR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - fixable error"]
    #[inline(always)]
    pub fn mem_fix(&self) -> MemFixR {
        MemFixR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - un-fixable error"]
    #[inline(always)]
    pub fn mem_nofix(&self) -> MemNofixR {
        MemNofixR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "HECC Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hecc_imr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HeccImrSpec;
impl crate::RegisterSpec for HeccImrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hecc_imr::R`](R) reader structure"]
impl crate::Readable for HeccImrSpec {}
#[doc = "`reset()` method sets HECC_IMR to value 0"]
impl crate::Resettable for HeccImrSpec {}
