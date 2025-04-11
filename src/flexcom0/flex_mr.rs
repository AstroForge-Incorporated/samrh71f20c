#[doc = "Register `FLEX_MR` reader"]
pub type R = crate::R<FlexMrSpec>;
#[doc = "Register `FLEX_MR` writer"]
pub type W = crate::W<FlexMrSpec>;
#[doc = "FLEXCOM Operating Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Opmodeselect {
    #[doc = "0: No communication"]
    NoCom = 0,
    #[doc = "1: All UART related protocols are selected (RS232, RS485, IrDA, ISO7816, LIN, LON)SPI/TWI related registers are not accessible and have no impact on IOs."]
    Usart = 1,
    #[doc = "2: SPI operating mode is selected.USART/TWI related registers are not accessible and have no impact on IOs."]
    Spi = 2,
    #[doc = "3: All TWI related protocols are selected (TWI, SMBus).USART/SPI related registers are not accessible and have no impact on IOs."]
    Twi = 3,
}
impl From<Opmodeselect> for u8 {
    #[inline(always)]
    fn from(variant: Opmodeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Opmodeselect {
    type Ux = u8;
}
impl crate::IsEnum for Opmodeselect {}
#[doc = "Field `OPMODE` reader - FLEXCOM Operating Mode"]
pub type OpmodeR = crate::FieldReader<Opmodeselect>;
impl OpmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Opmodeselect {
        match self.bits {
            0 => Opmodeselect::NoCom,
            1 => Opmodeselect::Usart,
            2 => Opmodeselect::Spi,
            3 => Opmodeselect::Twi,
            _ => unreachable!(),
        }
    }
    #[doc = "No communication"]
    #[inline(always)]
    pub fn is_no_com(&self) -> bool {
        *self == Opmodeselect::NoCom
    }
    #[doc = "All UART related protocols are selected (RS232, RS485, IrDA, ISO7816, LIN, LON)SPI/TWI related registers are not accessible and have no impact on IOs."]
    #[inline(always)]
    pub fn is_usart(&self) -> bool {
        *self == Opmodeselect::Usart
    }
    #[doc = "SPI operating mode is selected.USART/TWI related registers are not accessible and have no impact on IOs."]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        *self == Opmodeselect::Spi
    }
    #[doc = "All TWI related protocols are selected (TWI, SMBus).USART/SPI related registers are not accessible and have no impact on IOs."]
    #[inline(always)]
    pub fn is_twi(&self) -> bool {
        *self == Opmodeselect::Twi
    }
}
#[doc = "Field `OPMODE` writer - FLEXCOM Operating Mode"]
pub type OpmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Opmodeselect, crate::Safe>;
impl<'a, REG> OpmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No communication"]
    #[inline(always)]
    pub fn no_com(self) -> &'a mut crate::W<REG> {
        self.variant(Opmodeselect::NoCom)
    }
    #[doc = "All UART related protocols are selected (RS232, RS485, IrDA, ISO7816, LIN, LON)SPI/TWI related registers are not accessible and have no impact on IOs."]
    #[inline(always)]
    pub fn usart(self) -> &'a mut crate::W<REG> {
        self.variant(Opmodeselect::Usart)
    }
    #[doc = "SPI operating mode is selected.USART/TWI related registers are not accessible and have no impact on IOs."]
    #[inline(always)]
    pub fn spi(self) -> &'a mut crate::W<REG> {
        self.variant(Opmodeselect::Spi)
    }
    #[doc = "All TWI related protocols are selected (TWI, SMBus).USART/SPI related registers are not accessible and have no impact on IOs."]
    #[inline(always)]
    pub fn twi(self) -> &'a mut crate::W<REG> {
        self.variant(Opmodeselect::Twi)
    }
}
impl R {
    #[doc = "Bits 0:1 - FLEXCOM Operating Mode"]
    #[inline(always)]
    pub fn opmode(&self) -> OpmodeR {
        OpmodeR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - FLEXCOM Operating Mode"]
    #[inline(always)]
    pub fn opmode(&mut self) -> OpmodeW<FlexMrSpec> {
        OpmodeW::new(self, 0)
    }
}
#[doc = "FLEXCOM Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flex_mr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flex_mr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlexMrSpec;
impl crate::RegisterSpec for FlexMrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flex_mr::R`](R) reader structure"]
impl crate::Readable for FlexMrSpec {}
#[doc = "`write(|w| ..)` method takes [`flex_mr::W`](W) writer structure"]
impl crate::Writable for FlexMrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLEX_MR to value 0"]
impl crate::Resettable for FlexMrSpec {}
