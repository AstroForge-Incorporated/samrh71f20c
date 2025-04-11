#[doc = "Register `CTRL3` reader"]
pub type R = crate::R<Ctrl3Spec>;
#[doc = "Field `FROMSTATUSWORD` reader - content of status word register"]
pub type FromstatuswordR = crate::FieldReader<u16>;
#[doc = "Field `FROMBITWORD` reader - content of bit word register"]
pub type FrombitwordR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - content of status word register"]
    #[inline(always)]
    pub fn fromstatusword(&self) -> FromstatuswordR {
        FromstatuswordR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - content of bit word register"]
    #[inline(always)]
    pub fn frombitword(&self) -> FrombitwordR {
        FrombitwordR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Control Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl3Spec;
impl crate::RegisterSpec for Ctrl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl3::R`](R) reader structure"]
impl crate::Readable for Ctrl3Spec {}
#[doc = "`reset()` method sets CTRL3 to value 0"]
impl crate::Resettable for Ctrl3Spec {}
