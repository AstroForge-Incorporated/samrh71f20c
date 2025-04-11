#[doc = "Register `TCH_PI_C` writer"]
pub type W = crate::W<TchPiCSpec>;
#[doc = "Field `TCEVENT` writer - TcEvent"]
pub type TceventW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMECODE` writer - Time Code"]
pub type TimecodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANYTIMECODE` writer - Any Time Code"]
pub type AnytimecodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LATEWD` writer - Late Watchdog"]
pub type LatewdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EARLYWD` writer - Early Watchdog"]
pub type EarlywdW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - TcEvent"]
    #[inline(always)]
    pub fn tcevent(&mut self) -> TceventW<TchPiCSpec> {
        TceventW::new(self, 0)
    }
    #[doc = "Bit 1 - Time Code"]
    #[inline(always)]
    pub fn timecode(&mut self) -> TimecodeW<TchPiCSpec> {
        TimecodeW::new(self, 1)
    }
    #[doc = "Bit 2 - Any Time Code"]
    #[inline(always)]
    pub fn anytimecode(&mut self) -> AnytimecodeW<TchPiCSpec> {
        AnytimecodeW::new(self, 2)
    }
    #[doc = "Bit 3 - Late Watchdog"]
    #[inline(always)]
    pub fn latewd(&mut self) -> LatewdW<TchPiCSpec> {
        LatewdW::new(self, 3)
    }
    #[doc = "Bit 4 - Early Watchdog"]
    #[inline(always)]
    pub fn earlywd(&mut self) -> EarlywdW<TchPiCSpec> {
        EarlywdW::new(self, 4)
    }
}
#[doc = "SpW Tch Clear Pending Interrupt\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tch_pi_c::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TchPiCSpec;
impl crate::RegisterSpec for TchPiCSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tch_pi_c::W`](W) writer structure"]
impl crate::Writable for TchPiCSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCH_PI_C to value 0"]
impl crate::Resettable for TchPiCSpec {}
