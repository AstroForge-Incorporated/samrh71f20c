#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `OUTOFRANGE` reader - out of range"]
pub type OutofrangeR = crate::BitReader;
#[doc = "Field `RDERRORACCESS` reader - Read access error"]
pub type RderroraccessR = crate::BitReader;
#[doc = "Field `WRERRORACCESS` reader - Write access error"]
pub type WrerroraccessR = crate::BitReader;
#[doc = "Field `USERERRORACCESS` reader - User or SuperUser Error Access"]
pub type UsererroraccessR = crate::BitReader;
impl R {
    #[doc = "Bit 1 - out of range"]
    #[inline(always)]
    pub fn outofrange(&self) -> OutofrangeR {
        OutofrangeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Read access error"]
    #[inline(always)]
    pub fn rderroraccess(&self) -> RderroraccessR {
        RderroraccessR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write access error"]
    #[inline(always)]
    pub fn wrerroraccess(&self) -> WrerroraccessR {
        WrerroraccessR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - User or SuperUser Error Access"]
    #[inline(always)]
    pub fn usererroraccess(&self) -> UsererroraccessR {
        UsererroraccessR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "HEMC Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {}
