#[doc = "Register `CKGR_PLLAR` reader"]
pub type R = crate::R<CkgrPllarSpec>;
#[doc = "Register `CKGR_PLLAR` writer"]
pub type W = crate::W<CkgrPllarSpec>;
#[doc = "PLLA Front End Divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Divaselect {
    #[doc = "0: PLLA is disabled."]
    _0 = 0,
    #[doc = "1: Divider is bypassed (divide by 1) and PLLA is enabled."]
    Bypass = 1,
}
impl From<Divaselect> for u8 {
    #[inline(always)]
    fn from(variant: Divaselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Divaselect {
    type Ux = u8;
}
impl crate::IsEnum for Divaselect {}
#[doc = "Field `DIVA` reader - PLLA Front End Divider"]
pub type DivaR = crate::FieldReader<Divaselect>;
impl DivaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Divaselect> {
        match self.bits {
            0 => Some(Divaselect::_0),
            1 => Some(Divaselect::Bypass),
            _ => None,
        }
    }
    #[doc = "PLLA is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == Divaselect::_0
    }
    #[doc = "Divider is bypassed (divide by 1) and PLLA is enabled."]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == Divaselect::Bypass
    }
}
#[doc = "Field `DIVA` writer - PLLA Front End Divider"]
pub type DivaW<'a, REG> = crate::FieldWriter<'a, REG, 8, Divaselect>;
impl<'a, REG> DivaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PLLA is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(Divaselect::_0)
    }
    #[doc = "Divider is bypassed (divide by 1) and PLLA is enabled."]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut crate::W<REG> {
        self.variant(Divaselect::Bypass)
    }
}
#[doc = "Field `PLLACOUNT` reader - PLLA Counter"]
pub type PllacountR = crate::FieldReader;
#[doc = "Field `PLLACOUNT` writer - PLLA Counter"]
pub type PllacountW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "VCO Frequency Configuratio\n\nValue on reset: 0"]
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
#[doc = "Field `FREQ_VCO` reader - VCO Frequency Configuratio"]
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
#[doc = "Field `FREQ_VCO` writer - VCO Frequency Configuratio"]
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
#[doc = "Field `MULA` reader - PLLA Multiplier"]
pub type MulaR = crate::FieldReader<u16>;
#[doc = "Field `MULA` writer - PLLA Multiplier"]
pub type MulaW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `ONE` reader - Must Be Set to 1"]
pub type OneR = crate::BitReader;
#[doc = "Field `ONE` writer - Must Be Set to 1"]
pub type OneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PLLA Front End Divider"]
    #[inline(always)]
    pub fn diva(&self) -> DivaR {
        DivaR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:13 - PLLA Counter"]
    #[inline(always)]
    pub fn pllacount(&self) -> PllacountR {
        PllacountR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - VCO Frequency Configuratio"]
    #[inline(always)]
    pub fn freq_vco(&self) -> FreqVcoR {
        FreqVcoR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:26 - PLLA Multiplier"]
    #[inline(always)]
    pub fn mula(&self) -> MulaR {
        MulaR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bit 29 - Must Be Set to 1"]
    #[inline(always)]
    pub fn one(&self) -> OneR {
        OneR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PLLA Front End Divider"]
    #[inline(always)]
    pub fn diva(&mut self) -> DivaW<CkgrPllarSpec> {
        DivaW::new(self, 0)
    }
    #[doc = "Bits 8:13 - PLLA Counter"]
    #[inline(always)]
    pub fn pllacount(&mut self) -> PllacountW<CkgrPllarSpec> {
        PllacountW::new(self, 8)
    }
    #[doc = "Bits 14:15 - VCO Frequency Configuratio"]
    #[inline(always)]
    pub fn freq_vco(&mut self) -> FreqVcoW<CkgrPllarSpec> {
        FreqVcoW::new(self, 14)
    }
    #[doc = "Bits 16:26 - PLLA Multiplier"]
    #[inline(always)]
    pub fn mula(&mut self) -> MulaW<CkgrPllarSpec> {
        MulaW::new(self, 16)
    }
    #[doc = "Bit 29 - Must Be Set to 1"]
    #[inline(always)]
    pub fn one(&mut self) -> OneW<CkgrPllarSpec> {
        OneW::new(self, 29)
    }
}
#[doc = "PLLA Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ckgr_pllar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckgr_pllar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CkgrPllarSpec;
impl crate::RegisterSpec for CkgrPllarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ckgr_pllar::R`](R) reader structure"]
impl crate::Readable for CkgrPllarSpec {}
#[doc = "`write(|w| ..)` method takes [`ckgr_pllar::W`](W) writer structure"]
impl crate::Writable for CkgrPllarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CKGR_PLLAR to value 0"]
impl crate::Resettable for CkgrPllarSpec {}
