#[doc = "Register `FLEX_US_RHR` reader"]
pub type R = crate::R<FlexUsRhrSpec>;
#[doc = "Field `RXCHR` reader - Received Character"]
pub type RxchrR = crate::FieldReader<u16>;
#[doc = "Field `RXSYNH` reader - Received Sync"]
pub type RxsynhR = crate::BitReader;
impl R {
    #[doc = "Bits 0:8 - Received Character"]
    #[inline(always)]
    pub fn rxchr(&self) -> RxchrR {
        RxchrR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 15 - Received Sync"]
    #[inline(always)]
    pub fn rxsynh(&self) -> RxsynhR {
        RxsynhR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "USART Receive Holding Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_rhr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexUsRhrSpec;
impl crate::RegisterSpec for FlexUsRhrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flex_us_rhr::R`](R) reader structure"]
impl crate::Readable for FlexUsRhrSpec {}
#[doc = "`reset()` method sets FLEX_US_RHR to value 0"]
impl crate::Resettable for FlexUsRhrSpec {}
