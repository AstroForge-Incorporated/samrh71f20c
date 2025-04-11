#[doc = "Register `MSKR` reader"]
pub type R = crate::R<MskrSpec>;
#[doc = "Register `MSKR` writer"]
pub type W = crate::W<MskrSpec>;
#[doc = "PIO Line 0 Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msk0select {
    #[doc = "0: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    Disabled = 0,
    #[doc = "1: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    Enabled = 1,
}
impl From<Msk0select> for bool {
    #[inline(always)]
    fn from(variant: Msk0select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSK0` reader - PIO Line 0 Mask"]
pub type Msk0R = crate::BitReader<Msk0select>;
impl Msk0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msk0select {
        match self.bits {
            false => Msk0select::Disabled,
            true => Msk0select::Enabled,
        }
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Msk0select::Disabled
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Msk0select::Enabled
    }
}
#[doc = "Field `MSK0` writer - PIO Line 0 Mask"]
pub type Msk0W<'a, REG> = crate::BitWriter<'a, REG, Msk0select>;
impl<'a, REG> Msk0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk0select::Disabled)
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk0select::Enabled)
    }
}
#[doc = "PIO Line 1 Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msk1select {
    #[doc = "0: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    Disabled = 0,
    #[doc = "1: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    Enabled = 1,
}
impl From<Msk1select> for bool {
    #[inline(always)]
    fn from(variant: Msk1select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSK1` reader - PIO Line 1 Mask"]
pub type Msk1R = crate::BitReader<Msk1select>;
impl Msk1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msk1select {
        match self.bits {
            false => Msk1select::Disabled,
            true => Msk1select::Enabled,
        }
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Msk1select::Disabled
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Msk1select::Enabled
    }
}
#[doc = "Field `MSK1` writer - PIO Line 1 Mask"]
pub type Msk1W<'a, REG> = crate::BitWriter<'a, REG, Msk1select>;
impl<'a, REG> Msk1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk1select::Disabled)
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk1select::Enabled)
    }
}
#[doc = "PIO Line 2 Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msk2select {
    #[doc = "0: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    Disabled = 0,
    #[doc = "1: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    Enabled = 1,
}
impl From<Msk2select> for bool {
    #[inline(always)]
    fn from(variant: Msk2select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSK2` reader - PIO Line 2 Mask"]
pub type Msk2R = crate::BitReader<Msk2select>;
impl Msk2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msk2select {
        match self.bits {
            false => Msk2select::Disabled,
            true => Msk2select::Enabled,
        }
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Msk2select::Disabled
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Msk2select::Enabled
    }
}
#[doc = "Field `MSK2` writer - PIO Line 2 Mask"]
pub type Msk2W<'a, REG> = crate::BitWriter<'a, REG, Msk2select>;
impl<'a, REG> Msk2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk2select::Disabled)
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk2select::Enabled)
    }
}
#[doc = "PIO Line 3 Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msk3select {
    #[doc = "0: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    Disabled = 0,
    #[doc = "1: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    Enabled = 1,
}
impl From<Msk3select> for bool {
    #[inline(always)]
    fn from(variant: Msk3select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSK3` reader - PIO Line 3 Mask"]
pub type Msk3R = crate::BitReader<Msk3select>;
impl Msk3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msk3select {
        match self.bits {
            false => Msk3select::Disabled,
            true => Msk3select::Enabled,
        }
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Msk3select::Disabled
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Msk3select::Enabled
    }
}
#[doc = "Field `MSK3` writer - PIO Line 3 Mask"]
pub type Msk3W<'a, REG> = crate::BitWriter<'a, REG, Msk3select>;
impl<'a, REG> Msk3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk3select::Disabled)
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk3select::Enabled)
    }
}
#[doc = "PIO Line 4 Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msk4select {
    #[doc = "0: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    Disabled = 0,
    #[doc = "1: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    Enabled = 1,
}
impl From<Msk4select> for bool {
    #[inline(always)]
    fn from(variant: Msk4select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSK4` reader - PIO Line 4 Mask"]
pub type Msk4R = crate::BitReader<Msk4select>;
impl Msk4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msk4select {
        match self.bits {
            false => Msk4select::Disabled,
            true => Msk4select::Enabled,
        }
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Msk4select::Disabled
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Msk4select::Enabled
    }
}
#[doc = "Field `MSK4` writer - PIO Line 4 Mask"]
pub type Msk4W<'a, REG> = crate::BitWriter<'a, REG, Msk4select>;
impl<'a, REG> Msk4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk4select::Disabled)
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk4select::Enabled)
    }
}
#[doc = "PIO Line 5 Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msk5select {
    #[doc = "0: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    Disabled = 0,
    #[doc = "1: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    Enabled = 1,
}
impl From<Msk5select> for bool {
    #[inline(always)]
    fn from(variant: Msk5select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSK5` reader - PIO Line 5 Mask"]
pub type Msk5R = crate::BitReader<Msk5select>;
impl Msk5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msk5select {
        match self.bits {
            false => Msk5select::Disabled,
            true => Msk5select::Enabled,
        }
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Msk5select::Disabled
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Msk5select::Enabled
    }
}
#[doc = "Field `MSK5` writer - PIO Line 5 Mask"]
pub type Msk5W<'a, REG> = crate::BitWriter<'a, REG, Msk5select>;
impl<'a, REG> Msk5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk5select::Disabled)
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk5select::Enabled)
    }
}
#[doc = "PIO Line 6 Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msk6select {
    #[doc = "0: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    Disabled = 0,
    #[doc = "1: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    Enabled = 1,
}
impl From<Msk6select> for bool {
    #[inline(always)]
    fn from(variant: Msk6select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSK6` reader - PIO Line 6 Mask"]
pub type Msk6R = crate::BitReader<Msk6select>;
impl Msk6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msk6select {
        match self.bits {
            false => Msk6select::Disabled,
            true => Msk6select::Enabled,
        }
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Msk6select::Disabled
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Msk6select::Enabled
    }
}
#[doc = "Field `MSK6` writer - PIO Line 6 Mask"]
pub type Msk6W<'a, REG> = crate::BitWriter<'a, REG, Msk6select>;
impl<'a, REG> Msk6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk6select::Disabled)
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk6select::Enabled)
    }
}
#[doc = "PIO Line 7 Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msk7select {
    #[doc = "0: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    Disabled = 0,
    #[doc = "1: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    Enabled = 1,
}
impl From<Msk7select> for bool {
    #[inline(always)]
    fn from(variant: Msk7select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSK7` reader - PIO Line 7 Mask"]
pub type Msk7R = crate::BitReader<Msk7select>;
impl Msk7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msk7select {
        match self.bits {
            false => Msk7select::Disabled,
            true => Msk7select::Enabled,
        }
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Msk7select::Disabled
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Msk7select::Enabled
    }
}
#[doc = "Field `MSK7` writer - PIO Line 7 Mask"]
pub type Msk7W<'a, REG> = crate::BitWriter<'a, REG, Msk7select>;
impl<'a, REG> Msk7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk7select::Disabled)
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk7select::Enabled)
    }
}
#[doc = "PIO Line 8 Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msk8select {
    #[doc = "0: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    Disabled = 0,
    #[doc = "1: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    Enabled = 1,
}
impl From<Msk8select> for bool {
    #[inline(always)]
    fn from(variant: Msk8select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSK8` reader - PIO Line 8 Mask"]
pub type Msk8R = crate::BitReader<Msk8select>;
impl Msk8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msk8select {
        match self.bits {
            false => Msk8select::Disabled,
            true => Msk8select::Enabled,
        }
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Msk8select::Disabled
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Msk8select::Enabled
    }
}
#[doc = "Field `MSK8` writer - PIO Line 8 Mask"]
pub type Msk8W<'a, REG> = crate::BitWriter<'a, REG, Msk8select>;
impl<'a, REG> Msk8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk8select::Disabled)
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk8select::Enabled)
    }
}
#[doc = "PIO Line 9 Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msk9select {
    #[doc = "0: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    Disabled = 0,
    #[doc = "1: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    Enabled = 1,
}
impl From<Msk9select> for bool {
    #[inline(always)]
    fn from(variant: Msk9select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSK9` reader - PIO Line 9 Mask"]
pub type Msk9R = crate::BitReader<Msk9select>;
impl Msk9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msk9select {
        match self.bits {
            false => Msk9select::Disabled,
            true => Msk9select::Enabled,
        }
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Msk9select::Disabled
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Msk9select::Enabled
    }
}
#[doc = "Field `MSK9` writer - PIO Line 9 Mask"]
pub type Msk9W<'a, REG> = crate::BitWriter<'a, REG, Msk9select>;
impl<'a, REG> Msk9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk9select::Disabled)
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk9select::Enabled)
    }
}
#[doc = "PIO Line 10 Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msk10select {
    #[doc = "0: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    Disabled = 0,
    #[doc = "1: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    Enabled = 1,
}
impl From<Msk10select> for bool {
    #[inline(always)]
    fn from(variant: Msk10select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSK10` reader - PIO Line 10 Mask"]
pub type Msk10R = crate::BitReader<Msk10select>;
impl Msk10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msk10select {
        match self.bits {
            false => Msk10select::Disabled,
            true => Msk10select::Enabled,
        }
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Msk10select::Disabled
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Msk10select::Enabled
    }
}
#[doc = "Field `MSK10` writer - PIO Line 10 Mask"]
pub type Msk10W<'a, REG> = crate::BitWriter<'a, REG, Msk10select>;
impl<'a, REG> Msk10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk10select::Disabled)
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk10select::Enabled)
    }
}
#[doc = "PIO Line 11 Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msk11select {
    #[doc = "0: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    Disabled = 0,
    #[doc = "1: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    Enabled = 1,
}
impl From<Msk11select> for bool {
    #[inline(always)]
    fn from(variant: Msk11select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSK11` reader - PIO Line 11 Mask"]
pub type Msk11R = crate::BitReader<Msk11select>;
impl Msk11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msk11select {
        match self.bits {
            false => Msk11select::Disabled,
            true => Msk11select::Enabled,
        }
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Msk11select::Disabled
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Msk11select::Enabled
    }
}
#[doc = "Field `MSK11` writer - PIO Line 11 Mask"]
pub type Msk11W<'a, REG> = crate::BitWriter<'a, REG, Msk11select>;
impl<'a, REG> Msk11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk11select::Disabled)
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk11select::Enabled)
    }
}
#[doc = "PIO Line 12 Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msk12select {
    #[doc = "0: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    Disabled = 0,
    #[doc = "1: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    Enabled = 1,
}
impl From<Msk12select> for bool {
    #[inline(always)]
    fn from(variant: Msk12select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSK12` reader - PIO Line 12 Mask"]
pub type Msk12R = crate::BitReader<Msk12select>;
impl Msk12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msk12select {
        match self.bits {
            false => Msk12select::Disabled,
            true => Msk12select::Enabled,
        }
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Msk12select::Disabled
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Msk12select::Enabled
    }
}
#[doc = "Field `MSK12` writer - PIO Line 12 Mask"]
pub type Msk12W<'a, REG> = crate::BitWriter<'a, REG, Msk12select>;
impl<'a, REG> Msk12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk12select::Disabled)
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk12select::Enabled)
    }
}
#[doc = "PIO Line 13 Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msk13select {
    #[doc = "0: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    Disabled = 0,
    #[doc = "1: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    Enabled = 1,
}
impl From<Msk13select> for bool {
    #[inline(always)]
    fn from(variant: Msk13select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSK13` reader - PIO Line 13 Mask"]
pub type Msk13R = crate::BitReader<Msk13select>;
impl Msk13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msk13select {
        match self.bits {
            false => Msk13select::Disabled,
            true => Msk13select::Enabled,
        }
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Msk13select::Disabled
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Msk13select::Enabled
    }
}
#[doc = "Field `MSK13` writer - PIO Line 13 Mask"]
pub type Msk13W<'a, REG> = crate::BitWriter<'a, REG, Msk13select>;
impl<'a, REG> Msk13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk13select::Disabled)
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk13select::Enabled)
    }
}
#[doc = "PIO Line 14 Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msk14select {
    #[doc = "0: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    Disabled = 0,
    #[doc = "1: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    Enabled = 1,
}
impl From<Msk14select> for bool {
    #[inline(always)]
    fn from(variant: Msk14select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSK14` reader - PIO Line 14 Mask"]
pub type Msk14R = crate::BitReader<Msk14select>;
impl Msk14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msk14select {
        match self.bits {
            false => Msk14select::Disabled,
            true => Msk14select::Enabled,
        }
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Msk14select::Disabled
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Msk14select::Enabled
    }
}
#[doc = "Field `MSK14` writer - PIO Line 14 Mask"]
pub type Msk14W<'a, REG> = crate::BitWriter<'a, REG, Msk14select>;
impl<'a, REG> Msk14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk14select::Disabled)
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk14select::Enabled)
    }
}
#[doc = "PIO Line 15 Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msk15select {
    #[doc = "0: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    Disabled = 0,
    #[doc = "1: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    Enabled = 1,
}
impl From<Msk15select> for bool {
    #[inline(always)]
    fn from(variant: Msk15select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSK15` reader - PIO Line 15 Mask"]
pub type Msk15R = crate::BitReader<Msk15select>;
impl Msk15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msk15select {
        match self.bits {
            false => Msk15select::Disabled,
            true => Msk15select::Enabled,
        }
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Msk15select::Disabled
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Msk15select::Enabled
    }
}
#[doc = "Field `MSK15` writer - PIO Line 15 Mask"]
pub type Msk15W<'a, REG> = crate::BitWriter<'a, REG, Msk15select>;
impl<'a, REG> Msk15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk15select::Disabled)
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk15select::Enabled)
    }
}
#[doc = "PIO Line 16 Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msk16select {
    #[doc = "0: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    Disabled = 0,
    #[doc = "1: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    Enabled = 1,
}
impl From<Msk16select> for bool {
    #[inline(always)]
    fn from(variant: Msk16select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSK16` reader - PIO Line 16 Mask"]
pub type Msk16R = crate::BitReader<Msk16select>;
impl Msk16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msk16select {
        match self.bits {
            false => Msk16select::Disabled,
            true => Msk16select::Enabled,
        }
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Msk16select::Disabled
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Msk16select::Enabled
    }
}
#[doc = "Field `MSK16` writer - PIO Line 16 Mask"]
pub type Msk16W<'a, REG> = crate::BitWriter<'a, REG, Msk16select>;
impl<'a, REG> Msk16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk16select::Disabled)
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk16select::Enabled)
    }
}
#[doc = "PIO Line 17 Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msk17select {
    #[doc = "0: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    Disabled = 0,
    #[doc = "1: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    Enabled = 1,
}
impl From<Msk17select> for bool {
    #[inline(always)]
    fn from(variant: Msk17select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSK17` reader - PIO Line 17 Mask"]
pub type Msk17R = crate::BitReader<Msk17select>;
impl Msk17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msk17select {
        match self.bits {
            false => Msk17select::Disabled,
            true => Msk17select::Enabled,
        }
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Msk17select::Disabled
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Msk17select::Enabled
    }
}
#[doc = "Field `MSK17` writer - PIO Line 17 Mask"]
pub type Msk17W<'a, REG> = crate::BitWriter<'a, REG, Msk17select>;
impl<'a, REG> Msk17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk17select::Disabled)
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk17select::Enabled)
    }
}
#[doc = "PIO Line 18 Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msk18select {
    #[doc = "0: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    Disabled = 0,
    #[doc = "1: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    Enabled = 1,
}
impl From<Msk18select> for bool {
    #[inline(always)]
    fn from(variant: Msk18select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSK18` reader - PIO Line 18 Mask"]
pub type Msk18R = crate::BitReader<Msk18select>;
impl Msk18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msk18select {
        match self.bits {
            false => Msk18select::Disabled,
            true => Msk18select::Enabled,
        }
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Msk18select::Disabled
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Msk18select::Enabled
    }
}
#[doc = "Field `MSK18` writer - PIO Line 18 Mask"]
pub type Msk18W<'a, REG> = crate::BitWriter<'a, REG, Msk18select>;
impl<'a, REG> Msk18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk18select::Disabled)
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk18select::Enabled)
    }
}
#[doc = "PIO Line 19 Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msk19select {
    #[doc = "0: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    Disabled = 0,
    #[doc = "1: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    Enabled = 1,
}
impl From<Msk19select> for bool {
    #[inline(always)]
    fn from(variant: Msk19select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSK19` reader - PIO Line 19 Mask"]
pub type Msk19R = crate::BitReader<Msk19select>;
impl Msk19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msk19select {
        match self.bits {
            false => Msk19select::Disabled,
            true => Msk19select::Enabled,
        }
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Msk19select::Disabled
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Msk19select::Enabled
    }
}
#[doc = "Field `MSK19` writer - PIO Line 19 Mask"]
pub type Msk19W<'a, REG> = crate::BitWriter<'a, REG, Msk19select>;
impl<'a, REG> Msk19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk19select::Disabled)
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk19select::Enabled)
    }
}
#[doc = "PIO Line 20 Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msk20select {
    #[doc = "0: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    Disabled = 0,
    #[doc = "1: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    Enabled = 1,
}
impl From<Msk20select> for bool {
    #[inline(always)]
    fn from(variant: Msk20select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSK20` reader - PIO Line 20 Mask"]
pub type Msk20R = crate::BitReader<Msk20select>;
impl Msk20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msk20select {
        match self.bits {
            false => Msk20select::Disabled,
            true => Msk20select::Enabled,
        }
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Msk20select::Disabled
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Msk20select::Enabled
    }
}
#[doc = "Field `MSK20` writer - PIO Line 20 Mask"]
pub type Msk20W<'a, REG> = crate::BitWriter<'a, REG, Msk20select>;
impl<'a, REG> Msk20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk20select::Disabled)
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk20select::Enabled)
    }
}
#[doc = "PIO Line 21 Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msk21select {
    #[doc = "0: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    Disabled = 0,
    #[doc = "1: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    Enabled = 1,
}
impl From<Msk21select> for bool {
    #[inline(always)]
    fn from(variant: Msk21select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSK21` reader - PIO Line 21 Mask"]
pub type Msk21R = crate::BitReader<Msk21select>;
impl Msk21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msk21select {
        match self.bits {
            false => Msk21select::Disabled,
            true => Msk21select::Enabled,
        }
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Msk21select::Disabled
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Msk21select::Enabled
    }
}
#[doc = "Field `MSK21` writer - PIO Line 21 Mask"]
pub type Msk21W<'a, REG> = crate::BitWriter<'a, REG, Msk21select>;
impl<'a, REG> Msk21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk21select::Disabled)
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk21select::Enabled)
    }
}
#[doc = "PIO Line 22 Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msk22select {
    #[doc = "0: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    Disabled = 0,
    #[doc = "1: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    Enabled = 1,
}
impl From<Msk22select> for bool {
    #[inline(always)]
    fn from(variant: Msk22select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSK22` reader - PIO Line 22 Mask"]
pub type Msk22R = crate::BitReader<Msk22select>;
impl Msk22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msk22select {
        match self.bits {
            false => Msk22select::Disabled,
            true => Msk22select::Enabled,
        }
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Msk22select::Disabled
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Msk22select::Enabled
    }
}
#[doc = "Field `MSK22` writer - PIO Line 22 Mask"]
pub type Msk22W<'a, REG> = crate::BitWriter<'a, REG, Msk22select>;
impl<'a, REG> Msk22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk22select::Disabled)
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk22select::Enabled)
    }
}
#[doc = "PIO Line 23 Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msk23select {
    #[doc = "0: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    Disabled = 0,
    #[doc = "1: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    Enabled = 1,
}
impl From<Msk23select> for bool {
    #[inline(always)]
    fn from(variant: Msk23select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSK23` reader - PIO Line 23 Mask"]
pub type Msk23R = crate::BitReader<Msk23select>;
impl Msk23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msk23select {
        match self.bits {
            false => Msk23select::Disabled,
            true => Msk23select::Enabled,
        }
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Msk23select::Disabled
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Msk23select::Enabled
    }
}
#[doc = "Field `MSK23` writer - PIO Line 23 Mask"]
pub type Msk23W<'a, REG> = crate::BitWriter<'a, REG, Msk23select>;
impl<'a, REG> Msk23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk23select::Disabled)
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk23select::Enabled)
    }
}
#[doc = "PIO Line 24 Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msk24select {
    #[doc = "0: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    Disabled = 0,
    #[doc = "1: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    Enabled = 1,
}
impl From<Msk24select> for bool {
    #[inline(always)]
    fn from(variant: Msk24select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSK24` reader - PIO Line 24 Mask"]
pub type Msk24R = crate::BitReader<Msk24select>;
impl Msk24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msk24select {
        match self.bits {
            false => Msk24select::Disabled,
            true => Msk24select::Enabled,
        }
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Msk24select::Disabled
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Msk24select::Enabled
    }
}
#[doc = "Field `MSK24` writer - PIO Line 24 Mask"]
pub type Msk24W<'a, REG> = crate::BitWriter<'a, REG, Msk24select>;
impl<'a, REG> Msk24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk24select::Disabled)
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk24select::Enabled)
    }
}
#[doc = "PIO Line 25 Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msk25select {
    #[doc = "0: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    Disabled = 0,
    #[doc = "1: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    Enabled = 1,
}
impl From<Msk25select> for bool {
    #[inline(always)]
    fn from(variant: Msk25select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSK25` reader - PIO Line 25 Mask"]
pub type Msk25R = crate::BitReader<Msk25select>;
impl Msk25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msk25select {
        match self.bits {
            false => Msk25select::Disabled,
            true => Msk25select::Enabled,
        }
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Msk25select::Disabled
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Msk25select::Enabled
    }
}
#[doc = "Field `MSK25` writer - PIO Line 25 Mask"]
pub type Msk25W<'a, REG> = crate::BitWriter<'a, REG, Msk25select>;
impl<'a, REG> Msk25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk25select::Disabled)
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk25select::Enabled)
    }
}
#[doc = "PIO Line 26 Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msk26select {
    #[doc = "0: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    Disabled = 0,
    #[doc = "1: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    Enabled = 1,
}
impl From<Msk26select> for bool {
    #[inline(always)]
    fn from(variant: Msk26select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSK26` reader - PIO Line 26 Mask"]
pub type Msk26R = crate::BitReader<Msk26select>;
impl Msk26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msk26select {
        match self.bits {
            false => Msk26select::Disabled,
            true => Msk26select::Enabled,
        }
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Msk26select::Disabled
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Msk26select::Enabled
    }
}
#[doc = "Field `MSK26` writer - PIO Line 26 Mask"]
pub type Msk26W<'a, REG> = crate::BitWriter<'a, REG, Msk26select>;
impl<'a, REG> Msk26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk26select::Disabled)
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk26select::Enabled)
    }
}
#[doc = "PIO Line 27 Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msk27select {
    #[doc = "0: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    Disabled = 0,
    #[doc = "1: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    Enabled = 1,
}
impl From<Msk27select> for bool {
    #[inline(always)]
    fn from(variant: Msk27select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSK27` reader - PIO Line 27 Mask"]
pub type Msk27R = crate::BitReader<Msk27select>;
impl Msk27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msk27select {
        match self.bits {
            false => Msk27select::Disabled,
            true => Msk27select::Enabled,
        }
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Msk27select::Disabled
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Msk27select::Enabled
    }
}
#[doc = "Field `MSK27` writer - PIO Line 27 Mask"]
pub type Msk27W<'a, REG> = crate::BitWriter<'a, REG, Msk27select>;
impl<'a, REG> Msk27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk27select::Disabled)
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk27select::Enabled)
    }
}
#[doc = "PIO Line 28 Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msk28select {
    #[doc = "0: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    Disabled = 0,
    #[doc = "1: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    Enabled = 1,
}
impl From<Msk28select> for bool {
    #[inline(always)]
    fn from(variant: Msk28select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSK28` reader - PIO Line 28 Mask"]
pub type Msk28R = crate::BitReader<Msk28select>;
impl Msk28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msk28select {
        match self.bits {
            false => Msk28select::Disabled,
            true => Msk28select::Enabled,
        }
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Msk28select::Disabled
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Msk28select::Enabled
    }
}
#[doc = "Field `MSK28` writer - PIO Line 28 Mask"]
pub type Msk28W<'a, REG> = crate::BitWriter<'a, REG, Msk28select>;
impl<'a, REG> Msk28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk28select::Disabled)
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk28select::Enabled)
    }
}
#[doc = "PIO Line 29 Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msk29select {
    #[doc = "0: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    Disabled = 0,
    #[doc = "1: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    Enabled = 1,
}
impl From<Msk29select> for bool {
    #[inline(always)]
    fn from(variant: Msk29select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSK29` reader - PIO Line 29 Mask"]
pub type Msk29R = crate::BitReader<Msk29select>;
impl Msk29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msk29select {
        match self.bits {
            false => Msk29select::Disabled,
            true => Msk29select::Enabled,
        }
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Msk29select::Disabled
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Msk29select::Enabled
    }
}
#[doc = "Field `MSK29` writer - PIO Line 29 Mask"]
pub type Msk29W<'a, REG> = crate::BitWriter<'a, REG, Msk29select>;
impl<'a, REG> Msk29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk29select::Disabled)
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk29select::Enabled)
    }
}
#[doc = "PIO Line 30 Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msk30select {
    #[doc = "0: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    Disabled = 0,
    #[doc = "1: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    Enabled = 1,
}
impl From<Msk30select> for bool {
    #[inline(always)]
    fn from(variant: Msk30select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSK30` reader - PIO Line 30 Mask"]
pub type Msk30R = crate::BitReader<Msk30select>;
impl Msk30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msk30select {
        match self.bits {
            false => Msk30select::Disabled,
            true => Msk30select::Enabled,
        }
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Msk30select::Disabled
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Msk30select::Enabled
    }
}
#[doc = "Field `MSK30` writer - PIO Line 30 Mask"]
pub type Msk30W<'a, REG> = crate::BitWriter<'a, REG, Msk30select>;
impl<'a, REG> Msk30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk30select::Disabled)
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk30select::Enabled)
    }
}
#[doc = "PIO Line 31 Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msk31select {
    #[doc = "0: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    Disabled = 0,
    #[doc = "1: Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    Enabled = 1,
}
impl From<Msk31select> for bool {
    #[inline(always)]
    fn from(variant: Msk31select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSK31` reader - PIO Line 31 Mask"]
pub type Msk31R = crate::BitReader<Msk31select>;
impl Msk31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msk31select {
        match self.bits {
            false => Msk31select::Disabled,
            true => Msk31select::Enabled,
        }
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Msk31select::Disabled
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Msk31select::Enabled
    }
}
#[doc = "Field `MSK31` writer - PIO Line 31 Mask"]
pub type Msk31W<'a, REG> = crate::BitWriter<'a, REG, Msk31select>;
impl<'a, REG> Msk31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx does not affect the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk31select::Disabled)
    }
    #[doc = "Writing the PIO_CFGRx, PIO_ODSRx or PIO_IOFRx updates the corresponding I/O line configuration."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Msk31select::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - PIO Line 0 Mask"]
    #[inline(always)]
    pub fn msk0(&self) -> Msk0R {
        Msk0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PIO Line 1 Mask"]
    #[inline(always)]
    pub fn msk1(&self) -> Msk1R {
        Msk1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PIO Line 2 Mask"]
    #[inline(always)]
    pub fn msk2(&self) -> Msk2R {
        Msk2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PIO Line 3 Mask"]
    #[inline(always)]
    pub fn msk3(&self) -> Msk3R {
        Msk3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PIO Line 4 Mask"]
    #[inline(always)]
    pub fn msk4(&self) -> Msk4R {
        Msk4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PIO Line 5 Mask"]
    #[inline(always)]
    pub fn msk5(&self) -> Msk5R {
        Msk5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PIO Line 6 Mask"]
    #[inline(always)]
    pub fn msk6(&self) -> Msk6R {
        Msk6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PIO Line 7 Mask"]
    #[inline(always)]
    pub fn msk7(&self) -> Msk7R {
        Msk7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PIO Line 8 Mask"]
    #[inline(always)]
    pub fn msk8(&self) -> Msk8R {
        Msk8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PIO Line 9 Mask"]
    #[inline(always)]
    pub fn msk9(&self) -> Msk9R {
        Msk9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PIO Line 10 Mask"]
    #[inline(always)]
    pub fn msk10(&self) -> Msk10R {
        Msk10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PIO Line 11 Mask"]
    #[inline(always)]
    pub fn msk11(&self) -> Msk11R {
        Msk11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PIO Line 12 Mask"]
    #[inline(always)]
    pub fn msk12(&self) -> Msk12R {
        Msk12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PIO Line 13 Mask"]
    #[inline(always)]
    pub fn msk13(&self) -> Msk13R {
        Msk13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PIO Line 14 Mask"]
    #[inline(always)]
    pub fn msk14(&self) -> Msk14R {
        Msk14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PIO Line 15 Mask"]
    #[inline(always)]
    pub fn msk15(&self) -> Msk15R {
        Msk15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - PIO Line 16 Mask"]
    #[inline(always)]
    pub fn msk16(&self) -> Msk16R {
        Msk16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PIO Line 17 Mask"]
    #[inline(always)]
    pub fn msk17(&self) -> Msk17R {
        Msk17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PIO Line 18 Mask"]
    #[inline(always)]
    pub fn msk18(&self) -> Msk18R {
        Msk18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PIO Line 19 Mask"]
    #[inline(always)]
    pub fn msk19(&self) -> Msk19R {
        Msk19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PIO Line 20 Mask"]
    #[inline(always)]
    pub fn msk20(&self) -> Msk20R {
        Msk20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PIO Line 21 Mask"]
    #[inline(always)]
    pub fn msk21(&self) -> Msk21R {
        Msk21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - PIO Line 22 Mask"]
    #[inline(always)]
    pub fn msk22(&self) -> Msk22R {
        Msk22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PIO Line 23 Mask"]
    #[inline(always)]
    pub fn msk23(&self) -> Msk23R {
        Msk23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - PIO Line 24 Mask"]
    #[inline(always)]
    pub fn msk24(&self) -> Msk24R {
        Msk24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PIO Line 25 Mask"]
    #[inline(always)]
    pub fn msk25(&self) -> Msk25R {
        Msk25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PIO Line 26 Mask"]
    #[inline(always)]
    pub fn msk26(&self) -> Msk26R {
        Msk26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PIO Line 27 Mask"]
    #[inline(always)]
    pub fn msk27(&self) -> Msk27R {
        Msk27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PIO Line 28 Mask"]
    #[inline(always)]
    pub fn msk28(&self) -> Msk28R {
        Msk28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - PIO Line 29 Mask"]
    #[inline(always)]
    pub fn msk29(&self) -> Msk29R {
        Msk29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - PIO Line 30 Mask"]
    #[inline(always)]
    pub fn msk30(&self) -> Msk30R {
        Msk30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - PIO Line 31 Mask"]
    #[inline(always)]
    pub fn msk31(&self) -> Msk31R {
        Msk31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PIO Line 0 Mask"]
    #[inline(always)]
    pub fn msk0(&mut self) -> Msk0W<MskrSpec> {
        Msk0W::new(self, 0)
    }
    #[doc = "Bit 1 - PIO Line 1 Mask"]
    #[inline(always)]
    pub fn msk1(&mut self) -> Msk1W<MskrSpec> {
        Msk1W::new(self, 1)
    }
    #[doc = "Bit 2 - PIO Line 2 Mask"]
    #[inline(always)]
    pub fn msk2(&mut self) -> Msk2W<MskrSpec> {
        Msk2W::new(self, 2)
    }
    #[doc = "Bit 3 - PIO Line 3 Mask"]
    #[inline(always)]
    pub fn msk3(&mut self) -> Msk3W<MskrSpec> {
        Msk3W::new(self, 3)
    }
    #[doc = "Bit 4 - PIO Line 4 Mask"]
    #[inline(always)]
    pub fn msk4(&mut self) -> Msk4W<MskrSpec> {
        Msk4W::new(self, 4)
    }
    #[doc = "Bit 5 - PIO Line 5 Mask"]
    #[inline(always)]
    pub fn msk5(&mut self) -> Msk5W<MskrSpec> {
        Msk5W::new(self, 5)
    }
    #[doc = "Bit 6 - PIO Line 6 Mask"]
    #[inline(always)]
    pub fn msk6(&mut self) -> Msk6W<MskrSpec> {
        Msk6W::new(self, 6)
    }
    #[doc = "Bit 7 - PIO Line 7 Mask"]
    #[inline(always)]
    pub fn msk7(&mut self) -> Msk7W<MskrSpec> {
        Msk7W::new(self, 7)
    }
    #[doc = "Bit 8 - PIO Line 8 Mask"]
    #[inline(always)]
    pub fn msk8(&mut self) -> Msk8W<MskrSpec> {
        Msk8W::new(self, 8)
    }
    #[doc = "Bit 9 - PIO Line 9 Mask"]
    #[inline(always)]
    pub fn msk9(&mut self) -> Msk9W<MskrSpec> {
        Msk9W::new(self, 9)
    }
    #[doc = "Bit 10 - PIO Line 10 Mask"]
    #[inline(always)]
    pub fn msk10(&mut self) -> Msk10W<MskrSpec> {
        Msk10W::new(self, 10)
    }
    #[doc = "Bit 11 - PIO Line 11 Mask"]
    #[inline(always)]
    pub fn msk11(&mut self) -> Msk11W<MskrSpec> {
        Msk11W::new(self, 11)
    }
    #[doc = "Bit 12 - PIO Line 12 Mask"]
    #[inline(always)]
    pub fn msk12(&mut self) -> Msk12W<MskrSpec> {
        Msk12W::new(self, 12)
    }
    #[doc = "Bit 13 - PIO Line 13 Mask"]
    #[inline(always)]
    pub fn msk13(&mut self) -> Msk13W<MskrSpec> {
        Msk13W::new(self, 13)
    }
    #[doc = "Bit 14 - PIO Line 14 Mask"]
    #[inline(always)]
    pub fn msk14(&mut self) -> Msk14W<MskrSpec> {
        Msk14W::new(self, 14)
    }
    #[doc = "Bit 15 - PIO Line 15 Mask"]
    #[inline(always)]
    pub fn msk15(&mut self) -> Msk15W<MskrSpec> {
        Msk15W::new(self, 15)
    }
    #[doc = "Bit 16 - PIO Line 16 Mask"]
    #[inline(always)]
    pub fn msk16(&mut self) -> Msk16W<MskrSpec> {
        Msk16W::new(self, 16)
    }
    #[doc = "Bit 17 - PIO Line 17 Mask"]
    #[inline(always)]
    pub fn msk17(&mut self) -> Msk17W<MskrSpec> {
        Msk17W::new(self, 17)
    }
    #[doc = "Bit 18 - PIO Line 18 Mask"]
    #[inline(always)]
    pub fn msk18(&mut self) -> Msk18W<MskrSpec> {
        Msk18W::new(self, 18)
    }
    #[doc = "Bit 19 - PIO Line 19 Mask"]
    #[inline(always)]
    pub fn msk19(&mut self) -> Msk19W<MskrSpec> {
        Msk19W::new(self, 19)
    }
    #[doc = "Bit 20 - PIO Line 20 Mask"]
    #[inline(always)]
    pub fn msk20(&mut self) -> Msk20W<MskrSpec> {
        Msk20W::new(self, 20)
    }
    #[doc = "Bit 21 - PIO Line 21 Mask"]
    #[inline(always)]
    pub fn msk21(&mut self) -> Msk21W<MskrSpec> {
        Msk21W::new(self, 21)
    }
    #[doc = "Bit 22 - PIO Line 22 Mask"]
    #[inline(always)]
    pub fn msk22(&mut self) -> Msk22W<MskrSpec> {
        Msk22W::new(self, 22)
    }
    #[doc = "Bit 23 - PIO Line 23 Mask"]
    #[inline(always)]
    pub fn msk23(&mut self) -> Msk23W<MskrSpec> {
        Msk23W::new(self, 23)
    }
    #[doc = "Bit 24 - PIO Line 24 Mask"]
    #[inline(always)]
    pub fn msk24(&mut self) -> Msk24W<MskrSpec> {
        Msk24W::new(self, 24)
    }
    #[doc = "Bit 25 - PIO Line 25 Mask"]
    #[inline(always)]
    pub fn msk25(&mut self) -> Msk25W<MskrSpec> {
        Msk25W::new(self, 25)
    }
    #[doc = "Bit 26 - PIO Line 26 Mask"]
    #[inline(always)]
    pub fn msk26(&mut self) -> Msk26W<MskrSpec> {
        Msk26W::new(self, 26)
    }
    #[doc = "Bit 27 - PIO Line 27 Mask"]
    #[inline(always)]
    pub fn msk27(&mut self) -> Msk27W<MskrSpec> {
        Msk27W::new(self, 27)
    }
    #[doc = "Bit 28 - PIO Line 28 Mask"]
    #[inline(always)]
    pub fn msk28(&mut self) -> Msk28W<MskrSpec> {
        Msk28W::new(self, 28)
    }
    #[doc = "Bit 29 - PIO Line 29 Mask"]
    #[inline(always)]
    pub fn msk29(&mut self) -> Msk29W<MskrSpec> {
        Msk29W::new(self, 29)
    }
    #[doc = "Bit 30 - PIO Line 30 Mask"]
    #[inline(always)]
    pub fn msk30(&mut self) -> Msk30W<MskrSpec> {
        Msk30W::new(self, 30)
    }
    #[doc = "Bit 31 - PIO Line 31 Mask"]
    #[inline(always)]
    pub fn msk31(&mut self) -> Msk31W<MskrSpec> {
        Msk31W::new(self, 31)
    }
}
#[doc = "PIO Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mskr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mskr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MskrSpec;
impl crate::RegisterSpec for MskrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mskr::R`](R) reader structure"]
impl crate::Readable for MskrSpec {}
#[doc = "`write(|w| ..)` method takes [`mskr::W`](W) writer structure"]
impl crate::Writable for MskrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MSKR to value 0"]
impl crate::Resettable for MskrSpec {}
