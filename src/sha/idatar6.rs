#[doc = "Register `IDATAR6` writer"]
pub type W = crate::W<Idatar6Spec>;
#[doc = "Field `IDATA` writer - Input Data"]
pub type IdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Input Data"]
    #[inline(always)]
    pub fn idata(&mut self) -> IdataW<Idatar6Spec> {
        IdataW::new(self, 0)
    }
}
#[doc = "Input Data 0 Register 6\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idatar6::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Idatar6Spec;
impl crate::RegisterSpec for Idatar6Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`idatar6::W`](W) writer structure"]
impl crate::Writable for Idatar6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDATAR6 to value 0"]
impl crate::Resettable for Idatar6Spec {}
