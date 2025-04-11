#[doc = "Register `TCH_SWRESET` reader"]
pub type R = crate::R<TchSwresetSpec>;
#[doc = "Register `TCH_SWRESET` writer"]
pub type W = crate::W<TchSwresetSpec>;
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
    pub fn pattern(&mut self) -> PatternW<TchSwresetSpec> {
        PatternW::new(self, 0)
    }
}
#[doc = "SpW Tch Software Reset\n\nYou can [`read`](crate::Reg::read) this register and get [`tch_swreset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tch_swreset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TchSwresetSpec;
impl crate::RegisterSpec for TchSwresetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tch_swreset::R`](R) reader structure"]
impl crate::Readable for TchSwresetSpec {}
#[doc = "`write(|w| ..)` method takes [`tch_swreset::W`](W) writer structure"]
impl crate::Writable for TchSwresetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCH_SWRESET to value 0"]
impl crate::Resettable for TchSwresetSpec {}
