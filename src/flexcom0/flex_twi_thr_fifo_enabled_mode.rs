#[doc = "Register `FLEX_TWI_THR_FIFO_ENABLED_MODE` writer"]
pub type W = crate::W<FlexTwiThrFifoEnabledModeSpec>;
#[doc = "Field `TXDATA0` writer - Master or Slave Transmit Holding Data 0"]
pub type Txdata0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TXDATA1` writer - Master or Slave Transmit Holding Data 1"]
pub type Txdata1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TXDATA2` writer - Master or Slave Transmit Holding Data 2"]
pub type Txdata2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TXDATA3` writer - Master or Slave Transmit Holding Data 3"]
pub type Txdata3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Master or Slave Transmit Holding Data 0"]
    #[inline(always)]
    pub fn txdata0(&mut self) -> Txdata0W<FlexTwiThrFifoEnabledModeSpec> {
        Txdata0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Master or Slave Transmit Holding Data 1"]
    #[inline(always)]
    pub fn txdata1(&mut self) -> Txdata1W<FlexTwiThrFifoEnabledModeSpec> {
        Txdata1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Master or Slave Transmit Holding Data 2"]
    #[inline(always)]
    pub fn txdata2(&mut self) -> Txdata2W<FlexTwiThrFifoEnabledModeSpec> {
        Txdata2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Master or Slave Transmit Holding Data 3"]
    #[inline(always)]
    pub fn txdata3(&mut self) -> Txdata3W<FlexTwiThrFifoEnabledModeSpec> {
        Txdata3W::new(self, 24)
    }
}
#[doc = "TWI Transmit Holding Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_twi_thr_fifo_enabled_mode::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexTwiThrFifoEnabledModeSpec;
impl crate::RegisterSpec for FlexTwiThrFifoEnabledModeSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`flex_twi_thr_fifo_enabled_mode::W`](W) writer structure"]
impl crate::Writable for FlexTwiThrFifoEnabledModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLEX_TWI_THR_FIFO_ENABLED_MODE to value 0"]
impl crate::Resettable for FlexTwiThrFifoEnabledModeSpec {}
