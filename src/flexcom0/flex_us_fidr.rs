#[doc = "Register `FLEX_US_FIDR` writer"]
pub type W = crate::W<FlexUsFidrSpec>;
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
#[doc = "Field `RXFTHF2` writer - RXFTHF2 Interrupt Disable"]
pub type Rxfthf2W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - TXFEF Interrupt Disable"]
    #[inline(always)]
    pub fn txfef(&mut self) -> TxfefW<FlexUsFidrSpec> {
        TxfefW::new(self, 0)
    }
    #[doc = "Bit 1 - TXFFF Interrupt Disable"]
    #[inline(always)]
    pub fn txfff(&mut self) -> TxfffW<FlexUsFidrSpec> {
        TxfffW::new(self, 1)
    }
    #[doc = "Bit 2 - TXFTHF Interrupt Disable"]
    #[inline(always)]
    pub fn txfthf(&mut self) -> TxfthfW<FlexUsFidrSpec> {
        TxfthfW::new(self, 2)
    }
    #[doc = "Bit 3 - RXFEF Interrupt Disable"]
    #[inline(always)]
    pub fn rxfef(&mut self) -> RxfefW<FlexUsFidrSpec> {
        RxfefW::new(self, 3)
    }
    #[doc = "Bit 4 - RXFFF Interrupt Disable"]
    #[inline(always)]
    pub fn rxfff(&mut self) -> RxfffW<FlexUsFidrSpec> {
        RxfffW::new(self, 4)
    }
    #[doc = "Bit 5 - RXFTHF Interrupt Disable"]
    #[inline(always)]
    pub fn rxfthf(&mut self) -> RxfthfW<FlexUsFidrSpec> {
        RxfthfW::new(self, 5)
    }
    #[doc = "Bit 6 - TXFPTEF Interrupt Disable"]
    #[inline(always)]
    pub fn txfptef(&mut self) -> TxfptefW<FlexUsFidrSpec> {
        TxfptefW::new(self, 6)
    }
    #[doc = "Bit 7 - RXFPTEF Interrupt Disable"]
    #[inline(always)]
    pub fn rxfptef(&mut self) -> RxfptefW<FlexUsFidrSpec> {
        RxfptefW::new(self, 7)
    }
    #[doc = "Bit 9 - RXFTHF2 Interrupt Disable"]
    #[inline(always)]
    pub fn rxfthf2(&mut self) -> Rxfthf2W<FlexUsFidrSpec> {
        Rxfthf2W::new(self, 9)
    }
}
#[doc = "USART FIFO Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_us_fidr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexUsFidrSpec;
impl crate::RegisterSpec for FlexUsFidrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`flex_us_fidr::W`](W) writer structure"]
impl crate::Writable for FlexUsFidrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLEX_US_FIDR to value 0"]
impl crate::Resettable for FlexUsFidrSpec {}
