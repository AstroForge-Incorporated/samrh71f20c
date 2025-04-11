#[doc = "Register `FLEX_US_IER` writer"]
pub type W = crate::W<FlexUsIerSpec>;
#[doc = "Field `RXRDY` writer - RXRDY Interrupt Enable"]
pub type RxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRDY` writer - TXRDY Interrupt Enable"]
pub type TxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBRK` writer - Receiver Break Interrupt Enable"]
pub type RxbrkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRE` writer - Overrun Error Interrupt Enable"]
pub type OvreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAME` writer - Framing Error Interrupt Enable"]
pub type FrameW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARE` writer - Parity Error Interrupt Enable"]
pub type PareW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT` writer - Timeout Interrupt Enable"]
pub type TimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEMPTY` writer - TXEMPTY Interrupt Enable"]
pub type TxemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITER` writer - Max number of Repetitions Reached Interrupt Enable"]
pub type IterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK` writer - Non Acknowledge Interrupt Enable"]
pub type NackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSIC` writer - Clear to Send Input Change Interrupt Enable"]
pub type CtsicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP` writer - Comparison Interrupt Enable"]
pub type CmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MANE` writer - Manchester Error Interrupt Enable"]
pub type ManeW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - RXRDY Interrupt Enable"]
    #[inline(always)]
    pub fn rxrdy(&mut self) -> RxrdyW<FlexUsIerSpec> {
        RxrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - TXRDY Interrupt Enable"]
    #[inline(always)]
    pub fn txrdy(&mut self) -> TxrdyW<FlexUsIerSpec> {
        TxrdyW::new(self, 1)
    }
    #[doc = "Bit 2 - Receiver Break Interrupt Enable"]
    #[inline(always)]
    pub fn rxbrk(&mut self) -> RxbrkW<FlexUsIerSpec> {
        RxbrkW::new(self, 2)
    }
    #[doc = "Bit 5 - Overrun Error Interrupt Enable"]
    #[inline(always)]
    pub fn ovre(&mut self) -> OvreW<FlexUsIerSpec> {
        OvreW::new(self, 5)
    }
    #[doc = "Bit 6 - Framing Error Interrupt Enable"]
    #[inline(always)]
    pub fn frame(&mut self) -> FrameW<FlexUsIerSpec> {
        FrameW::new(self, 6)
    }
    #[doc = "Bit 7 - Parity Error Interrupt Enable"]
    #[inline(always)]
    pub fn pare(&mut self) -> PareW<FlexUsIerSpec> {
        PareW::new(self, 7)
    }
    #[doc = "Bit 8 - Timeout Interrupt Enable"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TimeoutW<FlexUsIerSpec> {
        TimeoutW::new(self, 8)
    }
    #[doc = "Bit 9 - TXEMPTY Interrupt Enable"]
    #[inline(always)]
    pub fn txempty(&mut self) -> TxemptyW<FlexUsIerSpec> {
        TxemptyW::new(self, 9)
    }
    #[doc = "Bit 10 - Max number of Repetitions Reached Interrupt Enable"]
    #[inline(always)]
    pub fn iter(&mut self) -> IterW<FlexUsIerSpec> {
        IterW::new(self, 10)
    }
    #[doc = "Bit 13 - Non Acknowledge Interrupt Enable"]
    #[inline(always)]
    pub fn nack(&mut self) -> NackW<FlexUsIerSpec> {
        NackW::new(self, 13)
    }
    #[doc = "Bit 19 - Clear to Send Input Change Interrupt Enable"]
    #[inline(always)]
    pub fn ctsic(&mut self) -> CtsicW<FlexUsIerSpec> {
        CtsicW::new(self, 19)
    }
    #[doc = "Bit 22 - Comparison Interrupt Enable"]
    #[inline(always)]
    pub fn cmp(&mut self) -> CmpW<FlexUsIerSpec> {
        CmpW::new(self, 22)
    }
    #[doc = "Bit 24 - Manchester Error Interrupt Enable"]
    #[inline(always)]
    pub fn mane(&mut self) -> ManeW<FlexUsIerSpec> {
        ManeW::new(self, 24)
    }
}
#[doc = "USART Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_us_ier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexUsIerSpec;
impl crate::RegisterSpec for FlexUsIerSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`flex_us_ier::W`](W) writer structure"]
impl crate::Writable for FlexUsIerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLEX_US_IER to value 0"]
impl crate::Resettable for FlexUsIerSpec {}
