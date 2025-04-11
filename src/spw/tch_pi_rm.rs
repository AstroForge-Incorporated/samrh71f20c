#[doc = "Register `TCH_PI_RM` reader"]
pub type R = crate::R<TchPiRmSpec>;
#[doc = "Field `TCEVENT` reader - TcEvent"]
pub type TceventR = crate::BitReader;
#[doc = "Field `TIMECODE` reader - Time Code"]
pub type TimecodeR = crate::BitReader;
#[doc = "Field `ANYTIMECODE` reader - Any Time Code"]
pub type AnytimecodeR = crate::BitReader;
#[doc = "Field `LATEWD` reader - Late Watchdog"]
pub type LatewdR = crate::BitReader;
#[doc = "Field `EARLYWD` reader - Early Watchdog"]
pub type EarlywdR = crate::BitReader;
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
#[doc = "SpW Tch Pending Read Masked Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`tch_pi_rm::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TchPiRmSpec;
impl crate::RegisterSpec for TchPiRmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tch_pi_rm::R`](R) reader structure"]
impl crate::Readable for TchPiRmSpec {}
#[doc = "`reset()` method sets TCH_PI_RM to value 0"]
impl crate::Resettable for TchPiRmSpec {}
