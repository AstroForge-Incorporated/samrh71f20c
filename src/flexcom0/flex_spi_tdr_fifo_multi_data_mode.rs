#[doc = "Register `FLEX_SPI_TDR_FIFO_MULTI_DATA_MODE` writer"]
pub type W = crate::W<FlexSpiTdrFifoMultiDataModeSpec>;
#[doc = "Field `TD0` writer - Transmit Data"]
pub type Td0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TD1` writer - Transmit Data"]
pub type Td1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Transmit Data"]
    #[inline(always)]
    pub fn td0(&mut self) -> Td0W<FlexSpiTdrFifoMultiDataModeSpec> {
        Td0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Transmit Data"]
    #[inline(always)]
    pub fn td1(&mut self) -> Td1W<FlexSpiTdrFifoMultiDataModeSpec> {
        Td1W::new(self, 16)
    }
}
#[doc = "SPI Transmit Data Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_spi_tdr_fifo_multi_data_mode::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexSpiTdrFifoMultiDataModeSpec;
impl crate::RegisterSpec for FlexSpiTdrFifoMultiDataModeSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`flex_spi_tdr_fifo_multi_data_mode::W`](W) writer structure"]
impl crate::Writable for FlexSpiTdrFifoMultiDataModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLEX_SPI_TDR_FIFO_MULTI_DATA_MODE to value 0"]
impl crate::Resettable for FlexSpiTdrFifoMultiDataModeSpec {}
