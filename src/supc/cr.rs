#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `ZERO` writer - Shall be always write at '0'"]
pub type ZeroW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Timing Domain Crystal Oscillator Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tdxtalselselect {
    #[doc = "0: No effect."]
    NoEffect = 0,
    #[doc = "1: If KEY is correct, XTALSEL switches the slow clock of the timing domain (TD_SLCK) on the 32.768 kHz crystal oscillator output."]
    CrystalSel = 1,
}
impl From<Tdxtalselselect> for bool {
    #[inline(always)]
    fn from(variant: Tdxtalselselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDXTALSEL` writer - Timing Domain Crystal Oscillator Select"]
pub type TdxtalselW<'a, REG> = crate::BitWriter<'a, REG, Tdxtalselselect>;
impl<'a, REG> TdxtalselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Tdxtalselselect::NoEffect)
    }
    #[doc = "If KEY is correct, XTALSEL switches the slow clock of the timing domain (TD_SLCK) on the 32.768 kHz crystal oscillator output."]
    #[inline(always)]
    pub fn crystal_sel(self) -> &'a mut crate::W<REG> {
        self.variant(Tdxtalselselect::CrystalSel)
    }
}
#[doc = "Monitoring Domain RC Oscillator Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mdrcselselect {
    #[doc = "0: No effect."]
    NoEffect = 0,
    #[doc = "1: If KEY is correct, XTALSEL switches the slow clock of the monitoring domain (MD_SLCK) on the slow RC oscillator output."]
    CrystalSel = 1,
}
impl From<Mdrcselselect> for bool {
    #[inline(always)]
    fn from(variant: Mdrcselselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MDRCSEL` writer - Monitoring Domain RC Oscillator Select"]
pub type MdrcselW<'a, REG> = crate::BitWriter<'a, REG, Mdrcselselect>;
impl<'a, REG> MdrcselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Mdrcselselect::NoEffect)
    }
    #[doc = "If KEY is correct, XTALSEL switches the slow clock of the monitoring domain (MD_SLCK) on the slow RC oscillator output."]
    #[inline(always)]
    pub fn crystal_sel(self) -> &'a mut crate::W<REG> {
        self.variant(Mdrcselselect::CrystalSel)
    }
}
#[doc = "Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Keyselect {
    #[doc = "165: Writing any other value in this field aborts the write operation."]
    Passwd = 165,
}
impl From<Keyselect> for u8 {
    #[inline(always)]
    fn from(variant: Keyselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Keyselect {
    type Ux = u8;
}
impl crate::IsEnum for Keyselect {}
#[doc = "Field `KEY` writer - Password"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, Keyselect>;
impl<'a, REG> KeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Writing any other value in this field aborts the write operation."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut crate::W<REG> {
        self.variant(Keyselect::Passwd)
    }
}
impl W {
    #[doc = "Bit 2 - Shall be always write at '0'"]
    #[inline(always)]
    pub fn zero(&mut self) -> ZeroW<CrSpec> {
        ZeroW::new(self, 2)
    }
    #[doc = "Bit 3 - Timing Domain Crystal Oscillator Select"]
    #[inline(always)]
    pub fn tdxtalsel(&mut self) -> TdxtalselW<CrSpec> {
        TdxtalselW::new(self, 3)
    }
    #[doc = "Bit 5 - Monitoring Domain RC Oscillator Select"]
    #[inline(always)]
    pub fn mdrcsel(&mut self) -> MdrcselW<CrSpec> {
        MdrcselW::new(self, 5)
    }
    #[doc = "Bits 24:31 - Password"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<CrSpec> {
        KeyW::new(self, 24)
    }
}
#[doc = "Supply Controller Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
