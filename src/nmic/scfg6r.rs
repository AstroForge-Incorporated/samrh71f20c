#[doc = "Register `SCFG6R` reader"]
pub type R = crate::R<Scfg6rSpec>;
#[doc = "Register `SCFG6R` writer"]
pub type W = crate::W<Scfg6rSpec>;
#[doc = "Field `GFSEL` reader - Glitch Filter Selector (GFSEL field is read-only in NMIC_SCFG1 to 8)"]
pub type GfselR = crate::FieldReader;
#[doc = "Field `GFSEL` writer - Glitch Filter Selector (GFSEL field is read-only in NMIC_SCFG1 to 8)"]
pub type GfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GFEN` reader - Glitch Filter Enable (GFEN bit is read-only in NMIC_SCFG1 to 8)"]
pub type GfenR = crate::BitReader;
#[doc = "Field `GFEN` writer - Glitch Filter Enable (GFEN bit is read-only in NMIC_SCFG1 to 8)"]
pub type GfenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POL` reader - Polarity (POL bit is read-only in NMIC_SCFG1 to 8)"]
pub type PolR = crate::BitReader;
#[doc = "Field `POL` writer - Polarity (POL bit is read-only in NMIC_SCFG1 to 8)"]
pub type PolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LVL` reader - Level Detection (LVL bit is read-only in NMIC_SCFG1 to 8)"]
pub type LvlR = crate::BitReader;
#[doc = "Field `LVL` writer - Level Detection (LVL bit is read-only in NMIC_SCFG1 to 8)"]
pub type LvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN` reader - Source Enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Source Enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRZ` reader - Interrupt Line Freeze"]
pub type FrzR = crate::BitReader;
#[doc = "Field `FRZ` writer - Interrupt Line Freeze"]
pub type FrzW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Glitch Filter Selector (GFSEL field is read-only in NMIC_SCFG1 to 8)"]
    #[inline(always)]
    pub fn gfsel(&self) -> GfselR {
        GfselR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - Glitch Filter Enable (GFEN bit is read-only in NMIC_SCFG1 to 8)"]
    #[inline(always)]
    pub fn gfen(&self) -> GfenR {
        GfenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Polarity (POL bit is read-only in NMIC_SCFG1 to 8)"]
    #[inline(always)]
    pub fn pol(&self) -> PolR {
        PolR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Level Detection (LVL bit is read-only in NMIC_SCFG1 to 8)"]
    #[inline(always)]
    pub fn lvl(&self) -> LvlR {
        LvlR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Source Enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt Line Freeze"]
    #[inline(always)]
    pub fn frz(&self) -> FrzR {
        FrzR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Glitch Filter Selector (GFSEL field is read-only in NMIC_SCFG1 to 8)"]
    #[inline(always)]
    pub fn gfsel(&mut self) -> GfselW<Scfg6rSpec> {
        GfselW::new(self, 0)
    }
    #[doc = "Bit 4 - Glitch Filter Enable (GFEN bit is read-only in NMIC_SCFG1 to 8)"]
    #[inline(always)]
    pub fn gfen(&mut self) -> GfenW<Scfg6rSpec> {
        GfenW::new(self, 4)
    }
    #[doc = "Bit 8 - Polarity (POL bit is read-only in NMIC_SCFG1 to 8)"]
    #[inline(always)]
    pub fn pol(&mut self) -> PolW<Scfg6rSpec> {
        PolW::new(self, 8)
    }
    #[doc = "Bit 9 - Level Detection (LVL bit is read-only in NMIC_SCFG1 to 8)"]
    #[inline(always)]
    pub fn lvl(&mut self) -> LvlW<Scfg6rSpec> {
        LvlW::new(self, 9)
    }
    #[doc = "Bit 16 - Source Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<Scfg6rSpec> {
        EnW::new(self, 16)
    }
    #[doc = "Bit 31 - Interrupt Line Freeze"]
    #[inline(always)]
    pub fn frz(&mut self) -> FrzW<Scfg6rSpec> {
        FrzW::new(self, 31)
    }
}
#[doc = "Source Configuration Register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`scfg6r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scfg6r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scfg6rSpec;
impl crate::RegisterSpec for Scfg6rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scfg6r::R`](R) reader structure"]
impl crate::Readable for Scfg6rSpec {}
#[doc = "`write(|w| ..)` method takes [`scfg6r::W`](W) writer structure"]
impl crate::Writable for Scfg6rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCFG6R to value 0"]
impl crate::Resettable for Scfg6rSpec {}
