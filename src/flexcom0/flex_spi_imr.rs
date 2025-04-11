#[doc = "Register `FLEX_SPI_IMR` reader"]
pub type R = crate::R<FlexSpiImrSpec>;
#[doc = "Field `RDRF` reader - Receive Data Register Full Interrupt Mask"]
pub type RdrfR = crate::BitReader;
#[doc = "Field `TDRE` reader - SPI Transmit Data Register Empty Interrupt Mask"]
pub type TdreR = crate::BitReader;
#[doc = "Field `MODF` reader - Mode Fault Error Interrupt Mask"]
pub type ModfR = crate::BitReader;
#[doc = "Field `OVRES` reader - Overrun Error Interrupt Mask"]
pub type OvresR = crate::BitReader;
#[doc = "Field `NSSR` reader - NSS Rising Interrupt Mask"]
pub type NssrR = crate::BitReader;
#[doc = "Field `TXEMPTY` reader - Transmission Registers Empty Mask"]
pub type TxemptyR = crate::BitReader;
#[doc = "Field `UNDES` reader - Underrun Error Interrupt Mask"]
pub type UndesR = crate::BitReader;
#[doc = "Field `CMP` reader - Comparison Interrupt Mask"]
pub type CmpR = crate::BitReader;
#[doc = "Field `TXFEF` reader - TXFEF Interrupt Mask"]
pub type TxfefR = crate::BitReader;
#[doc = "Field `TXFFF` reader - TXFFF Interrupt Mask"]
pub type TxfffR = crate::BitReader;
#[doc = "Field `TXFTHF` reader - TXFTHF Interrupt Mask"]
pub type TxfthfR = crate::BitReader;
#[doc = "Field `RXFEF` reader - RXFEF Interrupt Mask"]
pub type RxfefR = crate::BitReader;
#[doc = "Field `RXFFF` reader - RXFFF Interrupt Mask"]
pub type RxfffR = crate::BitReader;
#[doc = "Field `RXFTHF` reader - RXFTHF Interrupt Mask"]
pub type RxfthfR = crate::BitReader;
#[doc = "Field `TXFPTEF` reader - TXFPTEF Interrupt Mask"]
pub type TxfptefR = crate::BitReader;
#[doc = "Field `RXFPTEF` reader - RXFPTEF Interrupt Mask"]
pub type RxfptefR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Receive Data Register Full Interrupt Mask"]
    #[inline(always)]
    pub fn rdrf(&self) -> RdrfR {
        RdrfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SPI Transmit Data Register Empty Interrupt Mask"]
    #[inline(always)]
    pub fn tdre(&self) -> TdreR {
        TdreR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mode Fault Error Interrupt Mask"]
    #[inline(always)]
    pub fn modf(&self) -> ModfR {
        ModfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn ovres(&self) -> OvresR {
        OvresR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - NSS Rising Interrupt Mask"]
    #[inline(always)]
    pub fn nssr(&self) -> NssrR {
        NssrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmission Registers Empty Mask"]
    #[inline(always)]
    pub fn txempty(&self) -> TxemptyR {
        TxemptyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Underrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn undes(&self) -> UndesR {
        UndesR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Comparison Interrupt Mask"]
    #[inline(always)]
    pub fn cmp(&self) -> CmpR {
        CmpR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 24 - TXFEF Interrupt Mask"]
    #[inline(always)]
    pub fn txfef(&self) -> TxfefR {
        TxfefR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - TXFFF Interrupt Mask"]
    #[inline(always)]
    pub fn txfff(&self) -> TxfffR {
        TxfffR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - TXFTHF Interrupt Mask"]
    #[inline(always)]
    pub fn txfthf(&self) -> TxfthfR {
        TxfthfR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - RXFEF Interrupt Mask"]
    #[inline(always)]
    pub fn rxfef(&self) -> RxfefR {
        RxfefR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - RXFFF Interrupt Mask"]
    #[inline(always)]
    pub fn rxfff(&self) -> RxfffR {
        RxfffR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - RXFTHF Interrupt Mask"]
    #[inline(always)]
    pub fn rxfthf(&self) -> RxfthfR {
        RxfthfR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - TXFPTEF Interrupt Mask"]
    #[inline(always)]
    pub fn txfptef(&self) -> TxfptefR {
        TxfptefR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - RXFPTEF Interrupt Mask"]
    #[inline(always)]
    pub fn rxfptef(&self) -> RxfptefR {
        RxfptefR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "SPI Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_spi_imr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexSpiImrSpec;
impl crate::RegisterSpec for FlexSpiImrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flex_spi_imr::R`](R) reader structure"]
impl crate::Readable for FlexSpiImrSpec {}
#[doc = "`reset()` method sets FLEX_SPI_IMR to value 0"]
impl crate::Resettable for FlexSpiImrSpec {}
