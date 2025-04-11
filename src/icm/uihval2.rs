#[doc = "Register `UIHVAL2` writer"]
pub type W = crate::W<Uihval2Spec>;
#[doc = "Field `VAL` writer - Initial Hash Value"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Initial Hash Value"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<Uihval2Spec> {
        ValW::new(self, 0)
    }
}
#[doc = "User Initial Hash Value 0 Register 2\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uihval2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uihval2Spec;
impl crate::RegisterSpec for Uihval2Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`uihval2::W`](W) writer structure"]
impl crate::Writable for Uihval2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UIHVAL2 to value 0"]
impl crate::Resettable for Uihval2Spec {}
