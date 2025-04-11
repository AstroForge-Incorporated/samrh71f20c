#[doc = "Register `FLEX_SPI_WPSR` reader"]
pub type R = crate::R<FlexSpiWpsrSpec>;
#[doc = "Field `WPVS` reader - Write Protection Violation Status"]
pub type WpvsR = crate::BitReader;
#[doc = "Field `WPVSRC` reader - Write Protection Violation Source"]
pub type WpvsrcR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Write Protection Violation Status"]
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
#[doc = "SPI Write Protection Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_spi_wpsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexSpiWpsrSpec;
impl crate::RegisterSpec for FlexSpiWpsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flex_spi_wpsr::R`](R) reader structure"]
impl crate::Readable for FlexSpiWpsrSpec {}
#[doc = "`reset()` method sets FLEX_SPI_WPSR to value 0"]
impl crate::Resettable for FlexSpiWpsrSpec {}
