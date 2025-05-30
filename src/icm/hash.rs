#[doc = "Register `HASH` reader"]
pub type R = crate::R<HashSpec>;
#[doc = "Register `HASH` writer"]
pub type W = crate::W<HashSpec>;
#[doc = "Field `HASA` reader - Hash Area Start Address"]
pub type HasaR = crate::FieldReader<u32>;
#[doc = "Field `HASA` writer - Hash Area Start Address"]
pub type HasaW<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 7:31 - Hash Area Start Address"]
    #[inline(always)]
    pub fn hasa(&self) -> HasaR {
        HasaR::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 7:31 - Hash Area Start Address"]
    #[inline(always)]
    pub fn hasa(&mut self) -> HasaW<HashSpec> {
        HasaW::new(self, 7)
    }
}
#[doc = "Region Hash Area Start Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hash::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashSpec;
impl crate::RegisterSpec for HashSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash::R`](R) reader structure"]
impl crate::Readable for HashSpec {}
#[doc = "`write(|w| ..)` method takes [`hash::W`](W) writer structure"]
impl crate::Writable for HashSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HASH to value 0"]
impl crate::Resettable for HashSpec {}
