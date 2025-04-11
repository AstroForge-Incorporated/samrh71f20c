#[doc = "Register `GROUP_IRQSTS2` reader"]
pub type R = crate::R<GroupIrqsts2Spec>;
#[doc = "Field `Link2` reader - Link 2"]
pub type Link2R = crate::BitReader;
#[doc = "Field `Dia2` reader - Distributed Interrupt Acknowledge, Link 2"]
pub type Dia2R = crate::BitReader;
#[doc = "Field `Di2` reader - Distributed Interrupt 2"]
pub type Di2R = crate::BitReader;
#[doc = "Field `Link1` reader - Link 1"]
pub type Link1R = crate::BitReader;
#[doc = "Field `Dia1` reader - Distributed Interrupt Acknowledge, Link 1"]
pub type Dia1R = crate::BitReader;
#[doc = "Field `Di1` reader - Distributed Interrupt 1"]
pub type Di1R = crate::BitReader;
impl R {
    #[doc = "Bit 18 - Link 2"]
    #[inline(always)]
    pub fn link2(&self) -> Link2R {
        Link2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Distributed Interrupt Acknowledge, Link 2"]
    #[inline(always)]
    pub fn dia2(&self) -> Dia2R {
        Dia2R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Distributed Interrupt 2"]
    #[inline(always)]
    pub fn di2(&self) -> Di2R {
        Di2R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Link 1"]
    #[inline(always)]
    pub fn link1(&self) -> Link1R {
        Link1R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Distributed Interrupt Acknowledge, Link 1"]
    #[inline(always)]
    pub fn dia1(&self) -> Dia1R {
        Dia1R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Distributed Interrupt 1"]
    #[inline(always)]
    pub fn di1(&self) -> Di1R {
        Di1R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "SpW Group Interrupt status 2\n\nYou can [`read`](crate::Reg::read) this register and get [`group_irqsts2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GroupIrqsts2Spec;
impl crate::RegisterSpec for GroupIrqsts2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`group_irqsts2::R`](R) reader structure"]
impl crate::Readable for GroupIrqsts2Spec {}
#[doc = "`reset()` method sets GROUP_IRQSTS2 to value 0"]
impl crate::Resettable for GroupIrqsts2Spec {}
