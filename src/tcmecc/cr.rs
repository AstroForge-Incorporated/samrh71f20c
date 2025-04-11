#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `ENABLE` reader - ECC protection enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - ECC protection enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEST_MODE_RD` reader - test mode of ECC protection - read mode"]
pub type TestModeRdR = crate::BitReader;
#[doc = "Field `TEST_MODE_RD` writer - test mode of ECC protection - read mode"]
pub type TestModeRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEST_MODE_WR` reader - test mode of ECC protection - write mode"]
pub type TestModeWrR = crate::BitReader;
#[doc = "Field `TEST_MODE_WR` writer - test mode of ECC protection - write mode"]
pub type TestModeWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_FIX_CPT` reader - reset the fixable error counter"]
pub type RstFixCptR = crate::BitReader;
#[doc = "Field `RST_FIX_CPT` writer - reset the fixable error counter"]
pub type RstFixCptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST_NOFIX_CPT` reader - reset the un-fixable error counter"]
pub type RstNofixCptR = crate::BitReader;
#[doc = "Field `RST_NOFIX_CPT` writer - reset the un-fixable error counter"]
pub type RstNofixCptW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ECC protection enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - test mode of ECC protection - read mode"]
    #[inline(always)]
    pub fn test_mode_rd(&self) -> TestModeRdR {
        TestModeRdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - test mode of ECC protection - write mode"]
    #[inline(always)]
    pub fn test_mode_wr(&self) -> TestModeWrR {
        TestModeWrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - reset the fixable error counter"]
    #[inline(always)]
    pub fn rst_fix_cpt(&self) -> RstFixCptR {
        RstFixCptR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - reset the un-fixable error counter"]
    #[inline(always)]
    pub fn rst_nofix_cpt(&self) -> RstNofixCptR {
        RstNofixCptR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ECC protection enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<CrSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - test mode of ECC protection - read mode"]
    #[inline(always)]
    pub fn test_mode_rd(&mut self) -> TestModeRdW<CrSpec> {
        TestModeRdW::new(self, 1)
    }
    #[doc = "Bit 2 - test mode of ECC protection - write mode"]
    #[inline(always)]
    pub fn test_mode_wr(&mut self) -> TestModeWrW<CrSpec> {
        TestModeWrW::new(self, 2)
    }
    #[doc = "Bit 4 - reset the fixable error counter"]
    #[inline(always)]
    pub fn rst_fix_cpt(&mut self) -> RstFixCptW<CrSpec> {
        RstFixCptW::new(self, 4)
    }
    #[doc = "Bit 5 - reset the un-fixable error counter"]
    #[inline(always)]
    pub fn rst_nofix_cpt(&mut self) -> RstNofixCptW<CrSpec> {
        RstNofixCptW::new(self, 5)
    }
}
#[doc = "TCMECC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
