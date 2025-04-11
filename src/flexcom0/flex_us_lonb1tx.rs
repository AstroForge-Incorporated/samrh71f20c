#[doc = "Register `FLEX_US_LONB1TX` reader"]
pub type R = crate::R<FlexUsLonb1txSpec>;
#[doc = "Register `FLEX_US_LONB1TX` writer"]
pub type W = crate::W<FlexUsLonb1txSpec>;
#[doc = "Field `BETA1TX` reader - LON Beta1 Length after Transmission"]
pub type Beta1txR = crate::FieldReader<u32>;
#[doc = "Field `BETA1TX` writer - LON Beta1 Length after Transmission"]
pub type Beta1txW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - LON Beta1 Length after Transmission"]
    #[inline(always)]
    pub fn beta1tx(&self) -> Beta1txR {
        Beta1txR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - LON Beta1 Length after Transmission"]
    #[inline(always)]
    pub fn beta1tx(&mut self) -> Beta1txW<FlexUsLonb1txSpec> {
        Beta1txW::new(self, 0)
    }
}
#[doc = "USART LON Beta1 Tx Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_lonb1tx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_us_lonb1tx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexUsLonb1txSpec;
impl crate::RegisterSpec for FlexUsLonb1txSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flex_us_lonb1tx::R`](R) reader structure"]
impl crate::Readable for FlexUsLonb1txSpec {}
#[doc = "`write(|w| ..)` method takes [`flex_us_lonb1tx::W`](W) writer structure"]
impl crate::Writable for FlexUsLonb1txSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLEX_US_LONB1TX to value 0"]
impl crate::Resettable for FlexUsLonb1txSpec {}
