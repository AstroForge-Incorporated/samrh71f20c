#[doc = "Register `LINK2_TRANSESC` reader"]
pub type R = crate::R<Link2TransescSpec>;
#[doc = "Register `LINK2_TRANSESC` writer"]
pub type W = crate::W<Link2TransescSpec>;
#[doc = "Field `CHAR` reader - Character"]
pub type CharR = crate::FieldReader;
#[doc = "Field `CHAR` writer - Character"]
pub type CharW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Character"]
    #[inline(always)]
    pub fn char(&self) -> CharR {
        CharR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Character"]
    #[inline(always)]
    pub fn char(&mut self) -> CharW<Link2TransescSpec> {
        CharW::new(self, 0)
    }
}
#[doc = "SpW Link 2 Transmit Escape Character\n\nYou can [`read`](crate::Reg::read) this register and get [`link2_transesc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link2_transesc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Link2TransescSpec;
impl crate::RegisterSpec for Link2TransescSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`link2_transesc::R`](R) reader structure"]
impl crate::Readable for Link2TransescSpec {}
#[doc = "`write(|w| ..)` method takes [`link2_transesc::W`](W) writer structure"]
impl crate::Writable for Link2TransescSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LINK2_TRANSESC to value 0"]
impl crate::Resettable for Link2TransescSpec {}
