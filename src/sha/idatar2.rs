#[doc = "Register `IDATAR2` writer"]
pub type W = crate::W<Idatar2Spec>;
#[doc = "Field `IDATA` writer - Input Data"]
pub type IdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Input Data"]
    #[inline(always)]
    pub fn idata(&mut self) -> IdataW<Idatar2Spec> {
        IdataW::new(self, 0)
    }
}
#[doc = "Input Data 0 Register 2\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idatar2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Idatar2Spec;
impl crate::RegisterSpec for Idatar2Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`idatar2::W`](W) writer structure"]
impl crate::Writable for Idatar2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDATAR2 to value 0"]
impl crate::Resettable for Idatar2Spec {}
