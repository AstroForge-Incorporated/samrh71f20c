#[doc = "Register `FLEX_SPI_SR` reader"]
pub type R = crate::R<FlexSpiSrSpec>;
#[doc = "Field `RDRF` reader - Receive Data Register Full (cleared by reading FLEX_SPI_RDR)"]
pub type RdrfR = crate::BitReader;
#[doc = "Field `TDRE` reader - Transmit Data Register Empty (cleared by writing FLEX_SPI_TDR)"]
pub type TdreR = crate::BitReader;
#[doc = "Field `MODF` reader - Mode Fault Error (cleared on read)"]
pub type ModfR = crate::BitReader;
#[doc = "Field `OVRES` reader - Overrun Error Status (cleared on read)"]
pub type OvresR = crate::BitReader;
#[doc = "Field `NSSR` reader - NSS Rising (cleared on read)"]
pub type NssrR = crate::BitReader;
#[doc = "Field `TXEMPTY` reader - Transmission Registers Empty (cleared by writing FLEX_SPI_TDR)"]
pub type TxemptyR = crate::BitReader;
#[doc = "Field `UNDES` reader - Underrun Error Status (Slave mode only) (cleared on read)"]
pub type UndesR = crate::BitReader;
#[doc = "Field `CMP` reader - Comparison Status (cleared on read)"]
pub type CmpR = crate::BitReader;
#[doc = "Field `SFERR` reader - Slave Frame Error (cleared on read)"]
pub type SferrR = crate::BitReader;
#[doc = "Field `SPIENS` reader - SPI Enable Status"]
pub type SpiensR = crate::BitReader;
#[doc = "Field `TXFEF` reader - Transmit FIFO Empty Flag (cleared on read)"]
pub type TxfefR = crate::BitReader;
#[doc = "Field `TXFFF` reader - Transmit FIFO Full Flag (cleared on read)"]
pub type TxfffR = crate::BitReader;
#[doc = "Field `TXFTHF` reader - Transmit FIFO Threshold Flag (cleared on read)"]
pub type TxfthfR = crate::BitReader;
#[doc = "Field `RXFEF` reader - Receive FIFO Empty Flag"]
pub type RxfefR = crate::BitReader;
#[doc = "Field `RXFFF` reader - Receive FIFO Full Flag"]
pub type RxfffR = crate::BitReader;
#[doc = "Field `RXFTHF` reader - Receive FIFO Threshold Flag"]
pub type RxfthfR = crate::BitReader;
#[doc = "Field `TXFPTEF` reader - Transmit FIFO Pointer Error Flag"]
pub type TxfptefR = crate::BitReader;
#[doc = "Field `RXFPTEF` reader - Receive FIFO Pointer Error Flag"]
pub type RxfptefR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Receive Data Register Full (cleared by reading FLEX_SPI_RDR)"]
    #[inline(always)]
    pub fn rdrf(&self) -> RdrfR {
        RdrfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Data Register Empty (cleared by writing FLEX_SPI_TDR)"]
    #[inline(always)]
    pub fn tdre(&self) -> TdreR {
        TdreR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mode Fault Error (cleared on read)"]
    #[inline(always)]
    pub fn modf(&self) -> ModfR {
        ModfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun Error Status (cleared on read)"]
    #[inline(always)]
    pub fn ovres(&self) -> OvresR {
        OvresR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - NSS Rising (cleared on read)"]
    #[inline(always)]
    pub fn nssr(&self) -> NssrR {
        NssrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmission Registers Empty (cleared by writing FLEX_SPI_TDR)"]
    #[inline(always)]
    pub fn txempty(&self) -> TxemptyR {
        TxemptyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Underrun Error Status (Slave mode only) (cleared on read)"]
    #[inline(always)]
    pub fn undes(&self) -> UndesR {
        UndesR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Comparison Status (cleared on read)"]
    #[inline(always)]
    pub fn cmp(&self) -> CmpR {
        CmpR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Slave Frame Error (cleared on read)"]
    #[inline(always)]
    pub fn sferr(&self) -> SferrR {
        SferrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - SPI Enable Status"]
    #[inline(always)]
    pub fn spiens(&self) -> SpiensR {
        SpiensR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Transmit FIFO Empty Flag (cleared on read)"]
    #[inline(always)]
    pub fn txfef(&self) -> TxfefR {
        TxfefR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Transmit FIFO Full Flag (cleared on read)"]
    #[inline(always)]
    pub fn txfff(&self) -> TxfffR {
        TxfffR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Transmit FIFO Threshold Flag (cleared on read)"]
    #[inline(always)]
    pub fn txfthf(&self) -> TxfthfR {
        TxfthfR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Receive FIFO Empty Flag"]
    #[inline(always)]
    pub fn rxfef(&self) -> RxfefR {
        RxfefR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Receive FIFO Full Flag"]
    #[inline(always)]
    pub fn rxfff(&self) -> RxfffR {
        RxfffR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Receive FIFO Threshold Flag"]
    #[inline(always)]
    pub fn rxfthf(&self) -> RxfthfR {
        RxfthfR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Transmit FIFO Pointer Error Flag"]
    #[inline(always)]
    pub fn txfptef(&self) -> TxfptefR {
        TxfptefR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Receive FIFO Pointer Error Flag"]
    #[inline(always)]
    pub fn rxfptef(&self) -> RxfptefR {
        RxfptefR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "SPI Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_spi_sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexSpiSrSpec;
impl crate::RegisterSpec for FlexSpiSrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flex_spi_sr::R`](R) reader structure"]
impl crate::Readable for FlexSpiSrSpec {}
#[doc = "`reset()` method sets FLEX_SPI_SR to value 0"]
impl crate::Resettable for FlexSpiSrSpec {}
