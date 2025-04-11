#[doc = "Register `FLEX_US_LONPRIO` reader"]
pub type R = crate::R<FlexUsLonprioSpec>;
#[doc = "Register `FLEX_US_LONPRIO` writer"]
pub type W = crate::W<FlexUsLonprioSpec>;
#[doc = "Field `PSNB` reader - LON Priority Slot Number"]
pub type PsnbR = crate::FieldReader;
#[doc = "Field `PSNB` writer - LON Priority Slot Number"]
pub type PsnbW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `NPS` reader - LON Node Priority Slot"]
pub type NpsR = crate::FieldReader;
#[doc = "Field `NPS` writer - LON Node Priority Slot"]
pub type NpsW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - LON Priority Slot Number"]
    #[inline(always)]
    pub fn psnb(&self) -> PsnbR {
        PsnbR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - LON Node Priority Slot"]
    #[inline(always)]
    pub fn nps(&self) -> NpsR {
        NpsR::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - LON Priority Slot Number"]
    #[inline(always)]
    pub fn psnb(&mut self) -> PsnbW<FlexUsLonprioSpec> {
        PsnbW::new(self, 0)
    }
    #[doc = "Bits 8:14 - LON Node Priority Slot"]
    #[inline(always)]
    pub fn nps(&mut self) -> NpsW<FlexUsLonprioSpec> {
        NpsW::new(self, 8)
    }
}
#[doc = "USART LON Priority Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_lonprio::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_us_lonprio::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexUsLonprioSpec;
impl crate::RegisterSpec for FlexUsLonprioSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flex_us_lonprio::R`](R) reader structure"]
impl crate::Readable for FlexUsLonprioSpec {}
#[doc = "`write(|w| ..)` method takes [`flex_us_lonprio::W`](W) writer structure"]
impl crate::Writable for FlexUsLonprioSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLEX_US_LONPRIO to value 0"]
impl crate::Resettable for FlexUsLonprioSpec {}
