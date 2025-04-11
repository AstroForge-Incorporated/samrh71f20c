#[doc = "Register `PKTTX1_SWRESET` reader"]
pub type R = crate::R<Pkttx1SwresetSpec>;
#[doc = "Register `PKTTX1_SWRESET` writer"]
pub type W = crate::W<Pkttx1SwresetSpec>;
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
    pub fn pattern(&mut self) -> PatternW<Pkttx1SwresetSpec> {
        PatternW::new(self, 0)
    }
}
#[doc = "PktTx Software Reset\n\nYou can [`read`](crate::Reg::read) this register and get [`pkttx1_swreset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkttx1_swreset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pkttx1SwresetSpec;
impl crate::RegisterSpec for Pkttx1SwresetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pkttx1_swreset::R`](R) reader structure"]
impl crate::Readable for Pkttx1SwresetSpec {}
#[doc = "`write(|w| ..)` method takes [`pkttx1_swreset::W`](W) writer structure"]
impl crate::Writable for Pkttx1SwresetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PKTTX1_SWRESET to value 0"]
impl crate::Resettable for Pkttx1SwresetSpec {}
