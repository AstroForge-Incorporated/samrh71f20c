#[doc = "Register `PKTTX1_PI_C` writer"]
pub type W = crate::W<Pkttx1PiCSpec>;
#[doc = "Field `DEACT` writer - Deactivated"]
pub type DeactW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACT` writer - Activated"]
pub type ActW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOP` writer - EOP sent"]
pub type EopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EEP` writer - EEP sent"]
pub type EepW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Deactivated"]
    #[inline(always)]
    pub fn deact(&mut self) -> DeactW<Pkttx1PiCSpec> {
        DeactW::new(self, 0)
    }
    #[doc = "Bit 1 - Activated"]
    #[inline(always)]
    pub fn act(&mut self) -> ActW<Pkttx1PiCSpec> {
        ActW::new(self, 1)
    }
    #[doc = "Bit 2 - EOP sent"]
    #[inline(always)]
    pub fn eop(&mut self) -> EopW<Pkttx1PiCSpec> {
        EopW::new(self, 2)
    }
    #[doc = "Bit 3 - EEP sent"]
    #[inline(always)]
    pub fn eep(&mut self) -> EepW<Pkttx1PiCSpec> {
        EepW::new(self, 3)
    }
}
#[doc = "PktTx Clear Pending Interrupt\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkttx1_pi_c::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pkttx1PiCSpec;
impl crate::RegisterSpec for Pkttx1PiCSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pkttx1_pi_c::W`](W) writer structure"]
impl crate::Writable for Pkttx1PiCSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PKTTX1_PI_C to value 0"]
impl crate::Resettable for Pkttx1PiCSpec {}
