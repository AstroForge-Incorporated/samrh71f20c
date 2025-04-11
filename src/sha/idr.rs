#[doc = "Register `IDR` writer"]
pub type W = crate::W<IdrSpec>;
#[doc = "Field `DATRDY` writer - Data Ready Interrupt Disable"]
pub type DatrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `URAD` writer - Unspecified Register Access Detection Interrupt Disable"]
pub type UradW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHECKF` writer - Check Done Interrupt Disable"]
pub type CheckfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECE` writer - Safety Event Interrupt Disable"]
pub type SeceW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Data Ready Interrupt Disable"]
    #[inline(always)]
    pub fn datrdy(&mut self) -> DatrdyW<IdrSpec> {
        DatrdyW::new(self, 0)
    }
    #[doc = "Bit 8 - Unspecified Register Access Detection Interrupt Disable"]
    #[inline(always)]
    pub fn urad(&mut self) -> UradW<IdrSpec> {
        UradW::new(self, 8)
    }
    #[doc = "Bit 16 - Check Done Interrupt Disable"]
    #[inline(always)]
    pub fn checkf(&mut self) -> CheckfW<IdrSpec> {
        CheckfW::new(self, 16)
    }
    #[doc = "Bit 24 - Safety Event Interrupt Disable"]
    #[inline(always)]
    pub fn sece(&mut self) -> SeceW<IdrSpec> {
        SeceW::new(self, 24)
    }
}
#[doc = "Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdrSpec;
impl crate::RegisterSpec for IdrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`idr::W`](W) writer structure"]
impl crate::Writable for IdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDR to value 0"]
impl crate::Resettable for IdrSpec {}
