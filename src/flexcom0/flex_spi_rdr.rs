#[doc = "Register `FLEX_SPI_RDR` reader"]
pub type R = crate::R<FlexSpiRdrSpec>;
#[doc = "Field `RD` reader - Receive Data"]
pub type RdR = crate::FieldReader<u16>;
#[doc = "Field `PCS` reader - Peripheral Chip Select"]
pub type PcsR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Receive Data"]
    #[inline(always)]
    pub fn rd(&self) -> RdR {
        RdR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Peripheral Chip Select"]
    #[inline(always)]
    pub fn pcs(&self) -> PcsR {
        PcsR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[doc = "SPI Receive Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_spi_rdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexSpiRdrSpec;
impl crate::RegisterSpec for FlexSpiRdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flex_spi_rdr::R`](R) reader structure"]
impl crate::Readable for FlexSpiRdrSpec {}
#[doc = "`reset()` method sets FLEX_SPI_RDR to value 0"]
impl crate::Resettable for FlexSpiRdrSpec {}
