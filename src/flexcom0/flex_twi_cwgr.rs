#[doc = "Register `FLEX_TWI_CWGR` reader"]
pub type R = crate::R<FlexTwiCwgrSpec>;
#[doc = "Register `FLEX_TWI_CWGR` writer"]
pub type W = crate::W<FlexTwiCwgrSpec>;
#[doc = "Field `CLDIV` reader - Clock Low Divider"]
pub type CldivR = crate::FieldReader;
#[doc = "Field `CLDIV` writer - Clock Low Divider"]
pub type CldivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CHDIV` reader - Clock High Divider"]
pub type ChdivR = crate::FieldReader;
#[doc = "Field `CHDIV` writer - Clock High Divider"]
pub type ChdivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CKDIV` reader - Clock Divider"]
pub type CkdivR = crate::FieldReader;
#[doc = "Field `CKDIV` writer - Clock Divider"]
pub type CkdivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Bit Rate Source Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brsrcclkselect {
    #[doc = "0: The peripheral clock is the source clock for the bit rate generation."]
    PeriphClk = 0,
    #[doc = "1: GCLK is the source clock for the bit rate generation, thus the bit rate can be independent of the core/peripheral clock."]
    Gclk = 1,
}
impl From<Brsrcclkselect> for bool {
    #[inline(always)]
    fn from(variant: Brsrcclkselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRSRCCLK` reader - Bit Rate Source Clock"]
pub type BrsrcclkR = crate::BitReader<Brsrcclkselect>;
impl BrsrcclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Brsrcclkselect {
        match self.bits {
            false => Brsrcclkselect::PeriphClk,
            true => Brsrcclkselect::Gclk,
        }
    }
    #[doc = "The peripheral clock is the source clock for the bit rate generation."]
    #[inline(always)]
    pub fn is_periph_clk(&self) -> bool {
        *self == Brsrcclkselect::PeriphClk
    }
    #[doc = "GCLK is the source clock for the bit rate generation, thus the bit rate can be independent of the core/peripheral clock."]
    #[inline(always)]
    pub fn is_gclk(&self) -> bool {
        *self == Brsrcclkselect::Gclk
    }
}
#[doc = "Field `BRSRCCLK` writer - Bit Rate Source Clock"]
pub type BrsrcclkW<'a, REG> = crate::BitWriter<'a, REG, Brsrcclkselect>;
impl<'a, REG> BrsrcclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The peripheral clock is the source clock for the bit rate generation."]
    #[inline(always)]
    pub fn periph_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Brsrcclkselect::PeriphClk)
    }
    #[doc = "GCLK is the source clock for the bit rate generation, thus the bit rate can be independent of the core/peripheral clock."]
    #[inline(always)]
    pub fn gclk(self) -> &'a mut crate::W<REG> {
        self.variant(Brsrcclkselect::Gclk)
    }
}
#[doc = "Field `HOLD` reader - TWD Hold Time Versus TWCK Falling"]
pub type HoldR = crate::FieldReader;
#[doc = "Field `HOLD` writer - TWD Hold Time Versus TWCK Falling"]
pub type HoldW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:7 - Clock Low Divider"]
    #[inline(always)]
    pub fn cldiv(&self) -> CldivR {
        CldivR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Clock High Divider"]
    #[inline(always)]
    pub fn chdiv(&self) -> ChdivR {
        ChdivR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - Clock Divider"]
    #[inline(always)]
    pub fn ckdiv(&self) -> CkdivR {
        CkdivR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20 - Bit Rate Source Clock"]
    #[inline(always)]
    pub fn brsrcclk(&self) -> BrsrcclkR {
        BrsrcclkR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:29 - TWD Hold Time Versus TWCK Falling"]
    #[inline(always)]
    pub fn hold(&self) -> HoldR {
        HoldR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock Low Divider"]
    #[inline(always)]
    pub fn cldiv(&mut self) -> CldivW<FlexTwiCwgrSpec> {
        CldivW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Clock High Divider"]
    #[inline(always)]
    pub fn chdiv(&mut self) -> ChdivW<FlexTwiCwgrSpec> {
        ChdivW::new(self, 8)
    }
    #[doc = "Bits 16:18 - Clock Divider"]
    #[inline(always)]
    pub fn ckdiv(&mut self) -> CkdivW<FlexTwiCwgrSpec> {
        CkdivW::new(self, 16)
    }
    #[doc = "Bit 20 - Bit Rate Source Clock"]
    #[inline(always)]
    pub fn brsrcclk(&mut self) -> BrsrcclkW<FlexTwiCwgrSpec> {
        BrsrcclkW::new(self, 20)
    }
    #[doc = "Bits 24:29 - TWD Hold Time Versus TWCK Falling"]
    #[inline(always)]
    pub fn hold(&mut self) -> HoldW<FlexTwiCwgrSpec> {
        HoldW::new(self, 24)
    }
}
#[doc = "TWI Clock Waveform Generator Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_twi_cwgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_twi_cwgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexTwiCwgrSpec;
impl crate::RegisterSpec for FlexTwiCwgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flex_twi_cwgr::R`](R) reader structure"]
impl crate::Readable for FlexTwiCwgrSpec {}
#[doc = "`write(|w| ..)` method takes [`flex_twi_cwgr::W`](W) writer structure"]
impl crate::Writable for FlexTwiCwgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLEX_TWI_CWGR to value 0"]
impl crate::Resettable for FlexTwiCwgrSpec {}
