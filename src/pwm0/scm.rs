#[doc = "Register `SCM` reader"]
pub type R = crate::R<ScmSpec>;
#[doc = "Register `SCM` writer"]
pub type W = crate::W<ScmSpec>;
#[doc = "Field `SYNC0` reader - Synchronous Channel 0"]
pub type Sync0R = crate::BitReader;
#[doc = "Field `SYNC0` writer - Synchronous Channel 0"]
pub type Sync0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC1` reader - Synchronous Channel 1"]
pub type Sync1R = crate::BitReader;
#[doc = "Field `SYNC1` writer - Synchronous Channel 1"]
pub type Sync1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC2` reader - Synchronous Channel 2"]
pub type Sync2R = crate::BitReader;
#[doc = "Field `SYNC2` writer - Synchronous Channel 2"]
pub type Sync2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC3` reader - Synchronous Channel 3"]
pub type Sync3R = crate::BitReader;
#[doc = "Field `SYNC3` writer - Synchronous Channel 3"]
pub type Sync3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Synchronous Channels Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Updmselect {
    #[doc = "0: Manual write of double buffer registers and manual update of synchronous channels"]
    Mode0 = 0,
    #[doc = "1: Manual write of double buffer registers and automatic update of synchronous channels"]
    Mode1 = 1,
    #[doc = "2: Automatic write of duty-cycle update registers by the DMA Controller and automatic update of synchronous channels"]
    Mode2 = 2,
}
impl From<Updmselect> for u8 {
    #[inline(always)]
    fn from(variant: Updmselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Updmselect {
    type Ux = u8;
}
impl crate::IsEnum for Updmselect {}
#[doc = "Field `UPDM` reader - Synchronous Channels Update Mode"]
pub type UpdmR = crate::FieldReader<Updmselect>;
impl UpdmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Updmselect> {
        match self.bits {
            0 => Some(Updmselect::Mode0),
            1 => Some(Updmselect::Mode1),
            2 => Some(Updmselect::Mode2),
            _ => None,
        }
    }
    #[doc = "Manual write of double buffer registers and manual update of synchronous channels"]
    #[inline(always)]
    pub fn is_mode0(&self) -> bool {
        *self == Updmselect::Mode0
    }
    #[doc = "Manual write of double buffer registers and automatic update of synchronous channels"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == Updmselect::Mode1
    }
    #[doc = "Automatic write of duty-cycle update registers by the DMA Controller and automatic update of synchronous channels"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == Updmselect::Mode2
    }
}
#[doc = "Field `UPDM` writer - Synchronous Channels Update Mode"]
pub type UpdmW<'a, REG> = crate::FieldWriter<'a, REG, 2, Updmselect>;
impl<'a, REG> UpdmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Manual write of double buffer registers and manual update of synchronous channels"]
    #[inline(always)]
    pub fn mode0(self) -> &'a mut crate::W<REG> {
        self.variant(Updmselect::Mode0)
    }
    #[doc = "Manual write of double buffer registers and automatic update of synchronous channels"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut crate::W<REG> {
        self.variant(Updmselect::Mode1)
    }
    #[doc = "Automatic write of duty-cycle update registers by the DMA Controller and automatic update of synchronous channels"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut crate::W<REG> {
        self.variant(Updmselect::Mode2)
    }
}
#[doc = "Field `PTRM` reader - DMA Controller Transfer Request Mode"]
pub type PtrmR = crate::BitReader;
#[doc = "Field `PTRM` writer - DMA Controller Transfer Request Mode"]
pub type PtrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTRCS` reader - DMA Controller Transfer Request Comparison Selection"]
pub type PtrcsR = crate::FieldReader;
#[doc = "Field `PTRCS` writer - DMA Controller Transfer Request Comparison Selection"]
pub type PtrcsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Synchronous Channel 0"]
    #[inline(always)]
    pub fn sync0(&self) -> Sync0R {
        Sync0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Synchronous Channel 1"]
    #[inline(always)]
    pub fn sync1(&self) -> Sync1R {
        Sync1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Synchronous Channel 2"]
    #[inline(always)]
    pub fn sync2(&self) -> Sync2R {
        Sync2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Synchronous Channel 3"]
    #[inline(always)]
    pub fn sync3(&self) -> Sync3R {
        Sync3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Synchronous Channels Update Mode"]
    #[inline(always)]
    pub fn updm(&self) -> UpdmR {
        UpdmR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 20 - DMA Controller Transfer Request Mode"]
    #[inline(always)]
    pub fn ptrm(&self) -> PtrmR {
        PtrmR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:23 - DMA Controller Transfer Request Comparison Selection"]
    #[inline(always)]
    pub fn ptrcs(&self) -> PtrcsR {
        PtrcsR::new(((self.bits >> 21) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Synchronous Channel 0"]
    #[inline(always)]
    pub fn sync0(&mut self) -> Sync0W<ScmSpec> {
        Sync0W::new(self, 0)
    }
    #[doc = "Bit 1 - Synchronous Channel 1"]
    #[inline(always)]
    pub fn sync1(&mut self) -> Sync1W<ScmSpec> {
        Sync1W::new(self, 1)
    }
    #[doc = "Bit 2 - Synchronous Channel 2"]
    #[inline(always)]
    pub fn sync2(&mut self) -> Sync2W<ScmSpec> {
        Sync2W::new(self, 2)
    }
    #[doc = "Bit 3 - Synchronous Channel 3"]
    #[inline(always)]
    pub fn sync3(&mut self) -> Sync3W<ScmSpec> {
        Sync3W::new(self, 3)
    }
    #[doc = "Bits 16:17 - Synchronous Channels Update Mode"]
    #[inline(always)]
    pub fn updm(&mut self) -> UpdmW<ScmSpec> {
        UpdmW::new(self, 16)
    }
    #[doc = "Bit 20 - DMA Controller Transfer Request Mode"]
    #[inline(always)]
    pub fn ptrm(&mut self) -> PtrmW<ScmSpec> {
        PtrmW::new(self, 20)
    }
    #[doc = "Bits 21:23 - DMA Controller Transfer Request Comparison Selection"]
    #[inline(always)]
    pub fn ptrcs(&mut self) -> PtrcsW<ScmSpec> {
        PtrcsW::new(self, 21)
    }
}
#[doc = "PWM Sync Channels Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScmSpec;
impl crate::RegisterSpec for ScmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scm::R`](R) reader structure"]
impl crate::Readable for ScmSpec {}
#[doc = "`write(|w| ..)` method takes [`scm::W`](W) writer structure"]
impl crate::Writable for ScmSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCM to value 0"]
impl crate::Resettable for ScmSpec {}
