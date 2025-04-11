#[doc = "Register `BCR` reader"]
pub type R = crate::R<BcrSpec>;
#[doc = "Register `BCR` writer"]
pub type W = crate::W<BcrSpec>;
#[doc = "Field `BYTCNT` reader - Remaining Byte Count Before Auto Padding"]
pub type BytcntR = crate::FieldReader<u32>;
#[doc = "Field `BYTCNT` writer - Remaining Byte Count Before Auto Padding"]
pub type BytcntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Remaining Byte Count Before Auto Padding"]
    #[inline(always)]
    pub fn bytcnt(&self) -> BytcntR {
        BytcntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Remaining Byte Count Before Auto Padding"]
    #[inline(always)]
    pub fn bytcnt(&mut self) -> BytcntW<BcrSpec> {
        BytcntW::new(self, 0)
    }
}
#[doc = "Bytes Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BcrSpec;
impl crate::RegisterSpec for BcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcr::R`](R) reader structure"]
impl crate::Readable for BcrSpec {}
#[doc = "`write(|w| ..)` method takes [`bcr::W`](W) writer structure"]
impl crate::Writable for BcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BCR to value 0"]
impl crate::Resettable for BcrSpec {}
