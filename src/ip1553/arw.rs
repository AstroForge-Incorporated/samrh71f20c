#[doc = "Register `ARW` reader"]
pub type R = crate::R<ArwSpec>;
#[doc = "Register `ARW` writer"]
pub type W = crate::W<ArwSpec>;
#[doc = "Field `REG_ADDR_APB_W` reader - TX base address"]
pub type RegAddrApbWR = crate::FieldReader<u32>;
#[doc = "Field `REG_ADDR_APB_W` writer - TX base address"]
pub type RegAddrApbWW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TX base address"]
    #[inline(always)]
    pub fn reg_addr_apb_w(&self) -> RegAddrApbWR {
        RegAddrApbWR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TX base address"]
    #[inline(always)]
    pub fn reg_addr_apb_w(&mut self) -> RegAddrApbWW<ArwSpec> {
        RegAddrApbWW::new(self, 0)
    }
}
#[doc = "Address Register Write\n\nYou can [`read`](crate::Reg::read) this register and get [`arw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ArwSpec;
impl crate::RegisterSpec for ArwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arw::R`](R) reader structure"]
impl crate::Readable for ArwSpec {}
#[doc = "`write(|w| ..)` method takes [`arw::W`](W) writer structure"]
impl crate::Writable for ArwSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ARW to value 0"]
impl crate::Resettable for ArwSpec {}
