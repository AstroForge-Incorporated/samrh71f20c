#[doc = "Register `WPSR` reader"]
pub type R = crate::R<WpsrSpec>;
#[doc = "Field `WPVS` reader - Write Protection Violation Status (Cleared on read)"]
pub type WpvsR = crate::BitReader;
#[doc = "Field `WPVSRC` reader - Write Protection Violation Source"]
pub type WpvsrcR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Write Protection Violation Status (Cleared on read)"]
    #[inline(always)]
    pub fn wpvs(&self) -> WpvsR {
        WpvsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15 - Write Protection Violation Source"]
    #[inline(always)]
    pub fn wpvsrc(&self) -> WpvsrcR {
        WpvsrcR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "ICM Write Protection Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WpsrSpec;
impl crate::RegisterSpec for WpsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wpsr::R`](R) reader structure"]
impl crate::Readable for WpsrSpec {}
#[doc = "`reset()` method sets WPSR to value 0"]
impl crate::Resettable for WpsrSpec {}
