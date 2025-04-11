#[doc = "Register `MID` reader"]
pub type R = crate::R<MidSpec>;
#[doc = "Field `MREV` reader - Module Revision"]
pub type MrevR = crate::FieldReader<u16>;
#[doc = "Field `MID` reader - Module Identification Number"]
pub type MidR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Module Revision"]
    #[inline(always)]
    pub fn mrev(&self) -> MrevR {
        MrevR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Module Identification Number"]
    #[inline(always)]
    pub fn mid(&self) -> MidR {
        MidR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Module ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MidSpec;
impl crate::RegisterSpec for MidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mid::R`](R) reader structure"]
impl crate::Readable for MidSpec {}
#[doc = "`reset()` method sets MID to value 0"]
impl crate::Resettable for MidSpec {}
