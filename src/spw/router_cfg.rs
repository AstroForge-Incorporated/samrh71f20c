#[doc = "Register `ROUTER_CFG` reader"]
pub type R = crate::R<RouterCfgSpec>;
#[doc = "Register `ROUTER_CFG` writer"]
pub type W = crate::W<RouterCfgSpec>;
#[doc = "Field `LAENA` reader - LA Routing Enable"]
pub type LaenaR = crate::BitReader;
#[doc = "Field `LAENA` writer - LA Routing Enable"]
pub type LaenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FALLBACK` reader - Fallback Routing"]
pub type FallbackR = crate::BitReader;
#[doc = "Field `FALLBACK` writer - Fallback Routing"]
pub type FallbackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISTIMEOUT` reader - Disable Timeout"]
pub type DistimeoutR = crate::BitReader;
#[doc = "Field `DISTIMEOUT` writer - Disable Timeout"]
pub type DistimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LA Routing Enable"]
    #[inline(always)]
    pub fn laena(&self) -> LaenaR {
        LaenaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fallback Routing"]
    #[inline(always)]
    pub fn fallback(&self) -> FallbackR {
        FallbackR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Disable Timeout"]
    #[inline(always)]
    pub fn distimeout(&self) -> DistimeoutR {
        DistimeoutR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LA Routing Enable"]
    #[inline(always)]
    pub fn laena(&mut self) -> LaenaW<RouterCfgSpec> {
        LaenaW::new(self, 0)
    }
    #[doc = "Bit 1 - Fallback Routing"]
    #[inline(always)]
    pub fn fallback(&mut self) -> FallbackW<RouterCfgSpec> {
        FallbackW::new(self, 1)
    }
    #[doc = "Bit 2 - Disable Timeout"]
    #[inline(always)]
    pub fn distimeout(&mut self) -> DistimeoutW<RouterCfgSpec> {
        DistimeoutW::new(self, 2)
    }
}
#[doc = "SpW Router Config\n\nYou can [`read`](crate::Reg::read) this register and get [`router_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`router_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RouterCfgSpec;
impl crate::RegisterSpec for RouterCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`router_cfg::R`](R) reader structure"]
impl crate::Readable for RouterCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`router_cfg::W`](W) writer structure"]
impl crate::Writable for RouterCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ROUTER_CFG to value 0"]
impl crate::Resettable for RouterCfgSpec {}
