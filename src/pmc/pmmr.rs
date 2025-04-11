#[doc = "Register `PMMR` reader"]
pub type R = crate::R<PmmrSpec>;
#[doc = "Register `PMMR` writer"]
pub type W = crate::W<PmmrSpec>;
#[doc = "Field `PLLA_MMAX` reader - PLLA Maximum Allowed Multiplier Value"]
pub type PllaMmaxR = crate::FieldReader<u16>;
#[doc = "Field `PLLA_MMAX` writer - PLLA Maximum Allowed Multiplier Value"]
pub type PllaMmaxW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PLLB_MMAX` reader - PLLB Maximum Allowed Multiplier Value"]
pub type PllbMmaxR = crate::FieldReader<u16>;
#[doc = "Field `PLLB_MMAX` writer - PLLB Maximum Allowed Multiplier Value"]
pub type PllbMmaxW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - PLLA Maximum Allowed Multiplier Value"]
    #[inline(always)]
    pub fn plla_mmax(&self) -> PllaMmaxR {
        PllaMmaxR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - PLLB Maximum Allowed Multiplier Value"]
    #[inline(always)]
    pub fn pllb_mmax(&self) -> PllbMmaxR {
        PllbMmaxR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - PLLA Maximum Allowed Multiplier Value"]
    #[inline(always)]
    pub fn plla_mmax(&mut self) -> PllaMmaxW<PmmrSpec> {
        PllaMmaxW::new(self, 0)
    }
    #[doc = "Bits 16:26 - PLLB Maximum Allowed Multiplier Value"]
    #[inline(always)]
    pub fn pllb_mmax(&mut self) -> PllbMmaxW<PmmrSpec> {
        PllbMmaxW::new(self, 16)
    }
}
#[doc = "PLL Maximum Multiplier Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmmrSpec;
impl crate::RegisterSpec for PmmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmmr::R`](R) reader structure"]
impl crate::Readable for PmmrSpec {}
#[doc = "`write(|w| ..)` method takes [`pmmr::W`](W) writer structure"]
impl crate::Writable for PmmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PMMR to value 0"]
impl crate::Resettable for PmmrSpec {}
