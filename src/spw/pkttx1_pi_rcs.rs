#[doc = "Register `PKTTX1_PI_RCS` reader"]
pub type R = crate::R<Pkttx1PiRcsSpec>;
#[doc = "Register `PKTTX1_PI_RCS` writer"]
pub type W = crate::W<Pkttx1PiRcsSpec>;
#[doc = "Field `DEACT` reader - Deactivated"]
pub type DeactR = crate::BitReader;
#[doc = "Field `DEACT` writer - Deactivated"]
pub type DeactW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACT` reader - Activated"]
pub type ActR = crate::BitReader;
#[doc = "Field `ACT` writer - Activated"]
pub type ActW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOP` reader - EOP sent"]
pub type EopR = crate::BitReader;
#[doc = "Field `EOP` writer - EOP sent"]
pub type EopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EEP` reader - EEP sent"]
pub type EepR = crate::BitReader;
#[doc = "Field `EEP` writer - EEP sent"]
pub type EepW<'a, REG> = crate::BitWriter<'a, REG>;
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
impl W {
    #[doc = "Bit 0 - Deactivated"]
    #[inline(always)]
    pub fn deact(&mut self) -> DeactW<Pkttx1PiRcsSpec> {
        DeactW::new(self, 0)
    }
    #[doc = "Bit 1 - Activated"]
    #[inline(always)]
    pub fn act(&mut self) -> ActW<Pkttx1PiRcsSpec> {
        ActW::new(self, 1)
    }
    #[doc = "Bit 2 - EOP sent"]
    #[inline(always)]
    pub fn eop(&mut self) -> EopW<Pkttx1PiRcsSpec> {
        EopW::new(self, 2)
    }
    #[doc = "Bit 3 - EEP sent"]
    #[inline(always)]
    pub fn eep(&mut self) -> EepW<Pkttx1PiRcsSpec> {
        EepW::new(self, 3)
    }
}
#[doc = "PktTx Pending Read, Clear and Enabled Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`pkttx1_pi_rcs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkttx1_pi_rcs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pkttx1PiRcsSpec;
impl crate::RegisterSpec for Pkttx1PiRcsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pkttx1_pi_rcs::R`](R) reader structure"]
impl crate::Readable for Pkttx1PiRcsSpec {}
#[doc = "`write(|w| ..)` method takes [`pkttx1_pi_rcs::W`](W) writer structure"]
impl crate::Writable for Pkttx1PiRcsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PKTTX1_PI_RCS to value 0"]
impl crate::Resettable for Pkttx1PiRcsSpec {}
