#[doc = "Register `FLEX_US_FIMR` reader"]
pub type R = crate::R<FlexUsFimrSpec>;
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
#[doc = "Field `RXFTHF2` reader - RXFTHF2 Interrupt Mask"]
pub type Rxfthf2R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TXFEF Interrupt Mask"]
    #[inline(always)]
    pub fn txfef(&self) -> TxfefR {
        TxfefR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXFFF Interrupt Mask"]
    #[inline(always)]
    pub fn txfff(&self) -> TxfffR {
        TxfffR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TXFTHF Interrupt Mask"]
    #[inline(always)]
    pub fn txfthf(&self) -> TxfthfR {
        TxfthfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RXFEF Interrupt Mask"]
    #[inline(always)]
    pub fn rxfef(&self) -> RxfefR {
        RxfefR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RXFFF Interrupt Mask"]
    #[inline(always)]
    pub fn rxfff(&self) -> RxfffR {
        RxfffR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXFTHF Interrupt Mask"]
    #[inline(always)]
    pub fn rxfthf(&self) -> RxfthfR {
        RxfthfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TXFPTEF Interrupt Mask"]
    #[inline(always)]
    pub fn txfptef(&self) -> TxfptefR {
        TxfptefR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RXFPTEF Interrupt Mask"]
    #[inline(always)]
    pub fn rxfptef(&self) -> RxfptefR {
        RxfptefR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - RXFTHF2 Interrupt Mask"]
    #[inline(always)]
    pub fn rxfthf2(&self) -> Rxfthf2R {
        Rxfthf2R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "USART FIFO Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_fimr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexUsFimrSpec;
impl crate::RegisterSpec for FlexUsFimrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flex_us_fimr::R`](R) reader structure"]
impl crate::Readable for FlexUsFimrSpec {}
#[doc = "`reset()` method sets FLEX_US_FIMR to value 0"]
impl crate::Resettable for FlexUsFimrSpec {}
