#[doc = "Register `SGPIO000` reader"]
pub type R = crate::R<Sgpio000Spec>;
#[doc = "Register `SGPIO000` writer"]
pub type W = crate::W<Sgpio000Spec>;
#[doc = "Enable of Serial GPIO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblOfSerialGpio {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<EnblOfSerialGpio> for bool {
    #[inline(always)]
    fn from(variant: EnblOfSerialGpio) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblOfSerialGPIO` reader - Enable of Serial GPIO"]
pub type EnblOfSerialGpioR = crate::BitReader<EnblOfSerialGpio>;
impl EnblOfSerialGpioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblOfSerialGpio {
        match self.bits {
            false => EnblOfSerialGpio::Disable,
            true => EnblOfSerialGpio::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EnblOfSerialGpio::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EnblOfSerialGpio::Enable
    }
}
#[doc = "Field `EnblOfSerialGPIO` writer - Enable of Serial GPIO"]
pub type EnblOfSerialGpioW<'a, REG> = crate::BitWriter<'a, REG, EnblOfSerialGpio>;
impl<'a, REG> EnblOfSerialGpioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EnblOfSerialGpio::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EnblOfSerialGpio::Enable)
    }
}
#[doc = "Inverse of Serial GPIO clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InverseOfSerialGpioclk {
    #[doc = "0: Not inverse."]
    NotInverse = 0,
    #[doc = "1: Inverse."]
    Inverse = 1,
}
impl From<InverseOfSerialGpioclk> for bool {
    #[inline(always)]
    fn from(variant: InverseOfSerialGpioclk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `InverseOfSerialGPIOClk` reader - Inverse of Serial GPIO clock"]
pub type InverseOfSerialGpioclkR = crate::BitReader<InverseOfSerialGpioclk>;
impl InverseOfSerialGpioclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> InverseOfSerialGpioclk {
        match self.bits {
            false => InverseOfSerialGpioclk::NotInverse,
            true => InverseOfSerialGpioclk::Inverse,
        }
    }
    #[doc = "Not inverse."]
    #[inline(always)]
    pub fn is_not_inverse(&self) -> bool {
        *self == InverseOfSerialGpioclk::NotInverse
    }
    #[doc = "Inverse."]
    #[inline(always)]
    pub fn is_inverse(&self) -> bool {
        *self == InverseOfSerialGpioclk::Inverse
    }
}
#[doc = "Field `InverseOfSerialGPIOClk` writer - Inverse of Serial GPIO clock"]
pub type InverseOfSerialGpioclkW<'a, REG> = crate::BitWriter<'a, REG, InverseOfSerialGpioclk>;
impl<'a, REG> InverseOfSerialGpioclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not inverse."]
    #[inline(always)]
    pub fn not_inverse(self) -> &'a mut crate::W<REG> {
        self.variant(InverseOfSerialGpioclk::NotInverse)
    }
    #[doc = "Inverse."]
    #[inline(always)]
    pub fn inverse(self) -> &'a mut crate::W<REG> {
        self.variant(InverseOfSerialGpioclk::Inverse)
    }
}
#[doc = "Inverse of Serial GPIO load\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InverseOfSerialGpioload {
    #[doc = "0: Not inverse."]
    NotInverse = 0,
    #[doc = "1: Inverse."]
    Inverse = 1,
}
impl From<InverseOfSerialGpioload> for bool {
    #[inline(always)]
    fn from(variant: InverseOfSerialGpioload) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `InverseOfSerialGPIOLoad` reader - Inverse of Serial GPIO load"]
pub type InverseOfSerialGpioloadR = crate::BitReader<InverseOfSerialGpioload>;
impl InverseOfSerialGpioloadR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> InverseOfSerialGpioload {
        match self.bits {
            false => InverseOfSerialGpioload::NotInverse,
            true => InverseOfSerialGpioload::Inverse,
        }
    }
    #[doc = "Not inverse."]
    #[inline(always)]
    pub fn is_not_inverse(&self) -> bool {
        *self == InverseOfSerialGpioload::NotInverse
    }
    #[doc = "Inverse."]
    #[inline(always)]
    pub fn is_inverse(&self) -> bool {
        *self == InverseOfSerialGpioload::Inverse
    }
}
#[doc = "Field `InverseOfSerialGPIOLoad` writer - Inverse of Serial GPIO load"]
pub type InverseOfSerialGpioloadW<'a, REG> = crate::BitWriter<'a, REG, InverseOfSerialGpioload>;
impl<'a, REG> InverseOfSerialGpioloadW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not inverse."]
    #[inline(always)]
    pub fn not_inverse(self) -> &'a mut crate::W<REG> {
        self.variant(InverseOfSerialGpioload::NotInverse)
    }
    #[doc = "Inverse."]
    #[inline(always)]
    pub fn inverse(self) -> &'a mut crate::W<REG> {
        self.variant(InverseOfSerialGpioload::Inverse)
    }
}
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
    pub fn enbl_of_serial_gpio(&mut self) -> EnblOfSerialGpioW<Sgpio000Spec> {
        EnblOfSerialGpioW::new(self, 0)
    }
    #[doc = "Bit 1 - Inverse of Serial GPIO clock"]
    #[inline(always)]
    pub fn inverse_of_serial_gpioclk(&mut self) -> InverseOfSerialGpioclkW<Sgpio000Spec> {
        InverseOfSerialGpioclkW::new(self, 1)
    }
    #[doc = "Bit 2 - Inverse of Serial GPIO load"]
    #[inline(always)]
    pub fn inverse_of_serial_gpioload(&mut self) -> InverseOfSerialGpioloadW<Sgpio000Spec> {
        InverseOfSerialGpioloadW::new(self, 2)
    }
    #[doc = "Bits 6:11 - Numbers of Serial GPIO pins"]
    #[inline(always)]
    pub fn numbers_of_serial_gpiopins(&mut self) -> NumbersOfSerialGpiopinsW<Sgpio000Spec> {
        NumbersOfSerialGpiopinsW::new(self, 6)
    }
    #[doc = "Bit 12 - Serial output stop update"]
    #[inline(always)]
    pub fn serial_output_stop_update(&mut self) -> SerialOutputStopUpdateW<Sgpio000Spec> {
        SerialOutputStopUpdateW::new(self, 12)
    }
    #[doc = "Bit 13 - Serial input stop update"]
    #[inline(always)]
    pub fn serial_input_stop_update(&mut self) -> SerialInputStopUpdateW<Sgpio000Spec> {
        SerialInputStopUpdateW::new(self, 13)
    }
    #[doc = "Bit 14 - Parallel output protect enable"]
    #[inline(always)]
    pub fn parallel_output_protect_enbl(&mut self) -> ParallelOutputProtectEnblW<Sgpio000Spec> {
        ParallelOutputProtectEnblW::new(self, 14)
    }
    #[doc = "Bits 16:31 - Serial GPIO clock division"]
    #[inline(always)]
    pub fn serial_gpioclk_division(&mut self) -> SerialGpioclkDivisionW<Sgpio000Spec> {
        SerialGpioclkDivisionW::new(self, 16)
    }
}
#[doc = "Serial GPIO Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sgpio000::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgpio000::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sgpio000Spec;
impl crate::RegisterSpec for Sgpio000Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sgpio000::R`](R) reader structure"]
impl crate::Readable for Sgpio000Spec {}
#[doc = "`write(|w| ..)` method takes [`sgpio000::W`](W) writer structure"]
impl crate::Writable for Sgpio000Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SGPIO000 to value 0"]
impl crate::Resettable for Sgpio000Spec {}
