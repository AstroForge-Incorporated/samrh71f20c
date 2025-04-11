#[doc = "Register `PKTRX1_CFG` reader"]
pub type R = crate::R<Pktrx1CfgSpec>;
#[doc = "Register `PKTRX1_CFG` writer"]
pub type W = crate::W<Pktrx1CfgSpec>;
#[doc = "Field `DISCARD` reader - Discard"]
pub type DiscardR = crate::BitReader;
#[doc = "Field `DISCARD` writer - Discard"]
pub type DiscardW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Discard"]
    #[inline(always)]
    pub fn discard(&self) -> DiscardR {
        DiscardR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Discard"]
    #[inline(always)]
    pub fn discard(&mut self) -> DiscardW<Pktrx1CfgSpec> {
        DiscardW::new(self, 0)
    }
}
#[doc = "PktRx Config\n\nYou can [`read`](crate::Reg::read) this register and get [`pktrx1_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pktrx1_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pktrx1CfgSpec;
impl crate::RegisterSpec for Pktrx1CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pktrx1_cfg::R`](R) reader structure"]
impl crate::Readable for Pktrx1CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`pktrx1_cfg::W`](W) writer structure"]
impl crate::Writable for Pktrx1CfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PKTRX1_CFG to value 0"]
impl crate::Resettable for Pktrx1CfgSpec {}
