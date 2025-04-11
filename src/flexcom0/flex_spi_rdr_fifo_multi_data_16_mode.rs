#[doc = "Register `FLEX_SPI_RDR_FIFO_MULTI_DATA_16_MODE` reader"]
pub type R = crate::R<FlexSpiRdrFifoMultiData16ModeSpec>;
#[doc = "Field `RD16_0` reader - Receive Data"]
pub type Rd16_0R = crate::FieldReader<u16>;
#[doc = "Field `RD16_1` reader - Receive Data"]
pub type Rd16_1R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Receive Data"]
    #[inline(always)]
    pub fn rd16_0(&self) -> Rd16_0R {
        Rd16_0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Receive Data"]
    #[inline(always)]
    pub fn rd16_1(&self) -> Rd16_1R {
        Rd16_1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "SPI Receive Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_spi_rdr_fifo_multi_data_16_mode::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexSpiRdrFifoMultiData16ModeSpec;
impl crate::RegisterSpec for FlexSpiRdrFifoMultiData16ModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flex_spi_rdr_fifo_multi_data_16_mode::R`](R) reader structure"]
impl crate::Readable for FlexSpiRdrFifoMultiData16ModeSpec {}
#[doc = "`reset()` method sets FLEX_SPI_RDR_FIFO_MULTI_DATA_16_MODE to value 0"]
impl crate::Resettable for FlexSpiRdrFifoMultiData16ModeSpec {}
