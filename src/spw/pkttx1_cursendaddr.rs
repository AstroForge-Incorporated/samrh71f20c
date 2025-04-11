#[doc = "Register `PKTTX1_CURSENDADDR` reader"]
pub type R = crate::R<Pkttx1CursendaddrSpec>;
#[doc = "Field `ADDR` reader - Address"]
pub type AddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
#[doc = "PktTx Current Send List Address\n\nYou can [`read`](crate::Reg::read) this register and get [`pkttx1_cursendaddr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pkttx1CursendaddrSpec;
impl crate::RegisterSpec for Pkttx1CursendaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pkttx1_cursendaddr::R`](R) reader structure"]
impl crate::Readable for Pkttx1CursendaddrSpec {}
#[doc = "`reset()` method sets PKTTX1_CURSENDADDR to value 0"]
impl crate::Resettable for Pkttx1CursendaddrSpec {}
