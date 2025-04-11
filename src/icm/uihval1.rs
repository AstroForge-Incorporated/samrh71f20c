#[doc = "Register `UIHVAL1` writer"]
pub type W = crate::W<Uihval1Spec>;
#[doc = "Field `VAL` writer - Initial Hash Value"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Initial Hash Value"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<Uihval1Spec> {
        ValW::new(self, 0)
    }
}
#[doc = "User Initial Hash Value 0 Register 1\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uihval1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uihval1Spec;
impl crate::RegisterSpec for Uihval1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`uihval1::W`](W) writer structure"]
impl crate::Writable for Uihval1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UIHVAL1 to value 0"]
impl crate::Resettable for Uihval1Spec {}
