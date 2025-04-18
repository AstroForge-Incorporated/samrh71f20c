#[doc = "Register `FMR` reader"]
pub type R = crate::R<FmrSpec>;
#[doc = "Register `FMR` writer"]
pub type W = crate::W<FmrSpec>;
#[doc = "Field `FPOL` reader - Fault Polarity"]
pub type FpolR = crate::FieldReader;
#[doc = "Field `FPOL` writer - Fault Polarity"]
pub type FpolW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FMOD` reader - Fault Activation Mode"]
pub type FmodR = crate::FieldReader;
#[doc = "Field `FMOD` writer - Fault Activation Mode"]
pub type FmodW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FFIL` reader - Fault Filtering"]
pub type FfilR = crate::FieldReader;
#[doc = "Field `FFIL` writer - Fault Filtering"]
pub type FfilW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Fault Polarity"]
    #[inline(always)]
    pub fn fpol(&self) -> FpolR {
        FpolR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Fault Activation Mode"]
    #[inline(always)]
    pub fn fmod(&self) -> FmodR {
        FmodR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Fault Filtering"]
    #[inline(always)]
    pub fn ffil(&self) -> FfilR {
        FfilR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Fault Polarity"]
    #[inline(always)]
    pub fn fpol(&mut self) -> FpolW<FmrSpec> {
        FpolW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Fault Activation Mode"]
    #[inline(always)]
    pub fn fmod(&mut self) -> FmodW<FmrSpec> {
        FmodW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Fault Filtering"]
    #[inline(always)]
    pub fn ffil(&mut self) -> FfilW<FmrSpec> {
        FfilW::new(self, 16)
    }
}
#[doc = "PWM Fault Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FmrSpec;
impl crate::RegisterSpec for FmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmr::R`](R) reader structure"]
impl crate::Readable for FmrSpec {}
#[doc = "`write(|w| ..)` method takes [`fmr::W`](W) writer structure"]
impl crate::Writable for FmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FMR to value 0"]
impl crate::Resettable for FmrSpec {}
