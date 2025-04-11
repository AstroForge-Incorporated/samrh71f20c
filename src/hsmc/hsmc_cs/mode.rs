#[doc = "Register `MODE` reader"]
pub type R = crate::R<ModeSpec>;
#[doc = "Register `MODE` writer"]
pub type W = crate::W<ModeSpec>;
#[doc = "Field `READ_MODE` reader - Read Mode"]
pub type ReadModeR = crate::BitReader;
#[doc = "Field `READ_MODE` writer - Read Mode"]
pub type ReadModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MODE` reader - Write Mode"]
pub type WriteModeR = crate::BitReader;
#[doc = "Field `WRITE_MODE` writer - Write Mode"]
pub type WriteModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RMW_ENABLE` reader - Read-Modify Write enable"]
pub type RmwEnableR = crate::BitReader;
#[doc = "Field `RMW_ENABLE` writer - Read-Modify Write enable"]
pub type RmwEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "NWAIT Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ExnwModeselect {
    #[doc = "0: Disabled-The NWAIT input signal is ignored on the corresponding chip select."]
    Disabled = 0,
    #[doc = "2: Frozen Mode-If asserted, the NWAIT signal freezes the current read or write cycle. After deassertion, the read/write cycle is resumed from the point where it was stopped."]
    Frozen = 2,
    #[doc = "3: Ready Mode-The NWAIT signal indicates the availability of the external device at the end of the pulse of the controlling read or write signal, to complete the access. If high, the access normally completes. If low, the access is extended until NWAIT returns high."]
    Ready = 3,
}
impl From<ExnwModeselect> for u8 {
    #[inline(always)]
    fn from(variant: ExnwModeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ExnwModeselect {
    type Ux = u8;
}
impl crate::IsEnum for ExnwModeselect {}
#[doc = "Field `EXNW_MODE` reader - NWAIT Mode"]
pub type ExnwModeR = crate::FieldReader<ExnwModeselect>;
impl ExnwModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ExnwModeselect> {
        match self.bits {
            0 => Some(ExnwModeselect::Disabled),
            2 => Some(ExnwModeselect::Frozen),
            3 => Some(ExnwModeselect::Ready),
            _ => None,
        }
    }
    #[doc = "Disabled-The NWAIT input signal is ignored on the corresponding chip select."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ExnwModeselect::Disabled
    }
    #[doc = "Frozen Mode-If asserted, the NWAIT signal freezes the current read or write cycle. After deassertion, the read/write cycle is resumed from the point where it was stopped."]
    #[inline(always)]
    pub fn is_frozen(&self) -> bool {
        *self == ExnwModeselect::Frozen
    }
    #[doc = "Ready Mode-The NWAIT signal indicates the availability of the external device at the end of the pulse of the controlling read or write signal, to complete the access. If high, the access normally completes. If low, the access is extended until NWAIT returns high."]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == ExnwModeselect::Ready
    }
}
#[doc = "Field `EXNW_MODE` writer - NWAIT Mode"]
pub type ExnwModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, ExnwModeselect>;
impl<'a, REG> ExnwModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled-The NWAIT input signal is ignored on the corresponding chip select."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ExnwModeselect::Disabled)
    }
    #[doc = "Frozen Mode-If asserted, the NWAIT signal freezes the current read or write cycle. After deassertion, the read/write cycle is resumed from the point where it was stopped."]
    #[inline(always)]
    pub fn frozen(self) -> &'a mut crate::W<REG> {
        self.variant(ExnwModeselect::Frozen)
    }
    #[doc = "Ready Mode-The NWAIT signal indicates the availability of the external device at the end of the pulse of the controlling read or write signal, to complete the access. If high, the access normally completes. If low, the access is extended until NWAIT returns high."]
    #[inline(always)]
    pub fn ready(self) -> &'a mut crate::W<REG> {
        self.variant(ExnwModeselect::Ready)
    }
}
#[doc = "Data Bus Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dbwselect {
    #[doc = "0: 8-bit Data Bus"]
    _8Bit = 0,
    #[doc = "1: 16-bit Data Bus"]
    _16Bit = 1,
    #[doc = "2: 32-bit Data Bus"]
    _32Bit = 2,
}
impl From<Dbwselect> for u8 {
    #[inline(always)]
    fn from(variant: Dbwselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dbwselect {
    type Ux = u8;
}
impl crate::IsEnum for Dbwselect {}
#[doc = "Field `DBW` reader - Data Bus Width"]
pub type DbwR = crate::FieldReader<Dbwselect>;
impl DbwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dbwselect> {
        match self.bits {
            0 => Some(Dbwselect::_8Bit),
            1 => Some(Dbwselect::_16Bit),
            2 => Some(Dbwselect::_32Bit),
            _ => None,
        }
    }
    #[doc = "8-bit Data Bus"]
    #[inline(always)]
    pub fn is_8_bit(&self) -> bool {
        *self == Dbwselect::_8Bit
    }
    #[doc = "16-bit Data Bus"]
    #[inline(always)]
    pub fn is_16_bit(&self) -> bool {
        *self == Dbwselect::_16Bit
    }
    #[doc = "32-bit Data Bus"]
    #[inline(always)]
    pub fn is_32_bit(&self) -> bool {
        *self == Dbwselect::_32Bit
    }
}
#[doc = "Field `DBW` writer - Data Bus Width"]
pub type DbwW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dbwselect>;
impl<'a, REG> DbwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8-bit Data Bus"]
    #[inline(always)]
    pub fn _8_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dbwselect::_8Bit)
    }
    #[doc = "16-bit Data Bus"]
    #[inline(always)]
    pub fn _16_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dbwselect::_16Bit)
    }
    #[doc = "32-bit Data Bus"]
    #[inline(always)]
    pub fn _32_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dbwselect::_32Bit)
    }
}
impl R {
    #[doc = "Bit 0 - Read Mode"]
    #[inline(always)]
    pub fn read_mode(&self) -> ReadModeR {
        ReadModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write Mode"]
    #[inline(always)]
    pub fn write_mode(&self) -> WriteModeR {
        WriteModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Read-Modify Write enable"]
    #[inline(always)]
    pub fn rmw_enable(&self) -> RmwEnableR {
        RmwEnableR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - NWAIT Mode"]
    #[inline(always)]
    pub fn exnw_mode(&self) -> ExnwModeR {
        ExnwModeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Data Bus Width"]
    #[inline(always)]
    pub fn dbw(&self) -> DbwR {
        DbwR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Read Mode"]
    #[inline(always)]
    pub fn read_mode(&mut self) -> ReadModeW<ModeSpec> {
        ReadModeW::new(self, 0)
    }
    #[doc = "Bit 1 - Write Mode"]
    #[inline(always)]
    pub fn write_mode(&mut self) -> WriteModeW<ModeSpec> {
        WriteModeW::new(self, 1)
    }
    #[doc = "Bit 2 - Read-Modify Write enable"]
    #[inline(always)]
    pub fn rmw_enable(&mut self) -> RmwEnableW<ModeSpec> {
        RmwEnableW::new(self, 2)
    }
    #[doc = "Bits 4:5 - NWAIT Mode"]
    #[inline(always)]
    pub fn exnw_mode(&mut self) -> ExnwModeW<ModeSpec> {
        ExnwModeW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Data Bus Width"]
    #[inline(always)]
    pub fn dbw(&mut self) -> DbwW<ModeSpec> {
        DbwW::new(self, 8)
    }
}
#[doc = "HSMC Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModeSpec;
impl crate::RegisterSpec for ModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mode::R`](R) reader structure"]
impl crate::Readable for ModeSpec {}
#[doc = "`write(|w| ..)` method takes [`mode::W`](W) writer structure"]
impl crate::Writable for ModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MODE to value 0"]
impl crate::Resettable for ModeSpec {}
