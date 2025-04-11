#[doc = "Register `CAN0` reader"]
pub type R = crate::R<Can0Spec>;
#[doc = "Register `CAN0` writer"]
pub type W = crate::W<Can0Spec>;
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
    pub fn ext_mem_addr(&mut self) -> ExtMemAddrW<Can0Spec> {
        ExtMemAddrW::new(self, 16)
    }
}
#[doc = "CAN0 MSB Base Address\n\nYou can [`read`](crate::Reg::read) this register and get [`can0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Can0Spec;
impl crate::RegisterSpec for Can0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can0::R`](R) reader structure"]
impl crate::Readable for Can0Spec {}
#[doc = "`write(|w| ..)` method takes [`can0::W`](W) writer structure"]
impl crate::Writable for Can0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN0 to value 0"]
impl crate::Resettable for Can0Spec {}
