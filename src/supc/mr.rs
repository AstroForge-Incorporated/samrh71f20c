#[doc = "Register `MR` reader"]
pub type R = crate::R<MrSpec>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MrSpec>;
#[doc = "VDDCORE Supply Monitor Reset Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Corsmrstenselect {
    #[doc = "0: The core reset signal vddcore_nreset is not affected when a VDDCORE supply monitor detection occurs."]
    NotEnable = 0,
    #[doc = "1: The core reset signal, vddcore_nreset is asserted when a VDDCORE supply monitor detection occurs."]
    Enable = 1,
}
impl From<Corsmrstenselect> for bool {
    #[inline(always)]
    fn from(variant: Corsmrstenselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CORSMRSTEN` reader - VDDCORE Supply Monitor Reset Enable"]
pub type CorsmrstenR = crate::BitReader<Corsmrstenselect>;
impl CorsmrstenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Corsmrstenselect {
        match self.bits {
            false => Corsmrstenselect::NotEnable,
            true => Corsmrstenselect::Enable,
        }
    }
    #[doc = "The core reset signal vddcore_nreset is not affected when a VDDCORE supply monitor detection occurs."]
    #[inline(always)]
    pub fn is_not_enable(&self) -> bool {
        *self == Corsmrstenselect::NotEnable
    }
    #[doc = "The core reset signal, vddcore_nreset is asserted when a VDDCORE supply monitor detection occurs."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Corsmrstenselect::Enable
    }
}
#[doc = "Field `CORSMRSTEN` writer - VDDCORE Supply Monitor Reset Enable"]
pub type CorsmrstenW<'a, REG> = crate::BitWriter<'a, REG, Corsmrstenselect>;
impl<'a, REG> CorsmrstenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The core reset signal vddcore_nreset is not affected when a VDDCORE supply monitor detection occurs."]
    #[inline(always)]
    pub fn not_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Corsmrstenselect::NotEnable)
    }
    #[doc = "The core reset signal, vddcore_nreset is asserted when a VDDCORE supply monitor detection occurs."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Corsmrstenselect::Enable)
    }
}
#[doc = "VDDCORE Supply Monitor Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Corsmdisselect {
    #[doc = "0: The VDDCORE supply monitor is enabled."]
    Enable = 0,
    #[doc = "1: The VDDCORE supply monitor is disabled."]
    Disable = 1,
}
impl From<Corsmdisselect> for bool {
    #[inline(always)]
    fn from(variant: Corsmdisselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CORSMDIS` reader - VDDCORE Supply Monitor Disable"]
pub type CorsmdisR = crate::BitReader<Corsmdisselect>;
impl CorsmdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Corsmdisselect {
        match self.bits {
            false => Corsmdisselect::Enable,
            true => Corsmdisselect::Disable,
        }
    }
    #[doc = "The VDDCORE supply monitor is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Corsmdisselect::Enable
    }
    #[doc = "The VDDCORE supply monitor is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Corsmdisselect::Disable
    }
}
#[doc = "Field `CORSMDIS` writer - VDDCORE Supply Monitor Disable"]
pub type CorsmdisW<'a, REG> = crate::BitWriter<'a, REG, Corsmdisselect>;
impl<'a, REG> CorsmdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The VDDCORE supply monitor is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Corsmdisselect::Enable)
    }
    #[doc = "The VDDCORE supply monitor is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Corsmdisselect::Disable)
    }
}
#[doc = "Oscillator Bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oscbypassselect {
    #[doc = "0: No effect. Clock selection depends on the value of XTALSEL (SUPC_CR)."]
    NoEffect = 0,
    #[doc = "1: The 32.768 kHz crystal oscillator is bypassed if XTALSEL (SUPC_CR) is set. OSCBYPASS must be set prior to setting XTALSEL."]
    Bypass = 1,
}
impl From<Oscbypassselect> for bool {
    #[inline(always)]
    fn from(variant: Oscbypassselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCBYPASS` reader - Oscillator Bypass"]
pub type OscbypassR = crate::BitReader<Oscbypassselect>;
impl OscbypassR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oscbypassselect {
        match self.bits {
            false => Oscbypassselect::NoEffect,
            true => Oscbypassselect::Bypass,
        }
    }
    #[doc = "No effect. Clock selection depends on the value of XTALSEL (SUPC_CR)."]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == Oscbypassselect::NoEffect
    }
    #[doc = "The 32.768 kHz crystal oscillator is bypassed if XTALSEL (SUPC_CR) is set. OSCBYPASS must be set prior to setting XTALSEL."]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == Oscbypassselect::Bypass
    }
}
#[doc = "Field `OSCBYPASS` writer - Oscillator Bypass"]
pub type OscbypassW<'a, REG> = crate::BitWriter<'a, REG, Oscbypassselect>;
impl<'a, REG> OscbypassW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect. Clock selection depends on the value of XTALSEL (SUPC_CR)."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Oscbypassselect::NoEffect)
    }
    #[doc = "The 32.768 kHz crystal oscillator is bypassed if XTALSEL (SUPC_CR) is set. OSCBYPASS must be set prior to setting XTALSEL."]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut crate::W<REG> {
        self.variant(Oscbypassselect::Bypass)
    }
}
#[doc = "Field `FXTALSTUP` reader - Fast Startup 32.768 kHz Crystal Oscillator"]
pub type FxtalstupR = crate::BitReader;
#[doc = "Field `FXTALSTUP` writer - Fast Startup 32.768 kHz Crystal Oscillator"]
pub type FxtalstupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Password Key\n\nValue on reset: 0"]
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
#[doc = "Field `KEY` reader - Password Key"]
pub type KeyR = crate::FieldReader<Keyselect>;
impl KeyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Keyselect> {
        match self.bits {
            165 => Some(Keyselect::Passwd),
            _ => None,
        }
    }
    #[doc = "Writing any other value in this field aborts the write operation."]
    #[inline(always)]
    pub fn is_passwd(&self) -> bool {
        *self == Keyselect::Passwd
    }
}
#[doc = "Field `KEY` writer - Password Key"]
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
impl R {
    #[doc = "Bit 12 - VDDCORE Supply Monitor Reset Enable"]
    #[inline(always)]
    pub fn corsmrsten(&self) -> CorsmrstenR {
        CorsmrstenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - VDDCORE Supply Monitor Disable"]
    #[inline(always)]
    pub fn corsmdis(&self) -> CorsmdisR {
        CorsmdisR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 20 - Oscillator Bypass"]
    #[inline(always)]
    pub fn oscbypass(&self) -> OscbypassR {
        OscbypassR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - Fast Startup 32.768 kHz Crystal Oscillator"]
    #[inline(always)]
    pub fn fxtalstup(&self) -> FxtalstupR {
        FxtalstupR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Password Key"]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 12 - VDDCORE Supply Monitor Reset Enable"]
    #[inline(always)]
    pub fn corsmrsten(&mut self) -> CorsmrstenW<MrSpec> {
        CorsmrstenW::new(self, 12)
    }
    #[doc = "Bit 13 - VDDCORE Supply Monitor Disable"]
    #[inline(always)]
    pub fn corsmdis(&mut self) -> CorsmdisW<MrSpec> {
        CorsmdisW::new(self, 13)
    }
    #[doc = "Bit 20 - Oscillator Bypass"]
    #[inline(always)]
    pub fn oscbypass(&mut self) -> OscbypassW<MrSpec> {
        OscbypassW::new(self, 20)
    }
    #[doc = "Bit 22 - Fast Startup 32.768 kHz Crystal Oscillator"]
    #[inline(always)]
    pub fn fxtalstup(&mut self) -> FxtalstupW<MrSpec> {
        FxtalstupW::new(self, 22)
    }
    #[doc = "Bits 24:31 - Password Key"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<MrSpec> {
        KeyW::new(self, 24)
    }
}
#[doc = "Supply Controller Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrSpec;
impl crate::RegisterSpec for MrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mr::R`](R) reader structure"]
impl crate::Readable for MrSpec {}
#[doc = "`write(|w| ..)` method takes [`mr::W`](W) writer structure"]
impl crate::Writable for MrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MrSpec {}
