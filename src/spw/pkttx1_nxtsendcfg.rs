#[doc = "Register `PKTTX1_NXTSENDCFG` reader"]
pub type R = crate::R<Pkttx1NxtsendcfgSpec>;
#[doc = "Register `PKTTX1_NXTSENDCFG` writer"]
pub type W = crate::W<Pkttx1NxtsendcfgSpec>;
#[doc = "Field `LEN` reader - Length"]
pub type LenR = crate::FieldReader<u16>;
#[doc = "Field `LEN` writer - Length"]
pub type LenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `VALUE` reader - Start Value"]
pub type ValueR = crate::FieldReader;
#[doc = "Field `VALUE` writer - Start Value"]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Start Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Startselect {
    #[doc = "0: Start if any bit in Start Value matches an incoming event"]
    Startevent = 0,
    #[doc = "1: Start immediately, if possible"]
    Startnow = 1,
    #[doc = "2: Start if Start Value matches an incoming Time Code from Time Code Handler 1"]
    Starttch1 = 2,
}
impl From<Startselect> for u8 {
    #[inline(always)]
    fn from(variant: Startselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Startselect {
    type Ux = u8;
}
impl crate::IsEnum for Startselect {}
#[doc = "Field `START` reader - Start Mode"]
pub type StartR = crate::FieldReader<Startselect>;
impl StartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Startselect> {
        match self.bits {
            0 => Some(Startselect::Startevent),
            1 => Some(Startselect::Startnow),
            2 => Some(Startselect::Starttch1),
            _ => None,
        }
    }
    #[doc = "Start if any bit in Start Value matches an incoming event"]
    #[inline(always)]
    pub fn is_startevent(&self) -> bool {
        *self == Startselect::Startevent
    }
    #[doc = "Start immediately, if possible"]
    #[inline(always)]
    pub fn is_startnow(&self) -> bool {
        *self == Startselect::Startnow
    }
    #[doc = "Start if Start Value matches an incoming Time Code from Time Code Handler 1"]
    #[inline(always)]
    pub fn is_starttch1(&self) -> bool {
        *self == Startselect::Starttch1
    }
}
#[doc = "Field `START` writer - Start Mode"]
pub type StartW<'a, REG> = crate::FieldWriter<'a, REG, 2, Startselect>;
impl<'a, REG> StartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Start if any bit in Start Value matches an incoming event"]
    #[inline(always)]
    pub fn startevent(self) -> &'a mut crate::W<REG> {
        self.variant(Startselect::Startevent)
    }
    #[doc = "Start immediately, if possible"]
    #[inline(always)]
    pub fn startnow(self) -> &'a mut crate::W<REG> {
        self.variant(Startselect::Startnow)
    }
    #[doc = "Start if Start Value matches an incoming Time Code from Time Code Handler 1"]
    #[inline(always)]
    pub fn starttch1(self) -> &'a mut crate::W<REG> {
        self.variant(Startselect::Starttch1)
    }
}
#[doc = "Field `ABORT` reader - Abort"]
pub type AbortR = crate::BitReader;
#[doc = "Field `ABORT` writer - Abort"]
pub type AbortW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Length"]
    #[inline(always)]
    pub fn len(&self) -> LenR {
        LenR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:21 - Start Value"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 22:23 - Start Mode"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 29 - Abort"]
    #[inline(always)]
    pub fn abort(&self) -> AbortR {
        AbortR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Length"]
    #[inline(always)]
    pub fn len(&mut self) -> LenW<Pkttx1NxtsendcfgSpec> {
        LenW::new(self, 0)
    }
    #[doc = "Bits 16:21 - Start Value"]
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<Pkttx1NxtsendcfgSpec> {
        ValueW::new(self, 16)
    }
    #[doc = "Bits 22:23 - Start Mode"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<Pkttx1NxtsendcfgSpec> {
        StartW::new(self, 22)
    }
    #[doc = "Bit 29 - Abort"]
    #[inline(always)]
    pub fn abort(&mut self) -> AbortW<Pkttx1NxtsendcfgSpec> {
        AbortW::new(self, 29)
    }
}
#[doc = "PktTx Next Send List Config\n\nYou can [`read`](crate::Reg::read) this register and get [`pkttx1_nxtsendcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkttx1_nxtsendcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pkttx1NxtsendcfgSpec;
impl crate::RegisterSpec for Pkttx1NxtsendcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pkttx1_nxtsendcfg::R`](R) reader structure"]
impl crate::Readable for Pkttx1NxtsendcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`pkttx1_nxtsendcfg::W`](W) writer structure"]
impl crate::Writable for Pkttx1NxtsendcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PKTTX1_NXTSENDCFG to value 0"]
impl crate::Resettable for Pkttx1NxtsendcfgSpec {}
