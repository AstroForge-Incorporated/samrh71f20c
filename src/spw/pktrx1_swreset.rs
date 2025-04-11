#[doc = "Register `PKTRX1_SWRESET` reader"]
pub type R = crate::R<Pktrx1SwresetSpec>;
#[doc = "Register `PKTRX1_SWRESET` writer"]
pub type W = crate::W<Pktrx1SwresetSpec>;
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
    pub fn pattern(&mut self) -> PatternW<Pktrx1SwresetSpec> {
        PatternW::new(self, 0)
    }
}
#[doc = "PktRx Software Reset\n\nYou can [`read`](crate::Reg::read) this register and get [`pktrx1_swreset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pktrx1_swreset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pktrx1SwresetSpec;
impl crate::RegisterSpec for Pktrx1SwresetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pktrx1_swreset::R`](R) reader structure"]
impl crate::Readable for Pktrx1SwresetSpec {}
#[doc = "`write(|w| ..)` method takes [`pktrx1_swreset::W`](W) writer structure"]
impl crate::Writable for Pktrx1SwresetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PKTRX1_SWRESET to value 0"]
impl crate::Resettable for Pktrx1SwresetSpec {}
