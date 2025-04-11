#[doc = "Register `FLEX_TWI_FSR` reader"]
pub type R = crate::R<FlexTwiFsrSpec>;
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
    #[doc = "Bit 0 - Transmit FIFO Empty Flag (cleared on read)"]
    #[inline(always)]
    pub fn txfef(&self) -> TxfefR {
        TxfefR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO Full Flag (cleared on read)"]
    #[inline(always)]
    pub fn txfff(&self) -> TxfffR {
        TxfffR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit FIFO Threshold Flag (cleared on read)"]
    #[inline(always)]
    pub fn txfthf(&self) -> TxfthfR {
        TxfthfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO Empty Flag"]
    #[inline(always)]
    pub fn rxfef(&self) -> RxfefR {
        RxfefR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO Full Flag"]
    #[inline(always)]
    pub fn rxfff(&self) -> RxfffR {
        RxfffR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive FIFO Threshold Flag"]
    #[inline(always)]
    pub fn rxfthf(&self) -> RxfthfR {
        RxfthfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit FIFO Pointer Error Flag"]
    #[inline(always)]
    pub fn txfptef(&self) -> TxfptefR {
        TxfptefR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive FIFO Pointer Error Flag"]
    #[inline(always)]
    pub fn rxfptef(&self) -> RxfptefR {
        RxfptefR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "TWI FIFO Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_twi_fsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexTwiFsrSpec;
impl crate::RegisterSpec for FlexTwiFsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flex_twi_fsr::R`](R) reader structure"]
impl crate::Readable for FlexTwiFsrSpec {}
#[doc = "`reset()` method sets FLEX_TWI_FSR to value 0"]
impl crate::Resettable for FlexTwiFsrSpec {}
