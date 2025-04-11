#[doc = "Register `SCER` writer"]
pub type W = crate::W<ScerSpec>;
#[doc = "Field `PCK0` writer - Programmable Clock 0 Output Enable"]
pub type Pck0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCK1` writer - Programmable Clock 1 Output Enable"]
pub type Pck1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCK2` writer - Programmable Clock 2 Output Enable"]
pub type Pck2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCK3` writer - Programmable Clock 3 Output Enable"]
pub type Pck3W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 8 - Programmable Clock 0 Output Enable"]
    #[inline(always)]
    pub fn pck0(&mut self) -> Pck0W<ScerSpec> {
        Pck0W::new(self, 8)
    }
    #[doc = "Bit 9 - Programmable Clock 1 Output Enable"]
    #[inline(always)]
    pub fn pck1(&mut self) -> Pck1W<ScerSpec> {
        Pck1W::new(self, 9)
    }
    #[doc = "Bit 10 - Programmable Clock 2 Output Enable"]
    #[inline(always)]
    pub fn pck2(&mut self) -> Pck2W<ScerSpec> {
        Pck2W::new(self, 10)
    }
    #[doc = "Bit 11 - Programmable Clock 3 Output Enable"]
    #[inline(always)]
    pub fn pck3(&mut self) -> Pck3W<ScerSpec> {
        Pck3W::new(self, 11)
    }
}
#[doc = "System Clock Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scer::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScerSpec;
impl crate::RegisterSpec for ScerSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`scer::W`](W) writer structure"]
impl crate::Writable for ScerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCER to value 0"]
impl crate::Resettable for ScerSpec {}
