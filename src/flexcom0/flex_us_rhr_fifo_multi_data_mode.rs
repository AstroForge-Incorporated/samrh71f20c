#[doc = "Register `FLEX_US_RHR_FIFO_MULTI_DATA_MODE` reader"]
pub type R = crate::R<FlexUsRhrFifoMultiDataModeSpec>;
#[doc = "Field `RXCHR0` reader - Received Character"]
pub type Rxchr0R = crate::FieldReader;
#[doc = "Field `RXCHR1` reader - Received Character"]
pub type Rxchr1R = crate::FieldReader;
#[doc = "Field `RXCHR2` reader - Received Character"]
pub type Rxchr2R = crate::FieldReader;
#[doc = "Field `RXCHR3` reader - Received Character"]
pub type Rxchr3R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Received Character"]
    #[inline(always)]
    pub fn rxchr0(&self) -> Rxchr0R {
        Rxchr0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Received Character"]
    #[inline(always)]
    pub fn rxchr1(&self) -> Rxchr1R {
        Rxchr1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Received Character"]
    #[inline(always)]
    pub fn rxchr2(&self) -> Rxchr2R {
        Rxchr2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Received Character"]
    #[inline(always)]
    pub fn rxchr3(&self) -> Rxchr3R {
        Rxchr3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "USART Receive Holding Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_rhr_fifo_multi_data_mode::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexUsRhrFifoMultiDataModeSpec;
impl crate::RegisterSpec for FlexUsRhrFifoMultiDataModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flex_us_rhr_fifo_multi_data_mode::R`](R) reader structure"]
impl crate::Readable for FlexUsRhrFifoMultiDataModeSpec {}
#[doc = "`reset()` method sets FLEX_US_RHR_FIFO_MULTI_DATA_MODE to value 0"]
impl crate::Resettable for FlexUsRhrFifoMultiDataModeSpec {}
