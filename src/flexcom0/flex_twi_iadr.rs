#[doc = "Register `FLEX_TWI_IADR` reader"]
pub type R = crate::R<FlexTwiIadrSpec>;
#[doc = "Register `FLEX_TWI_IADR` writer"]
pub type W = crate::W<FlexTwiIadrSpec>;
#[doc = "Field `IADR` reader - Internal Address"]
pub type IadrR = crate::FieldReader<u32>;
#[doc = "Field `IADR` writer - Internal Address"]
pub type IadrW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Internal Address"]
    #[inline(always)]
    pub fn iadr(&self) -> IadrR {
        IadrR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Internal Address"]
    #[inline(always)]
    pub fn iadr(&mut self) -> IadrW<FlexTwiIadrSpec> {
        IadrW::new(self, 0)
    }
}
#[doc = "TWI Internal Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_twi_iadr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_twi_iadr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexTwiIadrSpec;
impl crate::RegisterSpec for FlexTwiIadrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flex_twi_iadr::R`](R) reader structure"]
impl crate::Readable for FlexTwiIadrSpec {}
#[doc = "`write(|w| ..)` method takes [`flex_twi_iadr::W`](W) writer structure"]
impl crate::Writable for FlexTwiIadrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLEX_TWI_IADR to value 0"]
impl crate::Resettable for FlexTwiIadrSpec {}
