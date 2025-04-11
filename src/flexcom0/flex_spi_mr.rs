#[doc = "Register `FLEX_SPI_MR` reader"]
pub type R = crate::R<FlexSpiMrSpec>;
#[doc = "Register `FLEX_SPI_MR` writer"]
pub type W = crate::W<FlexSpiMrSpec>;
#[doc = "Field `MSTR` reader - Master/Slave Mode"]
pub type MstrR = crate::BitReader;
#[doc = "Field `MSTR` writer - Master/Slave Mode"]
pub type MstrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS` reader - Peripheral Select"]
pub type PsR = crate::BitReader;
#[doc = "Field `PS` writer - Peripheral Select"]
pub type PsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCSDEC` reader - Chip Select Decode"]
pub type PcsdecR = crate::BitReader;
#[doc = "Field `PCSDEC` writer - Chip Select Decode"]
pub type PcsdecW<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `MODFDIS` reader - Mode Fault Detection"]
pub type ModfdisR = crate::BitReader;
#[doc = "Field `MODFDIS` writer - Mode Fault Detection"]
pub type ModfdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDRBT` reader - Wait Data Read Before Transfer"]
pub type WdrbtR = crate::BitReader;
#[doc = "Field `WDRBT` writer - Wait Data Read Before Transfer"]
pub type WdrbtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LLB` reader - Local Loopback Enable"]
pub type LlbR = crate::BitReader;
#[doc = "Field `LLB` writer - Local Loopback Enable"]
pub type LlbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBHPC` reader - Last Bit Half Period Compatibility"]
pub type LbhpcR = crate::BitReader;
#[doc = "Field `LBHPC` writer - Last Bit Half Period Compatibility"]
pub type LbhpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Comparison Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmpmodeselect {
    #[doc = "0: Any character is received and comparison function drives CMP flag."]
    FlagOnly = 0,
    #[doc = "1: Comparison condition must be met to start reception of all incoming characters until REQCLR is set."]
    StartCondition = 1,
}
impl From<Cmpmodeselect> for bool {
    #[inline(always)]
    fn from(variant: Cmpmodeselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPMODE` reader - Comparison Mode"]
