#[doc = "Register `LINK2_SWRESET` reader"]
pub type R = crate::R<Link2SwresetSpec>;
#[doc = "Register `LINK2_SWRESET` writer"]
pub type W = crate::W<Link2SwresetSpec>;
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
    pub fn pattern(&mut self) -> PatternW<Link2SwresetSpec> {
        PatternW::new(self, 0)
    }
}
#[doc = "SpW Link 2 Software Reset\n\nYou can [`read`](crate::Reg::read) this register and get [`link2_swreset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link2_swreset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Link2SwresetSpec;
impl crate::RegisterSpec for Link2SwresetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`link2_swreset::R`](R) reader structure"]
impl crate::Readable for Link2SwresetSpec {}
#[doc = "`write(|w| ..)` method takes [`link2_swreset::W`](W) writer structure"]
impl crate::Writable for Link2SwresetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LINK2_SWRESET to value 0"]
impl crate::Resettable for Link2SwresetSpec {}
