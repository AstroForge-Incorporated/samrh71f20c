#[doc = "Register `TCH_CFGSEND` reader"]
pub type R = crate::R<TchCfgsendSpec>;
#[doc = "Register `TCH_CFGSEND` writer"]
pub type W = crate::W<TchCfgsendSpec>;
#[doc = "Field `S1` reader - Send link 1"]
pub type S1R = crate::BitReader;
#[doc = "Field `S1` writer - Send link 1"]
pub type S1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S2` reader - Send link 2"]
pub type S2R = crate::BitReader;
#[doc = "Field `S2` writer - Send link 2"]
pub type S2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Send link 1"]
    #[inline(always)]
    pub fn s1(&self) -> S1R {
        S1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Send link 2"]
    #[inline(always)]
    pub fn s2(&self) -> S2R {
        S2R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Send link 1"]
    #[inline(always)]
    pub fn s1(&mut self) -> S1W<TchCfgsendSpec> {
        S1W::new(self, 0)
    }
    #[doc = "Bit 1 - Send link 2"]
    #[inline(always)]
    pub fn s2(&mut self) -> S2W<TchCfgsendSpec> {
        S2W::new(self, 1)
    }
}
#[doc = "SpW Tch Config Sender\n\nYou can [`read`](crate::Reg::read) this register and get [`tch_cfgsend::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tch_cfgsend::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TchCfgsendSpec;
impl crate::RegisterSpec for TchCfgsendSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tch_cfgsend::R`](R) reader structure"]
impl crate::Readable for TchCfgsendSpec {}
#[doc = "`write(|w| ..)` method takes [`tch_cfgsend::W`](W) writer structure"]
impl crate::Writable for TchCfgsendSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCH_CFGSEND to value 0"]
impl crate::Resettable for TchCfgsendSpec {}
