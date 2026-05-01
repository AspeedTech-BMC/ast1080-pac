#[doc = "Register `SCU260` reader"]
pub type R = crate::R<Scu260Spec>;
#[doc = "Register `SCU260` writer"]
pub type W = crate::W<Scu260Spec>;
#[doc = "Field `SCUCLKSTOPUART5` reader - SCU_CLK_STOP_UART5"]
pub type Scuclkstopuart5R = crate::BitReader;
#[doc = "Field `SCUCLKSTOPUART5` writer - SCU_CLK_STOP_UART5"]
pub type Scuclkstopuart5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUCLKSTOPUART6` reader - SCU_CLK_STOP_UART6"]
pub type Scuclkstopuart6R = crate::BitReader;
#[doc = "Field `SCUCLKSTOPUART6` writer - SCU_CLK_STOP_UART6"]
pub type Scuclkstopuart6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUCLKSTOPUART7` reader - SCU_CLK_STOP_UART7"]
pub type Scuclkstopuart7R = crate::BitReader;
#[doc = "Field `SCUCLKSTOPUART7` writer - SCU_CLK_STOP_UART7"]
pub type Scuclkstopuart7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUCLKSTOPUART8` reader - SCU_CLK_STOP_UART8"]
pub type Scuclkstopuart8R = crate::BitReader;
#[doc = "Field `SCUCLKSTOPUART8` writer - SCU_CLK_STOP_UART8"]
pub type Scuclkstopuart8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUCLKSTOPUART9` reader - SCU_CLK_STOP_UART9"]
pub type Scuclkstopuart9R = crate::BitReader;
#[doc = "Field `SCUCLKSTOPUART9` writer - SCU_CLK_STOP_UART9"]
pub type Scuclkstopuart9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUCLKSTOPUART10` reader - SCU_CLK_STOP_UART10"]
pub type Scuclkstopuart10R = crate::BitReader;
#[doc = "Field `SCUCLKSTOPUART10` writer - SCU_CLK_STOP_UART10"]
pub type Scuclkstopuart10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUCLKSTOPUART11` reader - SCU_CLK_STOP_UART11"]
pub type Scuclkstopuart11R = crate::BitReader;
#[doc = "Field `SCUCLKSTOPUART11` writer - SCU_CLK_STOP_UART11"]
pub type Scuclkstopuart11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUCLKSTOPUART12` reader - SCU_CLK_STOP_UART12"]
pub type Scuclkstopuart12R = crate::BitReader;
#[doc = "Field `SCUCLKSTOPUART12` writer - SCU_CLK_STOP_UART12"]
pub type Scuclkstopuart12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `SCUCLKSTOPUHCI` reader - SCU_CLK_STOP_UHCI"]
pub type ScuclkstopuhciR = crate::BitReader;
#[doc = "Field `SCUCLKSTOPUHCI` writer - SCU_CLK_STOP_UHCI"]
pub type ScuclkstopuhciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `SCUCLKSTOPUSB2C` reader - SCU_CLK_STOP_USB2C"]
pub type Scuclkstopusb2cR = crate::BitReader;
#[doc = "Field `SCUCLKSTOPUSB2C` writer - SCU_CLK_STOP_USB2C"]
pub type Scuclkstopusb2cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUCLKSTOPUSB2D` reader - SCU_CLK_STOP_USB2D"]
pub type Scuclkstopusb2dR = crate::BitReader;
#[doc = "Field `SCUCLKSTOPUSB2D` writer - SCU_CLK_STOP_USB2D"]
pub type Scuclkstopusb2dW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_CLK_STOP_UART5"]
    #[inline(always)]
    pub fn scuclkstopuart5(&self) -> Scuclkstopuart5R {
        Scuclkstopuart5R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCU_CLK_STOP_UART6"]
    #[inline(always)]
    pub fn scuclkstopuart6(&self) -> Scuclkstopuart6R {
        Scuclkstopuart6R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_CLK_STOP_UART7"]
    #[inline(always)]
    pub fn scuclkstopuart7(&self) -> Scuclkstopuart7R {
        Scuclkstopuart7R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SCU_CLK_STOP_UART8"]
    #[inline(always)]
    pub fn scuclkstopuart8(&self) -> Scuclkstopuart8R {
        Scuclkstopuart8R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SCU_CLK_STOP_UART9"]
    #[inline(always)]
    pub fn scuclkstopuart9(&self) -> Scuclkstopuart9R {
        Scuclkstopuart9R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SCU_CLK_STOP_UART10"]
    #[inline(always)]
    pub fn scuclkstopuart10(&self) -> Scuclkstopuart10R {
        Scuclkstopuart10R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SCU_CLK_STOP_UART11"]
    #[inline(always)]
    pub fn scuclkstopuart11(&self) -> Scuclkstopuart11R {
        Scuclkstopuart11R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SCU_CLK_STOP_UART12"]
    #[inline(always)]
    pub fn scuclkstopuart12(&self) -> Scuclkstopuart12R {
        Scuclkstopuart12R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - SCU_CLK_STOP_UHCI"]
    #[inline(always)]
    pub fn scuclkstopuhci(&self) -> ScuclkstopuhciR {
        ScuclkstopuhciR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:16 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bit 17 - SCU_CLK_STOP_USB2C"]
    #[inline(always)]
    pub fn scuclkstopusb2c(&self) -> Scuclkstopusb2cR {
        Scuclkstopusb2cR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_CLK_STOP_USB2D"]
    #[inline(always)]
    pub fn scuclkstopusb2d(&self) -> Scuclkstopusb2dR {
        Scuclkstopusb2dR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_CLK_STOP_UART5"]
    #[inline(always)]
    pub fn scuclkstopuart5(&mut self) -> Scuclkstopuart5W<Scu260Spec> {
        Scuclkstopuart5W::new(self, 0)
    }
    #[doc = "Bit 1 - SCU_CLK_STOP_UART6"]
    #[inline(always)]
    pub fn scuclkstopuart6(&mut self) -> Scuclkstopuart6W<Scu260Spec> {
        Scuclkstopuart6W::new(self, 1)
    }
    #[doc = "Bit 2 - SCU_CLK_STOP_UART7"]
    #[inline(always)]
    pub fn scuclkstopuart7(&mut self) -> Scuclkstopuart7W<Scu260Spec> {
        Scuclkstopuart7W::new(self, 2)
    }
    #[doc = "Bit 3 - SCU_CLK_STOP_UART8"]
    #[inline(always)]
    pub fn scuclkstopuart8(&mut self) -> Scuclkstopuart8W<Scu260Spec> {
        Scuclkstopuart8W::new(self, 3)
    }
    #[doc = "Bit 4 - SCU_CLK_STOP_UART9"]
    #[inline(always)]
    pub fn scuclkstopuart9(&mut self) -> Scuclkstopuart9W<Scu260Spec> {
        Scuclkstopuart9W::new(self, 4)
    }
    #[doc = "Bit 5 - SCU_CLK_STOP_UART10"]
    #[inline(always)]
    pub fn scuclkstopuart10(&mut self) -> Scuclkstopuart10W<Scu260Spec> {
        Scuclkstopuart10W::new(self, 5)
    }
    #[doc = "Bit 6 - SCU_CLK_STOP_UART11"]
    #[inline(always)]
    pub fn scuclkstopuart11(&mut self) -> Scuclkstopuart11W<Scu260Spec> {
        Scuclkstopuart11W::new(self, 6)
    }
    #[doc = "Bit 7 - SCU_CLK_STOP_UART12"]
    #[inline(always)]
    pub fn scuclkstopuart12(&mut self) -> Scuclkstopuart12W<Scu260Spec> {
        Scuclkstopuart12W::new(self, 7)
    }
    #[doc = "Bit 12 - SCU_CLK_STOP_UHCI"]
    #[inline(always)]
    pub fn scuclkstopuhci(&mut self) -> ScuclkstopuhciW<Scu260Spec> {
        ScuclkstopuhciW::new(self, 12)
    }
    #[doc = "Bit 17 - SCU_CLK_STOP_USB2C"]
    #[inline(always)]
    pub fn scuclkstopusb2c(&mut self) -> Scuclkstopusb2cW<Scu260Spec> {
        Scuclkstopusb2cW::new(self, 17)
    }
    #[doc = "Bit 18 - SCU_CLK_STOP_USB2D"]
    #[inline(always)]
    pub fn scuclkstopusb2d(&mut self) -> Scuclkstopusb2dW<Scu260Spec> {
        Scuclkstopusb2dW::new(self, 18)
    }
}
#[doc = "Clock Stop Control 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu260::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu260::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu260Spec;
impl crate::RegisterSpec for Scu260Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu260::R`](R) reader structure"]
impl crate::Readable for Scu260Spec {}
#[doc = "`write(|w| ..)` method takes [`scu260::W`](W) writer structure"]
impl crate::Writable for Scu260Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU260 to value 0x1070"]
impl crate::Resettable for Scu260Spec {
    const RESET_VALUE: u32 = 0x1070;
}
