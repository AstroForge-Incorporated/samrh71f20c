#[doc = "Register `PKTRX1_NXTBUFCFG` reader"]
pub type R = crate::R<Pktrx1NxtbufcfgSpec>;
#[doc = "Register `PKTRX1_NXTBUFCFG` writer"]
pub type W = crate::W<Pktrx1NxtbufcfgSpec>;
#[doc = "Field `MAXCNT` reader - Max Count"]
pub type MaxcntR = crate::FieldReader<u16>;
#[doc = "Field `MAXCNT` writer - Max Count"]
pub type MaxcntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
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
    #[doc = "1: Start immediately. Request a deactivation on next packet boundary."]
    Startnow = 1,
    #[doc = "2: Start if Start Value matches an incoming Time Code from Time Code Handler 1"]
    Starttch1 = 2,
    #[doc = "4: Start when current buffer is deactivated (e.g., by buffer becoming full)"]
    Startlater = 4,
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
            4 => Some(Startselect::Startlater),
            _ => None,
        }
    }
    #[doc = "Start if any bit in Start Value matches an incoming event"]
    #[inline(always)]
    pub fn is_startevent(&self) -> bool {
        *self == Startselect::Startevent
    }
    #[doc = "Start immediately. Request a deactivation on next packet boundary."]
    #[inline(always)]
    pub fn is_startnow(&self) -> bool {
        *self == Startselect::Startnow
    }
    #[doc = "Start if Start Value matches an incoming Time Code from Time Code Handler 1"]
    #[inline(always)]
    pub fn is_starttch1(&self) -> bool {
        *self == Startselect::Starttch1
    }
    #[doc = "Start when current buffer is deactivated (e.g., by buffer becoming full)"]
    #[inline(always)]
    pub fn is_startlater(&self) -> bool {
        *self == Startselect::Startlater
    }
}
#[doc = "Field `START` writer - Start Mode"]
pub type StartW<'a, REG> = crate::FieldWriter<'a, REG, 3, Startselect>;
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
    #[doc = "Start immediately. Request a deactivation on next packet boundary."]
    #[inline(always)]
    pub fn startnow(self) -> &'a mut crate::W<REG> {
        self.variant(Startselect::Startnow)
    }
    #[doc = "Start if Start Value matches an incoming Time Code from Time Code Handler 1"]
    #[inline(always)]
    pub fn starttch1(self) -> &'a mut crate::W<REG> {
        self.variant(Startselect::Starttch1)
    }
    #[doc = "Start when current buffer is deactivated (e.g., by buffer becoming full)"]
    #[inline(always)]
    pub fn startlater(self) -> &'a mut crate::W<REG> {
        self.variant(Startselect::Startlater)
    }
}
#[doc = "Field `SPLIT` reader - Split Pkt"]
pub type SplitR = crate::BitReader;
#[doc = "Field `SPLIT` writer - Split Pkt"]
pub type SplitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Max Count"]
    #[inline(always)]
    pub fn maxcnt(&self) -> MaxcntR {
        MaxcntR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:21 - Start Value"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 22:24 - Start Mode"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bit 30 - Split Pkt"]
    #[inline(always)]
    pub fn split(&self) -> SplitR {
        SplitR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Max Count"]
    #[inline(always)]
    pub fn maxcnt(&mut self) -> MaxcntW<Pktrx1NxtbufcfgSpec> {
        MaxcntW::new(self, 0)
    }
    #[doc = "Bits 16:21 - Start Value"]
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<Pktrx1NxtbufcfgSpec> {
        ValueW::new(self, 16)
    }
    #[doc = "Bits 22:24 - Start Mode"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<Pktrx1NxtbufcfgSpec> {
        StartW::new(self, 22)
    }
    #[doc = "Bit 30 - Split Pkt"]
    #[inline(always)]
    pub fn split(&mut self) -> SplitW<Pktrx1NxtbufcfgSpec> {
        SplitW::new(self, 30)
    }
}
#[doc = "PktRx Next Buffer Config\n\nYou can [`read`](crate::Reg::read) this register and get [`pktrx1_nxtbufcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pktrx1_nxtbufcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pktrx1NxtbufcfgSpec;
impl crate::RegisterSpec for Pktrx1NxtbufcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pktrx1_nxtbufcfg::R`](R) reader structure"]
impl crate::Readable for Pktrx1NxtbufcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`pktrx1_nxtbufcfg::W`](W) writer structure"]
impl crate::Writable for Pktrx1NxtbufcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PKTRX1_NXTBUFCFG to value 0"]
impl crate::Resettable for Pktrx1NxtbufcfgSpec {}
