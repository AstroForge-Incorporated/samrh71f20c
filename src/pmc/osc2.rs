#[doc = "Register `OSC2` reader"]
pub type R = crate::R<Osc2Spec>;
#[doc = "Register `OSC2` writer"]
pub type W = crate::W<Osc2Spec>;
#[doc = "Field `EN` reader - Enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "2nd Oscillator Frequency Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Oscrcfselect {
    #[doc = "0: The 2nd RC oscillator frequency is at 4 MHZ"]
    _4Mhz = 0,
    #[doc = "1: The 2nd RC oscillator frequency is at 8 MHZ"]
    _8Mhz = 1,
    #[doc = "2: The 2nd RC oscillator frequency is at 10 MHZ"]
    _10Mhz = 2,
    #[doc = "3: The 2nd RC oscillator frequency is at 12 MHZ"]
    _12Mhz = 3,
}
impl From<Oscrcfselect> for u8 {
    #[inline(always)]
    fn from(variant: Oscrcfselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Oscrcfselect {
    type Ux = u8;
}
impl crate::IsEnum for Oscrcfselect {}
#[doc = "Field `OSCRCF` reader - 2nd Oscillator Frequency Selection"]
pub type OscrcfR = crate::FieldReader<Oscrcfselect>;
impl OscrcfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oscrcfselect {
        match self.bits {
            0 => Oscrcfselect::_4Mhz,
            1 => Oscrcfselect::_8Mhz,
            2 => Oscrcfselect::_10Mhz,
            3 => Oscrcfselect::_12Mhz,
            _ => unreachable!(),
        }
    }
    #[doc = "The 2nd RC oscillator frequency is at 4 MHZ"]
    #[inline(always)]
    pub fn is_4_mhz(&self) -> bool {
        *self == Oscrcfselect::_4Mhz
    }
    #[doc = "The 2nd RC oscillator frequency is at 8 MHZ"]
    #[inline(always)]
    pub fn is_8_mhz(&self) -> bool {
        *self == Oscrcfselect::_8Mhz
    }
    #[doc = "The 2nd RC oscillator frequency is at 10 MHZ"]
    #[inline(always)]
    pub fn is_10_mhz(&self) -> bool {
        *self == Oscrcfselect::_10Mhz
    }
    #[doc = "The 2nd RC oscillator frequency is at 12 MHZ"]
    #[inline(always)]
    pub fn is_12_mhz(&self) -> bool {
        *self == Oscrcfselect::_12Mhz
    }
}
#[doc = "Field `OSCRCF` writer - 2nd Oscillator Frequency Selection"]
pub type OscrcfW<'a, REG> = crate::FieldWriter<'a, REG, 2, Oscrcfselect, crate::Safe>;
impl<'a, REG> OscrcfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The 2nd RC oscillator frequency is at 4 MHZ"]
    #[inline(always)]
    pub fn _4_mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Oscrcfselect::_4Mhz)
    }
    #[doc = "The 2nd RC oscillator frequency is at 8 MHZ"]
    #[inline(always)]
    pub fn _8_mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Oscrcfselect::_8Mhz)
    }
    #[doc = "The 2nd RC oscillator frequency is at 10 MHZ"]
    #[inline(always)]
    pub fn _10_mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Oscrcfselect::_10Mhz)
    }
    #[doc = "The 2nd RC oscillator frequency is at 12 MHZ"]
    #[inline(always)]
    pub fn _12_mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Oscrcfselect::_12Mhz)
    }
}
#[doc = "Field `EN_WR_CALIB` reader - Enable Calibration Register Write"]
pub type EnWrCalibR = crate::BitReader;
#[doc = "Field `EN_WR_CALIB` writer - Enable Calibration Register Write"]
pub type EnWrCalibW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Register Write Access Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Keyselect {
    #[doc = "55: Writing any other value in this field aborts the write operation.Always reads as 0."]
    Passwd = 55,
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
#[doc = "Field `KEY` reader - Register Write Access Password"]
pub type KeyR = crate::FieldReader<Keyselect>;
impl KeyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Keyselect> {
        match self.bits {
            55 => Some(Keyselect::Passwd),
            _ => None,
        }
    }
    #[doc = "Writing any other value in this field aborts the write operation.Always reads as 0."]
    #[inline(always)]
    pub fn is_passwd(&self) -> bool {
        *self == Keyselect::Passwd
    }
}
#[doc = "Field `KEY` writer - Register Write Access Password"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, Keyselect>;
impl<'a, REG> KeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Writing any other value in this field aborts the write operation.Always reads as 0."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut crate::W<REG> {
        self.variant(Keyselect::Passwd)
    }
}
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - 2nd Oscillator Frequency Selection"]
    #[inline(always)]
    pub fn oscrcf(&self) -> OscrcfR {
        OscrcfR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - Enable Calibration Register Write"]
    #[inline(always)]
    pub fn en_wr_calib(&self) -> EnWrCalibR {
        EnWrCalibR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Register Write Access Password"]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<Osc2Spec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 4:5 - 2nd Oscillator Frequency Selection"]
    #[inline(always)]
    pub fn oscrcf(&mut self) -> OscrcfW<Osc2Spec> {
        OscrcfW::new(self, 4)
    }
    #[doc = "Bit 8 - Enable Calibration Register Write"]
    #[inline(always)]
    pub fn en_wr_calib(&mut self) -> EnWrCalibW<Osc2Spec> {
        EnWrCalibW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Register Write Access Password"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<Osc2Spec> {
        KeyW::new(self, 16)
    }
}
#[doc = "Oscillator Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`osc2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osc2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Osc2Spec;
impl crate::RegisterSpec for Osc2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osc2::R`](R) reader structure"]
impl crate::Readable for Osc2Spec {}
#[doc = "`write(|w| ..)` method takes [`osc2::W`](W) writer structure"]
impl crate::Writable for Osc2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OSC2 to value 0"]
impl crate::Resettable for Osc2Spec {}
