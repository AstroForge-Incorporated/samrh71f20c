#[doc = "Register `FLEX_US_THR_FIFO_MULTI_DATA_MODE` writer"]
pub type W = crate::W<FlexUsThrFifoMultiDataModeSpec>;
#[doc = "Field `TXCHR0` writer - Character to be Transmitted"]
pub type Txchr0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TXCHR1` writer - Character to be Transmitted"]
pub type Txchr1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TXCHR2` writer - Character to be Transmitted"]
pub type Txchr2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TXCHR3` writer - Character to be Transmitted"]
pub type Txchr3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Character to be Transmitted"]
    #[inline(always)]
    pub fn txchr0(&mut self) -> Txchr0W<FlexUsThrFifoMultiDataModeSpec> {
        Txchr0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Character to be Transmitted"]
    #[inline(always)]
    pub fn txchr1(&mut self) -> Txchr1W<FlexUsThrFifoMultiDataModeSpec> {
        Txchr1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Character to be Transmitted"]
    #[inline(always)]
    pub fn txchr2(&mut self) -> Txchr2W<FlexUsThrFifoMultiDataModeSpec> {
        Txchr2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Character to be Transmitted"]
    #[inline(always)]
    pub fn txchr3(&mut self) -> Txchr3W<FlexUsThrFifoMultiDataModeSpec> {
        Txchr3W::new(self, 24)
    }
}
#[doc = "USART Transmit Holding Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_us_thr_fifo_multi_data_mode::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexUsThrFifoMultiDataModeSpec;
impl crate::RegisterSpec for FlexUsThrFifoMultiDataModeSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`flex_us_thr_fifo_multi_data_mode::W`](W) writer structure"]
impl crate::Writable for FlexUsThrFifoMultiDataModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLEX_US_THR_FIFO_MULTI_DATA_MODE to value 0"]
impl crate::Resettable for FlexUsThrFifoMultiDataModeSpec {}
