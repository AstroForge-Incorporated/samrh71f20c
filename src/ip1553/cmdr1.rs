#[doc = "Register `CMDR1` writer"]
pub type W = crate::W<Cmdr1Spec>;
#[doc = "Field `DATAWORDCOUNT` writer - DATA WORD COUNT"]
pub type DatawordcountW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RTSUBADDRESS` writer - RT SUBADDRESS"]
pub type RtsubaddressW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `T_R` writer - T/R"]
pub type TRW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTADDRESS` writer - RT ADDRESS"]
pub type RtaddressW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl W {
    #[doc = "Bits 0:4 - DATA WORD COUNT"]
    #[inline(always)]
    pub fn datawordcount(&mut self) -> DatawordcountW<Cmdr1Spec> {
        DatawordcountW::new(self, 0)
    }
    #[doc = "Bits 5:9 - RT SUBADDRESS"]
    #[inline(always)]
    pub fn rtsubaddress(&mut self) -> RtsubaddressW<Cmdr1Spec> {
        RtsubaddressW::new(self, 5)
    }
    #[doc = "Bit 10 - T/R"]
    #[inline(always)]
    pub fn t_r(&mut self) -> TRW<Cmdr1Spec> {
        TRW::new(self, 10)
    }
    #[doc = "Bits 11:15 - RT ADDRESS"]
    #[inline(always)]
    pub fn rtaddress(&mut self) -> RtaddressW<Cmdr1Spec> {
        RtaddressW::new(self, 11)
    }
}
#[doc = "Command Register 1\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmdr1Spec;
impl crate::RegisterSpec for Cmdr1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmdr1::W`](W) writer structure"]
impl crate::Writable for Cmdr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMDR1 to value 0"]
impl crate::Resettable for Cmdr1Spec {}
