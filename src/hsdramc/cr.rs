#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Number of Column Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ncselect {
    #[doc = "0: 8 bits to define the column number, up to 256 columns"]
    Col8 = 0,
    #[doc = "1: 9 bits to define the column number, up to 512 columns"]
    Col9 = 1,
    #[doc = "2: 10 bits to define the column number, up to 1024 columns"]
    Col10 = 2,
    #[doc = "3: 11 bits to define the column number, up to 2048 columns"]
    Col11 = 3,
    #[doc = "4: 12 bits to define the column number, up to 4096 columns"]
    Col12 = 4,
}
impl From<Ncselect> for u8 {
    #[inline(always)]
    fn from(variant: Ncselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ncselect {
    type Ux = u8;
}
impl crate::IsEnum for Ncselect {}
#[doc = "Field `NC` reader - Number of Column Bits"]
pub type NcR = crate::FieldReader<Ncselect>;
impl NcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ncselect> {
        match self.bits {
            0 => Some(Ncselect::Col8),
            1 => Some(Ncselect::Col9),
            2 => Some(Ncselect::Col10),
            3 => Some(Ncselect::Col11),
            4 => Some(Ncselect::Col12),
            _ => None,
        }
    }
    #[doc = "8 bits to define the column number, up to 256 columns"]
    #[inline(always)]
    pub fn is_col8(&self) -> bool {
        *self == Ncselect::Col8
    }
    #[doc = "9 bits to define the column number, up to 512 columns"]
    #[inline(always)]
    pub fn is_col9(&self) -> bool {
        *self == Ncselect::Col9
    }
    #[doc = "10 bits to define the column number, up to 1024 columns"]
    #[inline(always)]
    pub fn is_col10(&self) -> bool {
        *self == Ncselect::Col10
    }
    #[doc = "11 bits to define the column number, up to 2048 columns"]
    #[inline(always)]
    pub fn is_col11(&self) -> bool {
        *self == Ncselect::Col11
    }
    #[doc = "12 bits to define the column number, up to 4096 columns"]
    #[inline(always)]
    pub fn is_col12(&self) -> bool {
        *self == Ncselect::Col12
    }
}
#[doc = "Field `NC` writer - Number of Column Bits"]
pub type NcW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ncselect>;
impl<'a, REG> NcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 bits to define the column number, up to 256 columns"]
    #[inline(always)]
    pub fn col8(self) -> &'a mut crate::W<REG> {
        self.variant(Ncselect::Col8)
    }
    #[doc = "9 bits to define the column number, up to 512 columns"]
    #[inline(always)]
    pub fn col9(self) -> &'a mut crate::W<REG> {
        self.variant(Ncselect::Col9)
    }
    #[doc = "10 bits to define the column number, up to 1024 columns"]
    #[inline(always)]
    pub fn col10(self) -> &'a mut crate::W<REG> {
        self.variant(Ncselect::Col10)
    }
    #[doc = "11 bits to define the column number, up to 2048 columns"]
    #[inline(always)]
    pub fn col11(self) -> &'a mut crate::W<REG> {
        self.variant(Ncselect::Col11)
    }
    #[doc = "12 bits to define the column number, up to 4096 columns"]
    #[inline(always)]
    pub fn col12(self) -> &'a mut crate::W<REG> {
        self.variant(Ncselect::Col12)
    }
}
#[doc = "Number of Row Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nrselect {
    #[doc = "0: 11 bits to define the row number, up to 2048 rows"]
    Row11 = 0,
    #[doc = "1: 12 bits to define the row number, up to 4096 rows"]
    Row12 = 1,
    #[doc = "2: 13 bits to define the row number, up to 8192 rows"]
    Row13 = 2,
}
impl From<Nrselect> for u8 {
    #[inline(always)]
    fn from(variant: Nrselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nrselect {
    type Ux = u8;
}
impl crate::IsEnum for Nrselect {}
#[doc = "Field `NR` reader - Number of Row Bits"]
pub type NrR = crate::FieldReader<Nrselect>;
impl NrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Nrselect> {
        match self.bits {
            0 => Some(Nrselect::Row11),
            1 => Some(Nrselect::Row12),
            2 => Some(Nrselect::Row13),
            _ => None,
        }
    }
    #[doc = "11 bits to define the row number, up to 2048 rows"]
    #[inline(always)]
    pub fn is_row11(&self) -> bool {
        *self == Nrselect::Row11
    }
    #[doc = "12 bits to define the row number, up to 4096 rows"]
    #[inline(always)]
    pub fn is_row12(&self) -> bool {
        *self == Nrselect::Row12
    }
    #[doc = "13 bits to define the row number, up to 8192 rows"]
    #[inline(always)]
    pub fn is_row13(&self) -> bool {
        *self == Nrselect::Row13
    }
}
#[doc = "Field `NR` writer - Number of Row Bits"]
pub type NrW<'a, REG> = crate::FieldWriter<'a, REG, 2, Nrselect>;
impl<'a, REG> NrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "11 bits to define the row number, up to 2048 rows"]
    #[inline(always)]
    pub fn row11(self) -> &'a mut crate::W<REG> {
        self.variant(Nrselect::Row11)
    }
    #[doc = "12 bits to define the row number, up to 4096 rows"]
    #[inline(always)]
    pub fn row12(self) -> &'a mut crate::W<REG> {
        self.variant(Nrselect::Row12)
    }
    #[doc = "13 bits to define the row number, up to 8192 rows"]
    #[inline(always)]
    pub fn row13(self) -> &'a mut crate::W<REG> {
        self.variant(Nrselect::Row13)
    }
}
#[doc = "Number of Banks\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nbselect {
    #[doc = "0: 2 banks"]
    Bank2 = 0,
    #[doc = "1: 4 banks"]
    Bank4 = 1,
}
impl From<Nbselect> for bool {
    #[inline(always)]
    fn from(variant: Nbselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NB` reader - Number of Banks"]
pub type NbR = crate::BitReader<Nbselect>;
impl NbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nbselect {
        match self.bits {
            false => Nbselect::Bank2,
            true => Nbselect::Bank4,
        }
    }
    #[doc = "2 banks"]
    #[inline(always)]
    pub fn is_bank2(&self) -> bool {
        *self == Nbselect::Bank2
    }
    #[doc = "4 banks"]
    #[inline(always)]
    pub fn is_bank4(&self) -> bool {
        *self == Nbselect::Bank4
    }
}
#[doc = "Field `NB` writer - Number of Banks"]
pub type NbW<'a, REG> = crate::BitWriter<'a, REG, Nbselect>;
impl<'a, REG> NbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "2 banks"]
    #[inline(always)]
    pub fn bank2(self) -> &'a mut crate::W<REG> {
        self.variant(Nbselect::Bank2)
    }
    #[doc = "4 banks"]
    #[inline(always)]
    pub fn bank4(self) -> &'a mut crate::W<REG> {
        self.variant(Nbselect::Bank4)
    }
}
#[doc = "CAS Latency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Casselect {
    #[doc = "1: 1 cycle latency"]
    Latency1 = 1,
    #[doc = "2: 2 cycle latency"]
    Latency2 = 2,
    #[doc = "3: 3 cycle latency"]
    Latency3 = 3,
}
impl From<Casselect> for u8 {
    #[inline(always)]
    fn from(variant: Casselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Casselect {
    type Ux = u8;
}
impl crate::IsEnum for Casselect {}
#[doc = "Field `CAS` reader - CAS Latency"]
pub type CasR = crate::FieldReader<Casselect>;
impl CasR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Casselect> {
        match self.bits {
            1 => Some(Casselect::Latency1),
            2 => Some(Casselect::Latency2),
            3 => Some(Casselect::Latency3),
            _ => None,
        }
    }
    #[doc = "1 cycle latency"]
    #[inline(always)]
    pub fn is_latency1(&self) -> bool {
        *self == Casselect::Latency1
    }
    #[doc = "2 cycle latency"]
    #[inline(always)]
    pub fn is_latency2(&self) -> bool {
        *self == Casselect::Latency2
    }
    #[doc = "3 cycle latency"]
    #[inline(always)]
    pub fn is_latency3(&self) -> bool {
        *self == Casselect::Latency3
    }
}
#[doc = "Field `CAS` writer - CAS Latency"]
pub type CasW<'a, REG> = crate::FieldWriter<'a, REG, 2, Casselect>;
impl<'a, REG> CasW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 cycle latency"]
    #[inline(always)]
    pub fn latency1(self) -> &'a mut crate::W<REG> {
        self.variant(Casselect::Latency1)
    }
    #[doc = "2 cycle latency"]
    #[inline(always)]
    pub fn latency2(self) -> &'a mut crate::W<REG> {
        self.variant(Casselect::Latency2)
    }
    #[doc = "3 cycle latency"]
    #[inline(always)]
    pub fn latency3(self) -> &'a mut crate::W<REG> {
        self.variant(Casselect::Latency3)
    }
}
#[doc = "Field `DBW` reader - Data Bus Width"]
pub type DbwR = crate::BitReader;
#[doc = "Field `DBW` writer - Data Bus Width"]
pub type DbwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RMW` reader - Read Modify Write"]
pub type RmwR = crate::BitReader;
#[doc = "Field `RMW` writer - Read Modify Write"]
pub type RmwW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Number of Column Bits"]
    #[inline(always)]
    pub fn nc(&self) -> NcR {
        NcR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - Number of Row Bits"]
    #[inline(always)]
    pub fn nr(&self) -> NrR {
        NrR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Number of Banks"]
    #[inline(always)]
    pub fn nb(&self) -> NbR {
        NbR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - CAS Latency"]
    #[inline(always)]
    pub fn cas(&self) -> CasR {
        CasR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Data Bus Width"]
    #[inline(always)]
    pub fn dbw(&self) -> DbwR {
        DbwR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Read Modify Write"]
    #[inline(always)]
    pub fn rmw(&self) -> RmwR {
        RmwR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Number of Column Bits"]
    #[inline(always)]
    pub fn nc(&mut self) -> NcW<CrSpec> {
        NcW::new(self, 0)
    }
    #[doc = "Bits 3:4 - Number of Row Bits"]
    #[inline(always)]
    pub fn nr(&mut self) -> NrW<CrSpec> {
        NrW::new(self, 3)
    }
    #[doc = "Bit 5 - Number of Banks"]
    #[inline(always)]
    pub fn nb(&mut self) -> NbW<CrSpec> {
        NbW::new(self, 5)
    }
    #[doc = "Bits 6:7 - CAS Latency"]
    #[inline(always)]
    pub fn cas(&mut self) -> CasW<CrSpec> {
        CasW::new(self, 6)
    }
    #[doc = "Bit 8 - Data Bus Width"]
    #[inline(always)]
    pub fn dbw(&mut self) -> DbwW<CrSpec> {
        DbwW::new(self, 8)
    }
    #[doc = "Bit 9 - Read Modify Write"]
    #[inline(always)]
    pub fn rmw(&mut self) -> RmwW<CrSpec> {
        RmwW::new(self, 9)
    }
}
#[doc = "Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
