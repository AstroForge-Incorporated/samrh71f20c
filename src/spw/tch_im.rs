#[doc = "Register `TCH_IM` reader"]
pub type R = crate::R<TchImSpec>;
#[doc = "Register `TCH_IM` writer"]
pub type W = crate::W<TchImSpec>;
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
    pub fn tcevent(&mut self) -> TceventW<TchImSpec> {
        TceventW::new(self, 0)
    }
    #[doc = "Bit 1 - Time Code"]
    #[inline(always)]
    pub fn timecode(&mut self) -> TimecodeW<TchImSpec> {
        TimecodeW::new(self, 1)
    }
    #[doc = "Bit 2 - Any Time Code"]
    #[inline(always)]
    pub fn anytimecode(&mut self) -> AnytimecodeW<TchImSpec> {
        AnytimecodeW::new(self, 2)
    }
    #[doc = "Bit 3 - Late Watchdog"]
    #[inline(always)]
    pub fn latewd(&mut self) -> LatewdW<TchImSpec> {
        LatewdW::new(self, 3)
    }
    #[doc = "Bit 4 - Early Watchdog"]
    #[inline(always)]
    pub fn earlywd(&mut self) -> EarlywdW<TchImSpec> {
        EarlywdW::new(self, 4)
    }
}
#[doc = "SpW Tch Interrupt Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`tch_im::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tch_im::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TchImSpec;
impl crate::RegisterSpec for TchImSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tch_im::R`](R) reader structure"]
impl crate::Readable for TchImSpec {}
#[doc = "`write(|w| ..)` method takes [`tch_im::W`](W) writer structure"]
impl crate::Writable for TchImSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCH_IM to value 0"]
impl crate::Resettable for TchImSpec {}
