#[doc = "Register `FLEX_US_THR` writer"]
pub type W = crate::W<FlexUsThrSpec>;
#[doc = "Field `TXCHR` writer - Character to be Transmitted"]
pub type TxchrW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `TXSYNH` writer - Sync Field to be Transmitted"]
pub type TxsynhW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:8 - Character to be Transmitted"]
    #[inline(always)]
    pub fn txchr(&mut self) -> TxchrW<FlexUsThrSpec> {
        TxchrW::new(self, 0)
    }
    #[doc = "Bit 15 - Sync Field to be Transmitted"]
    #[inline(always)]
    pub fn txsynh(&mut self) -> TxsynhW<FlexUsThrSpec> {
        TxsynhW::new(self, 15)
    }
}
#[doc = "USART Transmit Holding Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_us_thr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexUsThrSpec;
impl crate::RegisterSpec for FlexUsThrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`flex_us_thr::W`](W) writer structure"]
impl crate::Writable for FlexUsThrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLEX_US_THR to value 0"]
impl crate::Resettable for FlexUsThrSpec {}
