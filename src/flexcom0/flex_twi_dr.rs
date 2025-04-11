#[doc = "Register `FLEX_TWI_DR` reader"]
pub type R = crate::R<FlexTwiDrSpec>;
#[doc = "Field `SWEN` reader - SleepWalking Enable"]
pub type SwenR = crate::BitReader;
#[doc = "Field `CLKRQ` reader - Clock Request"]
pub type ClkrqR = crate::BitReader;
#[doc = "Field `SWMATCH` reader - SleepWalking Match"]
pub type SwmatchR = crate::BitReader;
#[doc = "Field `TRP` reader - Transfer Pending"]
pub type TrpR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SleepWalking Enable"]
    #[inline(always)]
    pub fn swen(&self) -> SwenR {
        SwenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock Request"]
    #[inline(always)]
    pub fn clkrq(&self) -> ClkrqR {
        ClkrqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SleepWalking Match"]
    #[inline(always)]
    pub fn swmatch(&self) -> SwmatchR {
        SwmatchR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transfer Pending"]
    #[inline(always)]
    pub fn trp(&self) -> TrpR {
        TrpR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "TWI Debug Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_twi_dr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexTwiDrSpec;
impl crate::RegisterSpec for FlexTwiDrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flex_twi_dr::R`](R) reader structure"]
impl crate::Readable for FlexTwiDrSpec {}
#[doc = "`reset()` method sets FLEX_TWI_DR to value 0"]
impl crate::Resettable for FlexTwiDrSpec {}
