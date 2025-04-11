#[doc = "Register `FLEX_SPI_IDR` writer"]
pub type W = crate::W<FlexSpiIdrSpec>;
#[doc = "Field `RDRF` writer - Receive Data Register Full Interrupt Disable"]
pub type RdrfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDRE` writer - SPI Transmit Data Register Empty Interrupt Disable"]
pub type TdreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODF` writer - Mode Fault Error Interrupt Disable"]
pub type ModfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRES` writer - Overrun Error Interrupt Disable"]
pub type OvresW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSSR` writer - NSS Rising Interrupt Disable"]
pub type NssrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEMPTY` writer - Transmission Registers Empty Disable"]
pub type TxemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDES` writer - Underrun Error Interrupt Disable"]
pub type UndesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP` writer - Comparison Interrupt Disable"]
pub type CmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFEF` writer - TXFEF Interrupt Disable"]
pub type TxfefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFFF` writer - TXFFF Interrupt Disable"]
pub type TxfffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFTHF` writer - TXFTHF Interrupt Disable"]
pub type TxfthfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFEF` writer - RXFEF Interrupt Disable"]
pub type RxfefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFFF` writer - RXFFF Interrupt Disable"]
pub type RxfffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFTHF` writer - RXFTHF Interrupt Disable"]
pub type RxfthfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFPTEF` writer - TXFPTEF Interrupt Disable"]
pub type TxfptefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFPTEF` writer - RXFPTEF Interrupt Disable"]
pub type RxfptefW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Receive Data Register Full Interrupt Disable"]
    #[inline(always)]
    pub fn rdrf(&mut self) -> RdrfW<FlexSpiIdrSpec> {
        RdrfW::new(self, 0)
    }
    #[doc = "Bit 1 - SPI Transmit Data Register Empty Interrupt Disable"]
    #[inline(always)]
    pub fn tdre(&mut self) -> TdreW<FlexSpiIdrSpec> {
        TdreW::new(self, 1)
    }
    #[doc = "Bit 2 - Mode Fault Error Interrupt Disable"]
    #[inline(always)]
    pub fn modf(&mut self) -> ModfW<FlexSpiIdrSpec> {
        ModfW::new(self, 2)
    }
    #[doc = "Bit 3 - Overrun Error Interrupt Disable"]
    #[inline(always)]
    pub fn ovres(&mut self) -> OvresW<FlexSpiIdrSpec> {
        OvresW::new(self, 3)
    }
    #[doc = "Bit 8 - NSS Rising Interrupt Disable"]
    #[inline(always)]
    pub fn nssr(&mut self) -> NssrW<FlexSpiIdrSpec> {
        NssrW::new(self, 8)
    }
    #[doc = "Bit 9 - Transmission Registers Empty Disable"]
    #[inline(always)]
    pub fn txempty(&mut self) -> TxemptyW<FlexSpiIdrSpec> {
        TxemptyW::new(self, 9)
    }
    #[doc = "Bit 10 - Underrun Error Interrupt Disable"]
    #[inline(always)]
    pub fn undes(&mut self) -> UndesW<FlexSpiIdrSpec> {
        UndesW::new(self, 10)
    }
    #[doc = "Bit 11 - Comparison Interrupt Disable"]
    #[inline(always)]
    pub fn cmp(&mut self) -> CmpW<FlexSpiIdrSpec> {
        CmpW::new(self, 11)
    }
    #[doc = "Bit 24 - TXFEF Interrupt Disable"]
    #[inline(always)]
    pub fn txfef(&mut self) -> TxfefW<FlexSpiIdrSpec> {
        TxfefW::new(self, 24)
    }
    #[doc = "Bit 25 - TXFFF Interrupt Disable"]
    #[inline(always)]
    pub fn txfff(&mut self) -> TxfffW<FlexSpiIdrSpec> {
        TxfffW::new(self, 25)
    }
    #[doc = "Bit 26 - TXFTHF Interrupt Disable"]
    #[inline(always)]
    pub fn txfthf(&mut self) -> TxfthfW<FlexSpiIdrSpec> {
        TxfthfW::new(self, 26)
    }
    #[doc = "Bit 27 - RXFEF Interrupt Disable"]
    #[inline(always)]
    pub fn rxfef(&mut self) -> RxfefW<FlexSpiIdrSpec> {
        RxfefW::new(self, 27)
    }
    #[doc = "Bit 28 - RXFFF Interrupt Disable"]
    #[inline(always)]
    pub fn rxfff(&mut self) -> RxfffW<FlexSpiIdrSpec> {
        RxfffW::new(self, 28)
    }
    #[doc = "Bit 29 - RXFTHF Interrupt Disable"]
    #[inline(always)]
    pub fn rxfthf(&mut self) -> RxfthfW<FlexSpiIdrSpec> {
        RxfthfW::new(self, 29)
    }
    #[doc = "Bit 30 - TXFPTEF Interrupt Disable"]
    #[inline(always)]
    pub fn txfptef(&mut self) -> TxfptefW<FlexSpiIdrSpec> {
        TxfptefW::new(self, 30)
    }
    #[doc = "Bit 31 - RXFPTEF Interrupt Disable"]
    #[inline(always)]
    pub fn rxfptef(&mut self) -> RxfptefW<FlexSpiIdrSpec> {
        RxfptefW::new(self, 31)
    }
}
#[doc = "SPI Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_spi_idr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexSpiIdrSpec;
impl crate::RegisterSpec for FlexSpiIdrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`flex_spi_idr::W`](W) writer structure"]
impl crate::Writable for FlexSpiIdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLEX_SPI_IDR to value 0"]
impl crate::Resettable for FlexSpiIdrSpec {}
