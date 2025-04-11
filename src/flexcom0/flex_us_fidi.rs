#[doc = "Register `FLEX_US_FIDI` reader"]
pub type R = crate::R<FlexUsFidiSpec>;
#[doc = "Register `FLEX_US_FIDI` writer"]
pub type W = crate::W<FlexUsFidiSpec>;
#[doc = "Field `FI_DI_RATIO` reader - FI Over DI Ratio Value"]
pub type FiDiRatioR = crate::FieldReader<u16>;
#[doc = "Field `FI_DI_RATIO` writer - FI Over DI Ratio Value"]
pub type FiDiRatioW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - FI Over DI Ratio Value"]
    #[inline(always)]
    pub fn fi_di_ratio(&self) -> FiDiRatioR {
        FiDiRatioR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - FI Over DI Ratio Value"]
    #[inline(always)]
    pub fn fi_di_ratio(&mut self) -> FiDiRatioW<FlexUsFidiSpec> {
        FiDiRatioW::new(self, 0)
    }
}
#[doc = "USART FI DI Ratio Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_fidi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_us_fidi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexUsFidiSpec;
impl crate::RegisterSpec for FlexUsFidiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flex_us_fidi::R`](R) reader structure"]
impl crate::Readable for FlexUsFidiSpec {}
#[doc = "`write(|w| ..)` method takes [`flex_us_fidi::W`](W) writer structure"]
impl crate::Writable for FlexUsFidiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLEX_US_FIDI to value 0"]
impl crate::Resettable for FlexUsFidiSpec {}
