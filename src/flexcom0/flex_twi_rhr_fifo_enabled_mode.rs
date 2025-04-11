#[doc = "Register `FLEX_TWI_RHR_FIFO_ENABLED_MODE` reader"]
pub type R = crate::R<FlexTwiRhrFifoEnabledModeSpec>;
#[doc = "Field `RXDATA0` reader - Master or Slave Receive Holding Data 0"]
pub type Rxdata0R = crate::FieldReader;
#[doc = "Field `RXDATA1` reader - Master or Slave Receive Holding Data 1"]
pub type Rxdata1R = crate::FieldReader;
#[doc = "Field `RXDATA2` reader - Master or Slave Receive Holding Data 2"]
pub type Rxdata2R = crate::FieldReader;
#[doc = "Field `RXDATA3` reader - Master or Slave Receive Holding Data 3"]
pub type Rxdata3R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Master or Slave Receive Holding Data 0"]
    #[inline(always)]
    pub fn rxdata0(&self) -> Rxdata0R {
        Rxdata0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Master or Slave Receive Holding Data 1"]
    #[inline(always)]
    pub fn rxdata1(&self) -> Rxdata1R {
        Rxdata1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Master or Slave Receive Holding Data 2"]
    #[inline(always)]
    pub fn rxdata2(&self) -> Rxdata2R {
        Rxdata2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Master or Slave Receive Holding Data 3"]
    #[inline(always)]
    pub fn rxdata3(&self) -> Rxdata3R {
        Rxdata3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "TWI Receive Holding Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_twi_rhr_fifo_enabled_mode::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexTwiRhrFifoEnabledModeSpec;
impl crate::RegisterSpec for FlexTwiRhrFifoEnabledModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flex_twi_rhr_fifo_enabled_mode::R`](R) reader structure"]
impl crate::Readable for FlexTwiRhrFifoEnabledModeSpec {}
#[doc = "`reset()` method sets FLEX_TWI_RHR_FIFO_ENABLED_MODE to value 0"]
impl crate::Resettable for FlexTwiRhrFifoEnabledModeSpec {}
