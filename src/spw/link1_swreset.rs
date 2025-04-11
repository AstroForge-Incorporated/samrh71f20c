#[doc = "Register `LINK1_SWRESET` reader"]
pub type R = crate::R<Link1SwresetSpec>;
#[doc = "Register `LINK1_SWRESET` writer"]
pub type W = crate::W<Link1SwresetSpec>;
#[doc = "Field `PATTERN` reader - Pattern"]
pub type PatternR = crate::FieldReader<u32>;
#[doc = "Field `PATTERN` writer - Pattern"]
pub type PatternW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Pattern"]
    #[inline(always)]
    pub fn pattern(&self) -> PatternR {
        PatternR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pattern"]
    #[inline(always)]
    pub fn pattern(&mut self) -> PatternW<Link1SwresetSpec> {
        PatternW::new(self, 0)
    }
}
#[doc = "SpW Link 1 Software Reset\n\nYou can [`read`](crate::Reg::read) this register and get [`link1_swreset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link1_swreset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Link1SwresetSpec;
impl crate::RegisterSpec for Link1SwresetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`link1_swreset::R`](R) reader structure"]
impl crate::Readable for Link1SwresetSpec {}
#[doc = "`write(|w| ..)` method takes [`link1_swreset::W`](W) writer structure"]
impl crate::Writable for Link1SwresetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LINK1_SWRESET to value 0"]
impl crate::Resettable for Link1SwresetSpec {}
