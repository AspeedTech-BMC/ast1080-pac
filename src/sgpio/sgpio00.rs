#[doc = "Register `SGPIO00` reader"]
pub type R = crate::R<Sgpio00Spec>;
#[doc = "Register `SGPIO00` writer"]
pub type W = crate::W<Sgpio00Spec>;
#[doc = "Field `EnblOfSerialGPIO` reader - Enable of Serial GPIO"]
pub type EnblOfSerialGpioR = crate::BitReader;
#[doc = "Field `EnblOfSerialGPIO` writer - Enable of Serial GPIO"]
pub type EnblOfSerialGpioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `InverseOfSerialGPIOClk` reader - Inverse of Serial GPIO clock"]
pub type InverseOfSerialGpioclkR = crate::BitReader;
#[doc = "Field `InverseOfSerialGPIOClk` writer - Inverse of Serial GPIO clock"]
pub type InverseOfSerialGpioclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `InverseOfSerialGPIOLoad` reader - Inverse of Serial GPIO load"]
pub type InverseOfSerialGpioloadR = crate::BitReader;
#[doc = "Field `InverseOfSerialGPIOLoad` writer - Inverse of Serial GPIO load"]
pub type InverseOfSerialGpioloadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `NumbersOfSerialGPIOPins` reader - Numbers of Serial GPIO pins"]
pub type NumbersOfSerialGpiopinsR = crate::FieldReader;
#[doc = "Field `NumbersOfSerialGPIOPins` writer - Numbers of Serial GPIO pins"]
pub type NumbersOfSerialGpiopinsW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SerialOutputStopUpdate` reader - Serial output stop update"]
pub type SerialOutputStopUpdateR = crate::BitReader;
#[doc = "Field `SerialOutputStopUpdate` writer - Serial output stop update"]
pub type SerialOutputStopUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SerialInputStopUpdate` reader - Serial input stop update"]
pub type SerialInputStopUpdateR = crate::BitReader;
#[doc = "Field `SerialInputStopUpdate` writer - Serial input stop update"]
pub type SerialInputStopUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ParallelOutputProtectEnbl` reader - Parallel output protect enable"]
pub type ParallelOutputProtectEnblR = crate::BitReader;
#[doc = "Field `ParallelOutputProtectEnbl` writer - Parallel output protect enable"]
pub type ParallelOutputProtectEnblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SerialGPIOClkDivision` reader - Serial GPIO clock division"]
pub type SerialGpioclkDivisionR = crate::FieldReader<u16>;
#[doc = "Field `SerialGPIOClkDivision` writer - Serial GPIO clock division"]
pub type SerialGpioclkDivisionW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Enable of Serial GPIO"]
    #[inline(always)]
    pub fn enbl_of_serial_gpio(&self) -> EnblOfSerialGpioR {
        EnblOfSerialGpioR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Inverse of Serial GPIO clock"]
    #[inline(always)]
    pub fn inverse_of_serial_gpioclk(&self) -> InverseOfSerialGpioclkR {
        InverseOfSerialGpioclkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Inverse of Serial GPIO load"]
    #[inline(always)]
    pub fn inverse_of_serial_gpioload(&self) -> InverseOfSerialGpioloadR {
        InverseOfSerialGpioloadR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:11 - Numbers of Serial GPIO pins"]
    #[inline(always)]
    pub fn numbers_of_serial_gpiopins(&self) -> NumbersOfSerialGpiopinsR {
        NumbersOfSerialGpiopinsR::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bit 12 - Serial output stop update"]
    #[inline(always)]
    pub fn serial_output_stop_update(&self) -> SerialOutputStopUpdateR {
        SerialOutputStopUpdateR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Serial input stop update"]
    #[inline(always)]
    pub fn serial_input_stop_update(&self) -> SerialInputStopUpdateR {
        SerialInputStopUpdateR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Parallel output protect enable"]
    #[inline(always)]
    pub fn parallel_output_protect_enbl(&self) -> ParallelOutputProtectEnblR {
        ParallelOutputProtectEnblR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Serial GPIO clock division"]
    #[inline(always)]
    pub fn serial_gpioclk_division(&self) -> SerialGpioclkDivisionR {
        SerialGpioclkDivisionR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Enable of Serial GPIO"]
    #[inline(always)]
    pub fn enbl_of_serial_gpio(&mut self) -> EnblOfSerialGpioW<Sgpio00Spec> {
        EnblOfSerialGpioW::new(self, 0)
    }
    #[doc = "Bit 1 - Inverse of Serial GPIO clock"]
    #[inline(always)]
    pub fn inverse_of_serial_gpioclk(&mut self) -> InverseOfSerialGpioclkW<Sgpio00Spec> {
        InverseOfSerialGpioclkW::new(self, 1)
    }
    #[doc = "Bit 2 - Inverse of Serial GPIO load"]
    #[inline(always)]
    pub fn inverse_of_serial_gpioload(&mut self) -> InverseOfSerialGpioloadW<Sgpio00Spec> {
        InverseOfSerialGpioloadW::new(self, 2)
    }
    #[doc = "Bits 6:11 - Numbers of Serial GPIO pins"]
    #[inline(always)]
    pub fn numbers_of_serial_gpiopins(&mut self) -> NumbersOfSerialGpiopinsW<Sgpio00Spec> {
        NumbersOfSerialGpiopinsW::new(self, 6)
    }
    #[doc = "Bit 12 - Serial output stop update"]
    #[inline(always)]
    pub fn serial_output_stop_update(&mut self) -> SerialOutputStopUpdateW<Sgpio00Spec> {
        SerialOutputStopUpdateW::new(self, 12)
    }
    #[doc = "Bit 13 - Serial input stop update"]
    #[inline(always)]
    pub fn serial_input_stop_update(&mut self) -> SerialInputStopUpdateW<Sgpio00Spec> {
        SerialInputStopUpdateW::new(self, 13)
    }
    #[doc = "Bit 14 - Parallel output protect enable"]
    #[inline(always)]
    pub fn parallel_output_protect_enbl(&mut self) -> ParallelOutputProtectEnblW<Sgpio00Spec> {
        ParallelOutputProtectEnblW::new(self, 14)
    }
    #[doc = "Bits 16:31 - Serial GPIO clock division"]
    #[inline(always)]
    pub fn serial_gpioclk_division(&mut self) -> SerialGpioclkDivisionW<Sgpio00Spec> {
        SerialGpioclkDivisionW::new(self, 16)
    }
}
#[doc = "Serial GPIO Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sgpio00::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgpio00::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sgpio00Spec;
impl crate::RegisterSpec for Sgpio00Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sgpio00::R`](R) reader structure"]
impl crate::Readable for Sgpio00Spec {}
#[doc = "`write(|w| ..)` method takes [`sgpio00::W`](W) writer structure"]
impl crate::Writable for Sgpio00Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SGPIO00 to value 0"]
impl crate::Resettable for Sgpio00Spec {}
