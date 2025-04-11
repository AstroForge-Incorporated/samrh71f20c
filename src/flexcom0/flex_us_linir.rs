#[doc = "Register `FLEX_US_LINIR` reader"]
pub type R = crate::R<FlexUsLinirSpec>;
#[doc = "Register `FLEX_US_LINIR` writer"]
pub type W = crate::W<FlexUsLinirSpec>;
#[doc = "Field `IDCHR` reader - Identifier Character"]
pub type IdchrR = crate::FieldReader;
#[doc = "Field `IDCHR` writer - Identifier Character"]
pub type IdchrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Identifier Character"]
    #[inline(always)]
    pub fn idchr(&self) -> IdchrR {
        IdchrR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Identifier Character"]
    #[inline(always)]
    pub fn idchr(&mut self) -> IdchrW<FlexUsLinirSpec> {
        IdchrW::new(self, 0)
    }
}
#[doc = "USART LIN Identifier Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_linir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_us_linir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexUsLinirSpec;
impl crate::RegisterSpec for FlexUsLinirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flex_us_linir::R`](R) reader structure"]
impl crate::Readable for FlexUsLinirSpec {}
#[doc = "`write(|w| ..)` method takes [`flex_us_linir::W`](W) writer structure"]
impl crate::Writable for FlexUsLinirSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLEX_US_LINIR to value 0"]
impl crate::Resettable for FlexUsLinirSpec {}
