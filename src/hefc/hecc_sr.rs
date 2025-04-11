#[doc = "Register `HECC_SR` reader"]
pub type R = crate::R<HeccSrSpec>;
#[doc = "Field `MEM_FIX` reader - Fixable error status"]
pub type MemFixR = crate::BitReader;
#[doc = "Field `CPT_FIX` reader - 5 bits counter"]
pub type CptFixR = crate::FieldReader;
#[doc = "Field `OVER_FIX` reader - counter overflow"]
pub type OverFixR = crate::BitReader;
#[doc = "Field `MEM_NOFIX` reader - Un-fixable error status"]
pub type MemNofixR = crate::BitReader;
#[doc = "Field `CPT_NOFIX` reader - 5 bits counter"]
pub type CptNofixR = crate::FieldReader;
#[doc = "Field `OVER_NOFIX` reader - counter overflow"]
pub type OverNofixR = crate::BitReader;
#[doc = "Field `HES` reader - Hardware Error Size"]
pub type HesR = crate::FieldReader;
#[doc = "Field `TYPE` reader - write or read access"]
pub type TypeR = crate::BitReader;
#[doc = "Field `MEM_ID` reader - memory identification number"]
pub type MemIdR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Fixable error status"]
    #[inline(always)]
    pub fn mem_fix(&self) -> MemFixR {
        MemFixR::new((self.bits & 1) != 0)
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
    #[doc = "Bit 8 - Un-fixable error status"]
    #[inline(always)]
    pub fn mem_nofix(&self) -> MemNofixR {
        MemNofixR::new(((self.bits >> 8) & 1) != 0)
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
    #[doc = "Bit 27 - write or read access"]
    #[inline(always)]
    pub fn type_(&self) -> TypeR {
        TypeR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - memory identification number"]
    #[inline(always)]
    pub fn mem_id(&self) -> MemIdR {
        MemIdR::new(((self.bits >> 28) & 3) as u8)
    }
}
#[doc = "HECC Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`hecc_sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HeccSrSpec;
impl crate::RegisterSpec for HeccSrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hecc_sr::R`](R) reader structure"]
impl crate::Readable for HeccSrSpec {}
#[doc = "`reset()` method sets HECC_SR to value 0"]
impl crate::Resettable for HeccSrSpec {}
