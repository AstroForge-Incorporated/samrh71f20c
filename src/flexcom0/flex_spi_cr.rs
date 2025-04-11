#[doc = "Register `FLEX_SPI_CR` writer"]
pub type W = crate::W<FlexSpiCrSpec>;
#[doc = "Field `SPIEN` writer - SPI Enable"]
pub type SpienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPIDIS` writer - SPI Disable"]
pub type SpidisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWRST` writer - SPI Software Reset"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REQCLR` writer - Request to Clear the Comparison Trigger"]
pub type ReqclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFCLR` writer - Transmit FIFO Clear"]
pub type TxfclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFCLR` writer - Receive FIFO Clear"]
pub type RxfclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LASTXFER` writer - Last Transfer"]
pub type LastxferW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOEN` writer - FIFO Enable"]
pub type FifoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFODIS` writer - FIFO Disable"]
pub type FifodisW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - SPI Enable"]
    #[inline(always)]
    pub fn spien(&mut self) -> SpienW<FlexSpiCrSpec> {
        SpienW::new(self, 0)
    }
    #[doc = "Bit 1 - SPI Disable"]
    #[inline(always)]
    pub fn spidis(&mut self) -> SpidisW<FlexSpiCrSpec> {
        SpidisW::new(self, 1)
    }
    #[doc = "Bit 7 - SPI Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SwrstW<FlexSpiCrSpec> {
        SwrstW::new(self, 7)
    }
    #[doc = "Bit 12 - Request to Clear the Comparison Trigger"]
    #[inline(always)]
    pub fn reqclr(&mut self) -> ReqclrW<FlexSpiCrSpec> {
        ReqclrW::new(self, 12)
    }
    #[doc = "Bit 16 - Transmit FIFO Clear"]
    #[inline(always)]
    pub fn txfclr(&mut self) -> TxfclrW<FlexSpiCrSpec> {
        TxfclrW::new(self, 16)
    }
    #[doc = "Bit 17 - Receive FIFO Clear"]
    #[inline(always)]
    pub fn rxfclr(&mut self) -> RxfclrW<FlexSpiCrSpec> {
        RxfclrW::new(self, 17)
    }
    #[doc = "Bit 24 - Last Transfer"]
    #[inline(always)]
    pub fn lastxfer(&mut self) -> LastxferW<FlexSpiCrSpec> {
        LastxferW::new(self, 24)
    }
    #[doc = "Bit 30 - FIFO Enable"]
    #[inline(always)]
    pub fn fifoen(&mut self) -> FifoenW<FlexSpiCrSpec> {
        FifoenW::new(self, 30)
    }
    #[doc = "Bit 31 - FIFO Disable"]
    #[inline(always)]
    pub fn fifodis(&mut self) -> FifodisW<FlexSpiCrSpec> {
        FifodisW::new(self, 31)
    }
}
#[doc = "SPI Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_spi_cr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexSpiCrSpec;
impl crate::RegisterSpec for FlexSpiCrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`flex_spi_cr::W`](W) writer structure"]
impl crate::Writable for FlexSpiCrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLEX_SPI_CR to value 0"]
impl crate::Resettable for FlexSpiCrSpec {}
