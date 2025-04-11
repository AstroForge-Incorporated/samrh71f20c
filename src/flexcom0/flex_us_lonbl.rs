#[doc = "Register `FLEX_US_LONBL` reader"]
pub type R = crate::R<FlexUsLonblSpec>;
#[doc = "Field `LONBL` reader - LON Node Backlog Value"]
pub type LonblR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - LON Node Backlog Value"]
    #[inline(always)]
    pub fn lonbl(&self) -> LonblR {
        LonblR::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "USART LON Backlog Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_lonbl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexUsLonblSpec;
impl crate::RegisterSpec for FlexUsLonblSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flex_us_lonbl::R`](R) reader structure"]
impl crate::Readable for FlexUsLonblSpec {}
#[doc = "`reset()` method sets FLEX_US_LONBL to value 0"]
impl crate::Resettable for FlexUsLonblSpec {}
