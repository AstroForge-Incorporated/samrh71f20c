#[doc = "Register `FLEX_US_TTGR` reader"]
pub type R = crate::R<FlexUsTtgrSpec>;
#[doc = "Register `FLEX_US_TTGR` writer"]
pub type W = crate::W<FlexUsTtgrSpec>;
#[doc = "Field `TG` reader - Timeguard Value"]
pub type TgR = crate::FieldReader;
#[doc = "Field `TG` writer - Timeguard Value"]
pub type TgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Timeguard Value"]
    #[inline(always)]
    pub fn tg(&self) -> TgR {
        TgR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timeguard Value"]
    #[inline(always)]
    pub fn tg(&mut self) -> TgW<FlexUsTtgrSpec> {
        TgW::new(self, 0)
    }
}
#[doc = "USART Transmitter Timeguard Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_ttgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_us_ttgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexUsTtgrSpec;
impl crate::RegisterSpec for FlexUsTtgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flex_us_ttgr::R`](R) reader structure"]
impl crate::Readable for FlexUsTtgrSpec {}
#[doc = "`write(|w| ..)` method takes [`flex_us_ttgr::W`](W) writer structure"]
impl crate::Writable for FlexUsTtgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLEX_US_TTGR to value 0"]
impl crate::Resettable for FlexUsTtgrSpec {}
