#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `DATRDY` reader - Data Ready (cleared on read)"]
pub type DatrdyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Data Ready (cleared on read)"]
    #[inline(always)]
    pub fn datrdy(&self) -> DatrdyR {
        DatrdyR::new((self.bits & 1) != 0)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {}
