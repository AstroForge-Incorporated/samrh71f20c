#[doc = "Register `FLEX_THR` reader"]
pub type R = crate::R<FlexThrSpec>;
#[doc = "Register `FLEX_THR` writer"]
pub type W = crate::W<FlexThrSpec>;
#[doc = "Field `TXDATA` reader - Transmit Data"]
pub type TxdataR = crate::FieldReader<u16>;
#[doc = "Field `TXDATA` writer - Transmit Data"]
pub type TxdataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Transmit Data"]
    #[inline(always)]
    pub fn txdata(&self) -> TxdataR {
        TxdataR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit Data"]
    #[inline(always)]
    pub fn txdata(&mut self) -> TxdataW<FlexThrSpec> {
        TxdataW::new(self, 0)
    }
}
#[doc = "FLEXCOM Transmit Holding Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_thr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_thr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexThrSpec;
impl crate::RegisterSpec for FlexThrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flex_thr::R`](R) reader structure"]
impl crate::Readable for FlexThrSpec {}
#[doc = "`write(|w| ..)` method takes [`flex_thr::W`](W) writer structure"]
impl crate::Writable for FlexThrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLEX_THR to value 0"]
impl crate::Resettable for FlexThrSpec {}
