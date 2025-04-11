#[doc = "Register `GROUP_EDACSTS` reader"]
pub type R = crate::R<GroupEdacstsSpec>;
#[doc = "Field `CORR` reader - Correction Count"]
pub type CorrR = crate::FieldReader;
#[doc = "Field `UNCORR` reader - Uncorrectable Error"]
pub type UncorrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Correction Count"]
    #[inline(always)]
    pub fn corr(&self) -> CorrR {
        CorrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Uncorrectable Error"]
    #[inline(always)]
    pub fn uncorr(&self) -> UncorrR {
        UncorrR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "SpW Group Interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`group_edacsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GroupEdacstsSpec;
impl crate::RegisterSpec for GroupEdacstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`group_edacsts::R`](R) reader structure"]
impl crate::Readable for GroupEdacstsSpec {}
#[doc = "`reset()` method sets GROUP_EDACSTS to value 0"]
impl crate::Resettable for GroupEdacstsSpec {}
