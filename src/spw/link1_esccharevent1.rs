#[doc = "Register `LINK1_ESCCHAREVENT1` reader"]
pub type R = crate::R<Link1Esccharevent1Spec>;
#[doc = "Register `LINK1_ESCCHAREVENT1` writer"]
pub type W = crate::W<Link1Esccharevent1Spec>;
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
    pub fn value(&mut self) -> ValueW<Link1Esccharevent1Spec> {
        ValueW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Mask"]
    #[inline(always)]
    pub fn mask(&mut self) -> MaskW<Link1Esccharevent1Spec> {
        MaskW::new(self, 8)
    }
    #[doc = "Bit 16 - Active"]
    #[inline(always)]
    pub fn active(&mut self) -> ActiveW<Link1Esccharevent1Spec> {
        ActiveW::new(self, 16)
    }
    #[doc = "Bit 17 - HwEvent"]
    #[inline(always)]
    pub fn hwevent(&mut self) -> HweventW<Link1Esccharevent1Spec> {
        HweventW::new(self, 17)
    }
}
#[doc = "SpW Link 1 Escape Character Event 1\n\nYou can [`read`](crate::Reg::read) this register and get [`link1_esccharevent1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link1_esccharevent1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Link1Esccharevent1Spec;
impl crate::RegisterSpec for Link1Esccharevent1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`link1_esccharevent1::R`](R) reader structure"]
impl crate::Readable for Link1Esccharevent1Spec {}
#[doc = "`write(|w| ..)` method takes [`link1_esccharevent1::W`](W) writer structure"]
impl crate::Writable for Link1Esccharevent1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LINK1_ESCCHAREVENT1 to value 0"]
impl crate::Resettable for Link1Esccharevent1Spec {}
