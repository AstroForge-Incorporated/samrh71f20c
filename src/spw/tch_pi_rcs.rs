#[doc = "Register `TCH_PI_RCS` reader"]
pub type R = crate::R<TchPiRcsSpec>;
#[doc = "Register `TCH_PI_RCS` writer"]
pub type W = crate::W<TchPiRcsSpec>;
#[doc = "Field `TCEVENT` reader - TcEvent"]
pub type TceventR = crate::BitReader;
#[doc = "Field `TCEVENT` writer - TcEvent"]
pub type TceventW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMECODE` reader - Time Code"]
pub type TimecodeR = crate::BitReader;
#[doc = "Field `TIMECODE` writer - Time Code"]
pub type TimecodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANYTIMECODE` reader - Any Time Code"]
pub type AnytimecodeR = crate::BitReader;
#[doc = "Field `ANYTIMECODE` writer - Any Time Code"]
pub type AnytimecodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LATEWD` reader - Late Watchdog"]
pub type LatewdR = crate::BitReader;
#[doc = "Field `LATEWD` writer - Late Watchdog"]
pub type LatewdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EARLYWD` reader - Early Watchdog"]
pub type EarlywdR = crate::BitReader;
#[doc = "Field `EARLYWD` writer - Early Watchdog"]
pub type EarlywdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TcEvent"]
    #[inline(always)]
    pub fn tcevent(&self) -> TceventR {
        TceventR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Time Code"]
    #[inline(always)]
    pub fn timecode(&self) -> TimecodeR {
        TimecodeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Any Time Code"]
    #[inline(always)]
    pub fn anytimecode(&self) -> AnytimecodeR {
        AnytimecodeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Late Watchdog"]
    #[inline(always)]
    pub fn latewd(&self) -> LatewdR {
        LatewdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Early Watchdog"]
    #[inline(always)]
    pub fn earlywd(&self) -> EarlywdR {
        EarlywdR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TcEvent"]
    #[inline(always)]
    pub fn tcevent(&mut self) -> TceventW<TchPiRcsSpec> {
        TceventW::new(self, 0)
    }
    #[doc = "Bit 1 - Time Code"]
    #[inline(always)]
    pub fn timecode(&mut self) -> TimecodeW<TchPiRcsSpec> {
        TimecodeW::new(self, 1)
    }
    #[doc = "Bit 2 - Any Time Code"]
    #[inline(always)]
    pub fn anytimecode(&mut self) -> AnytimecodeW<TchPiRcsSpec> {
        AnytimecodeW::new(self, 2)
    }
    #[doc = "Bit 3 - Late Watchdog"]
    #[inline(always)]
    pub fn latewd(&mut self) -> LatewdW<TchPiRcsSpec> {
        LatewdW::new(self, 3)
    }
    #[doc = "Bit 4 - Early Watchdog"]
    #[inline(always)]
    pub fn earlywd(&mut self) -> EarlywdW<TchPiRcsSpec> {
        EarlywdW::new(self, 4)
    }
}
#[doc = "SpW Tch Pending Read, Clear and Enabled Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`tch_pi_rcs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tch_pi_rcs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TchPiRcsSpec;
impl crate::RegisterSpec for TchPiRcsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tch_pi_rcs::R`](R) reader structure"]
impl crate::Readable for TchPiRcsSpec {}
#[doc = "`write(|w| ..)` method takes [`tch_pi_rcs::W`](W) writer structure"]
impl crate::Writable for TchPiRcsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCH_PI_RCS to value 0"]
impl crate::Resettable for TchPiRcsSpec {}
