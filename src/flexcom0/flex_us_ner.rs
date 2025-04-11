#[doc = "Register `FLEX_US_NER` reader"]
pub type R = crate::R<FlexUsNerSpec>;
#[doc = "Field `NB_ERRORS` reader - Number of Errors"]
pub type NbErrorsR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Number of Errors"]
    #[inline(always)]
    pub fn nb_errors(&self) -> NbErrorsR {
        NbErrorsR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "USART Number of Errors Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_ner::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexUsNerSpec;
impl crate::RegisterSpec for FlexUsNerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flex_us_ner::R`](R) reader structure"]
impl crate::Readable for FlexUsNerSpec {}
#[doc = "`reset()` method sets FLEX_US_NER to value 0"]
impl crate::Resettable for FlexUsNerSpec {}
