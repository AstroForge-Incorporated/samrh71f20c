#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `MEM_FIX_I` reader - Fixable error status in instruction memory"]
pub type MemFixIR = crate::BitReader;
#[doc = "Field `MEM_FIX_D` reader - Fixable error status in data memory"]
pub type MemFixDR = crate::BitReader;
#[doc = "Field `CPT_FIX` reader - 5 bits counter"]
pub type CptFixR = crate::FieldReader;
#[doc = "Field `OVER_FIX` reader - counter overflow"]
pub type OverFixR = crate::BitReader;
#[doc = "Field `MEM_NOFIX_I` reader - Un-fixable error status in instruction memory"]
pub type MemNofixIR = crate::BitReader;
#[doc = "Field `MEM_NOFIX_D` reader - Un-fixable error status in data memory"]
pub type MemNofixDR = crate::BitReader;
#[doc = "Field `CPT_NOFIX` reader - 5 bits counter"]
pub type CptNofixR = crate::FieldReader;
#[doc = "Field `OVER_NOFIX` reader - counter overflow"]
pub type OverNofixR = crate::BitReader;
#[doc = "Field `HES` reader - Hardware Error Size"]
pub type HesR = crate::FieldReader;
#[doc = "Field `ONE` reader - one"]
pub type OneR = crate::BitReader;
#[doc = "Field `MEM_ID_I` reader - memory identification number"]
pub type MemIdIR = crate::BitReader;
#[doc = "Field `MEM_ID_D` reader - memory identification number"]
pub type MemIdDR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Fixable error status in instruction memory"]
    #[inline(always)]
    pub fn mem_fix_i(&self) -> MemFixIR {
        MemFixIR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fixable error status in data memory"]
    #[inline(always)]
    pub fn mem_fix_d(&self) -> MemFixDR {
        MemFixDR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6 - 5 bits counter"]
    #[inline(always)]
    pub fn cpt_fix(&self) -> CptFixR {
        CptFixR::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bit 7 - counter overflow"]
    #[inline(always)]
    pub fn over_fix(&self) -> OverFixR {
        OverFixR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Un-fixable error status in instruction memory"]
    #[inline(always)]
    pub fn mem_nofix_i(&self) -> MemNofixIR {
        MemNofixIR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Un-fixable error status in data memory"]
    #[inline(always)]
    pub fn mem_nofix_d(&self) -> MemNofixDR {
        MemNofixDR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:14 - 5 bits counter"]
    #[inline(always)]
    pub fn cpt_nofix(&self) -> CptNofixR {
        CptNofixR::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - counter overflow"]
    #[inline(always)]
    pub fn over_nofix(&self) -> OverNofixR {
        OverNofixR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Hardware Error Size"]
    #[inline(always)]
    pub fn hes(&self) -> HesR {
        HesR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - one"]
    #[inline(always)]
    pub fn one(&self) -> OneR {
        OneR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - memory identification number"]
    #[inline(always)]
    pub fn mem_id_i(&self) -> MemIdIR {
        MemIdIR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - memory identification number"]
    #[inline(always)]
    pub fn mem_id_d(&self) -> MemIdDR {
        MemIdDR::new(((self.bits >> 29) & 1) != 0)
    }
}
#[doc = "TCMECC Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {}
