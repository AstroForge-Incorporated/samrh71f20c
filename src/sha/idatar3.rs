#[doc = "Register `IDATAR3` writer"]
pub type W = crate::W<Idatar3Spec>;
#[doc = "Field `IDATA` writer - Input Data"]
pub type IdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Input Data"]
    #[inline(always)]
    pub fn idata(&mut self) -> IdataW<Idatar3Spec> {
        IdataW::new(self, 0)
    }
}
#[doc = "Input Data 0 Register 3\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idatar3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Idatar3Spec;
impl crate::RegisterSpec for Idatar3Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`idatar3::W`](W) writer structure"]
impl crate::Writable for Idatar3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDATAR3 to value 0"]
impl crate::Resettable for Idatar3Spec {}
