#[doc = "Register `PKTRX1_CURBUFDATALEN` reader"]
pub type R = crate::R<Pktrx1CurbufdatalenSpec>;
#[doc = "Field `LEN` reader - Length"]
pub type LenR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - Length"]
    #[inline(always)]
    pub fn len(&self) -> LenR {
        LenR::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "PktRx Current Buffer Data Length\n\nYou can [`read`](crate::Reg::read) this register and get [`pktrx1_curbufdatalen::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pktrx1CurbufdatalenSpec;
impl crate::RegisterSpec for Pktrx1CurbufdatalenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pktrx1_curbufdatalen::R`](R) reader structure"]
impl crate::Readable for Pktrx1CurbufdatalenSpec {}
#[doc = "`reset()` method sets PKTRX1_CURBUFDATALEN to value 0"]
impl crate::Resettable for Pktrx1CurbufdatalenSpec {}
