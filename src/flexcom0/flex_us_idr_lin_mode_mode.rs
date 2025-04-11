#[doc = "Register `FLEX_US_IDR_LIN_MODE_MODE` writer"]
pub type W = crate::W<FlexUsIdrLinModeModeSpec>;
#[doc = "Field `RXRDY` writer - RXRDY Interrupt Disable"]
pub type RxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRDY` writer - TXRDY Interrupt Disable"]
pub type TxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRE` writer - Overrun Error Interrupt Disable"]
pub type OvreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAME` writer - Framing Error Interrupt Disable"]
pub type FrameW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARE` writer - Parity Error Interrupt Disable"]
pub type PareW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT` writer - Timeout Interrupt Disable"]
pub type TimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEMPTY` writer - TXEMPTY Interrupt Disable"]
pub type TxemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINBK` writer - LIN Break Sent or LIN Break Received Interrupt Disable"]
pub type LinbkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINID` writer - LIN Identifier Sent or LIN Identifier Received Interrupt Disable"]
pub type LinidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINTC` writer - LIN Transfer Completed Interrupt Disable"]
pub type LintcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINBE` writer - LIN Bus Error Interrupt Disable"]
pub type LinbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINISFE` writer - LIN Inconsistent Synch Field Error Interrupt Disable"]
pub type LinisfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINIPE` writer - LIN Identifier Parity Interrupt Disable"]
pub type LinipeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINCE` writer - LIN Checksum Error Interrupt Disable"]
pub type LinceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINSNRE` writer - LIN Slave Not Responding Error Interrupt Disable"]
pub type LinsnreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINSTE` writer - LIN Synch Tolerance Error Interrupt Disable"]
pub type LinsteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINHTE` writer - LIN Header Timeout Error Interrupt Disable"]
pub type LinhteW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - RXRDY Interrupt Disable"]
    #[inline(always)]
    pub fn rxrdy(&mut self) -> RxrdyW<FlexUsIdrLinModeModeSpec> {
        RxrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - TXRDY Interrupt Disable"]
    #[inline(always)]
    pub fn txrdy(&mut self) -> TxrdyW<FlexUsIdrLinModeModeSpec> {
        TxrdyW::new(self, 1)
    }
    #[doc = "Bit 5 - Overrun Error Interrupt Disable"]
    #[inline(always)]
    pub fn ovre(&mut self) -> OvreW<FlexUsIdrLinModeModeSpec> {
        OvreW::new(self, 5)
    }
    #[doc = "Bit 6 - Framing Error Interrupt Disable"]
    #[inline(always)]
    pub fn frame(&mut self) -> FrameW<FlexUsIdrLinModeModeSpec> {
        FrameW::new(self, 6)
    }
    #[doc = "Bit 7 - Parity Error Interrupt Disable"]
    #[inline(always)]
    pub fn pare(&mut self) -> PareW<FlexUsIdrLinModeModeSpec> {
        PareW::new(self, 7)
    }
    #[doc = "Bit 8 - Timeout Interrupt Disable"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TimeoutW<FlexUsIdrLinModeModeSpec> {
        TimeoutW::new(self, 8)
    }
    #[doc = "Bit 9 - TXEMPTY Interrupt Disable"]
    #[inline(always)]
    pub fn txempty(&mut self) -> TxemptyW<FlexUsIdrLinModeModeSpec> {
        TxemptyW::new(self, 9)
    }
    #[doc = "Bit 13 - LIN Break Sent or LIN Break Received Interrupt Disable"]
    #[inline(always)]
    pub fn linbk(&mut self) -> LinbkW<FlexUsIdrLinModeModeSpec> {
        LinbkW::new(self, 13)
    }
    #[doc = "Bit 14 - LIN Identifier Sent or LIN Identifier Received Interrupt Disable"]
    #[inline(always)]
    pub fn linid(&mut self) -> LinidW<FlexUsIdrLinModeModeSpec> {
        LinidW::new(self, 14)
    }
    #[doc = "Bit 15 - LIN Transfer Completed Interrupt Disable"]
    #[inline(always)]
    pub fn lintc(&mut self) -> LintcW<FlexUsIdrLinModeModeSpec> {
        LintcW::new(self, 15)
    }
    #[doc = "Bit 25 - LIN Bus Error Interrupt Disable"]
    #[inline(always)]
    pub fn linbe(&mut self) -> LinbeW<FlexUsIdrLinModeModeSpec> {
        LinbeW::new(self, 25)
    }
    #[doc = "Bit 26 - LIN Inconsistent Synch Field Error Interrupt Disable"]
    #[inline(always)]
    pub fn linisfe(&mut self) -> LinisfeW<FlexUsIdrLinModeModeSpec> {
        LinisfeW::new(self, 26)
    }
    #[doc = "Bit 27 - LIN Identifier Parity Interrupt Disable"]
    #[inline(always)]
    pub fn linipe(&mut self) -> LinipeW<FlexUsIdrLinModeModeSpec> {
        LinipeW::new(self, 27)
    }
    #[doc = "Bit 28 - LIN Checksum Error Interrupt Disable"]
    #[inline(always)]
    pub fn lince(&mut self) -> LinceW<FlexUsIdrLinModeModeSpec> {
        LinceW::new(self, 28)
    }
    #[doc = "Bit 29 - LIN Slave Not Responding Error Interrupt Disable"]
    #[inline(always)]
    pub fn linsnre(&mut self) -> LinsnreW<FlexUsIdrLinModeModeSpec> {
        LinsnreW::new(self, 29)
    }
    #[doc = "Bit 30 - LIN Synch Tolerance Error Interrupt Disable"]
    #[inline(always)]
    pub fn linste(&mut self) -> LinsteW<FlexUsIdrLinModeModeSpec> {
        LinsteW::new(self, 30)
    }
    #[doc = "Bit 31 - LIN Header Timeout Error Interrupt Disable"]
    #[inline(always)]
    pub fn linhte(&mut self) -> LinhteW<FlexUsIdrLinModeModeSpec> {
        LinhteW::new(self, 31)
    }
}
#[doc = "USART Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_us_idr_lin_mode_mode::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexUsIdrLinModeModeSpec;
impl crate::RegisterSpec for FlexUsIdrLinModeModeSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`flex_us_idr_lin_mode_mode::W`](W) writer structure"]
impl crate::Writable for FlexUsIdrLinModeModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLEX_US_IDR_LIN_MODE_MODE to value 0"]
impl crate::Resettable for FlexUsIdrLinModeModeSpec {}
