#[doc = "Register `IAR` reader"]
pub type R = crate::R<IarSpec>;
#[doc = "Field `NMI0` reader - Active Interrupt 0"]
pub type Nmi0R = crate::BitReader;
#[doc = "Field `NMI1` reader - Active Interrupt 1"]
pub type Nmi1R = crate::BitReader;
#[doc = "Field `NMI2` reader - Active Interrupt 2"]
pub type Nmi2R = crate::BitReader;
#[doc = "Field `NMI3` reader - Active Interrupt 3"]
pub type Nmi3R = crate::BitReader;
#[doc = "Field `NMI4` reader - Active Interrupt 4"]
pub type Nmi4R = crate::BitReader;
#[doc = "Field `NMI5` reader - Active Interrupt 5"]
pub type Nmi5R = crate::BitReader;
#[doc = "Field `NMI6` reader - Active Interrupt 6"]
pub type Nmi6R = crate::BitReader;
#[doc = "Field `NMI7` reader - Active Interrupt 7"]
pub type Nmi7R = crate::BitReader;
#[doc = "Field `NMI8` reader - Active Interrupt 8"]
pub type Nmi8R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Active Interrupt 0"]
    #[inline(always)]
    pub fn nmi0(&self) -> Nmi0R {
        Nmi0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Active Interrupt 1"]
    #[inline(always)]
    pub fn nmi1(&self) -> Nmi1R {
        Nmi1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Active Interrupt 2"]
    #[inline(always)]
    pub fn nmi2(&self) -> Nmi2R {
        Nmi2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Active Interrupt 3"]
    #[inline(always)]
    pub fn nmi3(&self) -> Nmi3R {
        Nmi3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Active Interrupt 4"]
    #[inline(always)]
    pub fn nmi4(&self) -> Nmi4R {
        Nmi4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Active Interrupt 5"]
    #[inline(always)]
    pub fn nmi5(&self) -> Nmi5R {
        Nmi5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Active Interrupt 6"]
    #[inline(always)]
    pub fn nmi6(&self) -> Nmi6R {
        Nmi6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Active Interrupt 7"]
    #[inline(always)]
    pub fn nmi7(&self) -> Nmi7R {
        Nmi7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Active Interrupt 8"]
    #[inline(always)]
    pub fn nmi8(&self) -> Nmi8R {
        Nmi8R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Interrupt Active Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iar::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IarSpec;
impl crate::RegisterSpec for IarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iar::R`](R) reader structure"]
impl crate::Readable for IarSpec {}
#[doc = "`reset()` method sets IAR to value 0"]
impl crate::Resettable for IarSpec {}
