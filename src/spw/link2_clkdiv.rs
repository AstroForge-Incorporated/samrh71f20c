#[doc = "Register `LINK2_CLKDIV` reader"]
pub type R = crate::R<Link2ClkdivSpec>;
#[doc = "Register `LINK2_CLKDIV` writer"]
pub type W = crate::W<Link2ClkdivSpec>;
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
    pub fn txoperdiv(&mut self) -> TxoperdivW<Link2ClkdivSpec> {
        TxoperdivW::new(self, 0)
    }
    #[doc = "Bits 16:20 - TxInitDiv"]
    #[inline(always)]
    pub fn txinitdiv(&mut self) -> TxinitdivW<Link2ClkdivSpec> {
        TxinitdivW::new(self, 16)
    }
}
#[doc = "SpW Link 2 Clock Division\n\nYou can [`read`](crate::Reg::read) this register and get [`link2_clkdiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link2_clkdiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Link2ClkdivSpec;
impl crate::RegisterSpec for Link2ClkdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`link2_clkdiv::R`](R) reader structure"]
impl crate::Readable for Link2ClkdivSpec {}
#[doc = "`write(|w| ..)` method takes [`link2_clkdiv::W`](W) writer structure"]
impl crate::Writable for Link2ClkdivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LINK2_CLKDIV to value 0"]
impl crate::Resettable for Link2ClkdivSpec {}
