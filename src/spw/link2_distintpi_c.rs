#[doc = "Register `LINK2_DISTINTPI_C` writer"]
pub type W = crate::W<Link2DistintpiCSpec>;
#[doc = "Field `DI0` writer - Distributed Interrupt"]
pub type Di0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI1` writer - Distributed Interrupt"]
pub type Di1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI2` writer - Distributed Interrupt"]
pub type Di2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI3` writer - Distributed Interrupt"]
pub type Di3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI4` writer - Distributed Interrupt"]
pub type Di4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI5` writer - Distributed Interrupt"]
pub type Di5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI6` writer - Distributed Interrupt"]
pub type Di6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI7` writer - Distributed Interrupt"]
pub type Di7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI8` writer - Distributed Interrupt"]
pub type Di8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI9` writer - Distributed Interrupt"]
pub type Di9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI10` writer - Distributed Interrupt"]
pub type Di10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI11` writer - Distributed Interrupt"]
pub type Di11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI12` writer - Distributed Interrupt"]
pub type Di12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI13` writer - Distributed Interrupt"]
pub type Di13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI14` writer - Distributed Interrupt"]
pub type Di14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI15` writer - Distributed Interrupt"]
pub type Di15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI16` writer - Distributed Interrupt"]
pub type Di16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI17` writer - Distributed Interrupt"]
pub type Di17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI18` writer - Distributed Interrupt"]
pub type Di18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI19` writer - Distributed Interrupt"]
pub type Di19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI20` writer - Distributed Interrupt"]
pub type Di20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI21` writer - Distributed Interrupt"]
pub type Di21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI22` writer - Distributed Interrupt"]
pub type Di22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI23` writer - Distributed Interrupt"]
pub type Di23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI24` writer - Distributed Interrupt"]
pub type Di24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI25` writer - Distributed Interrupt"]
pub type Di25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI26` writer - Distributed Interrupt"]
pub type Di26W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI27` writer - Distributed Interrupt"]
pub type Di27W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI28` writer - Distributed Interrupt"]
pub type Di28W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI29` writer - Distributed Interrupt"]
pub type Di29W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI30` writer - Distributed Interrupt"]
pub type Di30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DI31` writer - Distributed Interrupt"]
pub type Di31W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di0(&mut self) -> Di0W<Link2DistintpiCSpec> {
        Di0W::new(self, 0)
    }
    #[doc = "Bit 1 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di1(&mut self) -> Di1W<Link2DistintpiCSpec> {
        Di1W::new(self, 1)
    }
    #[doc = "Bit 2 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di2(&mut self) -> Di2W<Link2DistintpiCSpec> {
        Di2W::new(self, 2)
    }
    #[doc = "Bit 3 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di3(&mut self) -> Di3W<Link2DistintpiCSpec> {
        Di3W::new(self, 3)
    }
    #[doc = "Bit 4 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di4(&mut self) -> Di4W<Link2DistintpiCSpec> {
        Di4W::new(self, 4)
    }
    #[doc = "Bit 5 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di5(&mut self) -> Di5W<Link2DistintpiCSpec> {
        Di5W::new(self, 5)
    }
    #[doc = "Bit 6 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di6(&mut self) -> Di6W<Link2DistintpiCSpec> {
        Di6W::new(self, 6)
    }
    #[doc = "Bit 7 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di7(&mut self) -> Di7W<Link2DistintpiCSpec> {
        Di7W::new(self, 7)
    }
    #[doc = "Bit 8 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di8(&mut self) -> Di8W<Link2DistintpiCSpec> {
        Di8W::new(self, 8)
    }
    #[doc = "Bit 9 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di9(&mut self) -> Di9W<Link2DistintpiCSpec> {
        Di9W::new(self, 9)
    }
    #[doc = "Bit 10 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di10(&mut self) -> Di10W<Link2DistintpiCSpec> {
        Di10W::new(self, 10)
    }
    #[doc = "Bit 11 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di11(&mut self) -> Di11W<Link2DistintpiCSpec> {
        Di11W::new(self, 11)
    }
    #[doc = "Bit 12 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di12(&mut self) -> Di12W<Link2DistintpiCSpec> {
        Di12W::new(self, 12)
    }
    #[doc = "Bit 13 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di13(&mut self) -> Di13W<Link2DistintpiCSpec> {
        Di13W::new(self, 13)
    }
    #[doc = "Bit 14 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di14(&mut self) -> Di14W<Link2DistintpiCSpec> {
        Di14W::new(self, 14)
    }
    #[doc = "Bit 15 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di15(&mut self) -> Di15W<Link2DistintpiCSpec> {
        Di15W::new(self, 15)
    }
    #[doc = "Bit 16 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di16(&mut self) -> Di16W<Link2DistintpiCSpec> {
        Di16W::new(self, 16)
    }
    #[doc = "Bit 17 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di17(&mut self) -> Di17W<Link2DistintpiCSpec> {
        Di17W::new(self, 17)
    }
    #[doc = "Bit 18 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di18(&mut self) -> Di18W<Link2DistintpiCSpec> {
        Di18W::new(self, 18)
    }
    #[doc = "Bit 19 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di19(&mut self) -> Di19W<Link2DistintpiCSpec> {
        Di19W::new(self, 19)
    }
    #[doc = "Bit 20 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di20(&mut self) -> Di20W<Link2DistintpiCSpec> {
        Di20W::new(self, 20)
    }
    #[doc = "Bit 21 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di21(&mut self) -> Di21W<Link2DistintpiCSpec> {
        Di21W::new(self, 21)
    }
    #[doc = "Bit 22 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di22(&mut self) -> Di22W<Link2DistintpiCSpec> {
        Di22W::new(self, 22)
    }
    #[doc = "Bit 23 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di23(&mut self) -> Di23W<Link2DistintpiCSpec> {
        Di23W::new(self, 23)
    }
    #[doc = "Bit 24 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di24(&mut self) -> Di24W<Link2DistintpiCSpec> {
        Di24W::new(self, 24)
    }
    #[doc = "Bit 25 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di25(&mut self) -> Di25W<Link2DistintpiCSpec> {
        Di25W::new(self, 25)
    }
    #[doc = "Bit 26 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di26(&mut self) -> Di26W<Link2DistintpiCSpec> {
        Di26W::new(self, 26)
    }
    #[doc = "Bit 27 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di27(&mut self) -> Di27W<Link2DistintpiCSpec> {
        Di27W::new(self, 27)
    }
    #[doc = "Bit 28 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di28(&mut self) -> Di28W<Link2DistintpiCSpec> {
        Di28W::new(self, 28)
    }
    #[doc = "Bit 29 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di29(&mut self) -> Di29W<Link2DistintpiCSpec> {
        Di29W::new(self, 29)
    }
    #[doc = "Bit 30 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di30(&mut self) -> Di30W<Link2DistintpiCSpec> {
        Di30W::new(self, 30)
    }
    #[doc = "Bit 31 - Distributed Interrupt"]
    #[inline(always)]
    pub fn di31(&mut self) -> Di31W<Link2DistintpiCSpec> {
        Di31W::new(self, 31)
    }
}
#[doc = "SpW Link 2 Distributed Interrupt Clear Pending Interrupt\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link2_distintpi_c::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Link2DistintpiCSpec;
impl crate::RegisterSpec for Link2DistintpiCSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`link2_distintpi_c::W`](W) writer structure"]
impl crate::Writable for Link2DistintpiCSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LINK2_DISTINTPI_C to value 0"]
impl crate::Resettable for Link2DistintpiCSpec {}
