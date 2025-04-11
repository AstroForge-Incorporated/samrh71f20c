#[doc = "Register `TCH_CFGLISTEN` reader"]
pub type R = crate::R<TchCfglistenSpec>;
#[doc = "Register `TCH_CFGLISTEN` writer"]
pub type W = crate::W<TchCfglistenSpec>;
#[doc = "Field `L1` reader - Listen link 1"]
pub type L1R = crate::BitReader;
#[doc = "Field `L1` writer - Listen link 1"]
pub type L1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L2` reader - Listen link 2"]
pub type L2R = crate::BitReader;
#[doc = "Field `L2` writer - Listen link 2"]
pub type L2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Listen link 1"]
    #[inline(always)]
    pub fn l1(&self) -> L1R {
        L1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Listen link 2"]
    #[inline(always)]
    pub fn l2(&self) -> L2R {
        L2R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Listen link 1"]
    #[inline(always)]
    pub fn l1(&mut self) -> L1W<TchCfglistenSpec> {
        L1W::new(self, 0)
    }
    #[doc = "Bit 1 - Listen link 2"]
    #[inline(always)]
    pub fn l2(&mut self) -> L2W<TchCfglistenSpec> {
        L2W::new(self, 1)
    }
}
#[doc = "SpW Tch Config Listener\n\nYou can [`read`](crate::Reg::read) this register and get [`tch_cfglisten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tch_cfglisten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TchCfglistenSpec;
impl crate::RegisterSpec for TchCfglistenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tch_cfglisten::R`](R) reader structure"]
impl crate::Readable for TchCfglistenSpec {}
#[doc = "`write(|w| ..)` method takes [`tch_cfglisten::W`](W) writer structure"]
impl crate::Writable for TchCfglistenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCH_CFGLISTEN to value 0"]
impl crate::Resettable for TchCfglistenSpec {}
