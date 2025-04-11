#[doc = "Register `UIHVAL3` writer"]
pub type W = crate::W<Uihval3Spec>;
#[doc = "Field `VAL` writer - Initial Hash Value"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Initial Hash Value"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<Uihval3Spec> {
        ValW::new(self, 0)
    }
}
#[doc = "User Initial Hash Value 0 Register 3\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uihval3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uihval3Spec;
impl crate::RegisterSpec for Uihval3Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`uihval3::W`](W) writer structure"]
impl crate::Writable for Uihval3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UIHVAL3 to value 0"]
impl crate::Resettable for Uihval3Spec {}
