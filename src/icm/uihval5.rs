#[doc = "Register `UIHVAL5` writer"]
pub type W = crate::W<Uihval5Spec>;
#[doc = "Field `VAL` writer - Initial Hash Value"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Initial Hash Value"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<Uihval5Spec> {
        ValW::new(self, 0)
    }
}
#[doc = "User Initial Hash Value 0 Register 5\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uihval5::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uihval5Spec;
impl crate::RegisterSpec for Uihval5Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`uihval5::W`](W) writer structure"]
impl crate::Writable for Uihval5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UIHVAL5 to value 0"]
impl crate::Resettable for Uihval5Spec {}
