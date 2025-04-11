#[doc = "Register `FLEX_TWI_ACR` reader"]
pub type R = crate::R<FlexTwiAcrSpec>;
#[doc = "Register `FLEX_TWI_ACR` writer"]
pub type W = crate::W<FlexTwiAcrSpec>;
#[doc = "Field `DATAL` reader - Data Length"]
pub type DatalR = crate::FieldReader;
#[doc = "Field `DATAL` writer - Data Length"]
pub type DatalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DIR` reader - Transfer Direction"]
pub type DirR = crate::BitReader;
#[doc = "Field `DIR` writer - Transfer Direction"]
pub type DirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEC` reader - PEC Request (SMBus Mode only)"]
pub type PecR = crate::BitReader;
#[doc = "Field `PEC` writer - PEC Request (SMBus Mode only)"]
pub type PecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NDATAL` reader - Next Data Length"]
pub type NdatalR = crate::FieldReader;
#[doc = "Field `NDATAL` writer - Next Data Length"]
pub type NdatalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NDIR` reader - Next Transfer Direction"]
pub type NdirR = crate::BitReader;
#[doc = "Field `NDIR` writer - Next Transfer Direction"]
pub type NdirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NPEC` reader - Next PEC Request (SMBus Mode only)"]
pub type NpecR = crate::BitReader;
#[doc = "Field `NPEC` writer - Next PEC Request (SMBus Mode only)"]
pub type NpecW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Data Length"]
    #[inline(always)]
    pub fn datal(&self) -> DatalR {
        DatalR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Transfer Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PEC Request (SMBus Mode only)"]
    #[inline(always)]
    pub fn pec(&self) -> PecR {
        PecR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Next Data Length"]
    #[inline(always)]
    pub fn ndatal(&self) -> NdatalR {
        NdatalR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Next Transfer Direction"]
    #[inline(always)]
    pub fn ndir(&self) -> NdirR {
        NdirR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Next PEC Request (SMBus Mode only)"]
    #[inline(always)]
    pub fn npec(&self) -> NpecR {
        NpecR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Length"]
    #[inline(always)]
    pub fn datal(&mut self) -> DatalW<FlexTwiAcrSpec> {
        DatalW::new(self, 0)
    }
    #[doc = "Bit 8 - Transfer Direction"]
    #[inline(always)]
    pub fn dir(&mut self) -> DirW<FlexTwiAcrSpec> {
        DirW::new(self, 8)
    }
    #[doc = "Bit 9 - PEC Request (SMBus Mode only)"]
    #[inline(always)]
    pub fn pec(&mut self) -> PecW<FlexTwiAcrSpec> {
        PecW::new(self, 9)
    }
    #[doc = "Bits 16:23 - Next Data Length"]
    #[inline(always)]
    pub fn ndatal(&mut self) -> NdatalW<FlexTwiAcrSpec> {
        NdatalW::new(self, 16)
    }
    #[doc = "Bit 24 - Next Transfer Direction"]
    #[inline(always)]
    pub fn ndir(&mut self) -> NdirW<FlexTwiAcrSpec> {
        NdirW::new(self, 24)
    }
    #[doc = "Bit 25 - Next PEC Request (SMBus Mode only)"]
    #[inline(always)]
    pub fn npec(&mut self) -> NpecW<FlexTwiAcrSpec> {
        NpecW::new(self, 25)
    }
}
#[doc = "TWI Alternative Command Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_twi_acr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_twi_acr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexTwiAcrSpec;
impl crate::RegisterSpec for FlexTwiAcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flex_twi_acr::R`](R) reader structure"]
impl crate::Readable for FlexTwiAcrSpec {}
#[doc = "`write(|w| ..)` method takes [`flex_twi_acr::W`](W) writer structure"]
impl crate::Writable for FlexTwiAcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLEX_TWI_ACR to value 0"]
impl crate::Resettable for FlexTwiAcrSpec {}
