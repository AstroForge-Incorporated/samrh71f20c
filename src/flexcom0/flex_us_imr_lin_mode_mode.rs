#[doc = "Register `FLEX_US_IMR_LIN_MODE_MODE` reader"]
pub type R = crate::R<FlexUsImrLinModeModeSpec>;
#[doc = "Field `RXRDY` reader - RXRDY Interrupt Mask"]
pub type RxrdyR = crate::BitReader;
#[doc = "Field `TXRDY` reader - TXRDY Interrupt Mask"]
pub type TxrdyR = crate::BitReader;
#[doc = "Field `OVRE` reader - Overrun Error Interrupt Mask"]
pub type OvreR = crate::BitReader;
#[doc = "Field `FRAME` reader - Framing Error Interrupt Mask"]
pub type FrameR = crate::BitReader;
#[doc = "Field `PARE` reader - Parity Error Interrupt Mask"]
pub type PareR = crate::BitReader;
#[doc = "Field `TIMEOUT` reader - Timeout Interrupt Mask"]
pub type TimeoutR = crate::BitReader;
#[doc = "Field `TXEMPTY` reader - TXEMPTY Interrupt Mask"]
pub type TxemptyR = crate::BitReader;
#[doc = "Field `LINBK` reader - LIN Break Sent or LIN Break Received Interrupt Mask"]
pub type LinbkR = crate::BitReader;
#[doc = "Field `LINID` reader - LIN Identifier Sent or LIN Identifier Received Interrupt Mask"]
pub type LinidR = crate::BitReader;
#[doc = "Field `LINTC` reader - LIN Transfer Completed Interrupt Mask"]
pub type LintcR = crate::BitReader;
#[doc = "Field `LINBE` reader - LIN Bus Error Interrupt Mask"]
pub type LinbeR = crate::BitReader;
#[doc = "Field `LINISFE` reader - LIN Inconsistent Synch Field Error Interrupt Mask"]
pub type LinisfeR = crate::BitReader;
#[doc = "Field `LINIPE` reader - LIN Identifier Parity Interrupt Mask"]
pub type LinipeR = crate::BitReader;
#[doc = "Field `LINCE` reader - LIN Checksum Error Interrupt Mask"]
pub type LinceR = crate::BitReader;
#[doc = "Field `LINSNRE` reader - LIN Slave Not Responding Error Interrupt Mask"]
pub type LinsnreR = crate::BitReader;
#[doc = "Field `LINSTE` reader - LIN Synch Tolerance Error Interrupt Mask"]
pub type LinsteR = crate::BitReader;
#[doc = "Field `LINHTE` reader - LIN Header Timeout Error Interrupt Mask"]
pub type LinhteR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - RXRDY Interrupt Mask"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RxrdyR {
        RxrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXRDY Interrupt Mask"]
    #[inline(always)]
    pub fn txrdy(&self) -> TxrdyR {
        TxrdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Overrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn ovre(&self) -> OvreR {
        OvreR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Framing Error Interrupt Mask"]
    #[inline(always)]
    pub fn frame(&self) -> FrameR {
        FrameR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Parity Error Interrupt Mask"]
    #[inline(always)]
    pub fn pare(&self) -> PareR {
        PareR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Timeout Interrupt Mask"]
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TXEMPTY Interrupt Mask"]
    #[inline(always)]
    pub fn txempty(&self) -> TxemptyR {
        TxemptyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - LIN Break Sent or LIN Break Received Interrupt Mask"]
    #[inline(always)]
    pub fn linbk(&self) -> LinbkR {
        LinbkR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LIN Identifier Sent or LIN Identifier Received Interrupt Mask"]
    #[inline(always)]
    pub fn linid(&self) -> LinidR {
        LinidR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LIN Transfer Completed Interrupt Mask"]
    #[inline(always)]
    pub fn lintc(&self) -> LintcR {
        LintcR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 25 - LIN Bus Error Interrupt Mask"]
    #[inline(always)]
    pub fn linbe(&self) -> LinbeR {
        LinbeR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - LIN Inconsistent Synch Field Error Interrupt Mask"]
    #[inline(always)]
    pub fn linisfe(&self) -> LinisfeR {
        LinisfeR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - LIN Identifier Parity Interrupt Mask"]
    #[inline(always)]
    pub fn linipe(&self) -> LinipeR {
        LinipeR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - LIN Checksum Error Interrupt Mask"]
    #[inline(always)]
    pub fn lince(&self) -> LinceR {
        LinceR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - LIN Slave Not Responding Error Interrupt Mask"]
    #[inline(always)]
    pub fn linsnre(&self) -> LinsnreR {
        LinsnreR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - LIN Synch Tolerance Error Interrupt Mask"]
    #[inline(always)]
    pub fn linste(&self) -> LinsteR {
        LinsteR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - LIN Header Timeout Error Interrupt Mask"]
    #[inline(always)]
    pub fn linhte(&self) -> LinhteR {
        LinhteR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "USART Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_imr_lin_mode_mode::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexUsImrLinModeModeSpec;
impl crate::RegisterSpec for FlexUsImrLinModeModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flex_us_imr_lin_mode_mode::R`](R) reader structure"]
impl crate::Readable for FlexUsImrLinModeModeSpec {}
#[doc = "`reset()` method sets FLEX_US_IMR_LIN_MODE_MODE to value 0"]
impl crate::Resettable for FlexUsImrLinModeModeSpec {}
