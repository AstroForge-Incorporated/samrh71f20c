#[doc = "Register `RMAP1_CFG` reader"]
pub type R = crate::R<Rmap1CfgSpec>;
#[doc = "Register `RMAP1_CFG` writer"]
pub type W = crate::W<Rmap1CfgSpec>;
#[doc = "Field `DESTKEY` reader - DestKey"]
pub type DestkeyR = crate::FieldReader;
#[doc = "Field `DESTKEY` writer - DestKey"]
pub type DestkeyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TLA` reader - Address"]
pub type TlaR = crate::FieldReader;
#[doc = "Field `TLA` writer - Address"]
pub type TlaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RMAPENA` reader - RMAP Enable"]
pub type RmapenaR = crate::BitReader;
#[doc = "Field `RMAPENA` writer - RMAP Enable"]
pub type RmapenaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - DestKey"]
    #[inline(always)]
    pub fn destkey(&self) -> DestkeyR {
        DestkeyR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Address"]
    #[inline(always)]
    pub fn tla(&self) -> TlaR {
        TlaR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - RMAP Enable"]
    #[inline(always)]
    pub fn rmapena(&self) -> RmapenaR {
        RmapenaR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - DestKey"]
    #[inline(always)]
    pub fn destkey(&mut self) -> DestkeyW<Rmap1CfgSpec> {
        DestkeyW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Address"]
    #[inline(always)]
    pub fn tla(&mut self) -> TlaW<Rmap1CfgSpec> {
        TlaW::new(self, 8)
    }
    #[doc = "Bit 16 - RMAP Enable"]
    #[inline(always)]
    pub fn rmapena(&mut self) -> RmapenaW<Rmap1CfgSpec> {
        RmapenaW::new(self, 16)
    }
}
#[doc = "SpW RMAP 1 Config\n\nYou can [`read`](crate::Reg::read) this register and get [`rmap1_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmap1_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rmap1CfgSpec;
impl crate::RegisterSpec for Rmap1CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rmap1_cfg::R`](R) reader structure"]
impl crate::Readable for Rmap1CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`rmap1_cfg::W`](W) writer structure"]
impl crate::Writable for Rmap1CfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RMAP1_CFG to value 0"]
impl crate::Resettable for Rmap1CfgSpec {}
