#[doc = "Register `SDR` reader"]
pub type R = crate::R<SdrSpec>;
#[doc = "Register `SDR` writer"]
pub type W = crate::W<SdrSpec>;
#[doc = "Field `TWR` reader - Write Recovery Delay"]
pub type TwrR = crate::FieldReader;
#[doc = "Field `TWR` writer - Write Recovery Delay"]
pub type TwrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRC_TRFC` reader - Row Cycle Delay and Row Refresh Cycle"]
pub type TrcTrfcR = crate::FieldReader;
#[doc = "Field `TRC_TRFC` writer - Row Cycle Delay and Row Refresh Cycle"]
pub type TrcTrfcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRP` reader - Row Precharge Delay"]
pub type TrpR = crate::FieldReader;
#[doc = "Field `TRP` writer - Row Precharge Delay"]
pub type TrpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRCD` reader - Row to Column Delay"]
pub type TrcdR = crate::FieldReader;
#[doc = "Field `TRCD` writer - Row to Column Delay"]
pub type TrcdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRAS` reader - Active to Precharge Delay"]
pub type TrasR = crate::FieldReader;
#[doc = "Field `TRAS` writer - Active to Precharge Delay"]
pub type TrasW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TXSR` reader - Exit Self-Refresh to Active Delay"]
pub type TxsrR = crate::FieldReader;
#[doc = "Field `TXSR` writer - Exit Self-Refresh to Active Delay"]
pub type TxsrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Write Recovery Delay"]
    #[inline(always)]
    pub fn twr(&self) -> TwrR {
        TwrR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Row Cycle Delay and Row Refresh Cycle"]
    #[inline(always)]
    pub fn trc_trfc(&self) -> TrcTrfcR {
        TrcTrfcR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Row Precharge Delay"]
    #[inline(always)]
    pub fn trp(&self) -> TrpR {
        TrpR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Row to Column Delay"]
    #[inline(always)]
    pub fn trcd(&self) -> TrcdR {
        TrcdR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Active to Precharge Delay"]
    #[inline(always)]
    pub fn tras(&self) -> TrasR {
        TrasR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Exit Self-Refresh to Active Delay"]
    #[inline(always)]
    pub fn txsr(&self) -> TxsrR {
        TxsrR::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Write Recovery Delay"]
    #[inline(always)]
    pub fn twr(&mut self) -> TwrW<SdrSpec> {
        TwrW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Row Cycle Delay and Row Refresh Cycle"]
    #[inline(always)]
    pub fn trc_trfc(&mut self) -> TrcTrfcW<SdrSpec> {
        TrcTrfcW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Row Precharge Delay"]
    #[inline(always)]
    pub fn trp(&mut self) -> TrpW<SdrSpec> {
        TrpW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Row to Column Delay"]
    #[inline(always)]
    pub fn trcd(&mut self) -> TrcdW<SdrSpec> {
        TrcdW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Active to Precharge Delay"]
    #[inline(always)]
    pub fn tras(&mut self) -> TrasW<SdrSpec> {
        TrasW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Exit Self-Refresh to Active Delay"]
    #[inline(always)]
    pub fn txsr(&mut self) -> TxsrW<SdrSpec> {
        TxsrW::new(self, 20)
    }
}
#[doc = "Setup Delay Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdrSpec;
impl crate::RegisterSpec for SdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdr::R`](R) reader structure"]
impl crate::Readable for SdrSpec {}
#[doc = "`write(|w| ..)` method takes [`sdr::W`](W) writer structure"]
impl crate::Writable for SdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDR to value 0"]
impl crate::Resettable for SdrSpec {}
