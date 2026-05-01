#[doc = "Register `UARTTHR` reader"]
pub type R = crate::R<UartthrSpec>;
#[doc = "Register `UARTTHR` writer"]
pub type W = crate::W<UartthrSpec>;
#[doc = "Field `UARTRBR1` reader - Receiving Buffer Register"]
pub type Uartrbr1R = crate::FieldReader;
#[doc = "Field `UARTTHR1` reader - Transmit Holding Register"]
pub type Uartthr1R = crate::FieldReader;
#[doc = "Field `UARTTHR1` writer - Transmit Holding Register"]
pub type Uartthr1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Reserved1` reader - Reserved (0)"]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - Receiving Buffer Register"]
    #[inline(always)]
    pub fn uartrbr1(&self) -> Uartrbr1R {
        Uartrbr1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Transmit Holding Register"]
    #[inline(always)]
    pub fn uartthr1(&self) -> Uartthr1R {
        Uartthr1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit Holding Register"]
    #[inline(always)]
    pub fn uartthr1(&mut self) -> Uartthr1W<UartthrSpec> {
        Uartthr1W::new(self, 0)
    }
}
#[doc = "Transmit Holding Register (DLAB = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`uartthr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartthr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartthrSpec;
impl crate::RegisterSpec for UartthrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartthr::R`](R) reader structure"]
impl crate::Readable for UartthrSpec {}
#[doc = "`write(|w| ..)` method takes [`uartthr::W`](W) writer structure"]
impl crate::Writable for UartthrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTTHR to value 0"]
impl crate::Resettable for UartthrSpec {}
