#[doc = "Register `FLEX_SPI_RDR_FIFO_MULTI_DATA_8_MODE` reader"]
pub type R = crate::R<FlexSpiRdrFifoMultiData8ModeSpec>;
#[doc = "Field `RD8_0` reader - Receive Data"]
pub type Rd8_0R = crate::FieldReader;
#[doc = "Field `RD8_1` reader - Receive Data"]
pub type Rd8_1R = crate::FieldReader;
#[doc = "Field `RD8_2` reader - Receive Data"]
pub type Rd8_2R = crate::FieldReader;
#[doc = "Field `RD8_3` reader - Receive Data"]
pub type Rd8_3R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Receive Data"]
    #[inline(always)]
    pub fn rd8_0(&self) -> Rd8_0R {
        Rd8_0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Receive Data"]
    #[inline(always)]
    pub fn rd8_1(&self) -> Rd8_1R {
        Rd8_1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Receive Data"]
    #[inline(always)]
    pub fn rd8_2(&self) -> Rd8_2R {
        Rd8_2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Receive Data"]
    #[inline(always)]
    pub fn rd8_3(&self) -> Rd8_3R {
        Rd8_3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "SPI Receive Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_spi_rdr_fifo_multi_data_8_mode::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexSpiRdrFifoMultiData8ModeSpec;
impl crate::RegisterSpec for FlexSpiRdrFifoMultiData8ModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flex_spi_rdr_fifo_multi_data_8_mode::R`](R) reader structure"]
impl crate::Readable for FlexSpiRdrFifoMultiData8ModeSpec {}
#[doc = "`reset()` method sets FLEX_SPI_RDR_FIFO_MULTI_DATA_8_MODE to value 0"]
impl crate::Resettable for FlexSpiRdrFifoMultiData8ModeSpec {}
