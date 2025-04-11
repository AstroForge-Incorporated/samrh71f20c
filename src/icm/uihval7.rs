#[doc = "Register `UIHVAL7` writer"]
pub type W = crate::W<Uihval7Spec>;
#[doc = "Field `VAL` writer - Initial Hash Value"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Initial Hash Value"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<Uihval7Spec> {
        ValW::new(self, 0)
    }
}
#[doc = "User Initial Hash Value 0 Register 7\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uihval7::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uihval7Spec;
impl crate::RegisterSpec for Uihval7Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`uihval7::W`](W) writer structure"]
impl crate::Writable for Uihval7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UIHVAL7 to value 0"]
impl crate::Resettable for Uihval7Spec {}
