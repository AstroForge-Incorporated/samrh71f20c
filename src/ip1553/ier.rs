#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `EMT` writer - EndMemTransfer"]
pub type EmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MTE` writer - MemTransferErr"]
pub type MteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERX` writer - End reception"]
pub type ErxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETX` writer - EndTransmission"]
pub type EtxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETRANS` writer - EndTransfer"]
pub type EtransW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TE` writer - TransErr"]
pub type TeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCE` writer - TransCodingErr"]
pub type TceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPE` writer - TransParityErr"]
pub type TpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDE` writer - TransDataTypeErr"]
pub type TdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TTE` writer - TransTimeOutErr"]
pub type TteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWE` writer - TransWordCounterErr"]
pub type TweW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BE` writer - BufIFErr"]
pub type BeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITR` writer - IllegalTransferReq"]
pub type ItrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TVR` writer - TransVecWordReq"]
pub type TvrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBR` writer - DynamicBusContReq"]
pub type DbrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STR` writer - InitSelfTestReq"]
pub type StrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSR` writer - TranShutdownReq"]
pub type TsrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSR` writer - OvTranShutdownReq"]
pub type OsrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDR` writer - SyncWithDataReq"]
pub type SdrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWD` writer - SyncWithoutDataReq"]
pub type SwdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RRT` writer - ResetRTReq"]
pub type RrtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITF` writer - InhibitTermFlagReq"]
pub type ItfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTF` writer - OvInhibitTermFlagReq"]
pub type OtfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPB` writer - IPBusy"]
pub type IpbW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - EndMemTransfer"]
    #[inline(always)]
    pub fn emt(&mut self) -> EmtW<IerSpec> {
        EmtW::new(self, 0)
    }
    #[doc = "Bit 1 - MemTransferErr"]
    #[inline(always)]
    pub fn mte(&mut self) -> MteW<IerSpec> {
        MteW::new(self, 1)
    }
    #[doc = "Bit 2 - End reception"]
    #[inline(always)]
    pub fn erx(&mut self) -> ErxW<IerSpec> {
        ErxW::new(self, 2)
    }
    #[doc = "Bit 3 - EndTransmission"]
    #[inline(always)]
    pub fn etx(&mut self) -> EtxW<IerSpec> {
        EtxW::new(self, 3)
    }
    #[doc = "Bits 4:5 - EndTransfer"]
    #[inline(always)]
    pub fn etrans(&mut self) -> EtransW<IerSpec> {
        EtransW::new(self, 4)
    }
    #[doc = "Bit 6 - TransErr"]
    #[inline(always)]
    pub fn te(&mut self) -> TeW<IerSpec> {
        TeW::new(self, 6)
    }
    #[doc = "Bit 7 - TransCodingErr"]
    #[inline(always)]
    pub fn tce(&mut self) -> TceW<IerSpec> {
        TceW::new(self, 7)
    }
    #[doc = "Bit 8 - TransParityErr"]
    #[inline(always)]
    pub fn tpe(&mut self) -> TpeW<IerSpec> {
        TpeW::new(self, 8)
    }
    #[doc = "Bit 9 - TransDataTypeErr"]
    #[inline(always)]
    pub fn tde(&mut self) -> TdeW<IerSpec> {
        TdeW::new(self, 9)
    }
    #[doc = "Bit 10 - TransTimeOutErr"]
    #[inline(always)]
    pub fn tte(&mut self) -> TteW<IerSpec> {
        TteW::new(self, 10)
    }
    #[doc = "Bit 11 - TransWordCounterErr"]
    #[inline(always)]
    pub fn twe(&mut self) -> TweW<IerSpec> {
        TweW::new(self, 11)
    }
    #[doc = "Bit 12 - BufIFErr"]
    #[inline(always)]
    pub fn be(&mut self) -> BeW<IerSpec> {
        BeW::new(self, 12)
    }
    #[doc = "Bit 13 - IllegalTransferReq"]
    #[inline(always)]
    pub fn itr(&mut self) -> ItrW<IerSpec> {
        ItrW::new(self, 13)
    }
    #[doc = "Bit 14 - TransVecWordReq"]
    #[inline(always)]
    pub fn tvr(&mut self) -> TvrW<IerSpec> {
        TvrW::new(self, 14)
    }
    #[doc = "Bit 15 - DynamicBusContReq"]
    #[inline(always)]
    pub fn dbr(&mut self) -> DbrW<IerSpec> {
        DbrW::new(self, 15)
    }
    #[doc = "Bit 16 - InitSelfTestReq"]
    #[inline(always)]
    pub fn str(&mut self) -> StrW<IerSpec> {
        StrW::new(self, 16)
    }
    #[doc = "Bit 17 - TranShutdownReq"]
    #[inline(always)]
    pub fn tsr(&mut self) -> TsrW<IerSpec> {
        TsrW::new(self, 17)
    }
    #[doc = "Bit 18 - OvTranShutdownReq"]
    #[inline(always)]
    pub fn osr(&mut self) -> OsrW<IerSpec> {
        OsrW::new(self, 18)
    }
    #[doc = "Bit 19 - SyncWithDataReq"]
    #[inline(always)]
    pub fn sdr(&mut self) -> SdrW<IerSpec> {
        SdrW::new(self, 19)
    }
    #[doc = "Bit 20 - SyncWithoutDataReq"]
    #[inline(always)]
    pub fn swd(&mut self) -> SwdW<IerSpec> {
        SwdW::new(self, 20)
    }
    #[doc = "Bit 21 - ResetRTReq"]
    #[inline(always)]
    pub fn rrt(&mut self) -> RrtW<IerSpec> {
        RrtW::new(self, 21)
    }
    #[doc = "Bit 22 - InhibitTermFlagReq"]
    #[inline(always)]
    pub fn itf(&mut self) -> ItfW<IerSpec> {
        ItfW::new(self, 22)
    }
    #[doc = "Bit 23 - OvInhibitTermFlagReq"]
    #[inline(always)]
    pub fn otf(&mut self) -> OtfW<IerSpec> {
        OtfW::new(self, 23)
    }
    #[doc = "Bit 24 - IPBusy"]
    #[inline(always)]
    pub fn ipb(&mut self) -> IpbW<IerSpec> {
        IpbW::new(self, 24)
    }
}
#[doc = "IRQ Mask Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IerSpec {}
