#[doc = "Register `LINK1_TRANSESC` reader"]
pub type R = crate::R<Link1TransescSpec>;
#[doc = "Register `LINK1_TRANSESC` writer"]
pub type W = crate::W<Link1TransescSpec>;
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
    pub fn char(&mut self) -> CharW<Link1TransescSpec> {
        CharW::new(self, 0)
    }
}
#[doc = "SpW Link 1 Transmit Escape Character\n\nYou can [`read`](crate::Reg::read) this register and get [`link1_transesc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link1_transesc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Link1TransescSpec;
impl crate::RegisterSpec for Link1TransescSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`link1_transesc::R`](R) reader structure"]
impl crate::Readable for Link1TransescSpec {}
#[doc = "`write(|w| ..)` method takes [`link1_transesc::W`](W) writer structure"]
impl crate::Writable for Link1TransescSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LINK1_TRANSESC to value 0"]
impl crate::Resettable for Link1TransescSpec {}
