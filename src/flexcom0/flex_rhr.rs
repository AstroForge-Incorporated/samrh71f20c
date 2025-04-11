#[doc = "Register `FLEX_RHR` reader"]
pub type R = crate::R<FlexRhrSpec>;
#[doc = "Field `RXDATA` reader - Receive Data"]
pub type RxdataR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Receive Data"]
    #[inline(always)]
    pub fn rxdata(&self) -> RxdataR {
        RxdataR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "FLEXCOM Receive Holding Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_rhr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexRhrSpec;
impl crate::RegisterSpec for FlexRhrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flex_rhr::R`](R) reader structure"]
impl crate::Readable for FlexRhrSpec {}
#[doc = "`reset()` method sets FLEX_RHR to value 0"]
impl crate::Resettable for FlexRhrSpec {}
