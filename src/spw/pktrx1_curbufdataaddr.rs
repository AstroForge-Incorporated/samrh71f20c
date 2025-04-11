#[doc = "Register `PKTRX1_CURBUFDATAADDR` reader"]
pub type R = crate::R<Pktrx1CurbufdataaddrSpec>;
#[doc = "Field `ADDR` reader - Address"]
pub type AddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
#[doc = "PktRx Current Buffer Data Address\n\nYou can [`read`](crate::Reg::read) this register and get [`pktrx1_curbufdataaddr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pktrx1CurbufdataaddrSpec;
impl crate::RegisterSpec for Pktrx1CurbufdataaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pktrx1_curbufdataaddr::R`](R) reader structure"]
impl crate::Readable for Pktrx1CurbufdataaddrSpec {}
#[doc = "`reset()` method sets PKTRX1_CURBUFDATAADDR to value 0"]
impl crate::Resettable for Pktrx1CurbufdataaddrSpec {}
