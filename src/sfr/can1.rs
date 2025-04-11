#[doc = "Register `CAN1` reader"]
pub type R = crate::R<Can1Spec>;
#[doc = "Register `CAN1` writer"]
pub type W = crate::W<Can1Spec>;
#[doc = "Field `EXT_MEM_ADDR` reader - MSB Base Address"]
pub type ExtMemAddrR = crate::FieldReader<u16>;
#[doc = "Field `EXT_MEM_ADDR` writer - MSB Base Address"]
pub type ExtMemAddrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 16:31 - MSB Base Address"]
    #[inline(always)]
    pub fn ext_mem_addr(&self) -> ExtMemAddrR {
        ExtMemAddrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - MSB Base Address"]
    #[inline(always)]
    pub fn ext_mem_addr(&mut self) -> ExtMemAddrW<Can1Spec> {
        ExtMemAddrW::new(self, 16)
    }
}
#[doc = "CAN1 MSB Base Address\n\nYou can [`read`](crate::Reg::read) this register and get [`can1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Can1Spec;
impl crate::RegisterSpec for Can1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can1::R`](R) reader structure"]
impl crate::Readable for Can1Spec {}
#[doc = "`write(|w| ..)` method takes [`can1::W`](W) writer structure"]
impl crate::Writable for Can1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN1 to value 0"]
impl crate::Resettable for Can1Spec {}
