#[doc = "Register `FLEX_TWI_FILTR` reader"]
pub type R = crate::R<FlexTwiFiltrSpec>;
#[doc = "Register `FLEX_TWI_FILTR` writer"]
pub type W = crate::W<FlexTwiFiltrSpec>;
#[doc = "Field `FILT` reader - RX Digital Filter"]
pub type FiltR = crate::BitReader;
#[doc = "Field `FILT` writer - RX Digital Filter"]
pub type FiltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADFEN` reader - PAD Filter Enable"]
pub type PadfenR = crate::BitReader;
#[doc = "Field `PADFEN` writer - PAD Filter Enable"]
pub type PadfenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THRES` reader - Digital Filter Threshold"]
pub type ThresR = crate::FieldReader;
#[doc = "Field `THRES` writer - Digital Filter Threshold"]
pub type ThresW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - RX Digital Filter"]
    #[inline(always)]
    pub fn filt(&self) -> FiltR {
        FiltR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PAD Filter Enable"]
    #[inline(always)]
    pub fn padfen(&self) -> PadfenR {
        PadfenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Digital Filter Threshold"]
    #[inline(always)]
    pub fn thres(&self) -> ThresR {
        ThresR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - RX Digital Filter"]
    #[inline(always)]
    pub fn filt(&mut self) -> FiltW<FlexTwiFiltrSpec> {
        FiltW::new(self, 0)
    }
    #[doc = "Bit 1 - PAD Filter Enable"]
    #[inline(always)]
    pub fn padfen(&mut self) -> PadfenW<FlexTwiFiltrSpec> {
        PadfenW::new(self, 1)
    }
    #[doc = "Bits 8:10 - Digital Filter Threshold"]
    #[inline(always)]
    pub fn thres(&mut self) -> ThresW<FlexTwiFiltrSpec> {
        ThresW::new(self, 8)
    }
}
#[doc = "TWI Filter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_twi_filtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_twi_filtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexTwiFiltrSpec;
impl crate::RegisterSpec for FlexTwiFiltrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flex_twi_filtr::R`](R) reader structure"]
impl crate::Readable for FlexTwiFiltrSpec {}
#[doc = "`write(|w| ..)` method takes [`flex_twi_filtr::W`](W) writer structure"]
impl crate::Writable for FlexTwiFiltrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLEX_TWI_FILTR to value 0"]
impl crate::Resettable for FlexTwiFiltrSpec {}
