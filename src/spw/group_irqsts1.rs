#[doc = "Register `GROUP_IRQSTS1` reader"]
pub type R = crate::R<GroupIrqsts1Spec>;
#[doc = "Field `TX1` reader - Tx 1"]
pub type Tx1R = crate::BitReader;
#[doc = "Field `RX1` reader - Rx 1"]
pub type Rx1R = crate::BitReader;
#[doc = "Field `TCH` reader - Time Code Handler"]
pub type TchR = crate::BitReader;
impl R {
    #[doc = "Bit 7 - Tx 1"]
    #[inline(always)]
    pub fn tx1(&self) -> Tx1R {
        Tx1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - Rx 1"]
    #[inline(always)]
    pub fn rx1(&self) -> Rx1R {
        Rx1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Time Code Handler"]
    #[inline(always)]
    pub fn tch(&self) -> TchR {
        TchR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "SpW Group Interrupt status 1\n\nYou can [`read`](crate::Reg::read) this register and get [`group_irqsts1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GroupIrqsts1Spec;
impl crate::RegisterSpec for GroupIrqsts1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`group_irqsts1::R`](R) reader structure"]
impl crate::Readable for GroupIrqsts1Spec {}
#[doc = "`reset()` method sets GROUP_IRQSTS1 to value 0"]
impl crate::Resettable for GroupIrqsts1Spec {}
