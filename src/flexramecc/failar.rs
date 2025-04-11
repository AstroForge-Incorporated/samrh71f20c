#[doc = "Register `FAILAR` reader"]
pub type R = crate::R<FailarSpec>;
#[doc = "Field `ADDRESS` reader - address of the error detected"]
pub type AddressR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - address of the error detected"]
    #[inline(always)]
    pub fn address(&self) -> AddressR {
        AddressR::new(self.bits)
    }
}
#[doc = "FLEXRAMECC Fail address register\n\nYou can [`read`](crate::Reg::read) this register and get [`failar::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FailarSpec;
impl crate::RegisterSpec for FailarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`failar::R`](R) reader structure"]
impl crate::Readable for FailarSpec {}
#[doc = "`reset()` method sets FAILAR to value 0"]
impl crate::Resettable for FailarSpec {}
