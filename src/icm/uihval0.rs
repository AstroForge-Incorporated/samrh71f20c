#[doc = "Register `UIHVAL0` writer"]
pub type W = crate::W<Uihval0Spec>;
#[doc = "Field `VAL` writer - Initial Hash Value"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Initial Hash Value"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<Uihval0Spec> {
        ValW::new(self, 0)
    }
}
#[doc = "User Initial Hash Value 0 Register 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uihval0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uihval0Spec;
impl crate::RegisterSpec for Uihval0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`uihval0::W`](W) writer structure"]
impl crate::Writable for Uihval0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UIHVAL0 to value 0"]
impl crate::Resettable for Uihval0Spec {}
