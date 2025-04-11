#[doc = "Register `LINK2_IM_C` writer"]
pub type W = crate::W<Link2ImCSpec>;
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
    pub fn diserr(&mut self) -> DiserrW<Link2ImCSpec> {
        DiserrW::new(self, 0)
    }
    #[doc = "Bit 1 - ParErr"]
    #[inline(always)]
    pub fn parerr(&mut self) -> ParerrW<Link2ImCSpec> {
        ParerrW::new(self, 1)
    }
    #[doc = "Bit 2 - ESCErr"]
    #[inline(always)]
    pub fn escerr(&mut self) -> EscerrW<Link2ImCSpec> {
        EscerrW::new(self, 2)
    }
    #[doc = "Bit 3 - CrErr"]
    #[inline(always)]
    pub fn crerr(&mut self) -> CrerrW<Link2ImCSpec> {
        CrerrW::new(self, 3)
    }
    #[doc = "Bit 4 - LinkAbort"]
    #[inline(always)]
    pub fn linkabort(&mut self) -> LinkabortW<Link2ImCSpec> {
        LinkabortW::new(self, 4)
    }
    #[doc = "Bit 5 - EEP transmitted"]
    #[inline(always)]
    pub fn eeptrans(&mut self) -> EeptransW<Link2ImCSpec> {
        EeptransW::new(self, 5)
    }
    #[doc = "Bit 6 - EEP received"]
    #[inline(always)]
    pub fn eeprec(&mut self) -> EeprecW<Link2ImCSpec> {
        EeprecW::new(self, 6)
    }
    #[doc = "Bit 7 - Discard"]
    #[inline(always)]
    pub fn discard(&mut self) -> DiscardW<Link2ImCSpec> {
        DiscardW::new(self, 7)
    }
    #[doc = "Bit 8 - Escape Event 2"]
    #[inline(always)]
    pub fn escevent2(&mut self) -> Escevent2W<Link2ImCSpec> {
        Escevent2W::new(self, 8)
    }
    #[doc = "Bit 9 - Escape Event 1"]
    #[inline(always)]
    pub fn escevent1(&mut self) -> Escevent1W<Link2ImCSpec> {
        Escevent1W::new(self, 9)
    }
}
#[doc = "SpW Link 2 Interrupt Clear Mask\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link2_im_c::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Link2ImCSpec;
impl crate::RegisterSpec for Link2ImCSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`link2_im_c::W`](W) writer structure"]
impl crate::Writable for Link2ImCSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LINK2_IM_C to value 0"]
impl crate::Resettable for Link2ImCSpec {}
