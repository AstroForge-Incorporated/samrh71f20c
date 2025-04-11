#[doc = "Register `MSR` reader"]
pub type R = crate::R<MsrSpec>;
#[doc = "Register `MSR` writer"]
pub type W = crate::W<MsrSpec>;
#[doc = "Field `MSGSIZE` reader - Message Size"]
pub type MsgsizeR = crate::FieldReader<u32>;
#[doc = "Field `MSGSIZE` writer - Message Size"]
pub type MsgsizeW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Message Size"]
    #[inline(always)]
    pub fn msgsize(&self) -> MsgsizeR {
        MsgsizeR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Message Size"]
    #[inline(always)]
    pub fn msgsize(&mut self) -> MsgsizeW<MsrSpec> {
        MsgsizeW::new(self, 0)
    }
}
#[doc = "Message Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`msr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsrSpec;
impl crate::RegisterSpec for MsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msr::R`](R) reader structure"]
impl crate::Readable for MsrSpec {}
#[doc = "`write(|w| ..)` method takes [`msr::W`](W) writer structure"]
impl crate::Writable for MsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MSR to value 0"]
impl crate::Resettable for MsrSpec {}
