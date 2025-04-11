#[doc = "Register `HECC_CR2` reader"]
pub type R = crate::R<HeccCr2Spec>;
#[doc = "Register `HECC_CR2` writer"]
pub type W = crate::W<HeccCr2Spec>;
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
#[doc = "Field `ECC12_ENABLE` reader - BCH ECC enable"]
pub type Ecc12EnableR = crate::BitReader;
#[doc = "Field `ECC12_ENABLE` writer - BCH ECC enable"]
pub type Ecc12EnableW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 6 - BCH ECC enable"]
    #[inline(always)]
    pub fn ecc12_enable(&self) -> Ecc12EnableR {
        Ecc12EnableR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ECC protection enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<HeccCr2Spec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - test mode of ECC protection - read mode"]
    #[inline(always)]
    pub fn test_mode_rd(&mut self) -> TestModeRdW<HeccCr2Spec> {
        TestModeRdW::new(self, 1)
    }
    #[doc = "Bit 2 - test mode of ECC protection - write mode"]
    #[inline(always)]
    pub fn test_mode_wr(&mut self) -> TestModeWrW<HeccCr2Spec> {
        TestModeWrW::new(self, 2)
    }
    #[doc = "Bit 4 - reset the fixable error counter"]
    #[inline(always)]
    pub fn rst_fix_cpt(&mut self) -> RstFixCptW<HeccCr2Spec> {
        RstFixCptW::new(self, 4)
    }
    #[doc = "Bit 5 - reset the un-fixable error counter"]
    #[inline(always)]
    pub fn rst_nofix_cpt(&mut self) -> RstNofixCptW<HeccCr2Spec> {
        RstNofixCptW::new(self, 5)
    }
    #[doc = "Bit 6 - BCH ECC enable"]
    #[inline(always)]
    pub fn ecc12_enable(&mut self) -> Ecc12EnableW<HeccCr2Spec> {
        Ecc12EnableW::new(self, 6)
    }
}
#[doc = "HECC Control Register Channel 2 (HSDRAMC)\n\nYou can [`read`](crate::Reg::read) this register and get [`hecc_cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hecc_cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HeccCr2Spec;
impl crate::RegisterSpec for HeccCr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hecc_cr2::R`](R) reader structure"]
impl crate::Readable for HeccCr2Spec {}
#[doc = "`write(|w| ..)` method takes [`hecc_cr2::W`](W) writer structure"]
impl crate::Writable for HeccCr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HECC_CR2 to value 0"]
impl crate::Resettable for HeccCr2Spec {}
