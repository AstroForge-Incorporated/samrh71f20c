#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `START` writer - Start Processing"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIRST` writer - First Block of a Message"]
pub type FirstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUIHV` writer - Write User Initial Hash Values"]
pub type WuihvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUIEHV` writer - Write User Initial or Expected Hash Values"]
pub type WuiehvW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Start Processing"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<CrSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bit 4 - First Block of a Message"]
    #[inline(always)]
    pub fn first(&mut self) -> FirstW<CrSpec> {
        FirstW::new(self, 4)
    }
    #[doc = "Bit 8 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SwrstW<CrSpec> {
        SwrstW::new(self, 8)
    }
    #[doc = "Bit 12 - Write User Initial Hash Values"]
    #[inline(always)]
    pub fn wuihv(&mut self) -> WuihvW<CrSpec> {
        WuihvW::new(self, 12)
    }
    #[doc = "Bit 13 - Write User Initial or Expected Hash Values"]
    #[inline(always)]
    pub fn wuiehv(&mut self) -> WuiehvW<CrSpec> {
        WuiehvW::new(self, 13)
    }
}
#[doc = "Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
