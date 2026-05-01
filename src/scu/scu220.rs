#[doc = "Register `SCU220` reader"]
pub type R = crate::R<Scu220Spec>;
#[doc = "Register `SCU220` writer"]
pub type W = crate::W<Scu220Spec>;
#[doc = "Field `Reserved8` reader - reserved"]
pub type Reserved8R = crate::FieldReader;
#[doc = "Field `SCURESETH2AS` reader - SCU_RESET_H2AS"]
pub type Scureseth2asR = crate::BitReader;
#[doc = "Field `SCURESETH2AS` writer - SCU_RESET_H2AS"]
pub type Scureseth2asW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `SCURESETUART0` reader - SCU_RESET_UART0"]
pub type Scuresetuart0R = crate::BitReader;
#[doc = "Field `SCURESETUART0` writer - SCU_RESET_UART0"]
pub type Scuresetuart0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCURESETUART1` reader - SCU_RESET_UART1"]
pub type Scuresetuart1R = crate::BitReader;
#[doc = "Field `SCURESETUART1` writer - SCU_RESET_UART1"]
pub type Scuresetuart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCURESETUART2` reader - SCU_RESET_UART2"]
pub type Scuresetuart2R = crate::BitReader;
#[doc = "Field `SCURESETUART2` writer - SCU_RESET_UART2"]
pub type Scuresetuart2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCURESETUART3` reader - SCU_RESET_UART3"]
pub type Scuresetuart3R = crate::BitReader;
#[doc = "Field `SCURESETUART3` writer - SCU_RESET_UART3"]
pub type Scuresetuart3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCURESETSMBF` reader - SCU_RESET_SMBF"]
pub type ScuresetsmbfR = crate::BitReader;
#[doc = "Field `SCURESETSMBF` writer - SCU_RESET_SMBF"]
pub type ScuresetsmbfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `SCURESETI2C` reader - SCU_RESET_I2C"]
pub type Scureseti2cR = crate::BitReader;
#[doc = "Field `SCURESETI2C` writer - SCU_RESET_I2C"]
pub type Scureseti2cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `Reserved3` writer - reserved"]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCURESETRTC` reader - SCU_RESET_RTC"]
pub type ScuresetrtcR = crate::BitReader;
#[doc = "Field `SCURESETRTC` writer - SCU_RESET_RTC"]
pub type ScuresetrtcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCURESETTM` reader - SCU_RESET_TM"]
pub type ScuresettmR = crate::BitReader;
#[doc = "Field `SCURESETTM` writer - SCU_RESET_TM"]
pub type ScuresettmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCURESETWDT` reader - SCU_RESET_WDT"]
pub type ScuresetwdtR = crate::BitReader;
#[doc = "Field `SCURESETWDT` writer - SCU_RESET_WDT"]
pub type ScuresetwdtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCURESETUART` reader - SCU_RESET_UART"]
pub type ScuresetuartR = crate::BitReader;
#[doc = "Field `SCURESETUART` writer - SCU_RESET_UART"]
pub type ScuresetuartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCURESETUHCI` reader - SCU_RESET_UHCI"]
pub type ScuresetuhciR = crate::BitReader;
#[doc = "Field `SCURESETUHCI` writer - SCU_RESET_UHCI"]
pub type ScuresetuhciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCURESETUSB2UARTC` reader - SCU_RESET_USB2UARTC"]
pub type Scuresetusb2uartcR = crate::BitReader;
#[doc = "Field `SCURESETUSB2UARTC` writer - SCU_RESET_USB2UARTC"]
pub type Scuresetusb2uartcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCURESETUSB2C` reader - SCU_RESET_USB2C"]
pub type Scuresetusb2cR = crate::BitReader;
#[doc = "Field `SCURESETUSB2C` writer - SCU_RESET_USB2C"]
pub type Scuresetusb2cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCURESETUSB2UARTD` reader - SCU_RESET_USB2UARTD"]
pub type Scuresetusb2uartdR = crate::BitReader;
#[doc = "Field `SCURESETUSB2UARTD` writer - SCU_RESET_USB2UARTD"]
pub type Scuresetusb2uartdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCURESETUSB2D` reader - SCU_RESET_USB2D"]
pub type Scuresetusb2dR = crate::BitReader;
#[doc = "Field `SCURESETUSB2D` writer - SCU_RESET_USB2D"]
pub type Scuresetusb2dW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCURESETI3CDMA` reader - SCU_RESET_I3CDMA"]
pub type Scureseti3cdmaR = crate::BitReader;
#[doc = "Field `SCURESETI3CDMA` writer - SCU_RESET_I3CDMA"]
pub type Scureseti3cdmaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - reserved"]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - SCU_RESET_H2AS"]
    #[inline(always)]
    pub fn scureseth2as(&self) -> Scureseth2asR {
        Scureseth2asR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 2 - reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SCU_RESET_UART0"]
    #[inline(always)]
    pub fn scuresetuart0(&self) -> Scuresetuart0R {
        Scuresetuart0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SCU_RESET_UART1"]
    #[inline(always)]
    pub fn scuresetuart1(&self) -> Scuresetuart1R {
        Scuresetuart1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SCU_RESET_UART2"]
    #[inline(always)]
    pub fn scuresetuart2(&self) -> Scuresetuart2R {
        Scuresetuart2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SCU_RESET_UART3"]
    #[inline(always)]
    pub fn scuresetuart3(&self) -> Scuresetuart3R {
        Scuresetuart3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 8 - SCU_RESET_SMBF"]
    #[inline(always)]
    pub fn scuresetsmbf(&self) -> ScuresetsmbfR {
        ScuresetsmbfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:14 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 9) & 0x3f) as u8)
    }
    #[doc = "Bit 15 - SCU_RESET_I2C"]
    #[inline(always)]
    pub fn scureseti2c(&self) -> Scureseti2cR {
        Scureseti2cR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - SCU_RESET_RTC"]
    #[inline(always)]
    pub fn scuresetrtc(&self) -> ScuresetrtcR {
        ScuresetrtcR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SCU_RESET_TM"]
    #[inline(always)]
    pub fn scuresettm(&self) -> ScuresettmR {
        ScuresettmR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SCU_RESET_WDT"]
    #[inline(always)]
    pub fn scuresetwdt(&self) -> ScuresetwdtR {
        ScuresetwdtR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SCU_RESET_UART"]
    #[inline(always)]
    pub fn scuresetuart(&self) -> ScuresetuartR {
        ScuresetuartR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:24 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bit 25 - SCU_RESET_UHCI"]
    #[inline(always)]
    pub fn scuresetuhci(&self) -> ScuresetuhciR {
        ScuresetuhciR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - SCU_RESET_USB2UARTC"]
    #[inline(always)]
    pub fn scuresetusb2uartc(&self) -> Scuresetusb2uartcR {
        Scuresetusb2uartcR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - SCU_RESET_USB2C"]
    #[inline(always)]
    pub fn scuresetusb2c(&self) -> Scuresetusb2cR {
        Scuresetusb2cR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - SCU_RESET_USB2UARTD"]
    #[inline(always)]
    pub fn scuresetusb2uartd(&self) -> Scuresetusb2uartdR {
        Scuresetusb2uartdR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SCU_RESET_USB2D"]
    #[inline(always)]
    pub fn scuresetusb2d(&self) -> Scuresetusb2dR {
        Scuresetusb2dR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - SCU_RESET_I3CDMA"]
    #[inline(always)]
    pub fn scureseti3cdma(&self) -> Scureseti3cdmaR {
        Scureseti3cdmaR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - SCU_RESET_H2AS"]
    #[inline(always)]
    pub fn scureseth2as(&mut self) -> Scureseth2asW<Scu220Spec> {
        Scureseth2asW::new(self, 2)
    }
    #[doc = "Bit 2 - reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Scu220Spec> {
        Reserved7W::new(self, 2)
    }
    #[doc = "Bit 4 - SCU_RESET_UART0"]
    #[inline(always)]
    pub fn scuresetuart0(&mut self) -> Scuresetuart0W<Scu220Spec> {
        Scuresetuart0W::new(self, 4)
    }
    #[doc = "Bit 5 - SCU_RESET_UART1"]
    #[inline(always)]
    pub fn scuresetuart1(&mut self) -> Scuresetuart1W<Scu220Spec> {
        Scuresetuart1W::new(self, 5)
    }
    #[doc = "Bit 6 - SCU_RESET_UART2"]
    #[inline(always)]
    pub fn scuresetuart2(&mut self) -> Scuresetuart2W<Scu220Spec> {
        Scuresetuart2W::new(self, 6)
    }
    #[doc = "Bit 7 - SCU_RESET_UART3"]
    #[inline(always)]
    pub fn scuresetuart3(&mut self) -> Scuresetuart3W<Scu220Spec> {
        Scuresetuart3W::new(self, 7)
    }
    #[doc = "Bit 8 - reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Scu220Spec> {
        Reserved5W::new(self, 8)
    }
    #[doc = "Bit 8 - SCU_RESET_SMBF"]
    #[inline(always)]
    pub fn scuresetsmbf(&mut self) -> ScuresetsmbfW<Scu220Spec> {
        ScuresetsmbfW::new(self, 8)
    }
    #[doc = "Bit 15 - SCU_RESET_I2C"]
    #[inline(always)]
    pub fn scureseti2c(&mut self) -> Scureseti2cW<Scu220Spec> {
        Scureseti2cW::new(self, 15)
    }
    #[doc = "Bits 16:17 - reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Scu220Spec> {
        Reserved3W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_RESET_RTC"]
    #[inline(always)]
    pub fn scuresetrtc(&mut self) -> ScuresetrtcW<Scu220Spec> {
        ScuresetrtcW::new(self, 18)
    }
    #[doc = "Bit 19 - SCU_RESET_TM"]
    #[inline(always)]
    pub fn scuresettm(&mut self) -> ScuresettmW<Scu220Spec> {
        ScuresettmW::new(self, 19)
    }
    #[doc = "Bit 20 - SCU_RESET_WDT"]
    #[inline(always)]
    pub fn scuresetwdt(&mut self) -> ScuresetwdtW<Scu220Spec> {
        ScuresetwdtW::new(self, 20)
    }
    #[doc = "Bit 21 - SCU_RESET_UART"]
    #[inline(always)]
    pub fn scuresetuart(&mut self) -> ScuresetuartW<Scu220Spec> {
        ScuresetuartW::new(self, 21)
    }
    #[doc = "Bits 23:24 - reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Scu220Spec> {
        Reserved1W::new(self, 23)
    }
    #[doc = "Bit 25 - SCU_RESET_UHCI"]
    #[inline(always)]
    pub fn scuresetuhci(&mut self) -> ScuresetuhciW<Scu220Spec> {
        ScuresetuhciW::new(self, 25)
    }
    #[doc = "Bit 26 - SCU_RESET_USB2UARTC"]
    #[inline(always)]
    pub fn scuresetusb2uartc(&mut self) -> Scuresetusb2uartcW<Scu220Spec> {
        Scuresetusb2uartcW::new(self, 26)
    }
    #[doc = "Bit 27 - SCU_RESET_USB2C"]
    #[inline(always)]
    pub fn scuresetusb2c(&mut self) -> Scuresetusb2cW<Scu220Spec> {
        Scuresetusb2cW::new(self, 27)
    }
    #[doc = "Bit 28 - SCU_RESET_USB2UARTD"]
    #[inline(always)]
    pub fn scuresetusb2uartd(&mut self) -> Scuresetusb2uartdW<Scu220Spec> {
        Scuresetusb2uartdW::new(self, 28)
    }
    #[doc = "Bit 29 - SCU_RESET_USB2D"]
    #[inline(always)]
    pub fn scuresetusb2d(&mut self) -> Scuresetusb2dW<Scu220Spec> {
        Scuresetusb2dW::new(self, 29)
    }
    #[doc = "Bit 31 - SCU_RESET_I3CDMA"]
    #[inline(always)]
    pub fn scureseti3cdma(&mut self) -> Scureseti3cdmaW<Scu220Spec> {
        Scureseti3cdmaW::new(self, 31)
    }
}
#[doc = "System Reset Control 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu220::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu220::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu220Spec;
impl crate::RegisterSpec for Scu220Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu220::R`](R) reader structure"]
impl crate::Readable for Scu220Spec {}
#[doc = "`write(|w| ..)` method takes [`scu220::W`](W) writer structure"]
impl crate::Writable for Scu220Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU220 to value 0xc003_0100"]
impl crate::Resettable for Scu220Spec {
    const RESET_VALUE: u32 = 0xc003_0100;
}
