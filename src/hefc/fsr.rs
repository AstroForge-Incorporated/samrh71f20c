#[doc = "Register `FSR` reader"]
pub type R = crate::R<FsrSpec>;
#[doc = "Field `FRDY` reader - Flash Ready Status (cleared when Flash is busy)"]
pub type FrdyR = crate::BitReader;
#[doc = "Field `FCMDE` reader - Flash Command Error Status (cleared on read or by writing HEFC_FCR)"]
pub type FcmdeR = crate::BitReader;
#[doc = "Field `FLOCKE` reader - Flash Lock Error Status (cleared on read)"]
pub type FlockeR = crate::BitReader;
#[doc = "Field `WREER` reader - Write Register Error Status (cleared on read)"]
pub type WreerR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Flash Ready Status (cleared when Flash is busy)"]
    #[inline(always)]
    pub fn frdy(&self) -> FrdyR {
        FrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Flash Command Error Status (cleared on read or by writing HEFC_FCR)"]
    #[inline(always)]
    pub fn fcmde(&self) -> FcmdeR {
        FcmdeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Flash Lock Error Status (cleared on read)"]
    #[inline(always)]
    pub fn flocke(&self) -> FlockeR {
        FlockeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Write Register Error Status (cleared on read)"]
    #[inline(always)]
    pub fn wreer(&self) -> WreerR {
        WreerR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "HEFC Flash Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsrSpec;
impl crate::RegisterSpec for FsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsr::R`](R) reader structure"]
impl crate::Readable for FsrSpec {}
#[doc = "`reset()` method sets FSR to value 0"]
impl crate::Resettable for FsrSpec {}
