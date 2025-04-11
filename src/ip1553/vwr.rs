#[doc = "Register `VWR` writer"]
pub type W = crate::W<VwrSpec>;
#[doc = "Field `TOVECTORWORD` writer - to vector word"]
pub type TovectorwordW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - to vector word"]
    #[inline(always)]
    pub fn tovectorword(&mut self) -> TovectorwordW<VwrSpec> {
        TovectorwordW::new(self, 0)
    }
}
#[doc = "Vector Word Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VwrSpec;
impl crate::RegisterSpec for VwrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`vwr::W`](W) writer structure"]
impl crate::Writable for VwrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VWR to value 0"]
impl crate::Resettable for VwrSpec {}
