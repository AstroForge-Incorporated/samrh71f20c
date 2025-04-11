#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `DATRDY` reader - Data Ready (cleared by writing a 1 to bit SWRST or START in SHA_CR, or by reading SHA_IODATARx)"]
pub type DatrdyR = crate::BitReader;
#[doc = "Field `WRDY` reader - Input Data Register Write Ready"]
pub type WrdyR = crate::BitReader;
#[doc = "Field `URAD` reader - Unspecified Register Access Detection Status (cleared by writing a 1 to SWRST bit in SHA_CR)"]
pub type UradR = crate::BitReader;
#[doc = "Field `URAT` reader - Unspecified Register Access Type (cleared by writing a 1 to SWRST bit in SHA_CR)"]
pub type UratR = crate::FieldReader;
#[doc = "Field `CHECKF` reader - Check Done Status (cleared by writing START or SWRST bits in SHA_CR or by reading SHA_IODATARx)"]
pub type CheckfR = crate::BitReader;
#[doc = "Field `CHKST` reader - Check Status (cleared by writing START or SWRST bits in SHA_CR or by reading SHA_IODATARx)"]
pub type ChkstR = crate::FieldReader;
#[doc = "Field `SECE` reader - Security and/or Safety Event"]
pub type SeceR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Data Ready (cleared by writing a 1 to bit SWRST or START in SHA_CR, or by reading SHA_IODATARx)"]
    #[inline(always)]
    pub fn datrdy(&self) -> DatrdyR {
        DatrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Input Data Register Write Ready"]
    #[inline(always)]
    pub fn wrdy(&self) -> WrdyR {
        WrdyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Unspecified Register Access Detection Status (cleared by writing a 1 to SWRST bit in SHA_CR)"]
    #[inline(always)]
    pub fn urad(&self) -> UradR {
        UradR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Unspecified Register Access Type (cleared by writing a 1 to SWRST bit in SHA_CR)"]
    #[inline(always)]
    pub fn urat(&self) -> UratR {
        UratR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 16 - Check Done Status (cleared by writing START or SWRST bits in SHA_CR or by reading SHA_IODATARx)"]
    #[inline(always)]
    pub fn checkf(&self) -> CheckfR {
        CheckfR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:23 - Check Status (cleared by writing START or SWRST bits in SHA_CR or by reading SHA_IODATARx)"]
    #[inline(always)]
    pub fn chkst(&self) -> ChkstR {
        ChkstR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Security and/or Safety Event"]
    #[inline(always)]
    pub fn sece(&self) -> SeceR {
        SeceR::new(((self.bits >> 24) & 1) != 0)
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
