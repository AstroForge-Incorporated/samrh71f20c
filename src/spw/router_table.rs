#[doc = "Register `ROUTER_TABLE[%s]` reader"]
pub type R = crate::R<RouterTableSpec>;
#[doc = "Register `ROUTER_TABLE[%s]` writer"]
pub type W = crate::W<RouterTableSpec>;
#[doc = "Field `ADDR` reader - Address"]
pub type AddrR = crate::FieldReader;
#[doc = "Field `ADDR` writer - Address"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DELHEAD` reader - Delete Header Byte"]
pub type DelheadR = crate::BitReader;
#[doc = "Field `DELHEAD` writer - Delete Header Byte"]
pub type DelheadW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - Delete Header Byte"]
    #[inline(always)]
    pub fn delhead(&self) -> DelheadR {
        DelheadR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Address"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<RouterTableSpec> {
        AddrW::new(self, 0)
    }
    #[doc = "Bit 8 - Delete Header Byte"]
    #[inline(always)]
    pub fn delhead(&mut self) -> DelheadW<RouterTableSpec> {
        DelheadW::new(self, 8)
    }
}
#[doc = "SpW Router Table (Logical addresses 32 to 255, index 0 for logical address 32)\n\nYou can [`read`](crate::Reg::read) this register and get [`router_table::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`router_table::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RouterTableSpec;
impl crate::RegisterSpec for RouterTableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`router_table::R`](R) reader structure"]
impl crate::Readable for RouterTableSpec {}
#[doc = "`write(|w| ..)` method takes [`router_table::W`](W) writer structure"]
impl crate::Writable for RouterTableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ROUTER_TABLE[%s] to value 0"]
impl crate::Resettable for RouterTableSpec {}