pub type CmpmodeR = crate::BitReader<Cmpmodeselect>;
impl CmpmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpmodeselect {
        match self.bits {
            false => Cmpmodeselect::FlagOnly,
            true => Cmpmodeselect::StartCondition,
        }
    }
    #[doc = "Any character is received and comparison function drives CMP flag."]
    #[inline(always)]
    pub fn is_flag_only(&self) -> bool {
        *self == Cmpmodeselect::FlagOnly
    }
    #[doc = "Comparison condition must be met to start reception of all incoming characters until REQCLR is set."]
    #[inline(always)]
    pub fn is_start_condition(&self) -> bool {
        *self == Cmpmodeselect::StartCondition
    }
}
#[doc = "Field `CMPMODE` writer - Comparison Mode"]
pub type CmpmodeW<'a, REG> = crate::BitWriter<'a, REG, Cmpmodeselect>;
impl<'a, REG> CmpmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Any character is received and comparison function drives CMP flag."]
    #[inline(always)]
    pub fn flag_only(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpmodeselect::FlagOnly)
    }
    #[doc = "Comparison condition must be met to start reception of all incoming characters until REQCLR is set."]
    #[inline(always)]
    pub fn start_condition(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpmodeselect::StartCondition)
    }
}
#[doc = "Field `PCS` reader - Peripheral Chip Select"]
pub type PcsR = crate::FieldReader;
#[doc = "Field `PCS` writer - Peripheral Chip Select"]
pub type PcsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DLYBCS` reader - Delay Between Chip Selects"]
pub type DlybcsR = crate::FieldReader;
#[doc = "Field `DLYBCS` writer - Delay Between Chip Selects"]
pub type DlybcsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Master/Slave Mode"]
    #[inline(always)]
    pub fn mstr(&self) -> MstrR {
        MstrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Peripheral Select"]
    #[inline(always)]
    pub fn ps(&self) -> PsR {
        PsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Chip Select Decode"]
    #[inline(always)]
    pub fn pcsdec(&self) -> PcsdecR {
        PcsdecR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bit Rate Source Clock"]
    #[inline(always)]
    pub fn brsrcclk(&self) -> BrsrcclkR {
        BrsrcclkR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mode Fault Detection"]
    #[inline(always)]
    pub fn modfdis(&self) -> ModfdisR {
        ModfdisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Wait Data Read Before Transfer"]
    #[inline(always)]
    pub fn wdrbt(&self) -> WdrbtR {
        WdrbtR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Local Loopback Enable"]
    #[inline(always)]
    pub fn llb(&self) -> LlbR {
        LlbR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Last Bit Half Period Compatibility"]
    #[inline(always)]
    pub fn lbhpc(&self) -> LbhpcR {
        LbhpcR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Comparison Mode"]
    #[inline(always)]
    pub fn cmpmode(&self) -> CmpmodeR {
        CmpmodeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Peripheral Chip Select"]
    #[inline(always)]
    pub fn pcs(&self) -> PcsR {
        PcsR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - Delay Between Chip Selects"]
    #[inline(always)]
    pub fn dlybcs(&self) -> DlybcsR {
        DlybcsR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Master/Slave Mode"]
    #[inline(always)]
    pub fn mstr(&mut self) -> MstrW<FlexSpiMrSpec> {
        MstrW::new(self, 0)
    }
    #[doc = "Bit 1 - Peripheral Select"]
    #[inline(always)]
    pub fn ps(&mut self) -> PsW<FlexSpiMrSpec> {
        PsW::new(self, 1)
    }
    #[doc = "Bit 2 - Chip Select Decode"]
    #[inline(always)]
    pub fn pcsdec(&mut self) -> PcsdecW<FlexSpiMrSpec> {
        PcsdecW::new(self, 2)
    }
    #[doc = "Bit 3 - Bit Rate Source Clock"]
    #[inline(always)]
    pub fn brsrcclk(&mut self) -> BrsrcclkW<FlexSpiMrSpec> {
        BrsrcclkW::new(self, 3)
    }
    #[doc = "Bit 4 - Mode Fault Detection"]
    #[inline(always)]
    pub fn modfdis(&mut self) -> ModfdisW<FlexSpiMrSpec> {
        ModfdisW::new(self, 4)
    }
    #[doc = "Bit 5 - Wait Data Read Before Transfer"]
    #[inline(always)]
    pub fn wdrbt(&mut self) -> WdrbtW<FlexSpiMrSpec> {
        WdrbtW::new(self, 5)
    }
    #[doc = "Bit 7 - Local Loopback Enable"]
    #[inline(always)]
    pub fn llb(&mut self) -> LlbW<FlexSpiMrSpec> {
        LlbW::new(self, 7)
    }
    #[doc = "Bit 8 - Last Bit Half Period Compatibility"]
    #[inline(always)]
    pub fn lbhpc(&mut self) -> LbhpcW<FlexSpiMrSpec> {
        LbhpcW::new(self, 8)
    }
    #[doc = "Bit 12 - Comparison Mode"]
    #[inline(always)]
    pub fn cmpmode(&mut self) -> CmpmodeW<FlexSpiMrSpec> {
        CmpmodeW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Peripheral Chip Select"]
    #[inline(always)]
    pub fn pcs(&mut self) -> PcsW<FlexSpiMrSpec> {
        PcsW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Delay Between Chip Selects"]
    #[inline(always)]
    pub fn dlybcs(&mut self) -> DlybcsW<FlexSpiMrSpec> {
        DlybcsW::new(self, 24)
    }
}
#[doc = "SPI Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_spi_mr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_spi_mr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexSpiMrSpec;
impl crate::RegisterSpec for FlexSpiMrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flex_spi_mr::R`](R) reader structure"]
impl crate::Readable for FlexSpiMrSpec {}
#[doc = "`write(|w| ..)` method takes [`flex_spi_mr::W`](W) writer structure"]
impl crate::Writable for FlexSpiMrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLEX_SPI_MR to value 0"]
impl crate::Resettable for FlexSpiMrSpec {}
