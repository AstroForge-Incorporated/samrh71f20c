#[doc = "Register `PKTTX1_NXTSENDROUT` reader"]
pub type R = crate::R<Pkttx1NxtsendroutSpec>;
#[doc = "Register `PKTTX1_NXTSENDROUT` writer"]
pub type W = crate::W<Pkttx1NxtsendroutSpec>;
#[doc = "Field `BYTE4` reader - Byte4"]
pub type Byte4R = crate::FieldReader;
#[doc = "Field `BYTE4` writer - Byte4"]
pub type Byte4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BYTE3` reader - Byte3"]
pub type Byte3R = crate::FieldReader;
#[doc = "Field `BYTE3` writer - Byte3"]
pub type Byte3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BYTE2` reader - Byte2"]
pub type Byte2R = crate::FieldReader;
#[doc = "Field `BYTE2` writer - Byte2"]
pub type Byte2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BYTE1` reader - Byte1"]
pub type Byte1R = crate::FieldReader;
#[doc = "Field `BYTE1` writer - Byte1"]
pub type Byte1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Byte4"]
    #[inline(always)]
    pub fn byte4(&self) -> Byte4R {
        Byte4R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Byte3"]
    #[inline(always)]
    pub fn byte3(&self) -> Byte3R {
        Byte3R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Byte2"]
    #[inline(always)]
    pub fn byte2(&self) -> Byte2R {
        Byte2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Byte1"]
    #[inline(always)]
    pub fn byte1(&self) -> Byte1R {
        Byte1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Byte4"]
    #[inline(always)]
    pub fn byte4(&mut self) -> Byte4W<Pkttx1NxtsendroutSpec> {
        Byte4W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Byte3"]
    #[inline(always)]
    pub fn byte3(&mut self) -> Byte3W<Pkttx1NxtsendroutSpec> {
        Byte3W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Byte2"]
    #[inline(always)]
    pub fn byte2(&mut self) -> Byte2W<Pkttx1NxtsendroutSpec> {
        Byte2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Byte1"]
    #[inline(always)]
    pub fn byte1(&mut self) -> Byte1W<Pkttx1NxtsendroutSpec> {
        Byte1W::new(self, 24)
    }
}
#[doc = "PktTx Next Send List Router Bytes\n\nYou can [`read`](crate::Reg::read) this register and get [`pkttx1_nxtsendrout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pkttx1_nxtsendrout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pkttx1NxtsendroutSpec;
impl crate::RegisterSpec for Pkttx1NxtsendroutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pkttx1_nxtsendrout::R`](R) reader structure"]
impl crate::Readable for Pkttx1NxtsendroutSpec {}
#[doc = "`write(|w| ..)` method takes [`pkttx1_nxtsendrout::W`](W) writer structure"]
impl crate::Writable for Pkttx1NxtsendroutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PKTTX1_NXTSENDROUT to value 0"]
impl crate::Resettable for Pkttx1NxtsendroutSpec {}
