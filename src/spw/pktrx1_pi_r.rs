#[doc = "Register `PKTRX1_PI_R` reader"]
pub type R = crate::R<Pktrx1PiRSpec>;
#[doc = "Field `DEACT` reader - Deactivated"]
pub type DeactR = crate::BitReader;
#[doc = "Field `EOP` reader - EOP seen"]
pub type EopR = crate::BitReader;
#[doc = "Field `EEP` reader - EEP seen"]
pub type EepR = crate::BitReader;
#[doc = "Field `DISCARD` reader - Packet Discard"]
pub type DiscardR = crate::BitReader;
#[doc = "Field `ACT` reader - Activated"]
pub type ActR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Deactivated"]
    #[inline(always)]
    pub fn deact(&self) -> DeactR {
        DeactR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EOP seen"]
    #[inline(always)]
    pub fn eop(&self) -> EopR {
        EopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EEP seen"]
    #[inline(always)]
    pub fn eep(&self) -> EepR {
        EepR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Packet Discard"]
    #[inline(always)]
    pub fn discard(&self) -> DiscardR {
        DiscardR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Activated"]
    #[inline(always)]
    pub fn act(&self) -> ActR {
        ActR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "PktRx Pending Read Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`pktrx1_pi_r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pktrx1PiRSpec;
impl crate::RegisterSpec for Pktrx1PiRSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pktrx1_pi_r::R`](R) reader structure"]
impl crate::Readable for Pktrx1PiRSpec {}
#[doc = "`reset()` method sets PKTRX1_PI_R to value 0"]
impl crate::Resettable for Pktrx1PiRSpec {}
