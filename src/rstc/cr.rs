#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `PROCRST` writer - Processor Reset"]
pub type ProcrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTRST` writer - External Reset"]
pub type ExtrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERIIDON` writer - External Reset"]
pub type PeriidonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERIID` writer - Peripheral Identifier"]
pub type PeriidW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "System Reset Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Keyselect {
    #[doc = "165: Writing any other value in this field aborts the write operation."]
    Passwd = 165,
}
impl From<Keyselect> for u8 {
    #[inline(always)]
    fn from(variant: Keyselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Keyselect {
    type Ux = u8;
}
impl crate::IsEnum for Keyselect {}
#[doc = "Field `KEY` writer - System Reset Key"]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, Keyselect>;
impl<'a, REG> KeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Writing any other value in this field aborts the write operation."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut crate::W<REG> {
        self.variant(Keyselect::Passwd)
    }
}
impl W {
    #[doc = "Bit 0 - Processor Reset"]
    #[inline(always)]
    pub fn procrst(&mut self) -> ProcrstW<CrSpec> {
        ProcrstW::new(self, 0)
    }
    #[doc = "Bit 3 - External Reset"]
    #[inline(always)]
    pub fn extrst(&mut self) -> ExtrstW<CrSpec> {
        ExtrstW::new(self, 3)
    }
    #[doc = "Bit 4 - External Reset"]
    #[inline(always)]
    pub fn periidon(&mut self) -> PeriidonW<CrSpec> {
        PeriidonW::new(self, 4)
    }
    #[doc = "Bits 8:15 - Peripheral Identifier"]
    #[inline(always)]
    pub fn periid(&mut self) -> PeriidW<CrSpec> {
        PeriidW::new(self, 8)
    }
    #[doc = "Bits 24:31 - System Reset Key"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<CrSpec> {
        KeyW::new(self, 24)
    }
}
#[doc = "Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
