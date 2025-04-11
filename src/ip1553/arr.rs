#[doc = "Register `ARR` reader"]
pub type R = crate::R<ArrSpec>;
#[doc = "Register `ARR` writer"]
pub type W = crate::W<ArrSpec>;
#[doc = "Field `REG_ADDR_APB_R` reader - RX base address"]
pub type RegAddrApbRR = crate::FieldReader<u32>;
#[doc = "Field `REG_ADDR_APB_R` writer - RX base address"]
pub type RegAddrApbRW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - RX base address"]
    #[inline(always)]
    pub fn reg_addr_apb_r(&self) -> RegAddrApbRR {
        RegAddrApbRR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - RX base address"]
    #[inline(always)]
    pub fn reg_addr_apb_r(&mut self) -> RegAddrApbRW<ArrSpec> {
        RegAddrApbRW::new(self, 0)
    }
}
#[doc = "Address Register Read\n\nYou can [`read`](crate::Reg::read) this register and get [`arr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ArrSpec;
impl crate::RegisterSpec for ArrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arr::R`](R) reader structure"]
impl crate::Readable for ArrSpec {}
#[doc = "`write(|w| ..)` method takes [`arr::W`](W) writer structure"]
impl crate::Writable for ArrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ARR to value 0"]
impl crate::Resettable for ArrSpec {}
