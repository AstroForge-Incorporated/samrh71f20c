#[doc = "Register `FLEX_US_IDR` writer"]
pub type W = crate::W<FlexUsIdrSpec>;
#[doc = "Field `RXRDY` writer - RXRDY Interrupt Disable"]
pub type RxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRDY` writer - TXRDY Interrupt Disable"]
pub type TxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBRK` writer - Receiver Break Interrupt Disable"]
pub type RxbrkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRE` writer - Overrun Error Interrupt Enable"]
pub type OvreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAME` writer - Framing Error Interrupt Disable"]
pub type FrameW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARE` writer - Parity Error Interrupt Disable"]
pub type PareW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT` writer - Timeout Interrupt Disable"]
pub type TimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEMPTY` writer - TXEMPTY Interrupt Disable"]
pub type TxemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITER` writer - Max Number of Repetitions Reached Interrupt Disable"]
pub type IterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK` writer - Non Acknowledge Interrupt Disable"]
pub type NackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSIC` writer - Clear to Send Input Change Interrupt Disable"]
pub type CtsicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP` writer - Comparison Interrupt Disable"]
pub type CmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MANE` writer - Manchester Error Interrupt Disable"]
pub type ManeW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - RXRDY Interrupt Disable"]
    #[inline(always)]
    pub fn rxrdy(&mut self) -> RxrdyW<FlexUsIdrSpec> {
        RxrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - TXRDY Interrupt Disable"]
    #[inline(always)]
    pub fn txrdy(&mut self) -> TxrdyW<FlexUsIdrSpec> {
        TxrdyW::new(self, 1)
    }
    #[doc = "Bit 2 - Receiver Break Interrupt Disable"]
    #[inline(always)]
    pub fn rxbrk(&mut self) -> RxbrkW<FlexUsIdrSpec> {
        RxbrkW::new(self, 2)
    }
    #[doc = "Bit 5 - Overrun Error Interrupt Enable"]
    #[inline(always)]
    pub fn ovre(&mut self) -> OvreW<FlexUsIdrSpec> {
        OvreW::new(self, 5)
    }
    #[doc = "Bit 6 - Framing Error Interrupt Disable"]
    #[inline(always)]
    pub fn frame(&mut self) -> FrameW<FlexUsIdrSpec> {
        FrameW::new(self, 6)
    }
    #[doc = "Bit 7 - Parity Error Interrupt Disable"]
    #[inline(always)]
    pub fn pare(&mut self) -> PareW<FlexUsIdrSpec> {
        PareW::new(self, 7)
    }
    #[doc = "Bit 8 - Timeout Interrupt Disable"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TimeoutW<FlexUsIdrSpec> {
        TimeoutW::new(self, 8)
    }
    #[doc = "Bit 9 - TXEMPTY Interrupt Disable"]
    #[inline(always)]
    pub fn txempty(&mut self) -> TxemptyW<FlexUsIdrSpec> {
        TxemptyW::new(self, 9)
    }
    #[doc = "Bit 10 - Max Number of Repetitions Reached Interrupt Disable"]
    #[inline(always)]
    pub fn iter(&mut self) -> IterW<FlexUsIdrSpec> {
        IterW::new(self, 10)
    }
    #[doc = "Bit 13 - Non Acknowledge Interrupt Disable"]
    #[inline(always)]
    pub fn nack(&mut self) -> NackW<FlexUsIdrSpec> {
        NackW::new(self, 13)
    }
    #[doc = "Bit 19 - Clear to Send Input Change Interrupt Disable"]
    #[inline(always)]
    pub fn ctsic(&mut self) -> CtsicW<FlexUsIdrSpec> {
        CtsicW::new(self, 19)
    }
    #[doc = "Bit 22 - Comparison Interrupt Disable"]
    #[inline(always)]
    pub fn cmp(&mut self) -> CmpW<FlexUsIdrSpec> {
        CmpW::new(self, 22)
    }
    #[doc = "Bit 24 - Manchester Error Interrupt Disable"]
    #[inline(always)]
    pub fn mane(&mut self) -> ManeW<FlexUsIdrSpec> {
        ManeW::new(self, 24)
    }
}
#[doc = "USART Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_us_idr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexUsIdrSpec;
impl crate::RegisterSpec for FlexUsIdrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`flex_us_idr::W`](W) writer structure"]
impl crate::Writable for FlexUsIdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLEX_US_IDR to value 0"]
impl crate::Resettable for FlexUsIdrSpec {}
