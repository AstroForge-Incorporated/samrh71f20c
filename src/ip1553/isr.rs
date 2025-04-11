#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `EMT` reader - EndMemTransfer"]
pub type EmtR = crate::BitReader;
#[doc = "Field `MTE` reader - MemTransferErr"]
pub type MteR = crate::BitReader;
#[doc = "Field `ERX` reader - End reception"]
pub type ErxR = crate::BitReader;
#[doc = "Field `ETX` reader - EndTransmission"]
pub type EtxR = crate::BitReader;
#[doc = "Field `ETRANS` reader - EndTransfer"]
pub type EtransR = crate::FieldReader;
#[doc = "Field `TE` reader - TransErr"]
pub type TeR = crate::BitReader;
#[doc = "Field `TCE` reader - TransCodingErr"]
pub type TceR = crate::BitReader;
#[doc = "Field `TPE` reader - TransParityErr"]
pub type TpeR = crate::BitReader;
#[doc = "Field `TDE` reader - TransDataTypeErr"]
pub type TdeR = crate::BitReader;
#[doc = "Field `TTE` reader - TransTimeOutErr"]
pub type TteR = crate::BitReader;
#[doc = "Field `TWE` reader - TransWordCounterErr"]
pub type TweR = crate::BitReader;
#[doc = "Field `BE` reader - BufIFErr"]
pub type BeR = crate::BitReader;
#[doc = "Field `ITR` reader - IllegalTransferReq"]
pub type ItrR = crate::BitReader;
#[doc = "Field `TVR` reader - TransVecWordReq"]
pub type TvrR = crate::BitReader;
#[doc = "Field `DBR` reader - DynamicBusContReq"]
pub type DbrR = crate::BitReader;
#[doc = "Field `STR` reader - InitSelfTestReq"]
pub type StrR = crate::BitReader;
#[doc = "Field `TSR` reader - TranShutdownReq"]
pub type TsrR = crate::BitReader;
#[doc = "Field `OSR` reader - OvTranShutdownReq"]
pub type OsrR = crate::BitReader;
#[doc = "Field `SDR` reader - SyncWithDataReq"]
pub type SdrR = crate::BitReader;
#[doc = "Field `SWD` reader - SyncWithoutDataReq"]
pub type SwdR = crate::BitReader;
#[doc = "Field `RRT` reader - ResetRTReq"]
pub type RrtR = crate::BitReader;
#[doc = "Field `ITF` reader - InhibitTermFlagReq"]
pub type ItfR = crate::BitReader;
#[doc = "Field `OTF` reader - OvInhibitTermFlagReq"]
pub type OtfR = crate::BitReader;
#[doc = "Field `IPB` reader - IPBusy"]
pub type IpbR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - EndMemTransfer"]
    #[inline(always)]
    pub fn emt(&self) -> EmtR {
        EmtR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MemTransferErr"]
    #[inline(always)]
    pub fn mte(&self) -> MteR {
        MteR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End reception"]
    #[inline(always)]
    pub fn erx(&self) -> ErxR {
        ErxR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EndTransmission"]
    #[inline(always)]
    pub fn etx(&self) -> EtxR {
        EtxR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - EndTransfer"]
    #[inline(always)]
    pub fn etrans(&self) -> EtransR {
        EtransR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - TransErr"]
    #[inline(always)]
    pub fn te(&self) -> TeR {
        TeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TransCodingErr"]
    #[inline(always)]
    pub fn tce(&self) -> TceR {
        TceR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TransParityErr"]
    #[inline(always)]
    pub fn tpe(&self) -> TpeR {
        TpeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TransDataTypeErr"]
    #[inline(always)]
    pub fn tde(&self) -> TdeR {
        TdeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TransTimeOutErr"]
    #[inline(always)]
    pub fn tte(&self) -> TteR {
        TteR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TransWordCounterErr"]
    #[inline(always)]
    pub fn twe(&self) -> TweR {
        TweR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - BufIFErr"]
    #[inline(always)]
    pub fn be(&self) -> BeR {
        BeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - IllegalTransferReq"]
    #[inline(always)]
    pub fn itr(&self) -> ItrR {
        ItrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TransVecWordReq"]
    #[inline(always)]
    pub fn tvr(&self) -> TvrR {
        TvrR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DynamicBusContReq"]
    #[inline(always)]
    pub fn dbr(&self) -> DbrR {
        DbrR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - InitSelfTestReq"]
    #[inline(always)]
    pub fn str(&self) -> StrR {
        StrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TranShutdownReq"]
    #[inline(always)]
    pub fn tsr(&self) -> TsrR {
        TsrR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - OvTranShutdownReq"]
    #[inline(always)]
    pub fn osr(&self) -> OsrR {
        OsrR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SyncWithDataReq"]
    #[inline(always)]
    pub fn sdr(&self) -> SdrR {
        SdrR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SyncWithoutDataReq"]
    #[inline(always)]
    pub fn swd(&self) -> SwdR {
        SwdR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ResetRTReq"]
    #[inline(always)]
    pub fn rrt(&self) -> RrtR {
        RrtR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - InhibitTermFlagReq"]
    #[inline(always)]
    pub fn itf(&self) -> ItfR {
        ItfR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - OvInhibitTermFlagReq"]
    #[inline(always)]
    pub fn otf(&self) -> OtfR {
        OtfR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - IPBusy"]
    #[inline(always)]
    pub fn ipb(&self) -> IpbR {
        IpbR::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "IRQ Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {}
