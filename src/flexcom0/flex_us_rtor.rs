#[doc = "Register `FLEX_US_RTOR` reader"]
pub type R = crate::R<FlexUsRtorSpec>;
#[doc = "Register `FLEX_US_RTOR` writer"]
pub type W = crate::W<FlexUsRtorSpec>;
#[doc = "Field `TO` reader - Timeout Value"]
pub type ToR = crate::FieldReader<u32>;
#[doc = "Field `TO` writer - Timeout Value"]
pub type ToW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - Timeout Value"]
    #[inline(always)]
    pub fn to(&self) -> ToR {
        ToR::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - Timeout Value"]
    #[inline(always)]
    pub fn to(&mut self) -> ToW<FlexUsRtorSpec> {
        ToW::new(self, 0)
    }
}
#[doc = "USART Receiver Timeout Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_rtor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_us_rtor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexUsRtorSpec;
impl crate::RegisterSpec for FlexUsRtorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flex_us_rtor::R`](R) reader structure"]
impl crate::Readable for FlexUsRtorSpec {}
#[doc = "`write(|w| ..)` method takes [`flex_us_rtor::W`](W) writer structure"]
impl crate::Writable for FlexUsRtorSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLEX_US_RTOR to value 0"]
impl crate::Resettable for FlexUsRtorSpec {}
