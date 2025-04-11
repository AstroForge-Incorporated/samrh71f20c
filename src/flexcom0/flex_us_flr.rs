#[doc = "Register `FLEX_US_FLR` reader"]
pub type R = crate::R<FlexUsFlrSpec>;
#[doc = "Field `TXFL` reader - Transmit FIFO Level"]
pub type TxflR = crate::FieldReader;
#[doc = "Field `RXFL` reader - Receive FIFO Level"]
pub type RxflR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Transmit FIFO Level"]
    #[inline(always)]
    pub fn txfl(&self) -> TxflR {
        TxflR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Receive FIFO Level"]
    #[inline(always)]
    pub fn rxfl(&self) -> RxflR {
        RxflR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
#[doc = "USART FIFO Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_flr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexUsFlrSpec;
impl crate::RegisterSpec for FlexUsFlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flex_us_flr::R`](R) reader structure"]
impl crate::Readable for FlexUsFlrSpec {}
#[doc = "`reset()` method sets FLEX_US_FLR to value 0"]
impl crate::Resettable for FlexUsFlrSpec {}
