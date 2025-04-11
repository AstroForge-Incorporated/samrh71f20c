#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `NMI0` reader - Non-maskable Interrupt Source 0 Pending (cleared on read)"]
pub type Nmi0R = crate::BitReader;
#[doc = "Field `NMI1` reader - Non-maskable Interrupt Source 1 Pending (cleared on read)"]
pub type Nmi1R = crate::BitReader;
#[doc = "Field `NMI2` reader - Non-maskable Interrupt Source 2 Pending (cleared on read)"]
pub type Nmi2R = crate::BitReader;
#[doc = "Field `NMI3` reader - Non-maskable Interrupt Source 3 Pending (cleared on read)"]
pub type Nmi3R = crate::BitReader;
#[doc = "Field `NMI4` reader - Non-maskable Interrupt Source 4 Pending (cleared on read)"]
pub type Nmi4R = crate::BitReader;
#[doc = "Field `NMI5` reader - Non-maskable Interrupt Source 5 Pending (cleared on read)"]
pub type Nmi5R = crate::BitReader;
#[doc = "Field `NMI6` reader - Non-maskable Interrupt Source 6 Pending (cleared on read)"]
pub type Nmi6R = crate::BitReader;
#[doc = "Field `NMI7` reader - Non-maskable Interrupt Source 7 Pending (cleared on read)"]
pub type Nmi7R = crate::BitReader;
#[doc = "Field `NMI8` reader - Non-maskable Interrupt Source 8 Pending (cleared on read)"]
pub type Nmi8R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Non-maskable Interrupt Source 0 Pending (cleared on read)"]
    #[inline(always)]
    pub fn nmi0(&self) -> Nmi0R {
        Nmi0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Non-maskable Interrupt Source 1 Pending (cleared on read)"]
    #[inline(always)]
    pub fn nmi1(&self) -> Nmi1R {
        Nmi1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Non-maskable Interrupt Source 2 Pending (cleared on read)"]
    #[inline(always)]
    pub fn nmi2(&self) -> Nmi2R {
        Nmi2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Non-maskable Interrupt Source 3 Pending (cleared on read)"]
    #[inline(always)]
    pub fn nmi3(&self) -> Nmi3R {
        Nmi3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Non-maskable Interrupt Source 4 Pending (cleared on read)"]
    #[inline(always)]
    pub fn nmi4(&self) -> Nmi4R {
        Nmi4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Non-maskable Interrupt Source 5 Pending (cleared on read)"]
    #[inline(always)]
    pub fn nmi5(&self) -> Nmi5R {
        Nmi5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Non-maskable Interrupt Source 6 Pending (cleared on read)"]
    #[inline(always)]
    pub fn nmi6(&self) -> Nmi6R {
        Nmi6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Non-maskable Interrupt Source 7 Pending (cleared on read)"]
    #[inline(always)]
    pub fn nmi7(&self) -> Nmi7R {
        Nmi7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Non-maskable Interrupt Source 8 Pending (cleared on read)"]
    #[inline(always)]
    pub fn nmi8(&self) -> Nmi8R {
        Nmi8R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {}
