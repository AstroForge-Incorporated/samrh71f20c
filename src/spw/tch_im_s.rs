#[doc = "Register `TCH_IM_S` writer"]
pub type W = crate::W<TchImSSpec>;
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
    pub fn tcevent(&mut self) -> TceventW<TchImSSpec> {
        TceventW::new(self, 0)
    }
    #[doc = "Bit 1 - Time Code"]
    #[inline(always)]
    pub fn timecode(&mut self) -> TimecodeW<TchImSSpec> {
        TimecodeW::new(self, 1)
    }
    #[doc = "Bit 2 - Any Time Code"]
    #[inline(always)]
    pub fn anytimecode(&mut self) -> AnytimecodeW<TchImSSpec> {
        AnytimecodeW::new(self, 2)
    }
    #[doc = "Bit 3 - Late Watchdog"]
    #[inline(always)]
    pub fn latewd(&mut self) -> LatewdW<TchImSSpec> {
        LatewdW::new(self, 3)
    }
    #[doc = "Bit 4 - Early Watchdog"]
    #[inline(always)]
    pub fn earlywd(&mut self) -> EarlywdW<TchImSSpec> {
        EarlywdW::new(self, 4)
    }
}
#[doc = "SpW Tch Interrupt Set Mask\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tch_im_s::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TchImSSpec;
impl crate::RegisterSpec for TchImSSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tch_im_s::W`](W) writer structure"]
impl crate::Writable for TchImSSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCH_IM_S to value 0"]
impl crate::Resettable for TchImSSpec {}
