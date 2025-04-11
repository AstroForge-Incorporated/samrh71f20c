#[doc = "Register `TCH_CFGRESTART` reader"]
pub type R = crate::R<TchCfgrestartSpec>;
#[doc = "Register `TCH_CFGRESTART` writer"]
pub type W = crate::W<TchCfgrestartSpec>;
#[doc = "Field `VALUE` reader - Value"]
pub type ValueR = crate::FieldReader;
#[doc = "Field `VALUE` writer - Value"]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `EVENT` reader - Event"]
pub type EventR = crate::FieldReader;
#[doc = "Field `EVENT` writer - Event"]
pub type EventW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PPS` reader - Pps"]
pub type PpsR = crate::BitReader;
#[doc = "Field `PPS` writer - Pps"]
pub type PpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONESHOT` reader - One Shot"]
pub type OneshotR = crate::BitReader;
#[doc = "Field `ONESHOT` writer - One Shot"]
pub type OneshotW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Value"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Event"]
    #[inline(always)]
    pub fn event(&self) -> EventR {
        EventR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - Pps"]
    #[inline(always)]
    pub fn pps(&self) -> PpsR {
        PpsR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - One Shot"]
    #[inline(always)]
    pub fn oneshot(&self) -> OneshotR {
        OneshotR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Value"]
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<TchCfgrestartSpec> {
        ValueW::new(self, 0)
    }
    #[doc = "Bits 8:13 - Event"]
    #[inline(always)]
    pub fn event(&mut self) -> EventW<TchCfgrestartSpec> {
        EventW::new(self, 8)
    }
    #[doc = "Bit 14 - Pps"]
    #[inline(always)]
    pub fn pps(&mut self) -> PpsW<TchCfgrestartSpec> {
        PpsW::new(self, 14)
    }
    #[doc = "Bit 15 - One Shot"]
    #[inline(always)]
    pub fn oneshot(&mut self) -> OneshotW<TchCfgrestartSpec> {
        OneshotW::new(self, 15)
    }
}
#[doc = "SpW Tch Config Restart\n\nYou can [`read`](crate::Reg::read) this register and get [`tch_cfgrestart::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tch_cfgrestart::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TchCfgrestartSpec;
impl crate::RegisterSpec for TchCfgrestartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tch_cfgrestart::R`](R) reader structure"]
impl crate::Readable for TchCfgrestartSpec {}
#[doc = "`write(|w| ..)` method takes [`tch_cfgrestart::W`](W) writer structure"]
impl crate::Writable for TchCfgrestartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCH_CFGRESTART to value 0"]
impl crate::Resettable for TchCfgrestartSpec {}
