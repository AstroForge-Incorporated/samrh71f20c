#[doc = "Register `FAILARD` reader"]
pub type R = crate::R<FailardSpec>;
#[doc = "Field `ADDRESS` reader - address of the error detected"]
pub type AddressR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - address of the error detected"]
    #[inline(always)]
    pub fn address(&self) -> AddressR {
        AddressR::new(self.bits)
    }
}
#[doc = "TCMECC Fail address register data\n\nYou can [`read`](crate::Reg::read) this register and get [`failard::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FailardSpec;
impl crate::RegisterSpec for FailardSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`failard::R`](R) reader structure"]
impl crate::Readable for FailardSpec {}
#[doc = "`reset()` method sets FAILARD to value 0"]
impl crate::Resettable for FailardSpec {}
