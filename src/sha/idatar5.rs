#[doc = "Register `IDATAR5` writer"]
pub type W = crate::W<Idatar5Spec>;
#[doc = "Field `IDATA` writer - Input Data"]
pub type IdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Input Data"]
    #[inline(always)]
    pub fn idata(&mut self) -> IdataW<Idatar5Spec> {
        IdataW::new(self, 0)
    }
}
#[doc = "Input Data 0 Register 5\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idatar5::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Idatar5Spec;
impl crate::RegisterSpec for Idatar5Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`idatar5::W`](W) writer structure"]
impl crate::Writable for Idatar5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDATAR5 to value 0"]
impl crate::Resettable for Idatar5Spec {}
