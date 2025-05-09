#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `ENABLE` reader - ICM Enable Register"]
pub type EnableR = crate::BitReader;
#[doc = "Field `RAWRMDIS` reader - Region Monitoring Disabled Raw Status"]
pub type RawrmdisR = crate::FieldReader;
#[doc = "Field `RMDIS` reader - Region Monitoring Disabled Status"]
pub type RmdisR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - ICM Enable Register"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - Region Monitoring Disabled Raw Status"]
    #[inline(always)]
    pub fn rawrmdis(&self) -> RawrmdisR {
        RawrmdisR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Region Monitoring Disabled Status"]
    #[inline(always)]
    pub fn rmdis(&self) -> RmdisR {
        RmdisR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {}
