#[doc = "Register `FMR` reader"]
pub type R = crate::R<FmrSpec>;
#[doc = "Register `FMR` writer"]
pub type W = crate::W<FmrSpec>;
#[doc = "Field `FRDY` reader - Flash Ready Interrupt Enable"]
pub type FrdyR = crate::BitReader;
#[doc = "Field `FRDY` writer - Flash Ready Interrupt Enable"]
pub type FrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPSI` reader - Flash Power Status Interrupt Enable"]
pub type FpsiR = crate::BitReader;
#[doc = "Field `FPSI` writer - Flash Power Status Interrupt Enable"]
pub type FpsiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONE` reader - Must be written to 1"]
pub type OneR = crate::BitReader;
#[doc = "Field `ONE` writer - Must be written to 1"]
pub type OneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Flash Ready Interrupt Enable"]
    #[inline(always)]
    pub fn frdy(&self) -> FrdyR {
        FrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Flash Power Status Interrupt Enable"]
    #[inline(always)]
    pub fn fpsi(&self) -> FpsiR {
        FpsiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - Must be written to 1"]
    #[inline(always)]
    pub fn one(&self) -> OneR {
        OneR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flash Ready Interrupt Enable"]
    #[inline(always)]
    pub fn frdy(&mut self) -> FrdyW<FmrSpec> {
        FrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - Flash Power Status Interrupt Enable"]
    #[inline(always)]
    pub fn fpsi(&mut self) -> FpsiW<FmrSpec> {
        FpsiW::new(self, 1)
    }
    #[doc = "Bit 16 - Must be written to 1"]
    #[inline(always)]
    pub fn one(&mut self) -> OneW<FmrSpec> {
        OneW::new(self, 16)
    }
}
#[doc = "HEFC Flash Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
