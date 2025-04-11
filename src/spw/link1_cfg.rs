#[doc = "Register `LINK1_CFG` reader"]
pub type R = crate::R<Link1CfgSpec>;
#[doc = "Register `LINK1_CFG` writer"]
pub type W = crate::W<Link1CfgSpec>;
#[doc = "Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Commandselect {
    #[doc = "0: The link proceeds directly to the ErrorReset state when reaching the Run state."]
    LinkDisable = 0,
    #[doc = "1: State is not actively changed."]
    NoCommand = 1,
    #[doc = "2: The Codec will wait in state Ready until the first NULL character is received."]
    AutoStart = 2,
    #[doc = "3: SpaceWire link can proceed to Started state."]
    LinkStart = 3,
}
impl From<Commandselect> for u8 {
    #[inline(always)]
    fn from(variant: Commandselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Commandselect {
    type Ux = u8;
}
impl crate::IsEnum for Commandselect {}
#[doc = "Field `COMMAND` reader - Command"]
pub type CommandR = crate::FieldReader<Commandselect>;
impl CommandR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Commandselect {
        match self.bits {
            0 => Commandselect::LinkDisable,
            1 => Commandselect::NoCommand,
            2 => Commandselect::AutoStart,
            3 => Commandselect::LinkStart,
            _ => unreachable!(),
        }
    }
    #[doc = "The link proceeds directly to the ErrorReset state when reaching the Run state."]
    #[inline(always)]
    pub fn is_link_disable(&self) -> bool {
        *self == Commandselect::LinkDisable
    }
    #[doc = "State is not actively changed."]
    #[inline(always)]
    pub fn is_no_command(&self) -> bool {
        *self == Commandselect::NoCommand
    }
    #[doc = "The Codec will wait in state Ready until the first NULL character is received."]
    #[inline(always)]
    pub fn is_auto_start(&self) -> bool {
        *self == Commandselect::AutoStart
    }
    #[doc = "SpaceWire link can proceed to Started state."]
    #[inline(always)]
    pub fn is_link_start(&self) -> bool {
        *self == Commandselect::LinkStart
    }
}
#[doc = "Field `COMMAND` writer - Command"]
pub type CommandW<'a, REG> = crate::FieldWriter<'a, REG, 2, Commandselect, crate::Safe>;
impl<'a, REG> CommandW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The link proceeds directly to the ErrorReset state when reaching the Run state."]
    #[inline(always)]
    pub fn link_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Commandselect::LinkDisable)
    }
    #[doc = "State is not actively changed."]
    #[inline(always)]
    pub fn no_command(self) -> &'a mut crate::W<REG> {
        self.variant(Commandselect::NoCommand)
    }
    #[doc = "The Codec will wait in state Ready until the first NULL character is received."]
    #[inline(always)]
    pub fn auto_start(self) -> &'a mut crate::W<REG> {
        self.variant(Commandselect::AutoStart)
    }
    #[doc = "SpaceWire link can proceed to Started state."]
    #[inline(always)]
    pub fn link_start(self) -> &'a mut crate::W<REG> {
        self.variant(Commandselect::LinkStart)
    }
}
impl R {
    #[doc = "Bits 0:1 - Command"]
    #[inline(always)]
    pub fn command(&self) -> CommandR {
        CommandR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Command"]
    #[inline(always)]
    pub fn command(&mut self) -> CommandW<Link1CfgSpec> {
        CommandW::new(self, 0)
    }
}
#[doc = "SpW Link 1 Config\n\nYou can [`read`](crate::Reg::read) this register and get [`link1_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link1_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Link1CfgSpec;
impl crate::RegisterSpec for Link1CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`link1_cfg::R`](R) reader structure"]
impl crate::Readable for Link1CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`link1_cfg::W`](W) writer structure"]
impl crate::Writable for Link1CfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LINK1_CFG to value 0"]
impl crate::Resettable for Link1CfgSpec {}
