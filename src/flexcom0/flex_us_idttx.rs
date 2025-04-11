#[doc = "Register `FLEX_US_IDTTX` reader"]
pub type R = crate::R<FlexUsIdttxSpec>;
#[doc = "Register `FLEX_US_IDTTX` writer"]
pub type W = crate::W<FlexUsIdttxSpec>;
#[doc = "Field `IDTTX` reader - LON Indeterminate Time after Transmission (comm_type = 1 mode only)"]
pub type IdttxR = crate::FieldReader<u32>;
#[doc = "Field `IDTTX` writer - LON Indeterminate Time after Transmission (comm_type = 1 mode only)"]
pub type IdttxW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - LON Indeterminate Time after Transmission (comm_type = 1 mode only)"]
    #[inline(always)]
    pub fn idttx(&self) -> IdttxR {
        IdttxR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - LON Indeterminate Time after Transmission (comm_type = 1 mode only)"]
    #[inline(always)]
    pub fn idttx(&mut self) -> IdttxW<FlexUsIdttxSpec> {
        IdttxW::new(self, 0)
    }
}
#[doc = "USART LON IDT Tx Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_us_idttx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_us_idttx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexUsIdttxSpec;
impl crate::RegisterSpec for FlexUsIdttxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flex_us_idttx::R`](R) reader structure"]
impl crate::Readable for FlexUsIdttxSpec {}
#[doc = "`write(|w| ..)` method takes [`flex_us_idttx::W`](W) writer structure"]
impl crate::Writable for FlexUsIdttxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLEX_US_IDTTX to value 0"]
impl crate::Resettable for FlexUsIdttxSpec {}
