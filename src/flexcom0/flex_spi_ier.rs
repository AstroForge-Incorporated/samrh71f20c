#[doc = "Register `FLEX_SPI_IER` writer"]
pub type W = crate::W<FlexSpiIerSpec>;
#[doc = "Field `RDRF` writer - Receive Data Register Full Interrupt Enable"]
pub type RdrfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDRE` writer - SPI Transmit Data Register Empty Interrupt Enable"]
pub type TdreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODF` writer - Mode Fault Error Interrupt Enable"]
pub type ModfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRES` writer - Overrun Error Interrupt Enable"]
pub type OvresW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSSR` writer - NSS Rising Interrupt Enable"]
pub type NssrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEMPTY` writer - Transmission Registers Empty Enable"]
pub type TxemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDES` writer - Underrun Error Interrupt Enable"]
pub type UndesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP` writer - Comparison Interrupt Enable"]
pub type CmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFEF` writer - TXFEF Interrupt Enable"]
pub type TxfefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFFF` writer - TXFFF Interrupt Enable"]
pub type TxfffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFTHF` writer - TXFTHF Interrupt Enable"]
pub type TxfthfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFEF` writer - RXFEF Interrupt Enable"]
pub type RxfefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFFF` writer - RXFFF Interrupt Enable"]
pub type RxfffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFTHF` writer - RXFTHF Interrupt Enable"]
pub type RxfthfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFPTEF` writer - TXFPTEF Interrupt Enable"]
pub type TxfptefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFPTEF` writer - RXFPTEF Interrupt Enable"]
pub type RxfptefW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Receive Data Register Full Interrupt Enable"]
    #[inline(always)]
    pub fn rdrf(&mut self) -> RdrfW<FlexSpiIerSpec> {
        RdrfW::new(self, 0)
    }
    #[doc = "Bit 1 - SPI Transmit Data Register Empty Interrupt Enable"]
    #[inline(always)]
    pub fn tdre(&mut self) -> TdreW<FlexSpiIerSpec> {
        TdreW::new(self, 1)
    }
    #[doc = "Bit 2 - Mode Fault Error Interrupt Enable"]
    #[inline(always)]
    pub fn modf(&mut self) -> ModfW<FlexSpiIerSpec> {
        ModfW::new(self, 2)
    }
    #[doc = "Bit 3 - Overrun Error Interrupt Enable"]
    #[inline(always)]
    pub fn ovres(&mut self) -> OvresW<FlexSpiIerSpec> {
        OvresW::new(self, 3)
    }
    #[doc = "Bit 8 - NSS Rising Interrupt Enable"]
    #[inline(always)]
    pub fn nssr(&mut self) -> NssrW<FlexSpiIerSpec> {
        NssrW::new(self, 8)
    }
    #[doc = "Bit 9 - Transmission Registers Empty Enable"]
    #[inline(always)]
    pub fn txempty(&mut self) -> TxemptyW<FlexSpiIerSpec> {
        TxemptyW::new(self, 9)
    }
    #[doc = "Bit 10 - Underrun Error Interrupt Enable"]
    #[inline(always)]
    pub fn undes(&mut self) -> UndesW<FlexSpiIerSpec> {
        UndesW::new(self, 10)
    }
    #[doc = "Bit 11 - Comparison Interrupt Enable"]
    #[inline(always)]
    pub fn cmp(&mut self) -> CmpW<FlexSpiIerSpec> {
        CmpW::new(self, 11)
    }
    #[doc = "Bit 24 - TXFEF Interrupt Enable"]
    #[inline(always)]
    pub fn txfef(&mut self) -> TxfefW<FlexSpiIerSpec> {
        TxfefW::new(self, 24)
    }
    #[doc = "Bit 25 - TXFFF Interrupt Enable"]
    #[inline(always)]
    pub fn txfff(&mut self) -> TxfffW<FlexSpiIerSpec> {
        TxfffW::new(self, 25)
    }
    #[doc = "Bit 26 - TXFTHF Interrupt Enable"]
    #[inline(always)]
    pub fn txfthf(&mut self) -> TxfthfW<FlexSpiIerSpec> {
        TxfthfW::new(self, 26)
    }
    #[doc = "Bit 27 - RXFEF Interrupt Enable"]
    #[inline(always)]
    pub fn rxfef(&mut self) -> RxfefW<FlexSpiIerSpec> {
        RxfefW::new(self, 27)
    }
    #[doc = "Bit 28 - RXFFF Interrupt Enable"]
    #[inline(always)]
    pub fn rxfff(&mut self) -> RxfffW<FlexSpiIerSpec> {
        RxfffW::new(self, 28)
    }
    #[doc = "Bit 29 - RXFTHF Interrupt Enable"]
    #[inline(always)]
    pub fn rxfthf(&mut self) -> RxfthfW<FlexSpiIerSpec> {
        RxfthfW::new(self, 29)
    }
    #[doc = "Bit 30 - TXFPTEF Interrupt Enable"]
    #[inline(always)]
    pub fn txfptef(&mut self) -> TxfptefW<FlexSpiIerSpec> {
        TxfptefW::new(self, 30)
    }
    #[doc = "Bit 31 - RXFPTEF Interrupt Enable"]
    #[inline(always)]
    pub fn rxfptef(&mut self) -> RxfptefW<FlexSpiIerSpec> {
        RxfptefW::new(self, 31)
    }
}
#[doc = "SPI Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_spi_ier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexSpiIerSpec;
impl crate::RegisterSpec for FlexSpiIerSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`flex_spi_ier::W`](W) writer structure"]
impl crate::Writable for FlexSpiIerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLEX_SPI_IER to value 0"]
impl crate::Resettable for FlexSpiIerSpec {}
