#[doc = "Register `FLEX_SPI_FMR` reader"]
pub type R = crate::R<FlexSpiFmrSpec>;
#[doc = "Register `FLEX_SPI_FMR` writer"]
pub type W = crate::W<FlexSpiFmrSpec>;
#[doc = "Transmit Data Register Empty Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Txrdymselect {
    #[doc = "0: TDRE will be at level '1' when at least one data can be written in the Transmit FIFO."]
    OneData = 0,
    #[doc = "1: TDRE will be at level '1' when at least two data can be written in the Transmit FIFO. Cannot be used if FLEX_SPI_MR.PS =1."]
    TwoData = 1,
}
impl From<Txrdymselect> for u8 {
    #[inline(always)]
    fn from(variant: Txrdymselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Txrdymselect {
    type Ux = u8;
}
impl crate::IsEnum for Txrdymselect {}
#[doc = "Field `TXRDYM` reader - Transmit Data Register Empty Mode"]
pub type TxrdymR = crate::FieldReader<Txrdymselect>;
impl TxrdymR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Txrdymselect> {
        match self.bits {
            0 => Some(Txrdymselect::OneData),
            1 => Some(Txrdymselect::TwoData),
            _ => None,
        }
    }
    #[doc = "TDRE will be at level '1' when at least one data can be written in the Transmit FIFO."]
    #[inline(always)]
    pub fn is_one_data(&self) -> bool {
        *self == Txrdymselect::OneData
    }
    #[doc = "TDRE will be at level '1' when at least two data can be written in the Transmit FIFO. Cannot be used if FLEX_SPI_MR.PS =1."]
    #[inline(always)]
    pub fn is_two_data(&self) -> bool {
        *self == Txrdymselect::TwoData
    }
}
#[doc = "Field `TXRDYM` writer - Transmit Data Register Empty Mode"]
pub type TxrdymW<'a, REG> = crate::FieldWriter<'a, REG, 2, Txrdymselect>;
impl<'a, REG> TxrdymW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TDRE will be at level '1' when at least one data can be written in the Transmit FIFO."]
    #[inline(always)]
    pub fn one_data(self) -> &'a mut crate::W<REG> {
        self.variant(Txrdymselect::OneData)
    }
    #[doc = "TDRE will be at level '1' when at least two data can be written in the Transmit FIFO. Cannot be used if FLEX_SPI_MR.PS =1."]
    #[inline(always)]
    pub fn two_data(self) -> &'a mut crate::W<REG> {
        self.variant(Txrdymselect::TwoData)
    }
}
#[doc = "Receive Data Register Full Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rxrdymselect {
    #[doc = "0: RDRF will be at level '1' when at least one unread data is in the Receive FIFO."]
    OneData = 0,
    #[doc = "1: RDRF will be at level '1' when at least two unread data are in the Receive FIFO. Cannot be used when FLEX_SPI_MR.MSTR =1, or if FLEX_SPI_MR.PS =1."]
    TwoData = 1,
    #[doc = "2: RDRF will be at level '1' when at least four unread data are in the Receive FIFO. Cannot be used when FLEX_SPI_CSRx.BITS is greater than 0, or if FLEX_SPI_MR.MSTR =1, or if FLEX_SPI_MR.PS =1."]
    FourData = 2,
}
impl From<Rxrdymselect> for u8 {
    #[inline(always)]
    fn from(variant: Rxrdymselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rxrdymselect {
    type Ux = u8;
}
impl crate::IsEnum for Rxrdymselect {}
#[doc = "Field `RXRDYM` reader - Receive Data Register Full Mode"]
pub type RxrdymR = crate::FieldReader<Rxrdymselect>;
impl RxrdymR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rxrdymselect> {
        match self.bits {
            0 => Some(Rxrdymselect::OneData),
            1 => Some(Rxrdymselect::TwoData),
            2 => Some(Rxrdymselect::FourData),
            _ => None,
        }
    }
    #[doc = "RDRF will be at level '1' when at least one unread data is in the Receive FIFO."]
    #[inline(always)]
    pub fn is_one_data(&self) -> bool {
        *self == Rxrdymselect::OneData
    }
    #[doc = "RDRF will be at level '1' when at least two unread data are in the Receive FIFO. Cannot be used when FLEX_SPI_MR.MSTR =1, or if FLEX_SPI_MR.PS =1."]
    #[inline(always)]
    pub fn is_two_data(&self) -> bool {
        *self == Rxrdymselect::TwoData
    }
    #[doc = "RDRF will be at level '1' when at least four unread data are in the Receive FIFO. Cannot be used when FLEX_SPI_CSRx.BITS is greater than 0, or if FLEX_SPI_MR.MSTR =1, or if FLEX_SPI_MR.PS =1."]
    #[inline(always)]
    pub fn is_four_data(&self) -> bool {
        *self == Rxrdymselect::FourData
    }
}
#[doc = "Field `RXRDYM` writer - Receive Data Register Full Mode"]
pub type RxrdymW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rxrdymselect>;
impl<'a, REG> RxrdymW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RDRF will be at level '1' when at least one unread data is in the Receive FIFO."]
    #[inline(always)]
    pub fn one_data(self) -> &'a mut crate::W<REG> {
        self.variant(Rxrdymselect::OneData)
    }
    #[doc = "RDRF will be at level '1' when at least two unread data are in the Receive FIFO. Cannot be used when FLEX_SPI_MR.MSTR =1, or if FLEX_SPI_MR.PS =1."]
    #[inline(always)]
    pub fn two_data(self) -> &'a mut crate::W<REG> {
        self.variant(Rxrdymselect::TwoData)
    }
    #[doc = "RDRF will be at level '1' when at least four unread data are in the Receive FIFO. Cannot be used when FLEX_SPI_CSRx.BITS is greater than 0, or if FLEX_SPI_MR.MSTR =1, or if FLEX_SPI_MR.PS =1."]
    #[inline(always)]
    pub fn four_data(self) -> &'a mut crate::W<REG> {
        self.variant(Rxrdymselect::FourData)
    }
}
#[doc = "Field `TXFTHRES` reader - Transmit FIFO Threshold"]
pub type TxfthresR = crate::FieldReader;
#[doc = "Field `TXFTHRES` writer - Transmit FIFO Threshold"]
pub type TxfthresW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RXFTHRES` reader - Receive FIFO Threshold"]
pub type RxfthresR = crate::FieldReader;
#[doc = "Field `RXFTHRES` writer - Receive FIFO Threshold"]
pub type RxfthresW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:1 - Transmit Data Register Empty Mode"]
    #[inline(always)]
    pub fn txrdym(&self) -> TxrdymR {
        TxrdymR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Receive Data Register Full Mode"]
    #[inline(always)]
    pub fn rxrdym(&self) -> RxrdymR {
        RxrdymR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 16:21 - Transmit FIFO Threshold"]
    #[inline(always)]
    pub fn txfthres(&self) -> TxfthresR {
        TxfthresR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Receive FIFO Threshold"]
    #[inline(always)]
    pub fn rxfthres(&self) -> RxfthresR {
        RxfthresR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Transmit Data Register Empty Mode"]
    #[inline(always)]
    pub fn txrdym(&mut self) -> TxrdymW<FlexSpiFmrSpec> {
        TxrdymW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Receive Data Register Full Mode"]
    #[inline(always)]
    pub fn rxrdym(&mut self) -> RxrdymW<FlexSpiFmrSpec> {
        RxrdymW::new(self, 4)
    }
    #[doc = "Bits 16:21 - Transmit FIFO Threshold"]
    #[inline(always)]
    pub fn txfthres(&mut self) -> TxfthresW<FlexSpiFmrSpec> {
        TxfthresW::new(self, 16)
    }
    #[doc = "Bits 24:29 - Receive FIFO Threshold"]
    #[inline(always)]
    pub fn rxfthres(&mut self) -> RxfthresW<FlexSpiFmrSpec> {
        RxfthresW::new(self, 24)
    }
}
#[doc = "SPI FIFO Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_spi_fmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_spi_fmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexSpiFmrSpec;
impl crate::RegisterSpec for FlexSpiFmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flex_spi_fmr::R`](R) reader structure"]
impl crate::Readable for FlexSpiFmrSpec {}
#[doc = "`write(|w| ..)` method takes [`flex_spi_fmr::W`](W) writer structure"]
impl crate::Writable for FlexSpiFmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLEX_SPI_FMR to value 0"]
impl crate::Resettable for FlexSpiFmrSpec {}
