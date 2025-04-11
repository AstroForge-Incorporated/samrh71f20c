#[doc = "Register `MEAR[%s]` reader"]
pub type R = crate::R<MearSpec>;
#[doc = "Field `ERRADD` reader - Master Error Address"]
pub type ErraddR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Master Error Address"]
    #[inline(always)]
    pub fn erradd(&self) -> ErraddR {
        ErraddR::new(self.bits)
    }
}
#[doc = "Master 0 Error Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mear::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MearSpec;
impl crate::RegisterSpec for MearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mear::R`](R) reader structure"]
impl crate::Readable for MearSpec {}
#[doc = "`reset()` method sets MEAR[%s] to value 0"]
impl crate::Resettable for MearSpec {}
