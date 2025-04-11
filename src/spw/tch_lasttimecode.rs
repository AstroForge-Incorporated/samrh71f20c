#[doc = "Register `TCH_LASTTIMECODE` reader"]
pub type R = crate::R<TchLasttimecodeSpec>;
#[doc = "Register `TCH_LASTTIMECODE` writer"]
pub type W = crate::W<TchLasttimecodeSpec>;
#[doc = "Field `VALUE` reader - Value"]
pub type ValueR = crate::FieldReader;
#[doc = "Field `VALUE` writer - Value"]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SEND` reader - Send Now"]
pub type SendR = crate::BitReader;
#[doc = "Field `SEND` writer - Send Now"]
pub type SendW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Value"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - Send Now"]
    #[inline(always)]
    pub fn send(&self) -> SendR {
        SendR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Value"]
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<TchLasttimecodeSpec> {
        ValueW::new(self, 0)
    }
    #[doc = "Bit 8 - Send Now"]
    #[inline(always)]
    pub fn send(&mut self) -> SendW<TchLasttimecodeSpec> {
        SendW::new(self, 8)
    }
}
#[doc = "SpW Tch Last Time Code\n\nYou can [`read`](crate::Reg::read) this register and get [`tch_lasttimecode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tch_lasttimecode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TchLasttimecodeSpec;
impl crate::RegisterSpec for TchLasttimecodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tch_lasttimecode::R`](R) reader structure"]
impl crate::Readable for TchLasttimecodeSpec {}
#[doc = "`write(|w| ..)` method takes [`tch_lasttimecode::W`](W) writer structure"]
impl crate::Writable for TchLasttimecodeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCH_LASTTIMECODE to value 0"]
impl crate::Resettable for TchLasttimecodeSpec {}
