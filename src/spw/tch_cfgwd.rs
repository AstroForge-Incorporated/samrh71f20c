#[doc = "Register `TCH_CFGWD` reader"]
pub type R = crate::R<TchCfgwdSpec>;
#[doc = "Register `TCH_CFGWD` writer"]
pub type W = crate::W<TchCfgwdSpec>;
#[doc = "Field `LATE` reader - Late"]
pub type LateR = crate::FieldReader<u16>;
#[doc = "Field `LATE` writer - Late"]
pub type LateW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `EARLY` reader - Early"]
pub type EarlyR = crate::FieldReader<u16>;
#[doc = "Field `EARLY` writer - Early"]
pub type EarlyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Late"]
    #[inline(always)]
    pub fn late(&self) -> LateR {
        LateR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Early"]
    #[inline(always)]
    pub fn early(&self) -> EarlyR {
        EarlyR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Late"]
    #[inline(always)]
    pub fn late(&mut self) -> LateW<TchCfgwdSpec> {
        LateW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Early"]
    #[inline(always)]
    pub fn early(&mut self) -> EarlyW<TchCfgwdSpec> {
        EarlyW::new(self, 16)
    }
}
#[doc = "SpW Tch Config Watchdog\n\nYou can [`read`](crate::Reg::read) this register and get [`tch_cfgwd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tch_cfgwd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TchCfgwdSpec;
impl crate::RegisterSpec for TchCfgwdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tch_cfgwd::R`](R) reader structure"]
impl crate::Readable for TchCfgwdSpec {}
#[doc = "`write(|w| ..)` method takes [`tch_cfgwd::W`](W) writer structure"]
impl crate::Writable for TchCfgwdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCH_CFGWD to value 0"]
impl crate::Resettable for TchCfgwdSpec {}
