#[doc = "Register `FLEX_SPI_TDR` writer"]
pub type W = crate::W<FlexSpiTdrSpec>;
#[doc = "Field `TD` writer - Transmit Data"]
pub type TdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PCS` writer - Peripheral Chip Select"]
pub type PcsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LASTXFER` writer - Last Transfer"]
pub type LastxferW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:15 - Transmit Data"]
    #[inline(always)]
    pub fn td(&mut self) -> TdW<FlexSpiTdrSpec> {
        TdW::new(self, 0)
    }
    #[doc = "Bits 16:19 - Peripheral Chip Select"]
    #[inline(always)]
    pub fn pcs(&mut self) -> PcsW<FlexSpiTdrSpec> {
        PcsW::new(self, 16)
    }
    #[doc = "Bit 24 - Last Transfer"]
    #[inline(always)]
    pub fn lastxfer(&mut self) -> LastxferW<FlexSpiTdrSpec> {
        LastxferW::new(self, 24)
    }
}
#[doc = "SPI Transmit Data Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_spi_tdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexSpiTdrSpec;
impl crate::RegisterSpec for FlexSpiTdrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`flex_spi_tdr::W`](W) writer structure"]
impl crate::Writable for FlexSpiTdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLEX_SPI_TDR to value 0"]
impl crate::Resettable for FlexSpiTdrSpec {}
