#[doc = "Register `FLEX_TWI_WPSR` reader"]
pub type R = crate::R<FlexTwiWpsrSpec>;
#[doc = "Field `WPVS` reader - Write Protect Violation Status"]
pub type WpvsR = crate::BitReader;
#[doc = "Field `WPVSRC` reader - Write Protection Violation Source"]
pub type WpvsrcR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Write Protect Violation Status"]
    #[inline(always)]
    pub fn wpvs(&self) -> WpvsR {
        WpvsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:31 - Write Protection Violation Source"]
    #[inline(always)]
    pub fn wpvsrc(&self) -> WpvsrcR {
        WpvsrcR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[doc = "TWI Write Protection Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_twi_wpsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexTwiWpsrSpec;
impl crate::RegisterSpec for FlexTwiWpsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flex_twi_wpsr::R`](R) reader structure"]
impl crate::Readable for FlexTwiWpsrSpec {}
#[doc = "`reset()` method sets FLEX_TWI_WPSR to value 0"]
impl crate::Resettable for FlexTwiWpsrSpec {}
