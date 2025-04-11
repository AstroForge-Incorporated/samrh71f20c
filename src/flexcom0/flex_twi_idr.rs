#[doc = "Register `FLEX_TWI_IDR` writer"]
pub type W = crate::W<FlexTwiIdrSpec>;
#[doc = "Field `TXCOMP` writer - Transmission Completed Interrupt Disable"]
pub type TxcompW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRDY` writer - Receive Holding Register Ready Interrupt Disable"]
pub type RxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRDY` writer - Transmit Holding Register Ready Interrupt Disable"]
pub type TxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVACC` writer - Slave Access Interrupt Disable"]
pub type SvaccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GACC` writer - General Call Access Interrupt Disable"]
pub type GaccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRE` writer - Overrun Error Interrupt Disable"]
pub type OvreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNRE` writer - Underrun Error Interrupt Disable"]
pub type UnreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK` writer - Not Acknowledge Interrupt Disable"]
pub type NackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARBLST` writer - Arbitration Lost Interrupt Disable"]
pub type ArblstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCL_WS` writer - Clock Wait State Interrupt Disable"]
pub type SclWsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOSACC` writer - End Of Slave Access Interrupt Disable"]
pub type EosaccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDRX` writer - End of Receive Buffer Interrupt Disable"]
pub type EndrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDTX` writer - End of Transmit Buffer Interrupt Disable"]
pub type EndtxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBUFF` writer - Receive Buffer Full Interrupt Disable"]
pub type RxbuffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBUFE` writer - Transmit Buffer Empty Interrupt Disable"]
pub type TxbufeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCACK` writer - Master Code Acknowledge Interrupt Disable"]
pub type McackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUT` writer - Timeout Error Interrupt Disable"]
pub type ToutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PECERR` writer - PEC Error Interrupt Disable"]
pub type PecerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMBDAM` writer - SMBus Default Address Match Interrupt Disable"]
pub type SmbdamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMBHHM` writer - SMBus Host Header Address Match Interrupt Disable"]
pub type SmbhhmW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Transmission Completed Interrupt Disable"]
    #[inline(always)]
    pub fn txcomp(&mut self) -> TxcompW<FlexTwiIdrSpec> {
        TxcompW::new(self, 0)
    }
    #[doc = "Bit 1 - Receive Holding Register Ready Interrupt Disable"]
    #[inline(always)]
    pub fn rxrdy(&mut self) -> RxrdyW<FlexTwiIdrSpec> {
        RxrdyW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit Holding Register Ready Interrupt Disable"]
    #[inline(always)]
    pub fn txrdy(&mut self) -> TxrdyW<FlexTwiIdrSpec> {
        TxrdyW::new(self, 2)
    }
    #[doc = "Bit 4 - Slave Access Interrupt Disable"]
    #[inline(always)]
    pub fn svacc(&mut self) -> SvaccW<FlexTwiIdrSpec> {
        SvaccW::new(self, 4)
    }
    #[doc = "Bit 5 - General Call Access Interrupt Disable"]
    #[inline(always)]
    pub fn gacc(&mut self) -> GaccW<FlexTwiIdrSpec> {
        GaccW::new(self, 5)
    }
    #[doc = "Bit 6 - Overrun Error Interrupt Disable"]
    #[inline(always)]
    pub fn ovre(&mut self) -> OvreW<FlexTwiIdrSpec> {
        OvreW::new(self, 6)
    }
    #[doc = "Bit 7 - Underrun Error Interrupt Disable"]
    #[inline(always)]
    pub fn unre(&mut self) -> UnreW<FlexTwiIdrSpec> {
        UnreW::new(self, 7)
    }
    #[doc = "Bit 8 - Not Acknowledge Interrupt Disable"]
    #[inline(always)]
    pub fn nack(&mut self) -> NackW<FlexTwiIdrSpec> {
        NackW::new(self, 8)
    }
    #[doc = "Bit 9 - Arbitration Lost Interrupt Disable"]
    #[inline(always)]
    pub fn arblst(&mut self) -> ArblstW<FlexTwiIdrSpec> {
        ArblstW::new(self, 9)
    }
    #[doc = "Bit 10 - Clock Wait State Interrupt Disable"]
    #[inline(always)]
    pub fn scl_ws(&mut self) -> SclWsW<FlexTwiIdrSpec> {
        SclWsW::new(self, 10)
    }
    #[doc = "Bit 11 - End Of Slave Access Interrupt Disable"]
    #[inline(always)]
    pub fn eosacc(&mut self) -> EosaccW<FlexTwiIdrSpec> {
        EosaccW::new(self, 11)
    }
    #[doc = "Bit 12 - End of Receive Buffer Interrupt Disable"]
    #[inline(always)]
    pub fn endrx(&mut self) -> EndrxW<FlexTwiIdrSpec> {
        EndrxW::new(self, 12)
    }
    #[doc = "Bit 13 - End of Transmit Buffer Interrupt Disable"]
    #[inline(always)]
    pub fn endtx(&mut self) -> EndtxW<FlexTwiIdrSpec> {
        EndtxW::new(self, 13)
    }
    #[doc = "Bit 14 - Receive Buffer Full Interrupt Disable"]
    #[inline(always)]
    pub fn rxbuff(&mut self) -> RxbuffW<FlexTwiIdrSpec> {
        RxbuffW::new(self, 14)
    }
    #[doc = "Bit 15 - Transmit Buffer Empty Interrupt Disable"]
    #[inline(always)]
    pub fn txbufe(&mut self) -> TxbufeW<FlexTwiIdrSpec> {
        TxbufeW::new(self, 15)
    }
    #[doc = "Bit 16 - Master Code Acknowledge Interrupt Disable"]
    #[inline(always)]
    pub fn mcack(&mut self) -> McackW<FlexTwiIdrSpec> {
        McackW::new(self, 16)
    }
    #[doc = "Bit 18 - Timeout Error Interrupt Disable"]
    #[inline(always)]
    pub fn tout(&mut self) -> ToutW<FlexTwiIdrSpec> {
        ToutW::new(self, 18)
    }
    #[doc = "Bit 19 - PEC Error Interrupt Disable"]
    #[inline(always)]
    pub fn pecerr(&mut self) -> PecerrW<FlexTwiIdrSpec> {
        PecerrW::new(self, 19)
    }
    #[doc = "Bit 20 - SMBus Default Address Match Interrupt Disable"]
    #[inline(always)]
    pub fn smbdam(&mut self) -> SmbdamW<FlexTwiIdrSpec> {
        SmbdamW::new(self, 20)
    }
    #[doc = "Bit 21 - SMBus Host Header Address Match Interrupt Disable"]
    #[inline(always)]
    pub fn smbhhm(&mut self) -> SmbhhmW<FlexTwiIdrSpec> {
        SmbhhmW::new(self, 21)
    }
}
#[doc = "TWI Interrupt Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_twi_idr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexTwiIdrSpec;
impl crate::RegisterSpec for FlexTwiIdrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`flex_twi_idr::W`](W) writer structure"]
impl crate::Writable for FlexTwiIdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLEX_TWI_IDR to value 0"]
impl crate::Resettable for FlexTwiIdrSpec {}
