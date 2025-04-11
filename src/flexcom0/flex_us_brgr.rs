#[doc = "Register `FLEX_US_BRGR` reader"]
pub type R = crate::R<FlexUsBrgrSpec>;
#[doc = "Register `FLEX_US_BRGR` writer"]
pub type W = crate::W<FlexUsBrgrSpec>;
#[doc = "Field `CD` reader - Clock Divider"]
pub type CdR = crate::FieldReader<u16>;
#[doc = "Field `CD` writer - Clock Divider"]
pub type CdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `FP` reader - Fractional Part"]
pub type FpR = crate::FieldReader;
#[doc = "Field `FP` writer - Fractional Part"]
pub type FpW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:15 - Clock Divider"]
    #[inline(always)]
    pub fn cd(&self) -> CdR {
        CdR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - Fractional Part"]
    #[inline(always)]
    pub fn fp(&self) -> FpR {
        FpR::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Clock Divider"]
    #[inline(always)]
    pub fn cd(&mut self) -> CdW<FlexUsBrgrSpec> {
        CdW::new(self, 0)
    }
    #[doc = "Bits 16:18 - Fractional Part"]
    #[inline(always)]
    pub fn fp(&mut self) -> FpW<FlexUsBrgrSpec> {
        FpW::new(self, 16)
    }
}
#[doc = "USART Baud Rate Generator Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_brgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_us_brgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexUsBrgrSpec;
impl crate::RegisterSpec for FlexUsBrgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flex_us_brgr::R`](R) reader structure"]
impl crate::Readable for FlexUsBrgrSpec {}
#[doc = "`write(|w| ..)` method takes [`flex_us_brgr::W`](W) writer structure"]
impl crate::Writable for FlexUsBrgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLEX_US_BRGR to value 0"]
impl crate::Resettable for FlexUsBrgrSpec {}
