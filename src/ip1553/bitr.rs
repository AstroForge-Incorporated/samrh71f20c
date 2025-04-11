#[doc = "Register `BITR` writer"]
pub type W = crate::W<BitrSpec>;
#[doc = "Field `TOBITWORD` writer - to bit word"]
pub type TobitwordW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - to bit word"]
    #[inline(always)]
    pub fn tobitword(&mut self) -> TobitwordW<BitrSpec> {
        TobitwordW::new(self, 0)
    }
}
#[doc = "BIT Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bitr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BitrSpec;
impl crate::RegisterSpec for BitrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`bitr::W`](W) writer structure"]
impl crate::Writable for BitrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BITR to value 0"]
impl crate::Resettable for BitrSpec {}
