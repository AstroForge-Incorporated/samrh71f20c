#[doc = "Register `IOFR` writer"]
pub type W = crate::W<IofrSpec>;
#[doc = "Field `FPHY` writer - Freeze Physical Configuration"]
pub type FphyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FINT` writer - Freeze Interrupt Configuration"]
pub type FintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Freeze Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Frzkeyselect {
    #[doc = "4804422: Writing any other value in this field aborts the write operation of the WPEN bit."]
    Passwd = 4804422,
}
impl From<Frzkeyselect> for u32 {
    #[inline(always)]
    fn from(variant: Frzkeyselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Frzkeyselect {
    type Ux = u32;
}
impl crate::IsEnum for Frzkeyselect {}
#[doc = "Field `FRZKEY` writer - Freeze Key"]
pub type FrzkeyW<'a, REG> = crate::FieldWriter<'a, REG, 24, Frzkeyselect>;
impl<'a, REG> FrzkeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Writing any other value in this field aborts the write operation of the WPEN bit."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut crate::W<REG> {
        self.variant(Frzkeyselect::Passwd)
    }
}
impl W {
    #[doc = "Bit 0 - Freeze Physical Configuration"]
    #[inline(always)]
    pub fn fphy(&mut self) -> FphyW<IofrSpec> {
        FphyW::new(self, 0)
    }
    #[doc = "Bit 1 - Freeze Interrupt Configuration"]
    #[inline(always)]
    pub fn fint(&mut self) -> FintW<IofrSpec> {
        FintW::new(self, 1)
    }
    #[doc = "Bits 8:31 - Freeze Key"]
    #[inline(always)]
    pub fn frzkey(&mut self) -> FrzkeyW<IofrSpec> {
        FrzkeyW::new(self, 8)
    }
}
#[doc = "PIO I/O Freeze Configuration Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iofr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IofrSpec;
impl crate::RegisterSpec for IofrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`iofr::W`](W) writer structure"]
impl crate::Writable for IofrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IOFR to value 0"]
impl crate::Resettable for IofrSpec {}
