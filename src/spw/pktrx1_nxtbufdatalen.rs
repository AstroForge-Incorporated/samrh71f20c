#[doc = "Register `PKTRX1_NXTBUFDATALEN` reader"]
pub type R = crate::R<Pktrx1NxtbufdatalenSpec>;
#[doc = "Register `PKTRX1_NXTBUFDATALEN` writer"]
pub type W = crate::W<Pktrx1NxtbufdatalenSpec>;
#[doc = "Field `LEN` reader - Length"]
pub type LenR = crate::FieldReader<u32>;
#[doc = "Field `LEN` writer - Length"]
pub type LenW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Length"]
    #[inline(always)]
    pub fn len(&self) -> LenR {
        LenR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Length"]
    #[inline(always)]
    pub fn len(&mut self) -> LenW<Pktrx1NxtbufdatalenSpec> {
        LenW::new(self, 0)
    }
}
#[doc = "PktRx Next Buffer Data Length\n\nYou can [`read`](crate::Reg::read) this register and get [`pktrx1_nxtbufdatalen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pktrx1_nxtbufdatalen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pktrx1NxtbufdatalenSpec;
impl crate::RegisterSpec for Pktrx1NxtbufdatalenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pktrx1_nxtbufdatalen::R`](R) reader structure"]
impl crate::Readable for Pktrx1NxtbufdatalenSpec {}
#[doc = "`write(|w| ..)` method takes [`pktrx1_nxtbufdatalen::W`](W) writer structure"]
impl crate::Writable for Pktrx1NxtbufdatalenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PKTRX1_NXTBUFDATALEN to value 0"]
impl crate::Resettable for Pktrx1NxtbufdatalenSpec {}
