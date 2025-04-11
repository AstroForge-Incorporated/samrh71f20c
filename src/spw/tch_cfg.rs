#[doc = "Register `TCH_CFG` reader"]
pub type R = crate::R<TchCfgSpec>;
#[doc = "Register `TCH_CFG` writer"]
pub type W = crate::W<TchCfgSpec>;
#[doc = "Field `EVENT` reader - Event"]
pub type EventR = crate::FieldReader;
#[doc = "Field `EVENT` writer - Event"]
pub type EventW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Event"]
    #[inline(always)]
    pub fn event(&self) -> EventR {
        EventR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Event"]
    #[inline(always)]
    pub fn event(&mut self) -> EventW<TchCfgSpec> {
        EventW::new(self, 0)
    }
}
#[doc = "SpW Tch Config\n\nYou can [`read`](crate::Reg::read) this register and get [`tch_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tch_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TchCfgSpec;
impl crate::RegisterSpec for TchCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tch_cfg::R`](R) reader structure"]
impl crate::Readable for TchCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`tch_cfg::W`](W) writer structure"]
impl crate::Writable for TchCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCH_CFG to value 0"]
impl crate::Resettable for TchCfgSpec {}
