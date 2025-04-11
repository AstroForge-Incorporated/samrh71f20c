#[doc = "Register `FLEX_TWI_CR_FIFO_ENABLED_MODE` writer"]
pub type W = crate::W<FlexTwiCrFifoEnabledModeSpec>;
#[doc = "Field `START` writer - Send a START Condition"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` writer - Send a STOP Condition"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSEN` writer - TWI Master Mode Enabled"]
pub type MsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSDIS` writer - TWI Master Mode Disabled"]
pub type MsdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVEN` writer - TWI Slave Mode Enabled"]
pub type SvenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVDIS` writer - TWI Slave Mode Disabled"]
pub type SvdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QUICK` writer - SMBus Quick Command"]
pub type QuickW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSEN` writer - TWI High-Speed Mode Enabled"]
pub type HsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSDIS` writer - TWI High-Speed Mode Disabled"]
pub type HsdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMBEN` writer - SMBus Mode Enabled"]
pub type SmbenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMBDIS` writer - SMBus Mode Disabled"]
pub type SmbdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PECEN` writer - Packet Error Checking Enable"]
pub type PecenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PECDIS` writer - Packet Error Checking Disable"]
pub type PecdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PECRQ` writer - PEC Request"]
pub type PecrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR` writer - Bus CLEAR Command"]
pub type ClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMEN` writer - Alternative Command Mode Enable"]
pub type AcmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMDIS` writer - Alternative Command Mode Disable"]
pub type AcmdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFCLR` writer - Transmit FIFO Clear"]
pub type TxfclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFCLR` writer - Receive FIFO Clear"]
pub type RxfclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFLCLR` writer - Transmit FIFO Lock CLEAR"]
pub type TxflclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFOEN` writer - FIFO Enable"]
pub type FifoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFODIS` writer - FIFO Disable"]
pub type FifodisW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Send a START Condition"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<FlexTwiCrFifoEnabledModeSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bit 1 - Send a STOP Condition"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<FlexTwiCrFifoEnabledModeSpec> {
        StopW::new(self, 1)
    }
    #[doc = "Bit 2 - TWI Master Mode Enabled"]
    #[inline(always)]
    pub fn msen(&mut self) -> MsenW<FlexTwiCrFifoEnabledModeSpec> {
        MsenW::new(self, 2)
    }
    #[doc = "Bit 3 - TWI Master Mode Disabled"]
    #[inline(always)]
    pub fn msdis(&mut self) -> MsdisW<FlexTwiCrFifoEnabledModeSpec> {
        MsdisW::new(self, 3)
    }
    #[doc = "Bit 4 - TWI Slave Mode Enabled"]
    #[inline(always)]
    pub fn sven(&mut self) -> SvenW<FlexTwiCrFifoEnabledModeSpec> {
        SvenW::new(self, 4)
    }
    #[doc = "Bit 5 - TWI Slave Mode Disabled"]
    #[inline(always)]
    pub fn svdis(&mut self) -> SvdisW<FlexTwiCrFifoEnabledModeSpec> {
        SvdisW::new(self, 5)
    }
    #[doc = "Bit 6 - SMBus Quick Command"]
    #[inline(always)]
    pub fn quick(&mut self) -> QuickW<FlexTwiCrFifoEnabledModeSpec> {
        QuickW::new(self, 6)
    }
    #[doc = "Bit 7 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SwrstW<FlexTwiCrFifoEnabledModeSpec> {
        SwrstW::new(self, 7)
    }
    #[doc = "Bit 8 - TWI High-Speed Mode Enabled"]
    #[inline(always)]
    pub fn hsen(&mut self) -> HsenW<FlexTwiCrFifoEnabledModeSpec> {
        HsenW::new(self, 8)
    }
    #[doc = "Bit 9 - TWI High-Speed Mode Disabled"]
    #[inline(always)]
    pub fn hsdis(&mut self) -> HsdisW<FlexTwiCrFifoEnabledModeSpec> {
        HsdisW::new(self, 9)
    }
    #[doc = "Bit 10 - SMBus Mode Enabled"]
    #[inline(always)]
    pub fn smben(&mut self) -> SmbenW<FlexTwiCrFifoEnabledModeSpec> {
        SmbenW::new(self, 10)
    }
    #[doc = "Bit 11 - SMBus Mode Disabled"]
    #[inline(always)]
    pub fn smbdis(&mut self) -> SmbdisW<FlexTwiCrFifoEnabledModeSpec> {
        SmbdisW::new(self, 11)
    }
    #[doc = "Bit 12 - Packet Error Checking Enable"]
    #[inline(always)]
    pub fn pecen(&mut self) -> PecenW<FlexTwiCrFifoEnabledModeSpec> {
        PecenW::new(self, 12)
    }
    #[doc = "Bit 13 - Packet Error Checking Disable"]
    #[inline(always)]
    pub fn pecdis(&mut self) -> PecdisW<FlexTwiCrFifoEnabledModeSpec> {
        PecdisW::new(self, 13)
    }
    #[doc = "Bit 14 - PEC Request"]
    #[inline(always)]
    pub fn pecrq(&mut self) -> PecrqW<FlexTwiCrFifoEnabledModeSpec> {
        PecrqW::new(self, 14)
    }
    #[doc = "Bit 15 - Bus CLEAR Command"]
    #[inline(always)]
    pub fn clear(&mut self) -> ClearW<FlexTwiCrFifoEnabledModeSpec> {
        ClearW::new(self, 15)
    }
    #[doc = "Bit 16 - Alternative Command Mode Enable"]
    #[inline(always)]
    pub fn acmen(&mut self) -> AcmenW<FlexTwiCrFifoEnabledModeSpec> {
        AcmenW::new(self, 16)
    }
    #[doc = "Bit 17 - Alternative Command Mode Disable"]
    #[inline(always)]
    pub fn acmdis(&mut self) -> AcmdisW<FlexTwiCrFifoEnabledModeSpec> {
        AcmdisW::new(self, 17)
    }
    #[doc = "Bit 24 - Transmit FIFO Clear"]
    #[inline(always)]
    pub fn txfclr(&mut self) -> TxfclrW<FlexTwiCrFifoEnabledModeSpec> {
        TxfclrW::new(self, 24)
    }
    #[doc = "Bit 25 - Receive FIFO Clear"]
    #[inline(always)]
    pub fn rxfclr(&mut self) -> RxfclrW<FlexTwiCrFifoEnabledModeSpec> {
        RxfclrW::new(self, 25)
    }
    #[doc = "Bit 26 - Transmit FIFO Lock CLEAR"]
    #[inline(always)]
    pub fn txflclr(&mut self) -> TxflclrW<FlexTwiCrFifoEnabledModeSpec> {
        TxflclrW::new(self, 26)
    }
    #[doc = "Bit 28 - FIFO Enable"]
    #[inline(always)]
    pub fn fifoen(&mut self) -> FifoenW<FlexTwiCrFifoEnabledModeSpec> {
        FifoenW::new(self, 28)
    }
    #[doc = "Bit 29 - FIFO Disable"]
    #[inline(always)]
    pub fn fifodis(&mut self) -> FifodisW<FlexTwiCrFifoEnabledModeSpec> {
        FifodisW::new(self, 29)
    }
}
#[doc = "TWI Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_twi_cr_fifo_enabled_mode::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexTwiCrFifoEnabledModeSpec;
impl crate::RegisterSpec for FlexTwiCrFifoEnabledModeSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`flex_twi_cr_fifo_enabled_mode::W`](W) writer structure"]
impl crate::Writable for FlexTwiCrFifoEnabledModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLEX_TWI_CR_FIFO_ENABLED_MODE to value 0"]
impl crate::Resettable for FlexTwiCrFifoEnabledModeSpec {}
