#[doc = "Register `HECC_FAILAR` reader"]
pub type R = crate::R<HeccFailarSpec>;
#[doc = "Field `ADDRESS` reader - address of the error detected"]
pub type AddressR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - address of the error detected"]
    #[inline(always)]
    pub fn address(&self) -> AddressR {
        AddressR::new(self.bits)
    }
}
#[doc = "HECC Fail address register\n\nYou can [`read`](crate::Reg::read) this register and get [`hecc_failar::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HeccFailarSpec;
impl crate::RegisterSpec for HeccFailarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hecc_failar::R`](R) reader structure"]
impl crate::Readable for HeccFailarSpec {}
#[doc = "`reset()` method sets HECC_FAILAR to value 0"]
impl crate::Resettable for HeccFailarSpec {}
