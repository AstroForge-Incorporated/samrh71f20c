#[doc = "Register `LINK1_PI_C` writer"]
pub type W = crate::W<Link1PiCSpec>;
#[doc = "Field `DISERR` writer - DisErr"]
pub type DiserrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARERR` writer - ParErr"]
pub type ParerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESCERR` writer - ESCErr"]
pub type EscerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRERR` writer - CrErr"]
pub type CrerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINKABORT` writer - LinkAbort"]
pub type LinkabortW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EEPTRANS` writer - EEP transmitted"]
pub type EeptransW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EEPREC` writer - EEP received"]
pub type EeprecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCARD` writer - Discard"]
pub type DiscardW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESCEVENT2` writer - Escape Event 2"]
pub type Escevent2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESCEVENT1` writer - Escape Event 1"]
pub type Escevent1W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - DisErr"]
    #[inline(always)]
    pub fn diserr(&mut self) -> DiserrW<Link1PiCSpec> {
        DiserrW::new(self, 0)
    }
    #[doc = "Bit 1 - ParErr"]
    #[inline(always)]
    pub fn parerr(&mut self) -> ParerrW<Link1PiCSpec> {
        ParerrW::new(self, 1)
    }
    #[doc = "Bit 2 - ESCErr"]
    #[inline(always)]
    pub fn escerr(&mut self) -> EscerrW<Link1PiCSpec> {
        EscerrW::new(self, 2)
    }
    #[doc = "Bit 3 - CrErr"]
    #[inline(always)]
    pub fn crerr(&mut self) -> CrerrW<Link1PiCSpec> {
        CrerrW::new(self, 3)
    }
    #[doc = "Bit 4 - LinkAbort"]
    #[inline(always)]
    pub fn linkabort(&mut self) -> LinkabortW<Link1PiCSpec> {
        LinkabortW::new(self, 4)
    }
    #[doc = "Bit 5 - EEP transmitted"]
    #[inline(always)]
    pub fn eeptrans(&mut self) -> EeptransW<Link1PiCSpec> {
        EeptransW::new(self, 5)
    }
    #[doc = "Bit 6 - EEP received"]
    #[inline(always)]
    pub fn eeprec(&mut self) -> EeprecW<Link1PiCSpec> {
        EeprecW::new(self, 6)
    }
    #[doc = "Bit 7 - Discard"]
    #[inline(always)]
    pub fn discard(&mut self) -> DiscardW<Link1PiCSpec> {
        DiscardW::new(self, 7)
    }
    #[doc = "Bit 8 - Escape Event 2"]
    #[inline(always)]
    pub fn escevent2(&mut self) -> Escevent2W<Link1PiCSpec> {
        Escevent2W::new(self, 8)
    }
    #[doc = "Bit 9 - Escape Event 1"]
    #[inline(always)]
    pub fn escevent1(&mut self) -> Escevent1W<Link1PiCSpec> {
        Escevent1W::new(self, 9)
    }
}
#[doc = "SpW Link 1 Clear Pending Interrupt\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link1_pi_c::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Link1PiCSpec;
impl crate::RegisterSpec for Link1PiCSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`link1_pi_c::W`](W) writer structure"]
impl crate::Writable for Link1PiCSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LINK1_PI_C to value 0"]
impl crate::Resettable for Link1PiCSpec {}
