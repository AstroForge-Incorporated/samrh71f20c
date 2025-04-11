#[doc = "Register `PKTRX1_NXTBUFDATAADDR` reader"]
pub type R = crate::R<Pktrx1NxtbufdataaddrSpec>;
#[doc = "Register `PKTRX1_NXTBUFDATAADDR` writer"]
pub type W = crate::W<Pktrx1NxtbufdataaddrSpec>;
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
    pub fn addr(&mut self) -> AddrW<Pktrx1NxtbufdataaddrSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "PktRx Next Buffer Data Address\n\nYou can [`read`](crate::Reg::read) this register and get [`pktrx1_nxtbufdataaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pktrx1_nxtbufdataaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pktrx1NxtbufdataaddrSpec;
impl crate::RegisterSpec for Pktrx1NxtbufdataaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pktrx1_nxtbufdataaddr::R`](R) reader structure"]
impl crate::Readable for Pktrx1NxtbufdataaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`pktrx1_nxtbufdataaddr::W`](W) writer structure"]
impl crate::Writable for Pktrx1NxtbufdataaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PKTRX1_NXTBUFDATAADDR to value 0"]
impl crate::Resettable for Pktrx1NxtbufdataaddrSpec {}
