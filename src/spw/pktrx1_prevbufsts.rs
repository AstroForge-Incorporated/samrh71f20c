#[doc = "Register `PKTRX1_PREVBUFSTS` reader"]
pub type R = crate::R<Pktrx1PrevbufstsSpec>;
#[doc = "Field `CNT` reader - Count"]
pub type CntR = crate::FieldReader<u16>;
#[doc = "Field `EEP` reader - EEP seen"]
pub type EepR = crate::BitReader;
#[doc = "Field `FULLI` reader - Buffer Info Full"]
pub type FulliR = crate::BitReader;
#[doc = "Field `FULLD` reader - Buffer Data Full"]
pub type FulldR = crate::BitReader;
#[doc = "Field `DMAERR` reader - DMA Error"]
pub type DmaerrR = crate::BitReader;
#[doc = "Field `LOCKED` reader - Locked"]
pub type LockedR = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - Count"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - EEP seen"]
    #[inline(always)]
    pub fn eep(&self) -> EepR {
        EepR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Buffer Info Full"]
    #[inline(always)]
    pub fn fulli(&self) -> FulliR {
        FulliR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Buffer Data Full"]
    #[inline(always)]
    pub fn fulld(&self) -> FulldR {
        FulldR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DMA Error"]
    #[inline(always)]
    pub fn dmaerr(&self) -> DmaerrR {
        DmaerrR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 31 - Locked"]
    #[inline(always)]
    pub fn locked(&self) -> LockedR {
        LockedR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "PktRx Previous Buffer Status\n\nYou can [`read`](crate::Reg::read) this register and get [`pktrx1_prevbufsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pktrx1PrevbufstsSpec;
impl crate::RegisterSpec for Pktrx1PrevbufstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pktrx1_prevbufsts::R`](R) reader structure"]
impl crate::Readable for Pktrx1PrevbufstsSpec {}
#[doc = "`reset()` method sets PKTRX1_PREVBUFSTS to value 0"]
impl crate::Resettable for Pktrx1PrevbufstsSpec {}
