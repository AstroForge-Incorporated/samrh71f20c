#[doc = "Register `FLEX_TWI_IMR` reader"]
pub type R = crate::R<FlexTwiImrSpec>;
#[doc = "Field `TXCOMP` reader - Transmission Completed Interrupt Mask"]
pub type TxcompR = crate::BitReader;
#[doc = "Field `RXRDY` reader - Receive Holding Register Ready Interrupt Mask"]
pub type RxrdyR = crate::BitReader;
#[doc = "Field `TXRDY` reader - Transmit Holding Register Ready Interrupt Mask"]
pub type TxrdyR = crate::BitReader;
#[doc = "Field `SVACC` reader - Slave Access Interrupt Mask"]
pub type SvaccR = crate::BitReader;
#[doc = "Field `GACC` reader - General Call Access Interrupt Mask"]
pub type GaccR = crate::BitReader;
#[doc = "Field `OVRE` reader - Overrun Error Interrupt Mask"]
pub type OvreR = crate::BitReader;
#[doc = "Field `UNRE` reader - Underrun Error Interrupt Mask"]
pub type UnreR = crate::BitReader;
#[doc = "Field `NACK` reader - Not Acknowledge Interrupt Mask"]
pub type NackR = crate::BitReader;
#[doc = "Field `ARBLST` reader - Arbitration Lost Interrupt Mask"]
pub type ArblstR = crate::BitReader;
#[doc = "Field `SCL_WS` reader - Clock Wait State Interrupt Mask"]
pub type SclWsR = crate::BitReader;
#[doc = "Field `EOSACC` reader - End Of Slave Access Interrupt Mask"]
pub type EosaccR = crate::BitReader;
#[doc = "Field `ENDRX` reader - End of Receive Buffer Interrupt Mask"]
pub type EndrxR = crate::BitReader;
#[doc = "Field `ENDTX` reader - End of Transmit Buffer Interrupt Mask"]
pub type EndtxR = crate::BitReader;
#[doc = "Field `RXBUFF` reader - Receive Buffer Full Interrupt Mask"]
pub type RxbuffR = crate::BitReader;
#[doc = "Field `TXBUFE` reader - Transmit Buffer Empty Interrupt Mask"]
pub type TxbufeR = crate::BitReader;
#[doc = "Field `MCACK` reader - Master Code Acknowledge Interrupt Mask"]
pub type McackR = crate::BitReader;
#[doc = "Field `TOUT` reader - Timeout Error Interrupt Mask"]
pub type ToutR = crate::BitReader;
#[doc = "Field `PECERR` reader - PEC Error Interrupt Mask"]
pub type PecerrR = crate::BitReader;
#[doc = "Field `SMBDAM` reader - SMBus Default Address Match Interrupt Mask"]
pub type SmbdamR = crate::BitReader;
#[doc = "Field `SMBHHM` reader - SMBus Host Header Address Match Interrupt Mask"]
pub type SmbhhmR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmission Completed Interrupt Mask"]
    #[inline(always)]
    pub fn txcomp(&self) -> TxcompR {
        TxcompR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Holding Register Ready Interrupt Mask"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RxrdyR {
        RxrdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Holding Register Ready Interrupt Mask"]
    #[inline(always)]
    pub fn txrdy(&self) -> TxrdyR {
        TxrdyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Slave Access Interrupt Mask"]
    #[inline(always)]
    pub fn svacc(&self) -> SvaccR {
        SvaccR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - General Call Access Interrupt Mask"]
    #[inline(always)]
    pub fn gacc(&self) -> GaccR {
        GaccR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Overrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn ovre(&self) -> OvreR {
        OvreR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Underrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn unre(&self) -> UnreR {
        UnreR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Not Acknowledge Interrupt Mask"]
    #[inline(always)]
    pub fn nack(&self) -> NackR {
        NackR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Arbitration Lost Interrupt Mask"]
    #[inline(always)]
    pub fn arblst(&self) -> ArblstR {
        ArblstR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clock Wait State Interrupt Mask"]
    #[inline(always)]
    pub fn scl_ws(&self) -> SclWsR {
        SclWsR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - End Of Slave Access Interrupt Mask"]
    #[inline(always)]
    pub fn eosacc(&self) -> EosaccR {
        EosaccR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - End of Receive Buffer Interrupt Mask"]
    #[inline(always)]
    pub fn endrx(&self) -> EndrxR {
        EndrxR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - End of Transmit Buffer Interrupt Mask"]
    #[inline(always)]
    pub fn endtx(&self) -> EndtxR {
        EndtxR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Receive Buffer Full Interrupt Mask"]
    #[inline(always)]
    pub fn rxbuff(&self) -> RxbuffR {
        RxbuffR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Transmit Buffer Empty Interrupt Mask"]
    #[inline(always)]
    pub fn txbufe(&self) -> TxbufeR {
        TxbufeR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Master Code Acknowledge Interrupt Mask"]
    #[inline(always)]
    pub fn mcack(&self) -> McackR {
        McackR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Timeout Error Interrupt Mask"]
    #[inline(always)]
    pub fn tout(&self) -> ToutR {
        ToutR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PEC Error Interrupt Mask"]
    #[inline(always)]
    pub fn pecerr(&self) -> PecerrR {
        PecerrR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SMBus Default Address Match Interrupt Mask"]
    #[inline(always)]
    pub fn smbdam(&self) -> SmbdamR {
        SmbdamR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SMBus Host Header Address Match Interrupt Mask"]
    #[inline(always)]
    pub fn smbhhm(&self) -> SmbhhmR {
        SmbhhmR::new(((self.bits >> 21) & 1) != 0)
    }
}
#[doc = "TWI Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_twi_imr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexTwiImrSpec;
impl crate::RegisterSpec for FlexTwiImrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flex_twi_imr::R`](R) reader structure"]
impl crate::Readable for FlexTwiImrSpec {}
#[doc = "`reset()` method sets FLEX_TWI_IMR to value 0"]
impl crate::Resettable for FlexTwiImrSpec {}
