#[doc = "Register `CKGR_MOR` reader"]
pub type R = crate::R<CkgrMorSpec>;
#[doc = "Register `CKGR_MOR` writer"]
pub type W = crate::W<CkgrMorSpec>;
#[doc = "Field `MOSCXTEN` reader - Main Crystal Oscillator Enable"]
pub type MoscxtenR = crate::BitReader;
#[doc = "Field `MOSCXTEN` writer - Main Crystal Oscillator Enable"]
pub type MoscxtenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOSCXTBY` reader - Main Crystal Oscillator Bypass"]
pub type MoscxtbyR = crate::BitReader;
#[doc = "Field `MOSCXTBY` writer - Main Crystal Oscillator Bypass"]
pub type MoscxtbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MOSCRCEN` reader - Main RC Oscillator Enable"]
pub type MoscrcenR = crate::BitReader;
#[doc = "Field `MOSCRCEN` writer - Main RC Oscillator Enable"]
pub type MoscrcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Main RC Oscillator Frequency Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Moscrcfselect {
    #[doc = "0: The RC oscillator frequency is at 4 MHz"]
    _4Mhz = 0,
    #[doc = "1: The RC oscillator frequency is at 8 MHz"]
    _8Mhz = 1,
    #[doc = "2: The RC oscillator frequency is at 10 MHz"]
    _10Mhz = 2,
    #[doc = "3: The RC oscillator frequency is at 12 MHz"]
    _12Mhz = 3,
}
impl From<Moscrcfselect> for u8 {
    #[inline(always)]
    fn from(variant: Moscrcfselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Moscrcfselect {
    type Ux = u8;
}
impl crate::IsEnum for Moscrcfselect {}
#[doc = "Field `MOSCRCF` reader - Main RC Oscillator Frequency Selection"]
pub type MoscrcfR = crate::FieldReader<Moscrcfselect>;
impl MoscrcfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Moscrcfselect> {
        match self.bits {
            0 => Some(Moscrcfselect::_4Mhz),
            1 => Some(Moscrcfselect::_8Mhz),
            2 => Some(Moscrcfselect::_10Mhz),
            3 => Some(Moscrcfselect::_12Mhz),
            _ => None,
        }
    }
    #[doc = "The RC oscillator frequency is at 4 MHz"]
    #[inline(always)]
    pub fn is_4_mhz(&self) -> bool {
        *self == Moscrcfselect::_4Mhz
    }
    #[doc = "The RC oscillator frequency is at 8 MHz"]
    #[inline(always)]
    pub fn is_8_mhz(&self) -> bool {
        *self == Moscrcfselect::_8Mhz
    }
    #[doc = "The RC oscillator frequency is at 10 MHz"]
    #[inline(always)]
    pub fn is_10_mhz(&self) -> bool {
        *self == Moscrcfselect::_10Mhz
    }
    #[doc = "The RC oscillator frequency is at 12 MHz"]
    #[inline(always)]
    pub fn is_12_mhz(&self) -> bool {
        *self == Moscrcfselect::_12Mhz
    }
}
#[doc = "Field `MOSCRCF` writer - Main RC Oscillator Frequency Selection"]
pub type MoscrcfW<'a, REG> = crate::FieldWriter<'a, REG, 3, Moscrcfselect>;
impl<'a, REG> MoscrcfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The RC oscillator frequency is at 4 MHz"]
    #[inline(always)]
    pub fn _4_mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Moscrcfselect::_4Mhz)
    }
    #[doc = "The RC oscillator frequency is at 8 MHz"]
    #[inline(always)]
    pub fn _8_mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Moscrcfselect::_8Mhz)
    }
    #[doc = "The RC oscillator frequency is at 10 MHz"]
    #[inline(always)]
    pub fn _10_mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Moscrcfselect::_10Mhz)
    }
    #[doc = "The RC oscillator frequency is at 12 MHz"]
    #[inline(always)]
    pub fn _12_mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Moscrcfselect::_12Mhz)
    }
}
#[doc = "Field `MOSCXTST` reader - Main Crystal Oscillator Startup Time"]
pub type MoscxtstR = crate::FieldReader;
#[doc = "Field `MOSCXTST` writer - Main Crystal Oscillator Startup Time"]
pub type MoscxtstW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Write Access Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Keyselect {
    #[doc = "55: Writing any other value in this field aborts the write operation. Always reads as 0."]
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
#[doc = "Field `KEY` reader - Write Access Password"]
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
    #[doc = "Writing any other value in this field aborts the write operation. Always reads as 0."]
    #[inline(always)]
    pub fn is_passwd(&self) -> bool {
        *self == Keyselect::Passwd
    }
}
#[doc = "Field `KEY` writer - Write Access Password"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, Keyselect>;
impl<'a, REG> KeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Writing any other value in this field aborts the write operation. Always reads as 0."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut crate::W<REG> {
        self.variant(Keyselect::Passwd)
    }
}
#[doc = "Field `MOSCSEL` reader - Main Clock Oscillator Selection"]
pub type MoscselR = crate::BitReader;
#[doc = "Field `MOSCSEL` writer - Main Clock Oscillator Selection"]
pub type MoscselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFDEN` reader - Clock Failure Detector Enable"]
pub type CfdenR = crate::BitReader;
#[doc = "Field `CFDEN` writer - Clock Failure Detector Enable"]
pub type CfdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXT32KFME` reader - 32.768 kHz Crystal Oscillator Frequency Monitoring Enable"]
pub type Ext32kfmeR = crate::BitReader;
#[doc = "Field `EXT32KFME` writer - 32.768 kHz Crystal Oscillator Frequency Monitoring Enable"]
pub type Ext32kfmeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BCPURST` reader - Bad CPU Clock Reset Enable"]
pub type BcpurstR = crate::BitReader;
#[doc = "Field `BCPURST` writer - Bad CPU Clock Reset Enable"]
pub type BcpurstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BCPUNMIC` reader - Bad CPU Clock Interrupt to NMIC Enable"]
pub type BcpunmicR = crate::BitReader;
#[doc = "Field `BCPUNMIC` writer - Bad CPU Clock Interrupt to NMIC Enable"]
pub type BcpunmicW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Main Crystal Oscillator Enable"]
    #[inline(always)]
    pub fn moscxten(&self) -> MoscxtenR {
        MoscxtenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Main Crystal Oscillator Bypass"]
    #[inline(always)]
    pub fn moscxtby(&self) -> MoscxtbyR {
        MoscxtbyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Main RC Oscillator Enable"]
    #[inline(always)]
    pub fn moscrcen(&self) -> MoscrcenR {
        MoscrcenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Main RC Oscillator Frequency Selection"]
    #[inline(always)]
    pub fn moscrcf(&self) -> MoscrcfR {
        MoscrcfR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:15 - Main Crystal Oscillator Startup Time"]
    #[inline(always)]
    pub fn moscxtst(&self) -> MoscxtstR {
        MoscxtstR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Write Access Password"]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Main Clock Oscillator Selection"]
    #[inline(always)]
    pub fn moscsel(&self) -> MoscselR {
        MoscselR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Clock Failure Detector Enable"]
    #[inline(always)]
    pub fn cfden(&self) -> CfdenR {
        CfdenR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 32.768 kHz Crystal Oscillator Frequency Monitoring Enable"]
    #[inline(always)]
    pub fn ext32kfme(&self) -> Ext32kfmeR {
        Ext32kfmeR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Bad CPU Clock Reset Enable"]
    #[inline(always)]
    pub fn bcpurst(&self) -> BcpurstR {
        BcpurstR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Bad CPU Clock Interrupt to NMIC Enable"]
    #[inline(always)]
    pub fn bcpunmic(&self) -> BcpunmicR {
        BcpunmicR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Main Crystal Oscillator Enable"]
    #[inline(always)]
    pub fn moscxten(&mut self) -> MoscxtenW<CkgrMorSpec> {
        MoscxtenW::new(self, 0)
    }
    #[doc = "Bit 1 - Main Crystal Oscillator Bypass"]
    #[inline(always)]
    pub fn moscxtby(&mut self) -> MoscxtbyW<CkgrMorSpec> {
        MoscxtbyW::new(self, 1)
    }
    #[doc = "Bit 3 - Main RC Oscillator Enable"]
    #[inline(always)]
    pub fn moscrcen(&mut self) -> MoscrcenW<CkgrMorSpec> {
        MoscrcenW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Main RC Oscillator Frequency Selection"]
    #[inline(always)]
    pub fn moscrcf(&mut self) -> MoscrcfW<CkgrMorSpec> {
        MoscrcfW::new(self, 4)
    }
    #[doc = "Bits 8:15 - Main Crystal Oscillator Startup Time"]
    #[inline(always)]
    pub fn moscxtst(&mut self) -> MoscxtstW<CkgrMorSpec> {
        MoscxtstW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Write Access Password"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<CkgrMorSpec> {
        KeyW::new(self, 16)
    }
    #[doc = "Bit 24 - Main Clock Oscillator Selection"]
    #[inline(always)]
    pub fn moscsel(&mut self) -> MoscselW<CkgrMorSpec> {
        MoscselW::new(self, 24)
    }
    #[doc = "Bit 25 - Clock Failure Detector Enable"]
    #[inline(always)]
    pub fn cfden(&mut self) -> CfdenW<CkgrMorSpec> {
        CfdenW::new(self, 25)
    }
    #[doc = "Bit 26 - 32.768 kHz Crystal Oscillator Frequency Monitoring Enable"]
    #[inline(always)]
    pub fn ext32kfme(&mut self) -> Ext32kfmeW<CkgrMorSpec> {
        Ext32kfmeW::new(self, 26)
    }
    #[doc = "Bit 27 - Bad CPU Clock Reset Enable"]
    #[inline(always)]
    pub fn bcpurst(&mut self) -> BcpurstW<CkgrMorSpec> {
        BcpurstW::new(self, 27)
    }
    #[doc = "Bit 28 - Bad CPU Clock Interrupt to NMIC Enable"]
    #[inline(always)]
    pub fn bcpunmic(&mut self) -> BcpunmicW<CkgrMorSpec> {
        BcpunmicW::new(self, 28)
    }
}
#[doc = "Main Oscillator Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ckgr_mor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckgr_mor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CkgrMorSpec;
impl crate::RegisterSpec for CkgrMorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ckgr_mor::R`](R) reader structure"]
impl crate::Readable for CkgrMorSpec {}
#[doc = "`write(|w| ..)` method takes [`ckgr_mor::W`](W) writer structure"]
impl crate::Writable for CkgrMorSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CKGR_MOR to value 0"]
impl crate::Resettable for CkgrMorSpec {}
