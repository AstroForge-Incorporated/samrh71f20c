#[doc = "Register `PKTRX1_IM` reader"]
pub type R = crate::R<Pktrx1ImSpec>;
#[doc = "Register `PKTRX1_IM` writer"]
pub type W = crate::W<Pktrx1ImSpec>;
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
    pub fn deact(&mut self) -> DeactW<Pktrx1ImSpec> {
        DeactW::new(self, 0)
    }
    #[doc = "Bit 1 - EOP seen"]
    #[inline(always)]
    pub fn eop(&mut self) -> EopW<Pktrx1ImSpec> {
        EopW::new(self, 1)
    }
    #[doc = "Bit 2 - EEP seen"]
    #[inline(always)]
    pub fn eep(&mut self) -> EepW<Pktrx1ImSpec> {
        EepW::new(self, 2)
    }
    #[doc = "Bit 3 - Packet Discard"]
    #[inline(always)]
    pub fn discard(&mut self) -> DiscardW<Pktrx1ImSpec> {
        DiscardW::new(self, 3)
    }
    #[doc = "Bit 4 - Activated"]
    #[inline(always)]
    pub fn act(&mut self) -> ActW<Pktrx1ImSpec> {
        ActW::new(self, 4)
    }
}
#[doc = "PktRx Interrupt Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`pktrx1_im::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pktrx1_im::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pktrx1ImSpec;
impl crate::RegisterSpec for Pktrx1ImSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pktrx1_im::R`](R) reader structure"]
impl crate::Readable for Pktrx1ImSpec {}
#[doc = "`write(|w| ..)` method takes [`pktrx1_im::W`](W) writer structure"]
impl crate::Writable for Pktrx1ImSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PKTRX1_IM to value 0"]
impl crate::Resettable for Pktrx1ImSpec {}
