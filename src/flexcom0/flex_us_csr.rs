#[doc = "Register `FLEX_US_CSR` reader"]
pub type R = crate::R<FlexUsCsrSpec>;
#[doc = "Field `RXRDY` reader - Receiver Ready (cleared by reading FLEX_US_RHR)"]
pub type RxrdyR = crate::BitReader;
#[doc = "Field `TXRDY` reader - Transmitter Ready (cleared by writing FLEX_US_THR)"]
pub type TxrdyR = crate::BitReader;
#[doc = "Field `RXBRK` reader - Break Received/End of Break"]
pub type RxbrkR = crate::BitReader;
#[doc = "Field `OVRE` reader - Overrun Error"]
pub type OvreR = crate::BitReader;
#[doc = "Field `FRAME` reader - Framing Error"]
pub type FrameR = crate::BitReader;
#[doc = "Field `PARE` reader - Parity Error"]
pub type PareR = crate::BitReader;
#[doc = "Field `TIMEOUT` reader - Receiver Timeout"]
pub type TimeoutR = crate::BitReader;
#[doc = "Field `TXEMPTY` reader - Transmitter Empty (cleared by writing FLEX_US_THR)"]
pub type TxemptyR = crate::BitReader;
#[doc = "Field `ITER` reader - Max Number of Repetitions Reached"]
pub type IterR = crate::BitReader;
#[doc = "Field `NACK` reader - Non Acknowledge Interrupt"]
pub type NackR = crate::BitReader;
#[doc = "Field `CTSIC` reader - Clear to Send Input Change Flag"]
pub type CtsicR = crate::BitReader;
#[doc = "Field `CMP` reader - Comparison Status"]
pub type CmpR = crate::BitReader;
#[doc = "Field `CTS` reader - Image of CTS Input"]
pub type CtsR = crate::BitReader;
#[doc = "Field `MANE` reader - Manchester Error"]
pub type ManeR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Receiver Ready (cleared by reading FLEX_US_RHR)"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RxrdyR {
        RxrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmitter Ready (cleared by writing FLEX_US_THR)"]
    #[inline(always)]
    pub fn txrdy(&self) -> TxrdyR {
        TxrdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Break Received/End of Break"]
    #[inline(always)]
    pub fn rxbrk(&self) -> RxbrkR {
        RxbrkR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Overrun Error"]
    #[inline(always)]
    pub fn ovre(&self) -> OvreR {
        OvreR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Framing Error"]
    #[inline(always)]
    pub fn frame(&self) -> FrameR {
        FrameR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Parity Error"]
    #[inline(always)]
    pub fn pare(&self) -> PareR {
        PareR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receiver Timeout"]
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmitter Empty (cleared by writing FLEX_US_THR)"]
    #[inline(always)]
    pub fn txempty(&self) -> TxemptyR {
        TxemptyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Max Number of Repetitions Reached"]
    #[inline(always)]
    pub fn iter(&self) -> IterR {
        IterR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Non Acknowledge Interrupt"]
    #[inline(always)]
    pub fn nack(&self) -> NackR {
        NackR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 19 - Clear to Send Input Change Flag"]
    #[inline(always)]
    pub fn ctsic(&self) -> CtsicR {
        CtsicR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 22 - Comparison Status"]
    #[inline(always)]
    pub fn cmp(&self) -> CmpR {
        CmpR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Image of CTS Input"]
    #[inline(always)]
    pub fn cts(&self) -> CtsR {
        CtsR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Manchester Error"]
    #[inline(always)]
    pub fn mane(&self) -> ManeR {
        ManeR::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "USART Channel Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_csr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexUsCsrSpec;
impl crate::RegisterSpec for FlexUsCsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flex_us_csr::R`](R) reader structure"]
impl crate::Readable for FlexUsCsrSpec {}
#[doc = "`reset()` method sets FLEX_US_CSR to value 0"]
impl crate::Resettable for FlexUsCsrSpec {}
