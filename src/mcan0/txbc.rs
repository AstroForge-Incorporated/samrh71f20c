#[doc = "Register `TXBC` reader"]
pub type R = crate::R<TxbcSpec>;
#[doc = "Register `TXBC` writer"]
pub type W = crate::W<TxbcSpec>;
#[doc = "Field `TBSA` reader - Tx Buffers Start Address"]
pub type TbsaR = crate::FieldReader<u16>;
#[doc = "Field `TBSA` writer - Tx Buffers Start Address"]
pub type TbsaW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `NDTB` reader - Number of Dedicated Transmit Buffers"]
pub type NdtbR = crate::FieldReader;
#[doc = "Field `NDTB` writer - Number of Dedicated Transmit Buffers"]
pub type NdtbW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TFQS` reader - Transmit FIFO/Queue Size"]
pub type TfqsR = crate::FieldReader;
#[doc = "Field `TFQS` writer - Transmit FIFO/Queue Size"]
pub type TfqsW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TFQM` reader - Tx FIFO/Queue Mode"]
pub type TfqmR = crate::BitReader;
#[doc = "Field `TFQM` writer - Tx FIFO/Queue Mode"]
pub type TfqmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 2:15 - Tx Buffers Start Address"]
    #[inline(always)]
    pub fn tbsa(&self) -> TbsaR {
        TbsaR::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:21 - Number of Dedicated Transmit Buffers"]
    #[inline(always)]
    pub fn ndtb(&self) -> NdtbR {
        NdtbR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Transmit FIFO/Queue Size"]
    #[inline(always)]
    pub fn tfqs(&self) -> TfqsR {
        TfqsR::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - Tx FIFO/Queue Mode"]
    #[inline(always)]
    pub fn tfqm(&self) -> TfqmR {
        TfqmR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:15 - Tx Buffers Start Address"]
    #[inline(always)]
    pub fn tbsa(&mut self) -> TbsaW<TxbcSpec> {
        TbsaW::new(self, 2)
    }
    #[doc = "Bits 16:21 - Number of Dedicated Transmit Buffers"]
    #[inline(always)]
    pub fn ndtb(&mut self) -> NdtbW<TxbcSpec> {
        NdtbW::new(self, 16)
    }
    #[doc = "Bits 24:29 - Transmit FIFO/Queue Size"]
    #[inline(always)]
    pub fn tfqs(&mut self) -> TfqsW<TxbcSpec> {
        TfqsW::new(self, 24)
    }
    #[doc = "Bit 30 - Tx FIFO/Queue Mode"]
    #[inline(always)]
    pub fn tfqm(&mut self) -> TfqmW<TxbcSpec> {
        TfqmW::new(self, 30)
    }
}
#[doc = "Transmit Buffer Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txbc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxbcSpec;
impl crate::RegisterSpec for TxbcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbc::R`](R) reader structure"]
impl crate::Readable for TxbcSpec {}
#[doc = "`write(|w| ..)` method takes [`txbc::W`](W) writer structure"]
impl crate::Writable for TxbcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXBC to value 0"]
impl crate::Resettable for TxbcSpec {}
