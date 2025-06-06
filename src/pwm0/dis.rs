#[doc = "Register `DIS` writer"]
pub type W = crate::W<DisSpec>;
#[doc = "Field `CHID0` writer - Channel ID"]
pub type Chid0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHID1` writer - Channel ID"]
pub type Chid1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHID2` writer - Channel ID"]
pub type Chid2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHID3` writer - Channel ID"]
pub type Chid3W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel ID"]
    #[inline(always)]
    pub fn chid0(&mut self) -> Chid0W<DisSpec> {
        Chid0W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel ID"]
    #[inline(always)]
    pub fn chid1(&mut self) -> Chid1W<DisSpec> {
        Chid1W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel ID"]
    #[inline(always)]
    pub fn chid2(&mut self) -> Chid2W<DisSpec> {
        Chid2W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel ID"]
    #[inline(always)]
    pub fn chid3(&mut self) -> Chid3W<DisSpec> {
        Chid3W::new(self, 3)
    }
}
#[doc = "PWM Disable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dis::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DisSpec;
impl crate::RegisterSpec for DisSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dis::W`](W) writer structure"]
impl crate::Writable for DisSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIS to value 0"]
impl crate::Resettable for DisSpec {}
