#[doc = "Register `FLEX_US_LONDL` reader"]
pub type R = crate::R<FlexUsLondlSpec>;
#[doc = "Register `FLEX_US_LONDL` writer"]
pub type W = crate::W<FlexUsLondlSpec>;
#[doc = "Field `LONDL` reader - LON Data Length"]
pub type LondlR = crate::FieldReader;
#[doc = "Field `LONDL` writer - LON Data Length"]
pub type LondlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - LON Data Length"]
    #[inline(always)]
    pub fn londl(&self) -> LondlR {
        LondlR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - LON Data Length"]
    #[inline(always)]
    pub fn londl(&mut self) -> LondlW<FlexUsLondlSpec> {
        LondlW::new(self, 0)
    }
}
#[doc = "USART LON Data Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_londl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_us_londl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexUsLondlSpec;
impl crate::RegisterSpec for FlexUsLondlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flex_us_londl::R`](R) reader structure"]
impl crate::Readable for FlexUsLondlSpec {}
#[doc = "`write(|w| ..)` method takes [`flex_us_londl::W`](W) writer structure"]
impl crate::Writable for FlexUsLondlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLEX_US_LONDL to value 0"]
impl crate::Resettable for FlexUsLondlSpec {}
