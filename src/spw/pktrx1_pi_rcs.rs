#[doc = "Register `PKTRX1_PI_RCS` reader"]
pub type R = crate::R<Pktrx1PiRcsSpec>;
#[doc = "Register `PKTRX1_PI_RCS` writer"]
pub type W = crate::W<Pktrx1PiRcsSpec>;
#[doc = "Field `DEACT` reader - Deactivated"]
pub type DeactR = crate::BitReader;
#[doc = "Field `DEACT` writer - Deactivated"]
pub type DeactW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOP` reader - EOP seen"]
pub type EopR = crate::BitReader;
#[doc = "Field `EOP` writer - EOP seen"]
pub type EopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EEP` reader - EEP seen"]
pub type EepR = crate::BitReader;
#[doc = "Field `EEP` writer - EEP seen"]
pub type EepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCARD` reader - Packet Discard"]
pub type DiscardR = crate::BitReader;
#[doc = "Field `DISCARD` writer - Packet Discard"]
pub type DiscardW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACT` reader - Activated"]
pub type ActR = crate::BitReader;
#[doc = "Field `ACT` writer - Activated"]
pub type ActW<'a, REG> = crate::BitWriter<'a, REG>;
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
impl W {
    #[doc = "Bit 0 - Deactivated"]
    #[inline(always)]
    pub fn deact(&mut self) -> DeactW<Pktrx1PiRcsSpec> {
        DeactW::new(self, 0)
    }
    #[doc = "Bit 1 - EOP seen"]
    #[inline(always)]
    pub fn eop(&mut self) -> EopW<Pktrx1PiRcsSpec> {
        EopW::new(self, 1)
    }
    #[doc = "Bit 2 - EEP seen"]
    #[inline(always)]
    pub fn eep(&mut self) -> EepW<Pktrx1PiRcsSpec> {
        EepW::new(self, 2)
    }
    #[doc = "Bit 3 - Packet Discard"]
    #[inline(always)]
    pub fn discard(&mut self) -> DiscardW<Pktrx1PiRcsSpec> {
        DiscardW::new(self, 3)
    }
    #[doc = "Bit 4 - Activated"]
    #[inline(always)]
    pub fn act(&mut self) -> ActW<Pktrx1PiRcsSpec> {
        ActW::new(self, 4)
    }
}
#[doc = "PktRx Pending Read, Clear and Enabled Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`pktrx1_pi_rcs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pktrx1_pi_rcs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pktrx1PiRcsSpec;
impl crate::RegisterSpec for Pktrx1PiRcsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pktrx1_pi_rcs::R`](R) reader structure"]
impl crate::Readable for Pktrx1PiRcsSpec {}
#[doc = "`write(|w| ..)` method takes [`pktrx1_pi_rcs::W`](W) writer structure"]
impl crate::Writable for Pktrx1PiRcsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PKTRX1_PI_RCS to value 0"]
impl crate::Resettable for Pktrx1PiRcsSpec {}
