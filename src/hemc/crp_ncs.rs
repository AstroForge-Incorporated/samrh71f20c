#[doc = "Register `CRP_NCS[%s]` reader"]
pub type R = crate::R<CrpNcsSpec>;
#[doc = "Register `CRP_NCS[%s]` writer"]
pub type W = crate::W<CrpNcsSpec>;
#[doc = "Field `MASTERNUMBER` reader - Master Number ID"]
pub type MasternumberR = crate::FieldReader<u16>;
#[doc = "Field `MASTERNUMBER` writer - Master Number ID"]
pub type MasternumberW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SUPERUSER` reader - User or Superuser access"]
pub type SuperuserR = crate::BitReader;
#[doc = "Field `SUPERUSER` writer - User or Superuser access"]
pub type SuperuserW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR` reader - Write Access"]
pub type WrR = crate::BitReader;
#[doc = "Field `WR` writer - Write Access"]
pub type WrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD` reader - Read Access"]
pub type RdR = crate::BitReader;
#[doc = "Field `RD` writer - Read Access"]
pub type RdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPLITBANKSIZE` reader - bank size internal separation"]
pub type SplitbanksizeR = crate::FieldReader;
#[doc = "Field `SPLITBANKSIZE` writer - bank size internal separation"]
pub type SplitbanksizeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PROTECTZONE` reader - select area protected"]
pub type ProtectzoneR = crate::BitReader;
#[doc = "Field `PROTECTZONE` writer - select area protected"]
pub type ProtectzoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROTECTON` reader - protection activation"]
pub type ProtectonR = crate::BitReader;
#[doc = "Field `PROTECTON` writer - protection activation"]
pub type ProtectonW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Master Number ID"]
    #[inline(always)]
    pub fn masternumber(&self) -> MasternumberR {
        MasternumberR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - User or Superuser access"]
    #[inline(always)]
    pub fn superuser(&self) -> SuperuserR {
        SuperuserR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Write Access"]
    #[inline(always)]
    pub fn wr(&self) -> WrR {
        WrR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Read Access"]
    #[inline(always)]
    pub fn rd(&self) -> RdR {
        RdR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:23 - bank size internal separation"]
    #[inline(always)]
    pub fn splitbanksize(&self) -> SplitbanksizeR {
        SplitbanksizeR::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - select area protected"]
    #[inline(always)]
    pub fn protectzone(&self) -> ProtectzoneR {
        ProtectzoneR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - protection activation"]
    #[inline(always)]
    pub fn protecton(&self) -> ProtectonR {
        ProtectonR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Master Number ID"]
    #[inline(always)]
    pub fn masternumber(&mut self) -> MasternumberW<CrpNcsSpec> {
        MasternumberW::new(self, 0)
    }
    #[doc = "Bit 16 - User or Superuser access"]
    #[inline(always)]
    pub fn superuser(&mut self) -> SuperuserW<CrpNcsSpec> {
        SuperuserW::new(self, 16)
    }
    #[doc = "Bit 17 - Write Access"]
    #[inline(always)]
    pub fn wr(&mut self) -> WrW<CrpNcsSpec> {
        WrW::new(self, 17)
    }
    #[doc = "Bit 18 - Read Access"]
    #[inline(always)]
    pub fn rd(&mut self) -> RdW<CrpNcsSpec> {
        RdW::new(self, 18)
    }
    #[doc = "Bits 19:23 - bank size internal separation"]
    #[inline(always)]
    pub fn splitbanksize(&mut self) -> SplitbanksizeW<CrpNcsSpec> {
        SplitbanksizeW::new(self, 19)
    }
    #[doc = "Bit 24 - select area protected"]
    #[inline(always)]
    pub fn protectzone(&mut self) -> ProtectzoneW<CrpNcsSpec> {
        ProtectzoneW::new(self, 24)
    }
    #[doc = "Bit 25 - protection activation"]
    #[inline(always)]
    pub fn protecton(&mut self) -> ProtectonW<CrpNcsSpec> {
        ProtectonW::new(self, 25)
    }
}
#[doc = "HEMC Control Register Protection NCS\n\nYou can [`read`](crate::Reg::read) this register and get [`crp_ncs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crp_ncs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrpNcsSpec;
impl crate::RegisterSpec for CrpNcsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crp_ncs::R`](R) reader structure"]
impl crate::Readable for CrpNcsSpec {}
#[doc = "`write(|w| ..)` method takes [`crp_ncs::W`](W) writer structure"]
impl crate::Writable for CrpNcsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRP_NCS[%s] to value 0"]
impl crate::Resettable for CrpNcsSpec {}
