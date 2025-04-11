#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `PT` reader - POTermConf"]
pub type PtR = crate::BitReader;
#[doc = "Field `PT` writer - POTermConf"]
pub type PtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TA` reader - TermAddressConf"]
pub type TaR = crate::FieldReader;
#[doc = "Field `TA` writer - TermAddressConf"]
pub type TaW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TC` reader - TRBitCmd"]
pub type TcR = crate::BitReader;
#[doc = "Field `TC` writer - TRBitCmd"]
pub type TcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SC` reader - SSBitCmd"]
pub type ScR = crate::BitReader;
#[doc = "Field `SC` writer - SSBitCmd"]
pub type ScW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BC` reader - BusyBitCmd"]
pub type BcR = crate::BitReader;
#[doc = "Field `BC` writer - BusyBitCmd"]
pub type BcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRC` reader - SREQBitCmd"]
pub type SrcR = crate::BitReader;
#[doc = "Field `SRC` writer - SREQBitCmd"]
pub type SrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BEC` reader - BCEnableCmd"]
pub type BecR = crate::BitReader;
#[doc = "Field `BEC` writer - BCEnableCmd"]
pub type BecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST` reader - Soft Reset"]
pub type RstR = crate::BitReader;
#[doc = "Field `RST` writer - Soft Reset"]
pub type RstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - POTermConf"]
    #[inline(always)]
    pub fn pt(&self) -> PtR {
        PtR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - TermAddressConf"]
    #[inline(always)]
    pub fn ta(&self) -> TaR {
        TaR::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 6 - TRBitCmd"]
    #[inline(always)]
    pub fn tc(&self) -> TcR {
        TcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SSBitCmd"]
    #[inline(always)]
    pub fn sc(&self) -> ScR {
        ScR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - BusyBitCmd"]
    #[inline(always)]
    pub fn bc(&self) -> BcR {
        BcR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SREQBitCmd"]
    #[inline(always)]
    pub fn src(&self) -> SrcR {
        SrcR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BCEnableCmd"]
    #[inline(always)]
    pub fn bec(&self) -> BecR {
        BecR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Soft Reset"]
    #[inline(always)]
    pub fn rst(&self) -> RstR {
        RstR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - POTermConf"]
    #[inline(always)]
    pub fn pt(&mut self) -> PtW<CrSpec> {
        PtW::new(self, 0)
    }
    #[doc = "Bits 1:5 - TermAddressConf"]
    #[inline(always)]
    pub fn ta(&mut self) -> TaW<CrSpec> {
        TaW::new(self, 1)
    }
    #[doc = "Bit 6 - TRBitCmd"]
    #[inline(always)]
    pub fn tc(&mut self) -> TcW<CrSpec> {
        TcW::new(self, 6)
    }
    #[doc = "Bit 7 - SSBitCmd"]
    #[inline(always)]
    pub fn sc(&mut self) -> ScW<CrSpec> {
        ScW::new(self, 7)
    }
    #[doc = "Bit 8 - BusyBitCmd"]
    #[inline(always)]
    pub fn bc(&mut self) -> BcW<CrSpec> {
        BcW::new(self, 8)
    }
    #[doc = "Bit 9 - SREQBitCmd"]
    #[inline(always)]
    pub fn src(&mut self) -> SrcW<CrSpec> {
        SrcW::new(self, 9)
    }
    #[doc = "Bit 10 - BCEnableCmd"]
    #[inline(always)]
    pub fn bec(&mut self) -> BecW<CrSpec> {
        BecW::new(self, 10)
    }
    #[doc = "Bit 11 - Soft Reset"]
    #[inline(always)]
    pub fn rst(&mut self) -> RstW<CrSpec> {
        RstW::new(self, 11)
    }
}
#[doc = "Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
