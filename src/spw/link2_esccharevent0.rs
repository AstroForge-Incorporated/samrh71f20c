#[doc = "Register `LINK2_ESCCHAREVENT0` reader"]
pub type R = crate::R<Link2Esccharevent0Spec>;
#[doc = "Register `LINK2_ESCCHAREVENT0` writer"]
pub type W = crate::W<Link2Esccharevent0Spec>;
#[doc = "Field `VALUE` reader - Value"]
pub type ValueR = crate::FieldReader;
#[doc = "Field `VALUE` writer - Value"]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MASK` reader - Mask"]
pub type MaskR = crate::FieldReader;
#[doc = "Field `MASK` writer - Mask"]
pub type MaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ACTIVE` reader - Active"]
pub type ActiveR = crate::BitReader;
#[doc = "Field `ACTIVE` writer - Active"]
pub type ActiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWEVENT` reader - HwEvent"]
pub type HweventR = crate::BitReader;
#[doc = "Field `HWEVENT` writer - HwEvent"]
pub type HweventW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Value"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Mask"]
    #[inline(always)]
    pub fn mask(&self) -> MaskR {
        MaskR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Active"]
    #[inline(always)]
    pub fn active(&self) -> ActiveR {
        ActiveR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HwEvent"]
    #[inline(always)]
    pub fn hwevent(&self) -> HweventR {
        HweventR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Value"]
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<Link2Esccharevent0Spec> {
        ValueW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Mask"]
    #[inline(always)]
    pub fn mask(&mut self) -> MaskW<Link2Esccharevent0Spec> {
        MaskW::new(self, 8)
    }
    #[doc = "Bit 16 - Active"]
    #[inline(always)]
    pub fn active(&mut self) -> ActiveW<Link2Esccharevent0Spec> {
        ActiveW::new(self, 16)
    }
    #[doc = "Bit 17 - HwEvent"]
    #[inline(always)]
    pub fn hwevent(&mut self) -> HweventW<Link2Esccharevent0Spec> {
        HweventW::new(self, 17)
    }
}
#[doc = "SpW Link 2 Escape Character Event 0\n\nYou can [`read`](crate::Reg::read) this register and get [`link2_esccharevent0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link2_esccharevent0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Link2Esccharevent0Spec;
impl crate::RegisterSpec for Link2Esccharevent0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`link2_esccharevent0::R`](R) reader structure"]
impl crate::Readable for Link2Esccharevent0Spec {}
#[doc = "`write(|w| ..)` method takes [`link2_esccharevent0::W`](W) writer structure"]
impl crate::Writable for Link2Esccharevent0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LINK2_ESCCHAREVENT0 to value 0"]
impl crate::Resettable for Link2Esccharevent0Spec {}
