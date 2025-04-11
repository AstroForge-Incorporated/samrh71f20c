#[doc = "Register `PKTRX1_CURBUFPKTADDR` reader"]
pub type R = crate::R<Pktrx1CurbufpktaddrSpec>;
#[doc = "Field `ADDR` reader - Address"]
pub type AddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
#[doc = "PktRx Current Buffer Packet Address\n\nYou can [`read`](crate::Reg::read) this register and get [`pktrx1_curbufpktaddr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pktrx1CurbufpktaddrSpec;
impl crate::RegisterSpec for Pktrx1CurbufpktaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pktrx1_curbufpktaddr::R`](R) reader structure"]
impl crate::Readable for Pktrx1CurbufpktaddrSpec {}
#[doc = "`reset()` method sets PKTRX1_CURBUFPKTADDR to value 0"]
impl crate::Resettable for Pktrx1CurbufpktaddrSpec {}
