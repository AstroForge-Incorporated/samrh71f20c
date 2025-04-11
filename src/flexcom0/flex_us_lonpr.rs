#[doc = "Register `FLEX_US_LONPR` reader"]
pub type R = crate::R<FlexUsLonprSpec>;
#[doc = "Register `FLEX_US_LONPR` writer"]
pub type W = crate::W<FlexUsLonprSpec>;
#[doc = "Field `LONPL` reader - LON Preamble Length"]
pub type LonplR = crate::FieldReader<u16>;
#[doc = "Field `LONPL` writer - LON Preamble Length"]
pub type LonplW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - LON Preamble Length"]
    #[inline(always)]
    pub fn lonpl(&self) -> LonplR {
        LonplR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - LON Preamble Length"]
    #[inline(always)]
    pub fn lonpl(&mut self) -> LonplW<FlexUsLonprSpec> {
        LonplW::new(self, 0)
    }
}
#[doc = "USART LON Preamble Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_lonpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_us_lonpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexUsLonprSpec;
impl crate::RegisterSpec for FlexUsLonprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flex_us_lonpr::R`](R) reader structure"]
impl crate::Readable for FlexUsLonprSpec {}
#[doc = "`write(|w| ..)` method takes [`flex_us_lonpr::W`](W) writer structure"]
impl crate::Writable for FlexUsLonprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLEX_US_LONPR to value 0"]
impl crate::Resettable for FlexUsLonprSpec {}
