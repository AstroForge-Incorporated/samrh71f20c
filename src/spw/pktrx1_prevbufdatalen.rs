#[doc = "Register `PKTRX1_PREVBUFDATALEN` reader"]
pub type R = crate::R<Pktrx1PrevbufdatalenSpec>;
#[doc = "Field `LEN` reader - Length"]
pub type LenR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - Length"]
    #[inline(always)]
    pub fn len(&self) -> LenR {
        LenR::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "PktRx Previous Buffer Data Length\n\nYou can [`read`](crate::Reg::read) this register and get [`pktrx1_prevbufdatalen::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pktrx1PrevbufdatalenSpec;
impl crate::RegisterSpec for Pktrx1PrevbufdatalenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pktrx1_prevbufdatalen::R`](R) reader structure"]
impl crate::Readable for Pktrx1PrevbufdatalenSpec {}
#[doc = "`reset()` method sets PKTRX1_PREVBUFDATALEN to value 0"]
impl crate::Resettable for Pktrx1PrevbufdatalenSpec {}
