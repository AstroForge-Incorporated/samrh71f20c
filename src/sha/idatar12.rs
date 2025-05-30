#[doc = "Register `IDATAR12` writer"]
pub type W = crate::W<Idatar12Spec>;
#[doc = "Field `IDATA` writer - Input Data"]
pub type IdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Input Data"]
    #[inline(always)]
    pub fn idata(&mut self) -> IdataW<Idatar12Spec> {
        IdataW::new(self, 0)
    }
}
#[doc = "Input Data 0 Register 12\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idatar12::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Idatar12Spec;
impl crate::RegisterSpec for Idatar12Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`idatar12::W`](W) writer structure"]
impl crate::Writable for Idatar12Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDATAR12 to value 0"]
impl crate::Resettable for Idatar12Spec {}
