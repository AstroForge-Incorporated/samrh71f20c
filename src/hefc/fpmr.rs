#[doc = "Register `FPMR` reader"]
pub type R = crate::R<FpmrSpec>;
#[doc = "Register `FPMR` writer"]
pub type W = crate::W<FpmrSpec>;
#[doc = "Field `PWS_EN` reader - Power switch enable"]
pub type PwsEnR = crate::BitReader;
#[doc = "Field `PWS_EN` writer - Power switch enable"]
pub type PwsEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWS_STAT` reader - Power switch Status"]
pub type PwsStatR = crate::BitReader;
#[doc = "Field `PWS_STAT` writer - Power switch Status"]
pub type PwsStatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Power switch Delay\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PwsDlyselect {
    #[doc = "0: delay is set to 75 usec"]
    _75us = 0,
    #[doc = "1: delay is set to 150 usec"]
    _150us = 1,
    #[doc = "2: delay is set to 300 usec"]
    _300us = 2,
    #[doc = "3: delay is set to 600 usec"]
    _600us = 3,
}
impl From<PwsDlyselect> for u8 {
    #[inline(always)]
    fn from(variant: PwsDlyselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PwsDlyselect {
    type Ux = u8;
}
impl crate::IsEnum for PwsDlyselect {}
#[doc = "Field `PWS_DLY` reader - Power switch Delay"]
pub type PwsDlyR = crate::FieldReader<PwsDlyselect>;
impl PwsDlyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PwsDlyselect {
        match self.bits {
            0 => PwsDlyselect::_75us,
            1 => PwsDlyselect::_150us,
            2 => PwsDlyselect::_300us,
            3 => PwsDlyselect::_600us,
            _ => unreachable!(),
        }
    }
    #[doc = "delay is set to 75 usec"]
    #[inline(always)]
    pub fn is_75us(&self) -> bool {
        *self == PwsDlyselect::_75us
    }
    #[doc = "delay is set to 150 usec"]
    #[inline(always)]
    pub fn is_150us(&self) -> bool {
        *self == PwsDlyselect::_150us
    }
    #[doc = "delay is set to 300 usec"]
    #[inline(always)]
    pub fn is_300us(&self) -> bool {
        *self == PwsDlyselect::_300us
    }
    #[doc = "delay is set to 600 usec"]
    #[inline(always)]
    pub fn is_600us(&self) -> bool {
        *self == PwsDlyselect::_600us
    }
}
#[doc = "Field `PWS_DLY` writer - Power switch Delay"]
pub type PwsDlyW<'a, REG> = crate::FieldWriter<'a, REG, 2, PwsDlyselect, crate::Safe>;
impl<'a, REG> PwsDlyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "delay is set to 75 usec"]
    #[inline(always)]
    pub fn _75us(self) -> &'a mut crate::W<REG> {
        self.variant(PwsDlyselect::_75us)
    }
    #[doc = "delay is set to 150 usec"]
    #[inline(always)]
    pub fn _150us(self) -> &'a mut crate::W<REG> {
        self.variant(PwsDlyselect::_150us)
    }
    #[doc = "delay is set to 300 usec"]
    #[inline(always)]
    pub fn _300us(self) -> &'a mut crate::W<REG> {
        self.variant(PwsDlyselect::_300us)
    }
    #[doc = "delay is set to 600 usec"]
    #[inline(always)]
    pub fn _600us(self) -> &'a mut crate::W<REG> {
        self.variant(PwsDlyselect::_600us)
    }
}
#[doc = "Field `VAR_FACTOR` reader - Variation Factor"]
pub type VarFactorR = crate::FieldReader;
#[doc = "Field `VAR_FACTOR` writer - Variation Factor"]
pub type VarFactorW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `FUNC_ISOL_CTRL_N` reader - Flash insulated Control Status"]
pub type FuncIsolCtrlNR = crate::BitReader;
#[doc = "Field `FUNC_ISOL_CTRL_N` writer - Flash insulated Control Status"]
pub type FuncIsolCtrlNW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Power switch enable"]
    #[inline(always)]
    pub fn pws_en(&self) -> PwsEnR {
        PwsEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power switch Status"]
    #[inline(always)]
    pub fn pws_stat(&self) -> PwsStatR {
        PwsStatR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Power switch Delay"]
    #[inline(always)]
    pub fn pws_dly(&self) -> PwsDlyR {
        PwsDlyR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 8:13 - Variation Factor"]
    #[inline(always)]
    pub fn var_factor(&self) -> VarFactorR {
        VarFactorR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - Flash insulated Control Status"]
    #[inline(always)]
    pub fn func_isol_ctrl_n(&self) -> FuncIsolCtrlNR {
        FuncIsolCtrlNR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power switch enable"]
    #[inline(always)]
    pub fn pws_en(&mut self) -> PwsEnW<FpmrSpec> {
        PwsEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Power switch Status"]
    #[inline(always)]
    pub fn pws_stat(&mut self) -> PwsStatW<FpmrSpec> {
        PwsStatW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Power switch Delay"]
    #[inline(always)]
    pub fn pws_dly(&mut self) -> PwsDlyW<FpmrSpec> {
        PwsDlyW::new(self, 2)
    }
    #[doc = "Bits 8:13 - Variation Factor"]
    #[inline(always)]
    pub fn var_factor(&mut self) -> VarFactorW<FpmrSpec> {
        VarFactorW::new(self, 8)
    }
    #[doc = "Bit 16 - Flash insulated Control Status"]
    #[inline(always)]
    pub fn func_isol_ctrl_n(&mut self) -> FuncIsolCtrlNW<FpmrSpec> {
        FuncIsolCtrlNW::new(self, 16)
    }
}
#[doc = "HEFC Flash Power Management Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fpmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FpmrSpec;
impl crate::RegisterSpec for FpmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpmr::R`](R) reader structure"]
impl crate::Readable for FpmrSpec {}
#[doc = "`write(|w| ..)` method takes [`fpmr::W`](W) writer structure"]
impl crate::Writable for FpmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FPMR to value 0"]
impl crate::Resettable for FpmrSpec {}
