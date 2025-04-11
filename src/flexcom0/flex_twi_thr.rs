#[doc = "Register `FLEX_TWI_THR` writer"]
pub type W = crate::W<FlexTwiThrSpec>;
#[doc = "Field `TXDATA` writer - Master or Slave Transmit Holding Data"]
pub type TxdataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Master or Slave Transmit Holding Data"]
    #[inline(always)]
    pub fn txdata(&mut self) -> TxdataW<FlexTwiThrSpec> {
        TxdataW::new(self, 0)
    }
}
#[doc = "TWI Transmit Holding Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_twi_thr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexTwiThrSpec;
impl crate::RegisterSpec for FlexTwiThrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`flex_twi_thr::W`](W) writer structure"]
impl crate::Writable for FlexTwiThrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLEX_TWI_THR to value 0"]
impl crate::Resettable for FlexTwiThrSpec {}
