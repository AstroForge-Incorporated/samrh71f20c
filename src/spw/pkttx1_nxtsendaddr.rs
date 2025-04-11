#[doc = "Register `PKTTX1_NXTSENDADDR` reader"]
pub type R = crate::R<Pkttx1NxtsendaddrSpec>;
#[doc = "Register `PKTTX1_NXTSENDADDR` writer"]
pub type W = crate::W<Pkttx1NxtsendaddrSpec>;
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
    pub fn addr(&mut self) -> AddrW<Pkttx1NxtsendaddrSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "PktTx Next Send List Address\n\nYou can [`read`](crate::Reg::read) this register and get [`pkttx1_nxtsendaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkttx1_nxtsendaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pkttx1NxtsendaddrSpec;
impl crate::RegisterSpec for Pkttx1NxtsendaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pkttx1_nxtsendaddr::R`](R) reader structure"]
impl crate::Readable for Pkttx1NxtsendaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`pkttx1_nxtsendaddr::W`](W) writer structure"]
impl crate::Writable for Pkttx1NxtsendaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PKTTX1_NXTSENDADDR to value 0"]
impl crate::Resettable for Pkttx1NxtsendaddrSpec {}
