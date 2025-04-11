#[doc = "Register `FLEX_US_FIER` writer"]
pub type W = crate::W<FlexUsFierSpec>;
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
#[doc = "Field `RXFTHF2` writer - RXFTHF2 Interrupt Enable"]
pub type Rxfthf2W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - TXFEF Interrupt Enable"]
    #[inline(always)]
    pub fn txfef(&mut self) -> TxfefW<FlexUsFierSpec> {
        TxfefW::new(self, 0)
    }
    #[doc = "Bit 1 - TXFFF Interrupt Enable"]
    #[inline(always)]
    pub fn txfff(&mut self) -> TxfffW<FlexUsFierSpec> {
        TxfffW::new(self, 1)
    }
    #[doc = "Bit 2 - TXFTHF Interrupt Enable"]
    #[inline(always)]
    pub fn txfthf(&mut self) -> TxfthfW<FlexUsFierSpec> {
        TxfthfW::new(self, 2)
    }
    #[doc = "Bit 3 - RXFEF Interrupt Enable"]
    #[inline(always)]
    pub fn rxfef(&mut self) -> RxfefW<FlexUsFierSpec> {
        RxfefW::new(self, 3)
    }
    #[doc = "Bit 4 - RXFFF Interrupt Enable"]
    #[inline(always)]
    pub fn rxfff(&mut self) -> RxfffW<FlexUsFierSpec> {
        RxfffW::new(self, 4)
    }
    #[doc = "Bit 5 - RXFTHF Interrupt Enable"]
    #[inline(always)]
    pub fn rxfthf(&mut self) -> RxfthfW<FlexUsFierSpec> {
        RxfthfW::new(self, 5)
    }
    #[doc = "Bit 6 - TXFPTEF Interrupt Enable"]
    #[inline(always)]
    pub fn txfptef(&mut self) -> TxfptefW<FlexUsFierSpec> {
        TxfptefW::new(self, 6)
    }
    #[doc = "Bit 7 - RXFPTEF Interrupt Enable"]
    #[inline(always)]
    pub fn rxfptef(&mut self) -> RxfptefW<FlexUsFierSpec> {
        RxfptefW::new(self, 7)
    }
    #[doc = "Bit 9 - RXFTHF2 Interrupt Enable"]
    #[inline(always)]
    pub fn rxfthf2(&mut self) -> Rxfthf2W<FlexUsFierSpec> {
        Rxfthf2W::new(self, 9)
    }
}
#[doc = "USART FIFO Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_us_fier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexUsFierSpec;
impl crate::RegisterSpec for FlexUsFierSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`flex_us_fier::W`](W) writer structure"]
impl crate::Writable for FlexUsFierSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLEX_US_FIER to value 0"]
impl crate::Resettable for FlexUsFierSpec {}
