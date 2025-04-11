#[doc = "Register `PRTSR[%s]` reader"]
pub type R = crate::R<PrtsrSpec>;
#[doc = "Register `PRTSR[%s]` writer"]
pub type W = crate::W<PrtsrSpec>;
#[doc = "Field `PRTOP0` reader - HSELx Protected Region Top"]
pub type Prtop0R = crate::FieldReader;
#[doc = "Field `PRTOP0` writer - HSELx Protected Region Top"]
pub type Prtop0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRTOP1` reader - HSELx Protected Region Top"]
pub type Prtop1R = crate::FieldReader;
#[doc = "Field `PRTOP1` writer - HSELx Protected Region Top"]
pub type Prtop1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRTOP2` reader - HSELx Protected Region Top"]
pub type Prtop2R = crate::FieldReader;
#[doc = "Field `PRTOP2` writer - HSELx Protected Region Top"]
pub type Prtop2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRTOP3` reader - HSELx Protected Region Top"]
pub type Prtop3R = crate::FieldReader;
#[doc = "Field `PRTOP3` writer - HSELx Protected Region Top"]
pub type Prtop3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRTOP4` reader - HSELx Protected Region Top"]
pub type Prtop4R = crate::FieldReader;
#[doc = "Field `PRTOP4` writer - HSELx Protected Region Top"]
pub type Prtop4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRTOP5` reader - HSELx Protected Region Top"]
pub type Prtop5R = crate::FieldReader;
#[doc = "Field `PRTOP5` writer - HSELx Protected Region Top"]
pub type Prtop5W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRTOP6` reader - HSELx Protected Region Top"]
pub type Prtop6R = crate::FieldReader;
#[doc = "Field `PRTOP6` writer - HSELx Protected Region Top"]
pub type Prtop6W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRTOP7` reader - HSELx Protected Region Top"]
pub type Prtop7R = crate::FieldReader;
#[doc = "Field `PRTOP7` writer - HSELx Protected Region Top"]
pub type Prtop7W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - HSELx Protected Region Top"]
    #[inline(always)]
    pub fn prtop0(&self) -> Prtop0R {
        Prtop0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - HSELx Protected Region Top"]
    #[inline(always)]
    pub fn prtop1(&self) -> Prtop1R {
        Prtop1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - HSELx Protected Region Top"]
    #[inline(always)]
    pub fn prtop2(&self) -> Prtop2R {
        Prtop2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - HSELx Protected Region Top"]
    #[inline(always)]
    pub fn prtop3(&self) -> Prtop3R {
        Prtop3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - HSELx Protected Region Top"]
    #[inline(always)]
    pub fn prtop4(&self) -> Prtop4R {
        Prtop4R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - HSELx Protected Region Top"]
    #[inline(always)]
    pub fn prtop5(&self) -> Prtop5R {
        Prtop5R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - HSELx Protected Region Top"]
    #[inline(always)]
    pub fn prtop6(&self) -> Prtop6R {
        Prtop6R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - HSELx Protected Region Top"]
    #[inline(always)]
    pub fn prtop7(&self) -> Prtop7R {
        Prtop7R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - HSELx Protected Region Top"]
    #[inline(always)]
    pub fn prtop0(&mut self) -> Prtop0W<PrtsrSpec> {
        Prtop0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - HSELx Protected Region Top"]
    #[inline(always)]
    pub fn prtop1(&mut self) -> Prtop1W<PrtsrSpec> {
        Prtop1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - HSELx Protected Region Top"]
    #[inline(always)]
    pub fn prtop2(&mut self) -> Prtop2W<PrtsrSpec> {
        Prtop2W::new(self, 8)
    }
    #[doc = "Bits 12:15 - HSELx Protected Region Top"]
    #[inline(always)]
    pub fn prtop3(&mut self) -> Prtop3W<PrtsrSpec> {
        Prtop3W::new(self, 12)
    }
    #[doc = "Bits 16:19 - HSELx Protected Region Top"]
    #[inline(always)]
    pub fn prtop4(&mut self) -> Prtop4W<PrtsrSpec> {
        Prtop4W::new(self, 16)
    }
    #[doc = "Bits 20:23 - HSELx Protected Region Top"]
    #[inline(always)]
    pub fn prtop5(&mut self) -> Prtop5W<PrtsrSpec> {
        Prtop5W::new(self, 20)
    }
    #[doc = "Bits 24:27 - HSELx Protected Region Top"]
    #[inline(always)]
    pub fn prtop6(&mut self) -> Prtop6W<PrtsrSpec> {
        Prtop6W::new(self, 24)
    }
    #[doc = "Bits 28:31 - HSELx Protected Region Top"]
    #[inline(always)]
    pub fn prtop7(&mut self) -> Prtop7W<PrtsrSpec> {
        Prtop7W::new(self, 28)
    }
}
#[doc = "Protected Region Top Slave Register\n\nYou can [`read`](crate::Reg::read) this register and get [`prtsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prtsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrtsrSpec;
impl crate::RegisterSpec for PrtsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prtsr::R`](R) reader structure"]
impl crate::Readable for PrtsrSpec {}
#[doc = "`write(|w| ..)` method takes [`prtsr::W`](W) writer structure"]
impl crate::Writable for PrtsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRTSR[%s] to value 0"]
impl crate::Resettable for PrtsrSpec {}
