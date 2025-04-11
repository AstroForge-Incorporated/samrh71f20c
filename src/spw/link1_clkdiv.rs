#[doc = "Register `LINK1_CLKDIV` reader"]
pub type R = crate::R<Link1ClkdivSpec>;
#[doc = "Register `LINK1_CLKDIV` writer"]
pub type W = crate::W<Link1ClkdivSpec>;
#[doc = "Field `TXOPERDIV` reader - TxOperDiv"]
pub type TxoperdivR = crate::FieldReader;
#[doc = "Field `TXOPERDIV` writer - TxOperDiv"]
pub type TxoperdivW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TXINITDIV` reader - TxInitDiv"]
pub type TxinitdivR = crate::FieldReader;
#[doc = "Field `TXINITDIV` writer - TxInitDiv"]
pub type TxinitdivW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - TxOperDiv"]
    #[inline(always)]
    pub fn txoperdiv(&self) -> TxoperdivR {
        TxoperdivR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - TxInitDiv"]
    #[inline(always)]
    pub fn txinitdiv(&self) -> TxinitdivR {
        TxinitdivR::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - TxOperDiv"]
    #[inline(always)]
    pub fn txoperdiv(&mut self) -> TxoperdivW<Link1ClkdivSpec> {
        TxoperdivW::new(self, 0)
    }
    #[doc = "Bits 16:20 - TxInitDiv"]
    #[inline(always)]
    pub fn txinitdiv(&mut self) -> TxinitdivW<Link1ClkdivSpec> {
        TxinitdivW::new(self, 16)
    }
}
#[doc = "SpW Link 1 Clock Division\n\nYou can [`read`](crate::Reg::read) this register and get [`link1_clkdiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link1_clkdiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Link1ClkdivSpec;
impl crate::RegisterSpec for Link1ClkdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`link1_clkdiv::R`](R) reader structure"]
impl crate::Readable for Link1ClkdivSpec {}
#[doc = "`write(|w| ..)` method takes [`link1_clkdiv::W`](W) writer structure"]
impl crate::Writable for Link1ClkdivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LINK1_CLKDIV to value 0"]
impl crate::Resettable for Link1ClkdivSpec {}
