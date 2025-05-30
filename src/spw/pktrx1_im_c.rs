#[doc = "Register `PKTRX1_IM_C` writer"]
pub type W = crate::W<Pktrx1ImCSpec>;
#[doc = "Field `DEACT` writer - Deactivated"]
pub type DeactW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOP` writer - EOP seen"]
pub type EopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EEP` writer - EEP seen"]
pub type EepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCARD` writer - Packet Discard"]
pub type DiscardW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACT` writer - Activated"]
pub type ActW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Deactivated"]
    #[inline(always)]
    pub fn deact(&mut self) -> DeactW<Pktrx1ImCSpec> {
        DeactW::new(self, 0)
    }
    #[doc = "Bit 1 - EOP seen"]
    #[inline(always)]
    pub fn eop(&mut self) -> EopW<Pktrx1ImCSpec> {
        EopW::new(self, 1)
    }
    #[doc = "Bit 2 - EEP seen"]
    #[inline(always)]
    pub fn eep(&mut self) -> EepW<Pktrx1ImCSpec> {
        EepW::new(self, 2)
    }
    #[doc = "Bit 3 - Packet Discard"]
    #[inline(always)]
    pub fn discard(&mut self) -> DiscardW<Pktrx1ImCSpec> {
        DiscardW::new(self, 3)
    }
    #[doc = "Bit 4 - Activated"]
    #[inline(always)]
    pub fn act(&mut self) -> ActW<Pktrx1ImCSpec> {
        ActW::new(self, 4)
    }
}
#[doc = "PktRx Interrupt Clear Mask\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pktrx1_im_c::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pktrx1ImCSpec;
impl crate::RegisterSpec for Pktrx1ImCSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pktrx1_im_c::W`](W) writer structure"]
impl crate::Writable for Pktrx1ImCSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PKTRX1_IM_C to value 0"]
impl crate::Resettable for Pktrx1ImCSpec {}
