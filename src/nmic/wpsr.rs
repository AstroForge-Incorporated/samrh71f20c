#[doc = "Register `WPSR` reader"]
pub type R = crate::R<WpsrSpec>;
#[doc = "Field `WPVS` reader - Write Protection Register Violation Status"]
pub type WpvsR = crate::BitReader;
#[doc = "Field `FZWVS` reader - Frozen Register Write Violation Status"]
pub type FzwvsR = crate::BitReader;
#[doc = "Field `BSWVS` reader - Busy Register Write Violation Status"]
pub type BswvsR = crate::BitReader;
#[doc = "Field `WVSRC` reader - Write Violation Source"]
pub type WvsrcR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Write Protection Register Violation Status"]
    #[inline(always)]
    pub fn wpvs(&self) -> WpvsR {
        WpvsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Frozen Register Write Violation Status"]
    #[inline(always)]
    pub fn fzwvs(&self) -> FzwvsR {
        FzwvsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Busy Register Write Violation Status"]
    #[inline(always)]
    pub fn bswvs(&self) -> BswvsR {
        BswvsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Write Violation Source"]
    #[inline(always)]
    pub fn wvsrc(&self) -> WvsrcR {
        WvsrcR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Write Protection Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WpsrSpec;
impl crate::RegisterSpec for WpsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wpsr::R`](R) reader structure"]
impl crate::Readable for WpsrSpec {}
#[doc = "`reset()` method sets WPSR to value 0"]
impl crate::Resettable for WpsrSpec {}
