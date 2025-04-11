#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `NMI0` writer - Non-maskable Interrupt 0 Enable"]
pub type Nmi0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NMI1` writer - Non-maskable Interrupt 1 Enable"]
pub type Nmi1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NMI2` writer - Non-maskable Interrupt 2 Enable"]
pub type Nmi2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NMI3` writer - Non-maskable Interrupt 3 Enable"]
pub type Nmi3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NMI4` writer - Non-maskable Interrupt 4 Enable"]
pub type Nmi4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NMI5` writer - Non-maskable Interrupt 5 Enable"]
pub type Nmi5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NMI6` writer - Non-maskable Interrupt 6 Enable"]
pub type Nmi6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NMI7` writer - Non-maskable Interrupt 7 Enable"]
pub type Nmi7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NMI8` writer - Non-maskable Interrupt 8 Enable"]
pub type Nmi8W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Non-maskable Interrupt 0 Enable"]
    #[inline(always)]
    pub fn nmi0(&mut self) -> Nmi0W<IerSpec> {
        Nmi0W::new(self, 0)
    }
    #[doc = "Bit 1 - Non-maskable Interrupt 1 Enable"]
    #[inline(always)]
    pub fn nmi1(&mut self) -> Nmi1W<IerSpec> {
        Nmi1W::new(self, 1)
    }
    #[doc = "Bit 2 - Non-maskable Interrupt 2 Enable"]
    #[inline(always)]
    pub fn nmi2(&mut self) -> Nmi2W<IerSpec> {
        Nmi2W::new(self, 2)
    }
    #[doc = "Bit 3 - Non-maskable Interrupt 3 Enable"]
    #[inline(always)]
    pub fn nmi3(&mut self) -> Nmi3W<IerSpec> {
        Nmi3W::new(self, 3)
    }
    #[doc = "Bit 4 - Non-maskable Interrupt 4 Enable"]
    #[inline(always)]
    pub fn nmi4(&mut self) -> Nmi4W<IerSpec> {
        Nmi4W::new(self, 4)
    }
    #[doc = "Bit 5 - Non-maskable Interrupt 5 Enable"]
    #[inline(always)]
    pub fn nmi5(&mut self) -> Nmi5W<IerSpec> {
        Nmi5W::new(self, 5)
    }
    #[doc = "Bit 6 - Non-maskable Interrupt 6 Enable"]
    #[inline(always)]
    pub fn nmi6(&mut self) -> Nmi6W<IerSpec> {
        Nmi6W::new(self, 6)
    }
    #[doc = "Bit 7 - Non-maskable Interrupt 7 Enable"]
    #[inline(always)]
    pub fn nmi7(&mut self) -> Nmi7W<IerSpec> {
        Nmi7W::new(self, 7)
    }
    #[doc = "Bit 8 - Non-maskable Interrupt 8 Enable"]
    #[inline(always)]
    pub fn nmi8(&mut self) -> Nmi8W<IerSpec> {
        Nmi8W::new(self, 8)
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
