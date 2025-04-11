#[doc = "Register `PKTRX1_NXTBUFPKTADDR` reader"]
pub type R = crate::R<Pktrx1NxtbufpktaddrSpec>;
#[doc = "Register `PKTRX1_NXTBUFPKTADDR` writer"]
pub type W = crate::W<Pktrx1NxtbufpktaddrSpec>;
#[doc = "Field `ADDR` reader - Address"]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - Address"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Address"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<Pktrx1NxtbufpktaddrSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "PktRx Next Buffer Packet Address\n\nYou can [`read`](crate::Reg::read) this register and get [`pktrx1_nxtbufpktaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pktrx1_nxtbufpktaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pktrx1NxtbufpktaddrSpec;
impl crate::RegisterSpec for Pktrx1NxtbufpktaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pktrx1_nxtbufpktaddr::R`](R) reader structure"]
impl crate::Readable for Pktrx1NxtbufpktaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`pktrx1_nxtbufpktaddr::W`](W) writer structure"]
impl crate::Writable for Pktrx1NxtbufpktaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PKTRX1_NXTBUFPKTADDR to value 0"]
impl crate::Resettable for Pktrx1NxtbufpktaddrSpec {}
