#[doc = "Register `RXF0C` reader"]
pub type R = crate::R<Rxf0cSpec>;
#[doc = "Register `RXF0C` writer"]
pub type W = crate::W<Rxf0cSpec>;
#[doc = "Field `F0SA` reader - Receive FIFO 0 Start Address"]
pub type F0saR = crate::FieldReader<u16>;
#[doc = "Field `F0SA` writer - Receive FIFO 0 Start Address"]
pub type F0saW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `F0S` reader - Receive FIFO 0 Size"]
pub type F0sR = crate::FieldReader;
#[doc = "Field `F0S` writer - Receive FIFO 0 Size"]
pub type F0sW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `F0WM` reader - Receive FIFO 0 Watermark"]
pub type F0wmR = crate::FieldReader;
#[doc = "Field `F0WM` writer - Receive FIFO 0 Watermark"]
pub type F0wmW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `F0OM` reader - FIFO 0 Operation Mode"]
pub type F0omR = crate::BitReader;
#[doc = "Field `F0OM` writer - FIFO 0 Operation Mode"]
pub type F0omW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 2:15 - Receive FIFO 0 Start Address"]
    #[inline(always)]
    pub fn f0sa(&self) -> F0saR {
        F0saR::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:22 - Receive FIFO 0 Size"]
    #[inline(always)]
    pub fn f0s(&self) -> F0sR {
        F0sR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - Receive FIFO 0 Watermark"]
    #[inline(always)]
    pub fn f0wm(&self) -> F0wmR {
        F0wmR::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - FIFO 0 Operation Mode"]
    #[inline(always)]
    pub fn f0om(&self) -> F0omR {
        F0omR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:15 - Receive FIFO 0 Start Address"]
    #[inline(always)]
    pub fn f0sa(&mut self) -> F0saW<Rxf0cSpec> {
        F0saW::new(self, 2)
    }
    #[doc = "Bits 16:22 - Receive FIFO 0 Size"]
    #[inline(always)]
    pub fn f0s(&mut self) -> F0sW<Rxf0cSpec> {
        F0sW::new(self, 16)
    }
    #[doc = "Bits 24:30 - Receive FIFO 0 Watermark"]
    #[inline(always)]
    pub fn f0wm(&mut self) -> F0wmW<Rxf0cSpec> {
        F0wmW::new(self, 24)
    }
    #[doc = "Bit 31 - FIFO 0 Operation Mode"]
    #[inline(always)]
    pub fn f0om(&mut self) -> F0omW<Rxf0cSpec> {
        F0omW::new(self, 31)
    }
}
#[doc = "Receive FIFO 0 Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxf0c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxf0c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rxf0cSpec;
impl crate::RegisterSpec for Rxf0cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxf0c::R`](R) reader structure"]
impl crate::Readable for Rxf0cSpec {}
#[doc = "`write(|w| ..)` method takes [`rxf0c::W`](W) writer structure"]
impl crate::Writable for Rxf0cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXF0C to value 0"]
impl crate::Resettable for Rxf0cSpec {}
