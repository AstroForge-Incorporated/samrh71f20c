#[doc = "Register `PKTTX1_IM_C` writer"]
pub type W = crate::W<Pkttx1ImCSpec>;
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
    pub fn deact(&mut self) -> DeactW<Pkttx1ImCSpec> {
        DeactW::new(self, 0)
    }
    #[doc = "Bit 1 - Activated"]
    #[inline(always)]
    pub fn act(&mut self) -> ActW<Pkttx1ImCSpec> {
        ActW::new(self, 1)
    }
    #[doc = "Bit 2 - EOP sent"]
    #[inline(always)]
    pub fn eop(&mut self) -> EopW<Pkttx1ImCSpec> {
        EopW::new(self, 2)
    }
    #[doc = "Bit 3 - EEP sent"]
    #[inline(always)]
    pub fn eep(&mut self) -> EepW<Pkttx1ImCSpec> {
        EepW::new(self, 3)
    }
}
#[doc = "PktTx Interrupt Clear Mask\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkttx1_im_c::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pkttx1ImCSpec;
impl crate::RegisterSpec for Pkttx1ImCSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pkttx1_im_c::W`](W) writer structure"]
impl crate::Writable for Pkttx1ImCSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PKTTX1_IM_C to value 0"]
impl crate::Resettable for Pkttx1ImCSpec {}
