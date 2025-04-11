#[doc = "Register `CKGR_PLLBR` reader"]
pub type R = crate::R<CkgrPllbrSpec>;
#[doc = "Register `CKGR_PLLBR` writer"]
pub type W = crate::W<CkgrPllbrSpec>;
#[doc = "PLLB Front End Divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Divbselect {
    #[doc = "0: PLLBis disabled."]
    _0 = 0,
    #[doc = "1: Divider is bypassed (divide by 1) and PLLB is enabled."]
    Bypass = 1,
}
impl From<Divbselect> for u8 {
    #[inline(always)]
    fn from(variant: Divbselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Divbselect {
    type Ux = u8;
}
impl crate::IsEnum for Divbselect {}
#[doc = "Field `DIVB` reader - PLLB Front End Divider"]
pub type DivbR = crate::FieldReader<Divbselect>;
impl DivbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Divbselect> {
        match self.bits {
            0 => Some(Divbselect::_0),
            1 => Some(Divbselect::Bypass),
            _ => None,
        }
    }
    #[doc = "PLLBis disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Divbselect::_0
    }
    #[doc = "Divider is bypassed (divide by 1) and PLLB is enabled."]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == Divbselect::Bypass
    }
}
#[doc = "Field `DIVB` writer - PLLB Front End Divider"]
pub type DivbW<'a, REG> = crate::FieldWriter<'a, REG, 8, Divbselect>;
impl<'a, REG> DivbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PLLBis disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Divbselect::_0)
    }
    #[doc = "Divider is bypassed (divide by 1) and PLLB is enabled."]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut crate::W<REG> {
        self.variant(Divbselect::Bypass)
    }
}
#[doc = "Field `PLLBCOUNT` reader - PLLB Counter"]
pub type PllbcountR = crate::FieldReader;
#[doc = "Field `PLLBCOUNT` writer - PLLB Counter"]
pub type PllbcountW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "VCO Frequency Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FreqVcoselect {
    #[doc = "0: Frequency range: 40-80 MHz"]
    Vco0 = 0,
    #[doc = "1: Frequency range: 70-150 MHz"]
    Vco1 = 1,
    #[doc = "2: Frequency range: 125-275 MHz"]
    Vco2 = 2,
    #[doc = "3: Frequency range: 250-450 MHz"]
    Vco3 = 3,
}
impl From<FreqVcoselect> for u8 {
    #[inline(always)]
    fn from(variant: FreqVcoselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FreqVcoselect {
    type Ux = u8;
}
impl crate::IsEnum for FreqVcoselect {}
#[doc = "Field `FREQ_VCO` reader - VCO Frequency Configuration"]
pub type FreqVcoR = crate::FieldReader<FreqVcoselect>;
impl FreqVcoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FreqVcoselect {
        match self.bits {
            0 => FreqVcoselect::Vco0,
            1 => FreqVcoselect::Vco1,
            2 => FreqVcoselect::Vco2,
            3 => FreqVcoselect::Vco3,
            _ => unreachable!(),
        }
    }
    #[doc = "Frequency range: 40-80 MHz"]
    #[inline(always)]
    pub fn is_vco0(&self) -> bool {
        *self == FreqVcoselect::Vco0
    }
    #[doc = "Frequency range: 70-150 MHz"]
    #[inline(always)]
    pub fn is_vco1(&self) -> bool {
        *self == FreqVcoselect::Vco1
    }
    #[doc = "Frequency range: 125-275 MHz"]
    #[inline(always)]
    pub fn is_vco2(&self) -> bool {
        *self == FreqVcoselect::Vco2
    }
    #[doc = "Frequency range: 250-450 MHz"]
    #[inline(always)]
    pub fn is_vco3(&self) -> bool {
        *self == FreqVcoselect::Vco3
    }
}
#[doc = "Field `FREQ_VCO` writer - VCO Frequency Configuration"]
pub type FreqVcoW<'a, REG> = crate::FieldWriter<'a, REG, 2, FreqVcoselect, crate::Safe>;
impl<'a, REG> FreqVcoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Frequency range: 40-80 MHz"]
    #[inline(always)]
    pub fn vco0(self) -> &'a mut crate::W<REG> {
        self.variant(FreqVcoselect::Vco0)
    }
    #[doc = "Frequency range: 70-150 MHz"]
    #[inline(always)]
    pub fn vco1(self) -> &'a mut crate::W<REG> {
        self.variant(FreqVcoselect::Vco1)
    }
    #[doc = "Frequency range: 125-275 MHz"]
    #[inline(always)]
    pub fn vco2(self) -> &'a mut crate::W<REG> {
        self.variant(FreqVcoselect::Vco2)
    }
    #[doc = "Frequency range: 250-450 MHz"]
    #[inline(always)]
    pub fn vco3(self) -> &'a mut crate::W<REG> {
        self.variant(FreqVcoselect::Vco3)
    }
}
#[doc = "Field `MULB` reader - PLLB Multiplier"]
pub type MulbR = crate::FieldReader<u16>;
#[doc = "Field `MULB` writer - PLLB Multiplier"]
pub type MulbW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "PLLB Source Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Srcbselect {
    #[doc = "0: MAINCK is the source clock of PLLB."]
    Mainck = 0,
    #[doc = "2: RC2CK is the source clock of PLLB."]
    Rc2ck = 2,
}
impl From<Srcbselect> for u8 {
    #[inline(always)]
    fn from(variant: Srcbselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Srcbselect {
    type Ux = u8;
}
impl crate::IsEnum for Srcbselect {}
#[doc = "Field `SRCB` reader - PLLB Source Clock Selection"]
pub type SrcbR = crate::FieldReader<Srcbselect>;
impl SrcbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Srcbselect> {
        match self.bits {
            0 => Some(Srcbselect::Mainck),
            2 => Some(Srcbselect::Rc2ck),
            _ => None,
        }
    }
    #[doc = "MAINCK is the source clock of PLLB."]
    #[inline(always)]
    pub fn is_mainck(&self) -> bool {
        *self == Srcbselect::Mainck
    }
    #[doc = "RC2CK is the source clock of PLLB."]
    #[inline(always)]
    pub fn is_rc2ck(&self) -> bool {
        *self == Srcbselect::Rc2ck
    }
}
#[doc = "Field `SRCB` writer - PLLB Source Clock Selection"]
pub type SrcbW<'a, REG> = crate::FieldWriter<'a, REG, 2, Srcbselect>;
impl<'a, REG> SrcbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MAINCK is the source clock of PLLB."]
    #[inline(always)]
    pub fn mainck(self) -> &'a mut crate::W<REG> {
        self.variant(Srcbselect::Mainck)
    }
    #[doc = "RC2CK is the source clock of PLLB."]
    #[inline(always)]
    pub fn rc2ck(self) -> &'a mut crate::W<REG> {
        self.variant(Srcbselect::Rc2ck)
    }
}
impl R {
    #[doc = "Bits 0:7 - PLLB Front End Divider"]
    #[inline(always)]
    pub fn divb(&self) -> DivbR {
        DivbR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:13 - PLLB Counter"]
    #[inline(always)]
    pub fn pllbcount(&self) -> PllbcountR {
        PllbcountR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - VCO Frequency Configuration"]
    #[inline(always)]
    pub fn freq_vco(&self) -> FreqVcoR {
        FreqVcoR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:26 - PLLB Multiplier"]
    #[inline(always)]
    pub fn mulb(&self) -> MulbR {
        MulbR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bits 29:30 - PLLB Source Clock Selection"]
    #[inline(always)]
    pub fn srcb(&self) -> SrcbR {
        SrcbR::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PLLB Front End Divider"]
    #[inline(always)]
    pub fn divb(&mut self) -> DivbW<CkgrPllbrSpec> {
        DivbW::new(self, 0)
    }
    #[doc = "Bits 8:13 - PLLB Counter"]
    #[inline(always)]
    pub fn pllbcount(&mut self) -> PllbcountW<CkgrPllbrSpec> {
        PllbcountW::new(self, 8)
    }
    #[doc = "Bits 14:15 - VCO Frequency Configuration"]
    #[inline(always)]
    pub fn freq_vco(&mut self) -> FreqVcoW<CkgrPllbrSpec> {
        FreqVcoW::new(self, 14)
    }
    #[doc = "Bits 16:26 - PLLB Multiplier"]
    #[inline(always)]
    pub fn mulb(&mut self) -> MulbW<CkgrPllbrSpec> {
        MulbW::new(self, 16)
    }
    #[doc = "Bits 29:30 - PLLB Source Clock Selection"]
    #[inline(always)]
    pub fn srcb(&mut self) -> SrcbW<CkgrPllbrSpec> {
        SrcbW::new(self, 29)
    }
}
#[doc = "PLLB Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ckgr_pllbr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckgr_pllbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CkgrPllbrSpec;
impl crate::RegisterSpec for CkgrPllbrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ckgr_pllbr::R`](R) reader structure"]
impl crate::Readable for CkgrPllbrSpec {}
#[doc = "`write(|w| ..)` method takes [`ckgr_pllbr::W`](W) writer structure"]
impl crate::Writable for CkgrPllbrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CKGR_PLLBR to value 0"]
impl crate::Resettable for CkgrPllbrSpec {}
