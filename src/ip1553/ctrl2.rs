#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<Ctrl2Spec>;
#[doc = "Field `FROMCOMMANDREG` reader - from command register"]
pub type FromcommandregR = crate::FieldReader;
#[doc = "Field `FROMVECTORWORD` reader - from vector register"]
pub type FromvectorwordR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - from command register"]
    #[inline(always)]
    pub fn fromcommandreg(&self) -> FromcommandregR {
        FromcommandregR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:19 - from vector register"]
    #[inline(always)]
    pub fn fromvectorword(&self) -> FromvectorwordR {
        FromvectorwordR::new(((self.bits >> 4) & 0xffff) as u16)
    }
}
#[doc = "Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl2Spec;
impl crate::RegisterSpec for Ctrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl2::R`](R) reader structure"]
impl crate::Readable for Ctrl2Spec {}
#[doc = "`reset()` method sets CTRL2 to value 0"]
impl crate::Resettable for Ctrl2Spec {}
