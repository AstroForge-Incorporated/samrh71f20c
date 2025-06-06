#[doc = "Register `OCR1` reader"]
pub type R = crate::R<Ocr1Spec>;
#[doc = "Register `OCR1` writer"]
pub type W = crate::W<Ocr1Spec>;
#[doc = "Field `CAL4` reader - Main RC Oscillator Calibration Bits for 4 MHz"]
pub type Cal4R = crate::FieldReader;
#[doc = "Field `CAL4` writer - Main RC Oscillator Calibration Bits for 4 MHz"]
pub type Cal4W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SEL4` reader - Selection of Main RC Oscillator Calibration Bits for 4 MHz"]
pub type Sel4R = crate::BitReader;
#[doc = "Field `SEL4` writer - Selection of Main RC Oscillator Calibration Bits for 4 MHz"]
pub type Sel4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAL8` reader - Main RC Oscillator Calibration Bits for 8 MHz"]
pub type Cal8R = crate::FieldReader;
#[doc = "Field `CAL8` writer - Main RC Oscillator Calibration Bits for 8 MHz"]
pub type Cal8W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SEL8` reader - Selection of Main RC Oscillator Calibration Bits for 8 MHz"]
pub type Sel8R = crate::BitReader;
#[doc = "Field `SEL8` writer - Selection of Main RC Oscillator Calibration Bits for 8 MHz"]
pub type Sel8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAL12` reader - Main RC Oscillator Calibration Bits for 12 MHz"]
pub type Cal12R = crate::FieldReader;
#[doc = "Field `CAL12` writer - Main RC Oscillator Calibration Bits for 12 MHz"]
pub type Cal12W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SEL10` reader - Selection of Main RC Oscillator Calibration Bits for 10 MHz"]
pub type Sel10R = crate::BitReader;
#[doc = "Field `SEL10` writer - Selection of Main RC Oscillator Calibration Bits for 10 MHz"]
pub type Sel10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAL10` reader - Main RC Oscillator Calibration Bits for 10 MHz"]
pub type Cal10R = crate::FieldReader;
#[doc = "Field `CAL10` writer - Main RC Oscillator Calibration Bits for 10 MHz"]
pub type Cal10W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SEL12` reader - Selection of Main RC Oscillator Calibration Bits for 12 MHz"]
pub type Sel12R = crate::BitReader;
#[doc = "Field `SEL12` writer - Selection of Main RC Oscillator Calibration Bits for 12 MHz"]
pub type Sel12W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Main RC Oscillator Calibration Bits for 4 MHz"]
    #[inline(always)]
    pub fn cal4(&self) -> Cal4R {
        Cal4R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Selection of Main RC Oscillator Calibration Bits for 4 MHz"]
    #[inline(always)]
    pub fn sel4(&self) -> Sel4R {
        Sel4R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - Main RC Oscillator Calibration Bits for 8 MHz"]
    #[inline(always)]
    pub fn cal8(&self) -> Cal8R {
        Cal8R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Selection of Main RC Oscillator Calibration Bits for 8 MHz"]
    #[inline(always)]
    pub fn sel8(&self) -> Sel8R {
        Sel8R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:22 - Main RC Oscillator Calibration Bits for 12 MHz"]
    #[inline(always)]
    pub fn cal12(&self) -> Cal12R {
        Cal12R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - Selection of Main RC Oscillator Calibration Bits for 10 MHz"]
    #[inline(always)]
    pub fn sel10(&self) -> Sel10R {
        Sel10R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:30 - Main RC Oscillator Calibration Bits for 10 MHz"]
    #[inline(always)]
    pub fn cal10(&self) -> Cal10R {
        Cal10R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - Selection of Main RC Oscillator Calibration Bits for 12 MHz"]
    #[inline(always)]
    pub fn sel12(&self) -> Sel12R {
        Sel12R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Main RC Oscillator Calibration Bits for 4 MHz"]
    #[inline(always)]
    pub fn cal4(&mut self) -> Cal4W<Ocr1Spec> {
        Cal4W::new(self, 0)
    }
    #[doc = "Bit 7 - Selection of Main RC Oscillator Calibration Bits for 4 MHz"]
    #[inline(always)]
    pub fn sel4(&mut self) -> Sel4W<Ocr1Spec> {
        Sel4W::new(self, 7)
    }
    #[doc = "Bits 8:14 - Main RC Oscillator Calibration Bits for 8 MHz"]
    #[inline(always)]
    pub fn cal8(&mut self) -> Cal8W<Ocr1Spec> {
        Cal8W::new(self, 8)
    }
    #[doc = "Bit 15 - Selection of Main RC Oscillator Calibration Bits for 8 MHz"]
    #[inline(always)]
    pub fn sel8(&mut self) -> Sel8W<Ocr1Spec> {
        Sel8W::new(self, 15)
    }
    #[doc = "Bits 16:22 - Main RC Oscillator Calibration Bits for 12 MHz"]
    #[inline(always)]
    pub fn cal12(&mut self) -> Cal12W<Ocr1Spec> {
        Cal12W::new(self, 16)
    }
    #[doc = "Bit 23 - Selection of Main RC Oscillator Calibration Bits for 10 MHz"]
    #[inline(always)]
    pub fn sel10(&mut self) -> Sel10W<Ocr1Spec> {
        Sel10W::new(self, 23)
    }
    #[doc = "Bits 24:30 - Main RC Oscillator Calibration Bits for 10 MHz"]
    #[inline(always)]
    pub fn cal10(&mut self) -> Cal10W<Ocr1Spec> {
        Cal10W::new(self, 24)
    }
    #[doc = "Bit 31 - Selection of Main RC Oscillator Calibration Bits for 12 MHz"]
    #[inline(always)]
    pub fn sel12(&mut self) -> Sel12W<Ocr1Spec> {
        Sel12W::new(self, 31)
    }
}
#[doc = "Oscillator Calibration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ocr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ocr1Spec;
impl crate::RegisterSpec for Ocr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ocr1::R`](R) reader structure"]
impl crate::Readable for Ocr1Spec {}
#[doc = "`write(|w| ..)` method takes [`ocr1::W`](W) writer structure"]
impl crate::Writable for Ocr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OCR1 to value 0"]
impl crate::Resettable for Ocr1Spec {}
