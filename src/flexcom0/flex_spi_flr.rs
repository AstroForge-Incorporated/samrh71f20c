#[doc = "Register `FLEX_SPI_FLR` reader"]
pub type R = crate::R<FlexSpiFlrSpec>;
#[doc = "Field `TXFL` reader - Transmit FIFO Level"]
pub type TxflR = crate::FieldReader;
#[doc = "Field `RXFL` reader - Receive FIFO Level"]
pub type RxflR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Transmit FIFO Level"]
    #[inline(always)]
    pub fn txfl(&self) -> TxflR {
        TxflR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Receive FIFO Level"]
    #[inline(always)]
    pub fn rxfl(&self) -> RxflR {
        RxflR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
#[doc = "SPI FIFO Level Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_spi_flr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexSpiFlrSpec;
impl crate::RegisterSpec for FlexSpiFlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flex_spi_flr::R`](R) reader structure"]
impl crate::Readable for FlexSpiFlrSpec {}
#[doc = "`reset()` method sets FLEX_SPI_FLR to value 0"]
impl crate::Resettable for FlexSpiFlrSpec {}
