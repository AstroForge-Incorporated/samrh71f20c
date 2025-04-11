#[doc = "Register `FLEX_US_CR` writer"]
pub type W = crate::W<FlexUsCrSpec>;
#[doc = "Field `RSTRX` writer - Reset Receiver"]
pub type RstrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTTX` writer - Reset Transmitter"]
pub type RsttxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXEN` writer - Receiver Enable"]
pub type RxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDIS` writer - Receiver Disable"]
pub type RxdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEN` writer - Transmitter Enable"]
pub type TxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDIS` writer - Transmitter Disable"]
pub type TxdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTSTA` writer - Reset Status Bits"]
pub type RststaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STTBRK` writer - Start Break"]
pub type SttbrkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STPBRK` writer - Stop Break"]
pub type StpbrkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STTTO` writer - Clear TIMEOUT Flag and Start Timeout After Next Character Received"]
pub type StttoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SENDA` writer - Send Address"]
pub type SendaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTIT` writer - Reset Iterations"]
pub type RstitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTNACK` writer - Reset Non Acknowledge"]
pub type RstnackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETTO` writer - Start Timeout Immediately"]
pub type RettoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSEN` writer - Request to Send Enable"]
pub type RtsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSDIS` writer - Request to Send Disable"]
pub type RtsdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINABT` writer - Abort LIN Transmission"]
pub type LinabtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINWKUP` writer - Send LIN Wakeup Signal"]
pub type LinwkupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFCLR` writer - Transmit FIFO Clear"]
pub type TxfclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFCLR` writer - Receive FIFO Clear"]
pub type RxfclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFLCLR` writer - Transmit FIFO Lock CLEAR"]
pub type TxflclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REQCLR` writer - Request to Clear the Comparison Trigger"]
pub type ReqclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOEN` writer - FIFO Enable"]
pub type FifoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFODIS` writer - FIFO Disable"]
pub type FifodisW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 2 - Reset Receiver"]
    #[inline(always)]
    pub fn rstrx(&mut self) -> RstrxW<FlexUsCrSpec> {
        RstrxW::new(self, 2)
    }
    #[doc = "Bit 3 - Reset Transmitter"]
    #[inline(always)]
    pub fn rsttx(&mut self) -> RsttxW<FlexUsCrSpec> {
        RsttxW::new(self, 3)
    }
    #[doc = "Bit 4 - Receiver Enable"]
    #[inline(always)]
    pub fn rxen(&mut self) -> RxenW<FlexUsCrSpec> {
        RxenW::new(self, 4)
    }
    #[doc = "Bit 5 - Receiver Disable"]
    #[inline(always)]
    pub fn rxdis(&mut self) -> RxdisW<FlexUsCrSpec> {
        RxdisW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmitter Enable"]
    #[inline(always)]
    pub fn txen(&mut self) -> TxenW<FlexUsCrSpec> {
        TxenW::new(self, 6)
    }
    #[doc = "Bit 7 - Transmitter Disable"]
    #[inline(always)]
    pub fn txdis(&mut self) -> TxdisW<FlexUsCrSpec> {
        TxdisW::new(self, 7)
    }
    #[doc = "Bit 8 - Reset Status Bits"]
    #[inline(always)]
    pub fn rststa(&mut self) -> RststaW<FlexUsCrSpec> {
        RststaW::new(self, 8)
    }
    #[doc = "Bit 9 - Start Break"]
    #[inline(always)]
    pub fn sttbrk(&mut self) -> SttbrkW<FlexUsCrSpec> {
        SttbrkW::new(self, 9)
    }
    #[doc = "Bit 10 - Stop Break"]
    #[inline(always)]
    pub fn stpbrk(&mut self) -> StpbrkW<FlexUsCrSpec> {
        StpbrkW::new(self, 10)
    }
    #[doc = "Bit 11 - Clear TIMEOUT Flag and Start Timeout After Next Character Received"]
    #[inline(always)]
    pub fn sttto(&mut self) -> StttoW<FlexUsCrSpec> {
        StttoW::new(self, 11)
    }
    #[doc = "Bit 12 - Send Address"]
    #[inline(always)]
    pub fn senda(&mut self) -> SendaW<FlexUsCrSpec> {
        SendaW::new(self, 12)
    }
    #[doc = "Bit 13 - Reset Iterations"]
    #[inline(always)]
    pub fn rstit(&mut self) -> RstitW<FlexUsCrSpec> {
        RstitW::new(self, 13)
    }
    #[doc = "Bit 14 - Reset Non Acknowledge"]
    #[inline(always)]
    pub fn rstnack(&mut self) -> RstnackW<FlexUsCrSpec> {
        RstnackW::new(self, 14)
    }
    #[doc = "Bit 15 - Start Timeout Immediately"]
    #[inline(always)]
    pub fn retto(&mut self) -> RettoW<FlexUsCrSpec> {
        RettoW::new(self, 15)
    }
    #[doc = "Bit 18 - Request to Send Enable"]
    #[inline(always)]
    pub fn rtsen(&mut self) -> RtsenW<FlexUsCrSpec> {
        RtsenW::new(self, 18)
    }
    #[doc = "Bit 19 - Request to Send Disable"]
    #[inline(always)]
    pub fn rtsdis(&mut self) -> RtsdisW<FlexUsCrSpec> {
        RtsdisW::new(self, 19)
    }
    #[doc = "Bit 20 - Abort LIN Transmission"]
    #[inline(always)]
    pub fn linabt(&mut self) -> LinabtW<FlexUsCrSpec> {
        LinabtW::new(self, 20)
    }
    #[doc = "Bit 21 - Send LIN Wakeup Signal"]
    #[inline(always)]
    pub fn linwkup(&mut self) -> LinwkupW<FlexUsCrSpec> {
        LinwkupW::new(self, 21)
    }
    #[doc = "Bit 24 - Transmit FIFO Clear"]
    #[inline(always)]
    pub fn txfclr(&mut self) -> TxfclrW<FlexUsCrSpec> {
        TxfclrW::new(self, 24)
    }
    #[doc = "Bit 25 - Receive FIFO Clear"]
    #[inline(always)]
    pub fn rxfclr(&mut self) -> RxfclrW<FlexUsCrSpec> {
        RxfclrW::new(self, 25)
    }
    #[doc = "Bit 26 - Transmit FIFO Lock CLEAR"]
    #[inline(always)]
    pub fn txflclr(&mut self) -> TxflclrW<FlexUsCrSpec> {
        TxflclrW::new(self, 26)
    }
    #[doc = "Bit 28 - Request to Clear the Comparison Trigger"]
    #[inline(always)]
    pub fn reqclr(&mut self) -> ReqclrW<FlexUsCrSpec> {
        ReqclrW::new(self, 28)
    }
    #[doc = "Bit 30 - FIFO Enable"]
    #[inline(always)]
    pub fn fifoen(&mut self) -> FifoenW<FlexUsCrSpec> {
        FifoenW::new(self, 30)
    }
    #[doc = "Bit 31 - FIFO Disable"]
    #[inline(always)]
    pub fn fifodis(&mut self) -> FifodisW<FlexUsCrSpec> {
        FifodisW::new(self, 31)
    }
}
#[doc = "USART Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_us_cr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexUsCrSpec;
impl crate::RegisterSpec for FlexUsCrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`flex_us_cr::W`](W) writer structure"]
impl crate::Writable for FlexUsCrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLEX_US_CR to value 0"]
impl crate::Resettable for FlexUsCrSpec {}
