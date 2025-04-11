#[doc = "Register `PLL_CFG` reader"]
pub type R = crate::R<PllCfgSpec>;
#[doc = "Register `PLL_CFG` writer"]
pub type W = crate::W<PllCfgSpec>;
#[doc = "PLLA Output Current\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OutcurPllaselect {
    #[doc = "0: 0.5 mA"]
    Icp0 = 0,
    #[doc = "1: 0.75 mA"]
    Icp1 = 1,
    #[doc = "2: 1 mA"]
    Icp2 = 2,
    #[doc = "3: 1.25 mA"]
    Icp3 = 3,
}
impl From<OutcurPllaselect> for u8 {
    #[inline(always)]
    fn from(variant: OutcurPllaselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OutcurPllaselect {
    type Ux = u8;
}
impl crate::IsEnum for OutcurPllaselect {}
#[doc = "Field `OUTCUR_PLLA` reader - PLLA Output Current"]
pub type OutcurPllaR = crate::FieldReader<OutcurPllaselect>;
impl OutcurPllaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<OutcurPllaselect> {
        match self.bits {
            0 => Some(OutcurPllaselect::Icp0),
            1 => Some(OutcurPllaselect::Icp1),
            2 => Some(OutcurPllaselect::Icp2),
            3 => Some(OutcurPllaselect::Icp3),
            _ => None,
        }
    }
    #[doc = "0.5 mA"]
    #[inline(always)]
    pub fn is_icp0(&self) -> bool {
        *self == OutcurPllaselect::Icp0
    }
    #[doc = "0.75 mA"]
    #[inline(always)]
    pub fn is_icp1(&self) -> bool {
        *self == OutcurPllaselect::Icp1
    }
    #[doc = "1 mA"]
    #[inline(always)]
    pub fn is_icp2(&self) -> bool {
        *self == OutcurPllaselect::Icp2
    }
    #[doc = "1.25 mA"]
    #[inline(always)]
    pub fn is_icp3(&self) -> bool {
        *self == OutcurPllaselect::Icp3
    }
}
#[doc = "Field `OUTCUR_PLLA` writer - PLLA Output Current"]
pub type OutcurPllaW<'a, REG> = crate::FieldWriter<'a, REG, 4, OutcurPllaselect>;
impl<'a, REG> OutcurPllaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0.5 mA"]
    #[inline(always)]
    pub fn icp0(self) -> &'a mut crate::W<REG> {
        self.variant(OutcurPllaselect::Icp0)
    }
    #[doc = "0.75 mA"]
    #[inline(always)]
    pub fn icp1(self) -> &'a mut crate::W<REG> {
        self.variant(OutcurPllaselect::Icp1)
    }
    #[doc = "1 mA"]
    #[inline(always)]
    pub fn icp2(self) -> &'a mut crate::W<REG> {
        self.variant(OutcurPllaselect::Icp2)
    }
    #[doc = "1.25 mA"]
    #[inline(always)]
    pub fn icp3(self) -> &'a mut crate::W<REG> {
        self.variant(OutcurPllaselect::Icp3)
    }
}
#[doc = "Internal Filter PLL - Select Internal Capaticance Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Scaselect {
    #[doc = "0: 20 pF"]
    ScVal20p = 0,
    #[doc = "1: 40 pF"]
    ScVal40p = 1,
    #[doc = "2: 30 pF"]
    ScVal30p = 2,
    #[doc = "3: 60 pF"]
    ScVal60p = 3,
}
impl From<Scaselect> for u8 {
    #[inline(always)]
    fn from(variant: Scaselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Scaselect {
    type Ux = u8;
}
impl crate::IsEnum for Scaselect {}
#[doc = "Field `SCA` reader - Internal Filter PLL - Select Internal Capaticance Value"]
pub type ScaR = crate::FieldReader<Scaselect>;
impl ScaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Scaselect {
        match self.bits {
            0 => Scaselect::ScVal20p,
            1 => Scaselect::ScVal40p,
            2 => Scaselect::ScVal30p,
            3 => Scaselect::ScVal60p,
            _ => unreachable!(),
        }
    }
    #[doc = "20 pF"]
    #[inline(always)]
    pub fn is_sc_val_20p(&self) -> bool {
        *self == Scaselect::ScVal20p
    }
    #[doc = "40 pF"]
    #[inline(always)]
    pub fn is_sc_val_40p(&self) -> bool {
        *self == Scaselect::ScVal40p
    }
    #[doc = "30 pF"]
    #[inline(always)]
    pub fn is_sc_val_30p(&self) -> bool {
        *self == Scaselect::ScVal30p
    }
    #[doc = "60 pF"]
    #[inline(always)]
    pub fn is_sc_val_60p(&self) -> bool {
        *self == Scaselect::ScVal60p
    }
}
#[doc = "Field `SCA` writer - Internal Filter PLL - Select Internal Capaticance Value"]
pub type ScaW<'a, REG> = crate::FieldWriter<'a, REG, 2, Scaselect, crate::Safe>;
impl<'a, REG> ScaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "20 pF"]
    #[inline(always)]
    pub fn sc_val_20p(self) -> &'a mut crate::W<REG> {
        self.variant(Scaselect::ScVal20p)
    }
    #[doc = "40 pF"]
    #[inline(always)]
    pub fn sc_val_40p(self) -> &'a mut crate::W<REG> {
        self.variant(Scaselect::ScVal40p)
    }
    #[doc = "30 pF"]
    #[inline(always)]
    pub fn sc_val_30p(self) -> &'a mut crate::W<REG> {
        self.variant(Scaselect::ScVal30p)
    }
    #[doc = "60 pF"]
    #[inline(always)]
    pub fn sc_val_60p(self) -> &'a mut crate::W<REG> {
        self.variant(Scaselect::ScVal60p)
    }
}
#[doc = "Internal Filter PLL - Select Internal Resistor Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sraselect {
    #[doc = "0: 24 Ohms"]
    SrVal24k = 0,
    #[doc = "1: 6 Ohms"]
    SrVal6k = 1,
    #[doc = "2: 3 Ohms"]
    SrVal3k = 2,
    #[doc = "3: 12 Ohms"]
    SrVal12k = 3,
}
impl From<Sraselect> for u8 {
    #[inline(always)]
    fn from(variant: Sraselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sraselect {
    type Ux = u8;
}
impl crate::IsEnum for Sraselect {}
#[doc = "Field `SRA` reader - Internal Filter PLL - Select Internal Resistor Value"]
pub type SraR = crate::FieldReader<Sraselect>;
impl SraR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sraselect {
        match self.bits {
            0 => Sraselect::SrVal24k,
            1 => Sraselect::SrVal6k,
            2 => Sraselect::SrVal3k,
            3 => Sraselect::SrVal12k,
            _ => unreachable!(),
        }
    }
    #[doc = "24 Ohms"]
    #[inline(always)]
    pub fn is_sr_val_24k(&self) -> bool {
        *self == Sraselect::SrVal24k
    }
    #[doc = "6 Ohms"]
    #[inline(always)]
    pub fn is_sr_val_6k(&self) -> bool {
        *self == Sraselect::SrVal6k
    }
    #[doc = "3 Ohms"]
    #[inline(always)]
    pub fn is_sr_val_3k(&self) -> bool {
        *self == Sraselect::SrVal3k
    }
    #[doc = "12 Ohms"]
    #[inline(always)]
    pub fn is_sr_val_12k(&self) -> bool {
        *self == Sraselect::SrVal12k
    }
}
#[doc = "Field `SRA` writer - Internal Filter PLL - Select Internal Resistor Value"]
pub type SraW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sraselect, crate::Safe>;
impl<'a, REG> SraW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "24 Ohms"]
    #[inline(always)]
    pub fn sr_val_24k(self) -> &'a mut crate::W<REG> {
        self.variant(Sraselect::SrVal24k)
    }
    #[doc = "6 Ohms"]
    #[inline(always)]
    pub fn sr_val_6k(self) -> &'a mut crate::W<REG> {
        self.variant(Sraselect::SrVal6k)
    }
    #[doc = "3 Ohms"]
    #[inline(always)]
    pub fn sr_val_3k(self) -> &'a mut crate::W<REG> {
        self.variant(Sraselect::SrVal3k)
    }
    #[doc = "12 Ohms"]
    #[inline(always)]
    pub fn sr_val_12k(self) -> &'a mut crate::W<REG> {
        self.variant(Sraselect::SrVal12k)
    }
}
#[doc = "PLLB Output Current\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OutcurPllbselect {
    #[doc = "0: 0.5 mA"]
    Icp0 = 0,
    #[doc = "1: 0.75 mA"]
    Icp1 = 1,
    #[doc = "2: 1 mA"]
    Icp2 = 2,
    #[doc = "3: 1.25 mA"]
    Icp3 = 3,
}
impl From<OutcurPllbselect> for u8 {
    #[inline(always)]
    fn from(variant: OutcurPllbselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OutcurPllbselect {
    type Ux = u8;
}
impl crate::IsEnum for OutcurPllbselect {}
#[doc = "Field `OUTCUR_PLLB` reader - PLLB Output Current"]
pub type OutcurPllbR = crate::FieldReader<OutcurPllbselect>;
impl OutcurPllbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<OutcurPllbselect> {
        match self.bits {
            0 => Some(OutcurPllbselect::Icp0),
            1 => Some(OutcurPllbselect::Icp1),
            2 => Some(OutcurPllbselect::Icp2),
            3 => Some(OutcurPllbselect::Icp3),
            _ => None,
        }
    }
    #[doc = "0.5 mA"]
    #[inline(always)]
    pub fn is_icp0(&self) -> bool {
        *self == OutcurPllbselect::Icp0
    }
    #[doc = "0.75 mA"]
    #[inline(always)]
    pub fn is_icp1(&self) -> bool {
        *self == OutcurPllbselect::Icp1
    }
    #[doc = "1 mA"]
    #[inline(always)]
    pub fn is_icp2(&self) -> bool {
        *self == OutcurPllbselect::Icp2
    }
    #[doc = "1.25 mA"]
    #[inline(always)]
    pub fn is_icp3(&self) -> bool {
        *self == OutcurPllbselect::Icp3
    }
}
#[doc = "Field `OUTCUR_PLLB` writer - PLLB Output Current"]
pub type OutcurPllbW<'a, REG> = crate::FieldWriter<'a, REG, 4, OutcurPllbselect>;
impl<'a, REG> OutcurPllbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0.5 mA"]
    #[inline(always)]
    pub fn icp0(self) -> &'a mut crate::W<REG> {
        self.variant(OutcurPllbselect::Icp0)
    }
    #[doc = "0.75 mA"]
    #[inline(always)]
    pub fn icp1(self) -> &'a mut crate::W<REG> {
        self.variant(OutcurPllbselect::Icp1)
    }
    #[doc = "1 mA"]
    #[inline(always)]
    pub fn icp2(self) -> &'a mut crate::W<REG> {
        self.variant(OutcurPllbselect::Icp2)
    }
    #[doc = "1.25 mA"]
    #[inline(always)]
    pub fn icp3(self) -> &'a mut crate::W<REG> {
        self.variant(OutcurPllbselect::Icp3)
    }
}
#[doc = "Internal Filter PLL - Select Internal Capaticance Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Scbselect {
    #[doc = "0: 20 pF"]
    ScVal20p = 0,
    #[doc = "1: 40 pF"]
    ScVal40p = 1,
    #[doc = "2: 30 pF"]
    ScVal30p = 2,
    #[doc = "3: 60 pF"]
    ScVal60p = 3,
}
impl From<Scbselect> for u8 {
    #[inline(always)]
    fn from(variant: Scbselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Scbselect {
    type Ux = u8;
}
impl crate::IsEnum for Scbselect {}
#[doc = "Field `SCB` reader - Internal Filter PLL - Select Internal Capaticance Value"]
pub type ScbR = crate::FieldReader<Scbselect>;
impl ScbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Scbselect {
        match self.bits {
            0 => Scbselect::ScVal20p,
            1 => Scbselect::ScVal40p,
            2 => Scbselect::ScVal30p,
            3 => Scbselect::ScVal60p,
            _ => unreachable!(),
        }
    }
    #[doc = "20 pF"]
    #[inline(always)]
    pub fn is_sc_val_20p(&self) -> bool {
        *self == Scbselect::ScVal20p
    }
    #[doc = "40 pF"]
    #[inline(always)]
    pub fn is_sc_val_40p(&self) -> bool {
        *self == Scbselect::ScVal40p
    }
    #[doc = "30 pF"]
    #[inline(always)]
    pub fn is_sc_val_30p(&self) -> bool {
        *self == Scbselect::ScVal30p
    }
    #[doc = "60 pF"]
    #[inline(always)]
    pub fn is_sc_val_60p(&self) -> bool {
        *self == Scbselect::ScVal60p
    }
}
#[doc = "Field `SCB` writer - Internal Filter PLL - Select Internal Capaticance Value"]
pub type ScbW<'a, REG> = crate::FieldWriter<'a, REG, 2, Scbselect, crate::Safe>;
impl<'a, REG> ScbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "20 pF"]
    #[inline(always)]
    pub fn sc_val_20p(self) -> &'a mut crate::W<REG> {
        self.variant(Scbselect::ScVal20p)
    }
    #[doc = "40 pF"]
    #[inline(always)]
    pub fn sc_val_40p(self) -> &'a mut crate::W<REG> {
        self.variant(Scbselect::ScVal40p)
    }
    #[doc = "30 pF"]
    #[inline(always)]
    pub fn sc_val_30p(self) -> &'a mut crate::W<REG> {
        self.variant(Scbselect::ScVal30p)
    }
    #[doc = "60 pF"]
    #[inline(always)]
    pub fn sc_val_60p(self) -> &'a mut crate::W<REG> {
        self.variant(Scbselect::ScVal60p)
    }
}
#[doc = "Internal Filter PLL - Select Internal Resistor Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Srbselect {
    #[doc = "0: 24 Ohms"]
    SrVal24k = 0,
    #[doc = "1: 6 Ohms"]
    SrVal6k = 1,
    #[doc = "2: 3 Ohms"]
    SrVal3k = 2,
    #[doc = "3: 12 Ohms"]
    SrVal12k = 3,
}
impl From<Srbselect> for u8 {
    #[inline(always)]
    fn from(variant: Srbselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Srbselect {
    type Ux = u8;
}
impl crate::IsEnum for Srbselect {}
#[doc = "Field `SRB` reader - Internal Filter PLL - Select Internal Resistor Value"]
pub type SrbR = crate::FieldReader<Srbselect>;
impl SrbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Srbselect {
        match self.bits {
            0 => Srbselect::SrVal24k,
            1 => Srbselect::SrVal6k,
            2 => Srbselect::SrVal3k,
            3 => Srbselect::SrVal12k,
            _ => unreachable!(),
        }
    }
    #[doc = "24 Ohms"]
    #[inline(always)]
    pub fn is_sr_val_24k(&self) -> bool {
        *self == Srbselect::SrVal24k
    }
    #[doc = "6 Ohms"]
    #[inline(always)]
    pub fn is_sr_val_6k(&self) -> bool {
        *self == Srbselect::SrVal6k
    }
    #[doc = "3 Ohms"]
    #[inline(always)]
    pub fn is_sr_val_3k(&self) -> bool {
        *self == Srbselect::SrVal3k
    }
    #[doc = "12 Ohms"]
    #[inline(always)]
    pub fn is_sr_val_12k(&self) -> bool {
        *self == Srbselect::SrVal12k
    }
}
#[doc = "Field `SRB` writer - Internal Filter PLL - Select Internal Resistor Value"]
pub type SrbW<'a, REG> = crate::FieldWriter<'a, REG, 2, Srbselect, crate::Safe>;
impl<'a, REG> SrbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "24 Ohms"]
    #[inline(always)]
    pub fn sr_val_24k(self) -> &'a mut crate::W<REG> {
        self.variant(Srbselect::SrVal24k)
    }
    #[doc = "6 Ohms"]
    #[inline(always)]
    pub fn sr_val_6k(self) -> &'a mut crate::W<REG> {
        self.variant(Srbselect::SrVal6k)
    }
    #[doc = "3 Ohms"]
    #[inline(always)]
    pub fn sr_val_3k(self) -> &'a mut crate::W<REG> {
        self.variant(Srbselect::SrVal3k)
    }
    #[doc = "12 Ohms"]
    #[inline(always)]
    pub fn sr_val_12k(self) -> &'a mut crate::W<REG> {
        self.variant(Srbselect::SrVal12k)
    }
}
impl R {
    #[doc = "Bits 0:3 - PLLA Output Current"]
    #[inline(always)]
    pub fn outcur_plla(&self) -> OutcurPllaR {
        OutcurPllaR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - Internal Filter PLL - Select Internal Capaticance Value"]
    #[inline(always)]
    pub fn sca(&self) -> ScaR {
        ScaR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Internal Filter PLL - Select Internal Resistor Value"]
    #[inline(always)]
    pub fn sra(&self) -> SraR {
        SraR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:19 - PLLB Output Current"]
    #[inline(always)]
    pub fn outcur_pllb(&self) -> OutcurPllbR {
        OutcurPllbR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - Internal Filter PLL - Select Internal Capaticance Value"]
    #[inline(always)]
    pub fn scb(&self) -> ScbR {
        ScbR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Internal Filter PLL - Select Internal Resistor Value"]
    #[inline(always)]
    pub fn srb(&self) -> SrbR {
        SrbR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PLLA Output Current"]
    #[inline(always)]
    pub fn outcur_plla(&mut self) -> OutcurPllaW<PllCfgSpec> {
        OutcurPllaW::new(self, 0)
    }
    #[doc = "Bits 12:13 - Internal Filter PLL - Select Internal Capaticance Value"]
    #[inline(always)]
    pub fn sca(&mut self) -> ScaW<PllCfgSpec> {
        ScaW::new(self, 12)
    }
    #[doc = "Bits 14:15 - Internal Filter PLL - Select Internal Resistor Value"]
    #[inline(always)]
    pub fn sra(&mut self) -> SraW<PllCfgSpec> {
        SraW::new(self, 14)
    }
    #[doc = "Bits 16:19 - PLLB Output Current"]
    #[inline(always)]
    pub fn outcur_pllb(&mut self) -> OutcurPllbW<PllCfgSpec> {
        OutcurPllbW::new(self, 16)
    }
    #[doc = "Bits 28:29 - Internal Filter PLL - Select Internal Capaticance Value"]
    #[inline(always)]
    pub fn scb(&mut self) -> ScbW<PllCfgSpec> {
        ScbW::new(self, 28)
    }
    #[doc = "Bits 30:31 - Internal Filter PLL - Select Internal Resistor Value"]
    #[inline(always)]
    pub fn srb(&mut self) -> SrbW<PllCfgSpec> {
        SrbW::new(self, 30)
    }
}
#[doc = "PLL Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllCfgSpec;
impl crate::RegisterSpec for PllCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll_cfg::R`](R) reader structure"]
impl crate::Readable for PllCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`pll_cfg::W`](W) writer structure"]
impl crate::Writable for PllCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLL_CFG to value 0"]
impl crate::Resettable for PllCfgSpec {}
