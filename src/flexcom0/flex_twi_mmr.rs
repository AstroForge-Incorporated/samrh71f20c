#[doc = "Register `FLEX_TWI_MMR` reader"]
pub type R = crate::R<FlexTwiMmrSpec>;
#[doc = "Register `FLEX_TWI_MMR` writer"]
pub type W = crate::W<FlexTwiMmrSpec>;
#[doc = "Internal Device Address Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Iadrszselect {
    #[doc = "0: No internal device address"]
    None = 0,
    #[doc = "1: One-byte internal device address"]
    _1Byte = 1,
    #[doc = "2: Two-byte internal device address"]
    _2Byte = 2,
    #[doc = "3: Three-byte internal device address"]
    _3Byte = 3,
}
impl From<Iadrszselect> for u8 {
    #[inline(always)]
    fn from(variant: Iadrszselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Iadrszselect {
    type Ux = u8;
}
impl crate::IsEnum for Iadrszselect {}
#[doc = "Field `IADRSZ` reader - Internal Device Address Size"]
pub type IadrszR = crate::FieldReader<Iadrszselect>;
impl IadrszR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iadrszselect {
        match self.bits {
            0 => Iadrszselect::None,
            1 => Iadrszselect::_1Byte,
            2 => Iadrszselect::_2Byte,
            3 => Iadrszselect::_3Byte,
            _ => unreachable!(),
        }
    }
    #[doc = "No internal device address"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Iadrszselect::None
    }
    #[doc = "One-byte internal device address"]
    #[inline(always)]
    pub fn is_1_byte(&self) -> bool {
        *self == Iadrszselect::_1Byte
    }
    #[doc = "Two-byte internal device address"]
    #[inline(always)]
    pub fn is_2_byte(&self) -> bool {
        *self == Iadrszselect::_2Byte
    }
    #[doc = "Three-byte internal device address"]
    #[inline(always)]
    pub fn is_3_byte(&self) -> bool {
        *self == Iadrszselect::_3Byte
    }
}
#[doc = "Field `IADRSZ` writer - Internal Device Address Size"]
pub type IadrszW<'a, REG> = crate::FieldWriter<'a, REG, 2, Iadrszselect, crate::Safe>;
impl<'a, REG> IadrszW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No internal device address"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Iadrszselect::None)
    }
    #[doc = "One-byte internal device address"]
    #[inline(always)]
    pub fn _1_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Iadrszselect::_1Byte)
    }
    #[doc = "Two-byte internal device address"]
    #[inline(always)]
    pub fn _2_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Iadrszselect::_2Byte)
    }
    #[doc = "Three-byte internal device address"]
    #[inline(always)]
    pub fn _3_byte(self) -> &'a mut crate::W<REG> {
        self.variant(Iadrszselect::_3Byte)
    }
}
#[doc = "Field `MREAD` reader - Master Read Direction"]
pub type MreadR = crate::BitReader;
#[doc = "Field `MREAD` writer - Master Read Direction"]
pub type MreadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DADR` reader - Device Address"]
pub type DadrR = crate::FieldReader;
#[doc = "Field `DADR` writer - Device Address"]
pub type DadrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `NOAP` reader - No Auto-Stop On NACK Error"]
pub type NoapR = crate::BitReader;
#[doc = "Field `NOAP` writer - No Auto-Stop On NACK Error"]
pub type NoapW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 8:9 - Internal Device Address Size"]
    #[inline(always)]
    pub fn iadrsz(&self) -> IadrszR {
        IadrszR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - Master Read Direction"]
    #[inline(always)]
    pub fn mread(&self) -> MreadR {
        MreadR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:22 - Device Address"]
    #[inline(always)]
    pub fn dadr(&self) -> DadrR {
        DadrR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - No Auto-Stop On NACK Error"]
    #[inline(always)]
    pub fn noap(&self) -> NoapR {
        NoapR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:9 - Internal Device Address Size"]
    #[inline(always)]
    pub fn iadrsz(&mut self) -> IadrszW<FlexTwiMmrSpec> {
        IadrszW::new(self, 8)
    }
    #[doc = "Bit 12 - Master Read Direction"]
    #[inline(always)]
    pub fn mread(&mut self) -> MreadW<FlexTwiMmrSpec> {
        MreadW::new(self, 12)
    }
    #[doc = "Bits 16:22 - Device Address"]
    #[inline(always)]
    pub fn dadr(&mut self) -> DadrW<FlexTwiMmrSpec> {
        DadrW::new(self, 16)
    }
    #[doc = "Bit 24 - No Auto-Stop On NACK Error"]
    #[inline(always)]
    pub fn noap(&mut self) -> NoapW<FlexTwiMmrSpec> {
        NoapW::new(self, 24)
    }
}
#[doc = "TWI Master Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_twi_mmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_twi_mmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexTwiMmrSpec;
impl crate::RegisterSpec for FlexTwiMmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flex_twi_mmr::R`](R) reader structure"]
impl crate::Readable for FlexTwiMmrSpec {}
#[doc = "`write(|w| ..)` method takes [`flex_twi_mmr::W`](W) writer structure"]
impl crate::Writable for FlexTwiMmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLEX_TWI_MMR to value 0"]
impl crate::Resettable for FlexTwiMmrSpec {}
