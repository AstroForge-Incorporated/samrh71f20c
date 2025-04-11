#[doc = "Register `IMR` reader"]
pub type R = crate::R<ImrSpec>;
#[doc = "Field `OUTOFRANGE` reader - out of range"]
pub type OutofrangeR = crate::BitReader;
#[doc = "Field `RDERRORACCESS` reader - read access error"]
pub type RderroraccessR = crate::BitReader;
#[doc = "Field `WRERRORACCESS` reader - write access error"]
pub type WrerroraccessR = crate::BitReader;
#[doc = "Field `USERERRORACCESS` reader - user or superuser access error"]
pub type UsererroraccessR = crate::BitReader;
impl R {
    #[doc = "Bit 1 - out of range"]
    #[inline(always)]
    pub fn outofrange(&self) -> OutofrangeR {
        OutofrangeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - read access error"]
    #[inline(always)]
    pub fn rderroraccess(&self) -> RderroraccessR {
        RderroraccessR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - write access error"]
    #[inline(always)]
    pub fn wrerroraccess(&self) -> WrerroraccessR {
        WrerroraccessR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - user or superuser access error"]
    #[inline(always)]
    pub fn usererroraccess(&self) -> UsererroraccessR {
        UsererroraccessR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "HEMC Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImrSpec;
impl crate::RegisterSpec for ImrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for ImrSpec {}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for ImrSpec {}
