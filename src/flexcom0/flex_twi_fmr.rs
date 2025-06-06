#[doc = "Register `FLEX_TWI_FMR` reader"]
pub type R = crate::R<FlexTwiFmrSpec>;
#[doc = "Register `FLEX_TWI_FMR` writer"]
pub type W = crate::W<FlexTwiFmrSpec>;
#[doc = "Transmitter Ready Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Txrdymselect {
    #[doc = "0: TXRDY will be at level '1' when at least one data can be written in the Transmit FIFO"]
    OneData = 0,
    #[doc = "1: TXRDY will be at level '1' when at least two data can be written in the Transmit FIFO"]
    TwoData = 1,
    #[doc = "2: TXRDY will be at level '1' when at least four data can be written in the Transmit FIFO"]
    FourData = 2,
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
#[doc = "Field `TXRDYM` reader - Transmitter Ready Mode"]
pub type TxrdymR = crate::FieldReader<Txrdymselect>;
impl TxrdymR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Txrdymselect> {
        match self.bits {
            0 => Some(Txrdymselect::OneData),
            1 => Some(Txrdymselect::TwoData),
            2 => Some(Txrdymselect::FourData),
            _ => None,
        }
    }
    #[doc = "TXRDY will be at level '1' when at least one data can be written in the Transmit FIFO"]
    #[inline(always)]
    pub fn is_one_data(&self) -> bool {
        *self == Txrdymselect::OneData
    }
    #[doc = "TXRDY will be at level '1' when at least two data can be written in the Transmit FIFO"]
    #[inline(always)]
    pub fn is_two_data(&self) -> bool {
        *self == Txrdymselect::TwoData
    }
    #[doc = "TXRDY will be at level '1' when at least four data can be written in the Transmit FIFO"]
    #[inline(always)]
    pub fn is_four_data(&self) -> bool {
        *self == Txrdymselect::FourData
    }
}
#[doc = "Field `TXRDYM` writer - Transmitter Ready Mode"]
pub type TxrdymW<'a, REG> = crate::FieldWriter<'a, REG, 2, Txrdymselect>;
impl<'a, REG> TxrdymW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TXRDY will be at level '1' when at least one data can be written in the Transmit FIFO"]
    #[inline(always)]
    pub fn one_data(self) -> &'a mut crate::W<REG> {
        self.variant(Txrdymselect::OneData)
    }
    #[doc = "TXRDY will be at level '1' when at least two data can be written in the Transmit FIFO"]
    #[inline(always)]
    pub fn two_data(self) -> &'a mut crate::W<REG> {
        self.variant(Txrdymselect::TwoData)
    }
    #[doc = "TXRDY will be at level '1' when at least four data can be written in the Transmit FIFO"]
    #[inline(always)]
    pub fn four_data(self) -> &'a mut crate::W<REG> {
        self.variant(Txrdymselect::FourData)
    }
}
#[doc = "Receiver Ready Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rxrdymselect {
    #[doc = "0: RXRDY will be at level '1' when at least one unread data is in the Receive FIFO"]
    OneData = 0,
    #[doc = "1: RXRDY will be at level '1' when at least two unread data are in the Receive FIFO"]
    TwoData = 1,
    #[doc = "2: RXRDY will be at level '1' when at least four unread data are in the Receive FIFO"]
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
#[doc = "Field `RXRDYM` reader - Receiver Ready Mode"]
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
    #[doc = "RXRDY will be at level '1' when at least one unread data is in the Receive FIFO"]
    #[inline(always)]
    pub fn is_one_data(&self) -> bool {
        *self == Rxrdymselect::OneData
    }
    #[doc = "RXRDY will be at level '1' when at least two unread data are in the Receive FIFO"]
    #[inline(always)]
    pub fn is_two_data(&self) -> bool {
        *self == Rxrdymselect::TwoData
    }
    #[doc = "RXRDY will be at level '1' when at least four unread data are in the Receive FIFO"]
    #[inline(always)]
    pub fn is_four_data(&self) -> bool {
        *self == Rxrdymselect::FourData
    }
}
#[doc = "Field `RXRDYM` writer - Receiver Ready Mode"]
pub type RxrdymW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rxrdymselect>;
impl<'a, REG> RxrdymW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RXRDY will be at level '1' when at least one unread data is in the Receive FIFO"]
    #[inline(always)]
    pub fn one_data(self) -> &'a mut crate::W<REG> {
        self.variant(Rxrdymselect::OneData)
    }
    #[doc = "RXRDY will be at level '1' when at least two unread data are in the Receive FIFO"]
    #[inline(always)]
    pub fn two_data(self) -> &'a mut crate::W<REG> {
        self.variant(Rxrdymselect::TwoData)
    }
    #[doc = "RXRDY will be at level '1' when at least four unread data are in the Receive FIFO"]
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
    #[doc = "Bits 0:1 - Transmitter Ready Mode"]
    #[inline(always)]
    pub fn txrdym(&self) -> TxrdymR {
        TxrdymR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Receiver Ready Mode"]
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
    #[doc = "Bits 0:1 - Transmitter Ready Mode"]
    #[inline(always)]
    pub fn txrdym(&mut self) -> TxrdymW<FlexTwiFmrSpec> {
        TxrdymW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Receiver Ready Mode"]
    #[inline(always)]
    pub fn rxrdym(&mut self) -> RxrdymW<FlexTwiFmrSpec> {
        RxrdymW::new(self, 4)
    }
    #[doc = "Bits 16:21 - Transmit FIFO Threshold"]
    #[inline(always)]
    pub fn txfthres(&mut self) -> TxfthresW<FlexTwiFmrSpec> {
        TxfthresW::new(self, 16)
    }
    #[doc = "Bits 24:29 - Receive FIFO Threshold"]
    #[inline(always)]
    pub fn rxfthres(&mut self) -> RxfthresW<FlexTwiFmrSpec> {
        RxfthresW::new(self, 24)
    }
}
#[doc = "TWI FIFO Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_twi_fmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_twi_fmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexTwiFmrSpec;
impl crate::RegisterSpec for FlexTwiFmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flex_twi_fmr::R`](R) reader structure"]
impl crate::Readable for FlexTwiFmrSpec {}
#[doc = "`write(|w| ..)` method takes [`flex_twi_fmr::W`](W) writer structure"]
impl crate::Writable for FlexTwiFmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLEX_TWI_FMR to value 0"]
impl crate::Resettable for FlexTwiFmrSpec {}
