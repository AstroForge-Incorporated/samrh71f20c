#[doc = "Register `FLEX_SPI_CMPR` reader"]
pub type R = crate::R<FlexSpiCmprSpec>;
#[doc = "Register `FLEX_SPI_CMPR` writer"]
pub type W = crate::W<FlexSpiCmprSpec>;
#[doc = "Field `VAL1` reader - First Comparison Value for Received Character"]
pub type Val1R = crate::FieldReader<u16>;
#[doc = "Field `VAL1` writer - First Comparison Value for Received Character"]
pub type Val1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `VAL2` reader - Second Comparison Value for Received Character"]
pub type Val2R = crate::FieldReader<u16>;
#[doc = "Field `VAL2` writer - Second Comparison Value for Received Character"]
pub type Val2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - First Comparison Value for Received Character"]
    #[inline(always)]
    pub fn val1(&self) -> Val1R {
        Val1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Second Comparison Value for Received Character"]
    #[inline(always)]
    pub fn val2(&self) -> Val2R {
        Val2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - First Comparison Value for Received Character"]
    #[inline(always)]
    pub fn val1(&mut self) -> Val1W<FlexSpiCmprSpec> {
        Val1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Second Comparison Value for Received Character"]
    #[inline(always)]
    pub fn val2(&mut self) -> Val2W<FlexSpiCmprSpec> {
        Val2W::new(self, 16)
    }
}
#[doc = "SPI Comparison Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_spi_cmpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_spi_cmpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexSpiCmprSpec;
impl crate::RegisterSpec for FlexSpiCmprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flex_spi_cmpr::R`](R) reader structure"]
impl crate::Readable for FlexSpiCmprSpec {}
#[doc = "`write(|w| ..)` method takes [`flex_spi_cmpr::W`](W) writer structure"]
impl crate::Writable for FlexSpiCmprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLEX_SPI_CMPR to value 0"]
impl crate::Resettable for FlexSpiCmprSpec {}
