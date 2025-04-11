#[doc = "Register `IDATAR14` writer"]
pub type W = crate::W<Idatar14Spec>;
#[doc = "Field `IDATA` writer - Input Data"]
pub type IdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Input Data"]
    #[inline(always)]
    pub fn idata(&mut self) -> IdataW<Idatar14Spec> {
        IdataW::new(self, 0)
    }
}
#[doc = "Input Data 0 Register 14\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idatar14::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Idatar14Spec;
impl crate::RegisterSpec for Idatar14Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`idatar14::W`](W) writer structure"]
impl crate::Writable for Idatar14Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDATAR14 to value 0"]
impl crate::Resettable for Idatar14Spec {}
