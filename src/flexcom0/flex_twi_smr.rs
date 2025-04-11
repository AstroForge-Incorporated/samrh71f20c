#[doc = "Register `FLEX_TWI_SMR` reader"]
pub type R = crate::R<FlexTwiSmrSpec>;
#[doc = "Register `FLEX_TWI_SMR` writer"]
pub type W = crate::W<FlexTwiSmrSpec>;
#[doc = "Field `NACKEN` reader - Slave Receiver Data Phase NACK Enable"]
pub type NackenR = crate::BitReader;
#[doc = "Field `NACKEN` writer - Slave Receiver Data Phase NACK Enable"]
pub type NackenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMDA` reader - SMBus Default Address"]
pub type SmdaR = crate::BitReader;
#[doc = "Field `SMDA` writer - SMBus Default Address"]
pub type SmdaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMHH` reader - SMBus Host Header"]
pub type SmhhR = crate::BitReader;
#[doc = "Field `SMHH` writer - SMBus Host Header"]
pub type SmhhW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SADAT` reader - Slave Address Treated as Data"]
pub type SadatR = crate::BitReader;
#[doc = "Field `SADAT` writer - Slave Address Treated as Data"]
pub type SadatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSEL` reader - TWI Bus Selection"]
pub type BselR = crate::BitReader;
#[doc = "Field `BSEL` writer - TWI Bus Selection"]
pub type BselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCLWSDIS` reader - Clock Wait State Disable"]
pub type SclwsdisR = crate::BitReader;
#[doc = "Field `SCLWSDIS` writer - Clock Wait State Disable"]
pub type SclwsdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNIFF` reader - Slave Sniffer Mode"]
pub type SniffR = crate::BitReader;
#[doc = "Field `SNIFF` writer - Slave Sniffer Mode"]
pub type SniffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASK` reader - Slave Address Mask"]
pub type MaskR = crate::FieldReader;
#[doc = "Field `MASK` writer - Slave Address Mask"]
pub type MaskW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SADR` reader - Slave Address"]
pub type SadrR = crate::FieldReader;
#[doc = "Field `SADR` writer - Slave Address"]
pub type SadrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - Slave Receiver Data Phase NACK Enable"]
    #[inline(always)]
    pub fn nacken(&self) -> NackenR {
        NackenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - SMBus Default Address"]
    #[inline(always)]
    pub fn smda(&self) -> SmdaR {
        SmdaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SMBus Host Header"]
    #[inline(always)]
    pub fn smhh(&self) -> SmhhR {
        SmhhR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Slave Address Treated as Data"]
    #[inline(always)]
    pub fn sadat(&self) -> SadatR {
        SadatR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TWI Bus Selection"]
    #[inline(always)]
    pub fn bsel(&self) -> BselR {
        BselR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Clock Wait State Disable"]
    #[inline(always)]
    pub fn sclwsdis(&self) -> SclwsdisR {
        SclwsdisR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Slave Sniffer Mode"]
    #[inline(always)]
    pub fn sniff(&self) -> SniffR {
        SniffR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - Slave Address Mask"]
    #[inline(always)]
    pub fn mask(&self) -> MaskR {
        MaskR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Slave Address"]
    #[inline(always)]
    pub fn sadr(&self) -> SadrR {
        SadrR::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Slave Receiver Data Phase NACK Enable"]
    #[inline(always)]
    pub fn nacken(&mut self) -> NackenW<FlexTwiSmrSpec> {
        NackenW::new(self, 0)
    }
    #[doc = "Bit 2 - SMBus Default Address"]
    #[inline(always)]
    pub fn smda(&mut self) -> SmdaW<FlexTwiSmrSpec> {
        SmdaW::new(self, 2)
    }
    #[doc = "Bit 3 - SMBus Host Header"]
    #[inline(always)]
    pub fn smhh(&mut self) -> SmhhW<FlexTwiSmrSpec> {
        SmhhW::new(self, 3)
    }
    #[doc = "Bit 4 - Slave Address Treated as Data"]
    #[inline(always)]
    pub fn sadat(&mut self) -> SadatW<FlexTwiSmrSpec> {
        SadatW::new(self, 4)
    }
    #[doc = "Bit 5 - TWI Bus Selection"]
    #[inline(always)]
    pub fn bsel(&mut self) -> BselW<FlexTwiSmrSpec> {
        BselW::new(self, 5)
    }
    #[doc = "Bit 6 - Clock Wait State Disable"]
    #[inline(always)]
    pub fn sclwsdis(&mut self) -> SclwsdisW<FlexTwiSmrSpec> {
        SclwsdisW::new(self, 6)
    }
    #[doc = "Bit 7 - Slave Sniffer Mode"]
    #[inline(always)]
    pub fn sniff(&mut self) -> SniffW<FlexTwiSmrSpec> {
        SniffW::new(self, 7)
    }
    #[doc = "Bits 8:14 - Slave Address Mask"]
    #[inline(always)]
    pub fn mask(&mut self) -> MaskW<FlexTwiSmrSpec> {
        MaskW::new(self, 8)
    }
    #[doc = "Bits 16:22 - Slave Address"]
    #[inline(always)]
    pub fn sadr(&mut self) -> SadrW<FlexTwiSmrSpec> {
        SadrW::new(self, 16)
    }
}
#[doc = "TWI Slave Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_twi_smr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_twi_smr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexTwiSmrSpec;
impl crate::RegisterSpec for FlexTwiSmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flex_twi_smr::R`](R) reader structure"]
impl crate::Readable for FlexTwiSmrSpec {}
#[doc = "`write(|w| ..)` method takes [`flex_twi_smr::W`](W) writer structure"]
impl crate::Writable for FlexTwiSmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLEX_TWI_SMR to value 0"]
impl crate::Resettable for FlexTwiSmrSpec {}
