#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `ACKEN` writer - Acknowledge Update Interrupt Enable"]
pub type AckenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALREN` writer - Alarm Interrupt Enable"]
pub type AlrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECEN` writer - Second Event Interrupt Enable"]
pub type SecenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEN` writer - Time Event Interrupt Enable"]
pub type TimenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALEN` writer - Calendar Event Interrupt Enable"]
pub type CalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDERREN` writer - Time and/or Date Error Interrupt Enable"]
pub type TderrenW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Acknowledge Update Interrupt Enable"]
    #[inline(always)]
    pub fn acken(&mut self) -> AckenW<IerSpec> {
        AckenW::new(self, 0)
    }
    #[doc = "Bit 1 - Alarm Interrupt Enable"]
    #[inline(always)]
    pub fn alren(&mut self) -> AlrenW<IerSpec> {
        AlrenW::new(self, 1)
    }
    #[doc = "Bit 2 - Second Event Interrupt Enable"]
    #[inline(always)]
    pub fn secen(&mut self) -> SecenW<IerSpec> {
        SecenW::new(self, 2)
    }
    #[doc = "Bit 3 - Time Event Interrupt Enable"]
    #[inline(always)]
    pub fn timen(&mut self) -> TimenW<IerSpec> {
        TimenW::new(self, 3)
    }
    #[doc = "Bit 4 - Calendar Event Interrupt Enable"]
    #[inline(always)]
    pub fn calen(&mut self) -> CalenW<IerSpec> {
        CalenW::new(self, 4)
    }
    #[doc = "Bit 5 - Time and/or Date Error Interrupt Enable"]
    #[inline(always)]
    pub fn tderren(&mut self) -> TderrenW<IerSpec> {
        TderrenW::new(self, 5)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IerSpec {}
