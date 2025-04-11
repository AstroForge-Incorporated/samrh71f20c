#[doc = "Register `PKTTX1_PI_R` reader"]
pub type R = crate::R<Pkttx1PiRSpec>;
#[doc = "Field `DEACT` reader - Deactivated"]
pub type DeactR = crate::BitReader;
#[doc = "Field `ACT` reader - Activated"]
pub type ActR = crate::BitReader;
#[doc = "Field `EOP` reader - EOP sent"]
pub type EopR = crate::BitReader;
#[doc = "Field `EEP` reader - EEP sent"]
pub type EepR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Deactivated"]
    #[inline(always)]
    pub fn deact(&self) -> DeactR {
        DeactR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Activated"]
    #[inline(always)]
    pub fn act(&self) -> ActR {
        ActR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EOP sent"]
    #[inline(always)]
    pub fn eop(&self) -> EopR {
        EopR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EEP sent"]
    #[inline(always)]
    pub fn eep(&self) -> EepR {
        EepR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "PktTx Pending Read Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`pkttx1_pi_r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pkttx1PiRSpec;
impl crate::RegisterSpec for Pkttx1PiRSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pkttx1_pi_r::R`](R) reader structure"]
impl crate::Readable for Pkttx1PiRSpec {}
#[doc = "`reset()` method sets PKTTX1_PI_R to value 0"]
impl crate::Resettable for Pkttx1PiRSpec {}
