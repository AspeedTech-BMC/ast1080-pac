#[doc = "Register `SCU2F4` reader"]
pub type R = crate::R<Scu2f4Spec>;
#[doc = "Register `SCU2F4` writer"]
pub type W = crate::W<Scu2f4Spec>;
#[doc = "Field `SCURESETEXTCPU` reader - SCU_RESET_EXT_CPU"]
pub type ScuresetextcpuR = crate::BitReader;
#[doc = "Field `SCURESETEXTCPU` writer - SCU_RESET_EXT_CPU"]
pub type ScuresetextcpuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCURESETEXTSOC` reader - SCU_RESET_EXT_SOC"]
pub type ScuresetextsocR = crate::BitReader;
#[doc = "Field `SCURESETEXTSOC` writer - SCU_RESET_EXT_SOC"]
pub type ScuresetextsocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCURESETEXTAHB` reader - SCU_RESET_EXT_AHB"]
pub type ScuresetextahbR = crate::BitReader;
#[doc = "Field `SCURESETEXTAHB` writer - SCU_RESET_EXT_AHB"]
pub type ScuresetextahbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved9` reader - reserved"]
pub type Reserved9R = crate::BitReader;
#[doc = "Field `SCURESETEXTUART0` reader - SCU_RESET_EXT_UART0"]
pub type Scuresetextuart0R = crate::BitReader;
#[doc = "Field `SCURESETEXTUART0` writer - SCU_RESET_EXT_UART0"]
pub type Scuresetextuart0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCURESETEXTUART1` reader - SCU_RESET_EXT_UART1"]
pub type Scuresetextuart1R = crate::BitReader;
#[doc = "Field `SCURESETEXTUART1` writer - SCU_RESET_EXT_UART1"]
pub type Scuresetextuart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCURESETEXTUART2` reader - SCU_RESET_EXT_UART2"]
pub type Scuresetextuart2R = crate::BitReader;
#[doc = "Field `SCURESETEXTUART2` writer - SCU_RESET_EXT_UART2"]
pub type Scuresetextuart2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCURESETEXTUART3` reader - SCU_RESET_EXT_UART3"]
pub type Scuresetextuart3R = crate::BitReader;
#[doc = "Field `SCURESETEXTUART3` writer - SCU_RESET_EXT_UART3"]
pub type Scuresetextuart3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCURESETEXTI2CF` reader - SCU_RESET_EXT_I2CF"]
pub type Scuresetexti2cfR = crate::BitReader;
#[doc = "Field `SCURESETEXTI2CF` writer - SCU_RESET_EXT_I2CF"]
pub type Scuresetexti2cfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCURESETEXTH2AS` reader - SCU_RESET_EXT_H2AS"]
pub type Scuresetexth2asR = crate::BitReader;
#[doc = "Field `SCURESETEXTH2AS` writer - SCU_RESET_EXT_H2AS"]
pub type Scuresetexth2asW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved8` reader - reserved"]
pub type Reserved8R = crate::BitReader;
#[doc = "Field `Reserved8` writer - reserved"]
pub type Reserved8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `SCURESETEXTGPIO` reader - SCU_RESET_EXT_GPIO"]
pub type ScuresetextgpioR = crate::BitReader;
#[doc = "Field `SCURESETEXTGPIO` writer - SCU_RESET_EXT_GPIO"]
pub type ScuresetextgpioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCURESETEXTRTC` reader - SCU_RESET_EXT_RTC"]
pub type ScuresetextrtcR = crate::BitReader;
#[doc = "Field `SCURESETEXTRTC` writer - SCU_RESET_EXT_RTC"]
pub type ScuresetextrtcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCURESETEXTTM` reader - SCU_RESET_EXT_TM"]
pub type ScuresetexttmR = crate::BitReader;
#[doc = "Field `SCURESETEXTTM` writer - SCU_RESET_EXT_TM"]
pub type ScuresetexttmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCURESETEXTWDT` reader - SCU_RESET_EXT_WDT"]
pub type ScuresetextwdtR = crate::BitReader;
#[doc = "Field `SCURESETEXTWDT` writer - SCU_RESET_EXT_WDT"]
pub type ScuresetextwdtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCURESETEXTUART` reader - SCU_RESET_EXT_UART"]
pub type ScuresetextuartR = crate::BitReader;
#[doc = "Field `SCURESETEXTUART` writer - SCU_RESET_EXT_UART"]
pub type ScuresetextuartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::FieldReader;
#[doc = "Field `SCURESETEXTMCU` reader - SCU_RESET_EXT_MCU"]
pub type ScuresetextmcuR = crate::BitReader;
#[doc = "Field `SCURESETEXTMCU` writer - SCU_RESET_EXT_MCU"]
pub type ScuresetextmcuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `Reserved4` writer - reserved"]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `Reserved2` writer - reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCURESETEXTCPTRA` reader - SCU_RESET_EXT_CPTRA"]
pub type ScuresetextcptraR = crate::BitReader;
#[doc = "Field `SCURESETEXTCPTRA` writer - SCU_RESET_EXT_CPTRA"]
pub type ScuresetextcptraW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCURESETEXTI3CDMA` reader - SCU_RESET_EXT_I3CDMA"]
pub type Scuresetexti3cdmaR = crate::BitReader;
#[doc = "Field `SCURESETEXTI3CDMA` writer - SCU_RESET_EXT_I3CDMA"]
pub type Scuresetexti3cdmaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_RESET_EXT_CPU"]
    #[inline(always)]
    pub fn scuresetextcpu(&self) -> ScuresetextcpuR {
        ScuresetextcpuR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCU_RESET_EXT_SOC"]
    #[inline(always)]
    pub fn scuresetextsoc(&self) -> ScuresetextsocR {
        ScuresetextsocR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_RESET_EXT_AHB"]
    #[inline(always)]
    pub fn scuresetextahb(&self) -> ScuresetextahbR {
        ScuresetextahbR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SCU_RESET_EXT_UART0"]
    #[inline(always)]
    pub fn scuresetextuart0(&self) -> Scuresetextuart0R {
        Scuresetextuart0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SCU_RESET_EXT_UART1"]
    #[inline(always)]
    pub fn scuresetextuart1(&self) -> Scuresetextuart1R {
        Scuresetextuart1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SCU_RESET_EXT_UART2"]
    #[inline(always)]
    pub fn scuresetextuart2(&self) -> Scuresetextuart2R {
        Scuresetextuart2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SCU_RESET_EXT_UART3"]
    #[inline(always)]
    pub fn scuresetextuart3(&self) -> Scuresetextuart3R {
        Scuresetextuart3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 8 - SCU_RESET_EXT_I2CF"]
    #[inline(always)]
    pub fn scuresetexti2cf(&self) -> Scuresetexti2cfR {
        Scuresetexti2cfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_RESET_EXT_H2AS"]
    #[inline(always)]
    pub fn scuresetexth2as(&self) -> Scuresetexth2asR {
        Scuresetexth2asR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 9 - reserved"]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SCU_RESET_EXT_GPIO"]
    #[inline(always)]
    pub fn scuresetextgpio(&self) -> ScuresetextgpioR {
        ScuresetextgpioR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SCU_RESET_EXT_RTC"]
    #[inline(always)]
    pub fn scuresetextrtc(&self) -> ScuresetextrtcR {
        ScuresetextrtcR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SCU_RESET_EXT_TM"]
    #[inline(always)]
    pub fn scuresetexttm(&self) -> ScuresetexttmR {
        ScuresetexttmR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SCU_RESET_EXT_WDT"]
    #[inline(always)]
    pub fn scuresetextwdt(&self) -> ScuresetextwdtR {
        ScuresetextwdtR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SCU_RESET_EXT_UART"]
    #[inline(always)]
    pub fn scuresetextuart(&self) -> ScuresetextuartR {
        ScuresetextuartR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - SCU_RESET_EXT_MCU"]
    #[inline(always)]
    pub fn scuresetextmcu(&self) -> ScuresetextmcuR {
        ScuresetextmcuR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 23:25 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bits 26:27 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SCU_RESET_EXT_CPTRA"]
    #[inline(always)]
    pub fn scuresetextcptra(&self) -> ScuresetextcptraR {
        ScuresetextcptraR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - SCU_RESET_EXT_I3CDMA"]
    #[inline(always)]
    pub fn scuresetexti3cdma(&self) -> Scuresetexti3cdmaR {
        Scuresetexti3cdmaR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_RESET_EXT_CPU"]
    #[inline(always)]
    pub fn scuresetextcpu(&mut self) -> ScuresetextcpuW<Scu2f4Spec> {
        ScuresetextcpuW::new(self, 0)
    }
    #[doc = "Bit 1 - SCU_RESET_EXT_SOC"]
    #[inline(always)]
    pub fn scuresetextsoc(&mut self) -> ScuresetextsocW<Scu2f4Spec> {
        ScuresetextsocW::new(self, 1)
    }
    #[doc = "Bit 2 - SCU_RESET_EXT_AHB"]
    #[inline(always)]
    pub fn scuresetextahb(&mut self) -> ScuresetextahbW<Scu2f4Spec> {
        ScuresetextahbW::new(self, 2)
    }
    #[doc = "Bit 4 - SCU_RESET_EXT_UART0"]
    #[inline(always)]
    pub fn scuresetextuart0(&mut self) -> Scuresetextuart0W<Scu2f4Spec> {
        Scuresetextuart0W::new(self, 4)
    }
    #[doc = "Bit 5 - SCU_RESET_EXT_UART1"]
    #[inline(always)]
    pub fn scuresetextuart1(&mut self) -> Scuresetextuart1W<Scu2f4Spec> {
        Scuresetextuart1W::new(self, 5)
    }
    #[doc = "Bit 6 - SCU_RESET_EXT_UART2"]
    #[inline(always)]
    pub fn scuresetextuart2(&mut self) -> Scuresetextuart2W<Scu2f4Spec> {
        Scuresetextuart2W::new(self, 6)
    }
    #[doc = "Bit 7 - SCU_RESET_EXT_UART3"]
    #[inline(always)]
    pub fn scuresetextuart3(&mut self) -> Scuresetextuart3W<Scu2f4Spec> {
        Scuresetextuart3W::new(self, 7)
    }
    #[doc = "Bit 8 - reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Scu2f4Spec> {
        Reserved7W::new(self, 8)
    }
    #[doc = "Bit 8 - SCU_RESET_EXT_I2CF"]
    #[inline(always)]
    pub fn scuresetexti2cf(&mut self) -> Scuresetexti2cfW<Scu2f4Spec> {
        Scuresetexti2cfW::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_RESET_EXT_H2AS"]
    #[inline(always)]
    pub fn scuresetexth2as(&mut self) -> Scuresetexth2asW<Scu2f4Spec> {
        Scuresetexth2asW::new(self, 9)
    }
    #[doc = "Bit 9 - reserved"]
    #[inline(always)]
    pub fn reserved8(&mut self) -> Reserved8W<Scu2f4Spec> {
        Reserved8W::new(self, 9)
    }
    #[doc = "Bit 11 - SCU_RESET_EXT_GPIO"]
    #[inline(always)]
    pub fn scuresetextgpio(&mut self) -> ScuresetextgpioW<Scu2f4Spec> {
        ScuresetextgpioW::new(self, 11)
    }
    #[doc = "Bit 12 - SCU_RESET_EXT_RTC"]
    #[inline(always)]
    pub fn scuresetextrtc(&mut self) -> ScuresetextrtcW<Scu2f4Spec> {
        ScuresetextrtcW::new(self, 12)
    }
    #[doc = "Bit 13 - SCU_RESET_EXT_TM"]
    #[inline(always)]
    pub fn scuresetexttm(&mut self) -> ScuresetexttmW<Scu2f4Spec> {
        ScuresetexttmW::new(self, 13)
    }
    #[doc = "Bit 14 - SCU_RESET_EXT_WDT"]
    #[inline(always)]
    pub fn scuresetextwdt(&mut self) -> ScuresetextwdtW<Scu2f4Spec> {
        ScuresetextwdtW::new(self, 14)
    }
    #[doc = "Bit 15 - SCU_RESET_EXT_UART"]
    #[inline(always)]
    pub fn scuresetextuart(&mut self) -> ScuresetextuartW<Scu2f4Spec> {
        ScuresetextuartW::new(self, 15)
    }
    #[doc = "Bit 20 - SCU_RESET_EXT_MCU"]
    #[inline(always)]
    pub fn scuresetextmcu(&mut self) -> ScuresetextmcuW<Scu2f4Spec> {
        ScuresetextmcuW::new(self, 20)
    }
    #[doc = "Bits 21:22 - reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Scu2f4Spec> {
        Reserved4W::new(self, 21)
    }
    #[doc = "Bits 26:27 - reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Scu2f4Spec> {
        Reserved2W::new(self, 26)
    }
    #[doc = "Bit 29 - SCU_RESET_EXT_CPTRA"]
    #[inline(always)]
    pub fn scuresetextcptra(&mut self) -> ScuresetextcptraW<Scu2f4Spec> {
        ScuresetextcptraW::new(self, 29)
    }
    #[doc = "Bit 31 - SCU_RESET_EXT_I3CDMA"]
    #[inline(always)]
    pub fn scuresetexti3cdma(&mut self) -> Scuresetexti3cdmaW<Scu2f4Spec> {
        Scuresetexti3cdmaW::new(self, 31)
    }
}
#[doc = "EXTRST\\# Reset Selection 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu2f4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu2f4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu2f4Spec;
impl crate::RegisterSpec for Scu2f4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu2f4::R`](R) reader structure"]
impl crate::Readable for Scu2f4Spec {}
#[doc = "`write(|w| ..)` method takes [`scu2f4::W`](W) writer structure"]
impl crate::Writable for Scu2f4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU2F4 to value 0x2010_e803"]
impl crate::Resettable for Scu2f4Spec {
    const RESET_VALUE: u32 = 0x2010_e803;
}
