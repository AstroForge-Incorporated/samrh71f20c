#[doc = "Register `FLEX_TWI_IER` writer"]
pub type W = crate::W<FlexTwiIerSpec>;
#[doc = "Field `TXCOMP` writer - Transmission Completed Interrupt Enable"]
pub type TxcompW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRDY` writer - Receive Holding Register Ready Interrupt Enable"]
pub type RxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRDY` writer - Transmit Holding Register Ready Interrupt Enable"]
pub type TxrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVACC` writer - Slave Access Interrupt Enable"]
pub type SvaccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GACC` writer - General Call Access Interrupt Enable"]
pub type GaccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRE` writer - Overrun Error Interrupt Enable"]
pub type OvreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNRE` writer - Underrun Error Interrupt Enable"]
pub type UnreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK` writer - Not Acknowledge Interrupt Enable"]
pub type NackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARBLST` writer - Arbitration Lost Interrupt Enable"]
pub type ArblstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCL_WS` writer - Clock Wait State Interrupt Enable"]
pub type SclWsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOSACC` writer - End Of Slave Access Interrupt Enable"]
pub type EosaccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDRX` writer - End of Receive Buffer Interrupt Enable"]
pub type EndrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDTX` writer - End of Transmit Buffer Interrupt Enable"]
pub type EndtxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBUFF` writer - Receive Buffer Full Interrupt Enable"]
pub type RxbuffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBUFE` writer - Transmit Buffer Empty Interrupt Enable"]
pub type TxbufeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCACK` writer - Master Code Acknowledge Interrupt Enable"]
pub type McackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUT` writer - Timeout Error Interrupt Enable"]
pub type ToutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PECERR` writer - PEC Error Interrupt Enable"]
pub type PecerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMBDAM` writer - SMBus Default Address Match Interrupt Enable"]
pub type SmbdamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMBHHM` writer - SMBus Host Header Address Match Interrupt Enable"]
pub type SmbhhmW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Transmission Completed Interrupt Enable"]
    #[inline(always)]
    pub fn txcomp(&mut self) -> TxcompW<FlexTwiIerSpec> {
        TxcompW::new(self, 0)
    }
    #[doc = "Bit 1 - Receive Holding Register Ready Interrupt Enable"]
    #[inline(always)]
    pub fn rxrdy(&mut self) -> RxrdyW<FlexTwiIerSpec> {
        RxrdyW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit Holding Register Ready Interrupt Enable"]
    #[inline(always)]
    pub fn txrdy(&mut self) -> TxrdyW<FlexTwiIerSpec> {
        TxrdyW::new(self, 2)
    }
    #[doc = "Bit 4 - Slave Access Interrupt Enable"]
    #[inline(always)]
    pub fn svacc(&mut self) -> SvaccW<FlexTwiIerSpec> {
        SvaccW::new(self, 4)
    }
    #[doc = "Bit 5 - General Call Access Interrupt Enable"]
    #[inline(always)]
    pub fn gacc(&mut self) -> GaccW<FlexTwiIerSpec> {
        GaccW::new(self, 5)
    }
    #[doc = "Bit 6 - Overrun Error Interrupt Enable"]
    #[inline(always)]
    pub fn ovre(&mut self) -> OvreW<FlexTwiIerSpec> {
        OvreW::new(self, 6)
    }
    #[doc = "Bit 7 - Underrun Error Interrupt Enable"]
    #[inline(always)]
    pub fn unre(&mut self) -> UnreW<FlexTwiIerSpec> {
        UnreW::new(self, 7)
    }
    #[doc = "Bit 8 - Not Acknowledge Interrupt Enable"]
    #[inline(always)]
    pub fn nack(&mut self) -> NackW<FlexTwiIerSpec> {
        NackW::new(self, 8)
    }
    #[doc = "Bit 9 - Arbitration Lost Interrupt Enable"]
    #[inline(always)]
    pub fn arblst(&mut self) -> ArblstW<FlexTwiIerSpec> {
        ArblstW::new(self, 9)
    }
    #[doc = "Bit 10 - Clock Wait State Interrupt Enable"]
    #[inline(always)]
    pub fn scl_ws(&mut self) -> SclWsW<FlexTwiIerSpec> {
        SclWsW::new(self, 10)
    }
    #[doc = "Bit 11 - End Of Slave Access Interrupt Enable"]
    #[inline(always)]
    pub fn eosacc(&mut self) -> EosaccW<FlexTwiIerSpec> {
        EosaccW::new(self, 11)
    }
    #[doc = "Bit 12 - End of Receive Buffer Interrupt Enable"]
    #[inline(always)]
    pub fn endrx(&mut self) -> EndrxW<FlexTwiIerSpec> {
        EndrxW::new(self, 12)
    }
    #[doc = "Bit 13 - End of Transmit Buffer Interrupt Enable"]
    #[inline(always)]
    pub fn endtx(&mut self) -> EndtxW<FlexTwiIerSpec> {
        EndtxW::new(self, 13)
    }
    #[doc = "Bit 14 - Receive Buffer Full Interrupt Enable"]
    #[inline(always)]
    pub fn rxbuff(&mut self) -> RxbuffW<FlexTwiIerSpec> {
        RxbuffW::new(self, 14)
    }
    #[doc = "Bit 15 - Transmit Buffer Empty Interrupt Enable"]
    #[inline(always)]
    pub fn txbufe(&mut self) -> TxbufeW<FlexTwiIerSpec> {
        TxbufeW::new(self, 15)
    }
    #[doc = "Bit 16 - Master Code Acknowledge Interrupt Enable"]
    #[inline(always)]
    pub fn mcack(&mut self) -> McackW<FlexTwiIerSpec> {
        McackW::new(self, 16)
    }
    #[doc = "Bit 18 - Timeout Error Interrupt Enable"]
    #[inline(always)]
    pub fn tout(&mut self) -> ToutW<FlexTwiIerSpec> {
        ToutW::new(self, 18)
    }
    #[doc = "Bit 19 - PEC Error Interrupt Enable"]
    #[inline(always)]
    pub fn pecerr(&mut self) -> PecerrW<FlexTwiIerSpec> {
        PecerrW::new(self, 19)
    }
    #[doc = "Bit 20 - SMBus Default Address Match Interrupt Enable"]
    #[inline(always)]
    pub fn smbdam(&mut self) -> SmbdamW<FlexTwiIerSpec> {
        SmbdamW::new(self, 20)
    }
    #[doc = "Bit 21 - SMBus Host Header Address Match Interrupt Enable"]
    #[inline(always)]
    pub fn smbhhm(&mut self) -> SmbhhmW<FlexTwiIerSpec> {
        SmbhhmW::new(self, 21)
    }
}
#[doc = "TWI Interrupt Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_twi_ier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexTwiIerSpec;
impl crate::RegisterSpec for FlexTwiIerSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`flex_twi_ier::W`](W) writer structure"]
impl crate::Writable for FlexTwiIerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLEX_TWI_IER to value 0"]
impl crate::Resettable for FlexTwiIerSpec {}
