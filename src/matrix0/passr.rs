#[doc = "Register `PASSR[%s]` reader"]
pub type R = crate::R<PassrSpec>;
#[doc = "Register `PASSR[%s]` writer"]
pub type W = crate::W<PassrSpec>;
#[doc = "Field `PASPLIT0` reader - Protected Areas Split for HSELx Protected Region"]
pub type Pasplit0R = crate::FieldReader;
#[doc = "Field `PASPLIT0` writer - Protected Areas Split for HSELx Protected Region"]
pub type Pasplit0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PASPLIT1` reader - Protected Areas Split for HSELx Protected Region"]
pub type Pasplit1R = crate::FieldReader;
#[doc = "Field `PASPLIT1` writer - Protected Areas Split for HSELx Protected Region"]
pub type Pasplit1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PASPLIT2` reader - Protected Areas Split for HSELx Protected Region"]
pub type Pasplit2R = crate::FieldReader;
#[doc = "Field `PASPLIT2` writer - Protected Areas Split for HSELx Protected Region"]
pub type Pasplit2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PASPLIT3` reader - Protected Areas Split for HSELx Protected Region"]
pub type Pasplit3R = crate::FieldReader;
#[doc = "Field `PASPLIT3` writer - Protected Areas Split for HSELx Protected Region"]
pub type Pasplit3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PASPLIT4` reader - Protected Areas Split for HSELx Protected Region"]
pub type Pasplit4R = crate::FieldReader;
#[doc = "Field `PASPLIT4` writer - Protected Areas Split for HSELx Protected Region"]
pub type Pasplit4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PASPLIT5` reader - Protected Areas Split for HSELx Protected Region"]
pub type Pasplit5R = crate::FieldReader;
#[doc = "Field `PASPLIT5` writer - Protected Areas Split for HSELx Protected Region"]
pub type Pasplit5W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PASPLIT6` reader - Protected Areas Split for HSELx Protected Region"]
pub type Pasplit6R = crate::FieldReader;
#[doc = "Field `PASPLIT6` writer - Protected Areas Split for HSELx Protected Region"]
pub type Pasplit6W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PASPLIT7` reader - Protected Areas Split for HSELx Protected Region"]
pub type Pasplit7R = crate::FieldReader;
#[doc = "Field `PASPLIT7` writer - Protected Areas Split for HSELx Protected Region"]
pub type Pasplit7W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Protected Areas Split for HSELx Protected Region"]
    #[inline(always)]
    pub fn pasplit0(&self) -> Pasplit0R {
        Pasplit0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Protected Areas Split for HSELx Protected Region"]
    #[inline(always)]
    pub fn pasplit1(&self) -> Pasplit1R {
        Pasplit1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Protected Areas Split for HSELx Protected Region"]
    #[inline(always)]
    pub fn pasplit2(&self) -> Pasplit2R {
        Pasplit2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Protected Areas Split for HSELx Protected Region"]
    #[inline(always)]
    pub fn pasplit3(&self) -> Pasplit3R {
        Pasplit3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Protected Areas Split for HSELx Protected Region"]
    #[inline(always)]
    pub fn pasplit4(&self) -> Pasplit4R {
        Pasplit4R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Protected Areas Split for HSELx Protected Region"]
    #[inline(always)]
    pub fn pasplit5(&self) -> Pasplit5R {
        Pasplit5R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Protected Areas Split for HSELx Protected Region"]
    #[inline(always)]
    pub fn pasplit6(&self) -> Pasplit6R {
        Pasplit6R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Protected Areas Split for HSELx Protected Region"]
    #[inline(always)]
    pub fn pasplit7(&self) -> Pasplit7R {
        Pasplit7R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Protected Areas Split for HSELx Protected Region"]
    #[inline(always)]
    pub fn pasplit0(&mut self) -> Pasplit0W<PassrSpec> {
        Pasplit0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Protected Areas Split for HSELx Protected Region"]
    #[inline(always)]
    pub fn pasplit1(&mut self) -> Pasplit1W<PassrSpec> {
        Pasplit1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Protected Areas Split for HSELx Protected Region"]
    #[inline(always)]
    pub fn pasplit2(&mut self) -> Pasplit2W<PassrSpec> {
        Pasplit2W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Protected Areas Split for HSELx Protected Region"]
    #[inline(always)]
    pub fn pasplit3(&mut self) -> Pasplit3W<PassrSpec> {
        Pasplit3W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Protected Areas Split for HSELx Protected Region"]
    #[inline(always)]
    pub fn pasplit4(&mut self) -> Pasplit4W<PassrSpec> {
        Pasplit4W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Protected Areas Split for HSELx Protected Region"]
    #[inline(always)]
    pub fn pasplit5(&mut self) -> Pasplit5W<PassrSpec> {
        Pasplit5W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Protected Areas Split for HSELx Protected Region"]
    #[inline(always)]
    pub fn pasplit6(&mut self) -> Pasplit6W<PassrSpec> {
        Pasplit6W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Protected Areas Split for HSELx Protected Region"]
    #[inline(always)]
    pub fn pasplit7(&mut self) -> Pasplit7W<PassrSpec> {
        Pasplit7W::new(self, 28)
    }
}
#[doc = "Protected Areas Split Slave Register\n\nYou can [`read`](crate::Reg::read) this register and get [`passr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`passr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PassrSpec;
impl crate::RegisterSpec for PassrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`passr::R`](R) reader structure"]
impl crate::Readable for PassrSpec {}
#[doc = "`write(|w| ..)` method takes [`passr::W`](W) writer structure"]
impl crate::Writable for PassrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PASSR[%s] to value 0"]
impl crate::Resettable for PassrSpec {}
