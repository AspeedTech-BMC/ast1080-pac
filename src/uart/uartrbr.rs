#[doc = "Register `UARTRBR` reader"]
pub type R = crate::R<UartrbrSpec>;
#[doc = "Register `UARTRBR` writer"]
pub type W = crate::W<UartrbrSpec>;
#[doc = "Field `UARTRBR` reader - Receiving Buffer Register"]
pub type UartrbrR = crate::FieldReader;
#[doc = "Field `UARTTHR` reader - Transmit Holding Register"]
pub type UartthrR = crate::FieldReader;
#[doc = "Field `UARTTHR` writer - Transmit Holding Register"]
pub type UartthrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Receiving Buffer Register"]
    #[inline(always)]
    pub fn uartrbr(&self) -> UartrbrR {
        UartrbrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Transmit Holding Register"]
    #[inline(always)]
    pub fn uartthr(&self) -> UartthrR {
        UartthrR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit Holding Register"]
    #[inline(always)]
    pub fn uartthr(&mut self) -> UartthrW<UartrbrSpec> {
        UartthrW::new(self, 0)
    }
}
#[doc = "Receiving Buffer Register (DLAB = 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`uartrbr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartrbr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartrbrSpec;
impl crate::RegisterSpec for UartrbrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartrbr::R`](R) reader structure"]
impl crate::Readable for UartrbrSpec {}
#[doc = "`write(|w| ..)` method takes [`uartrbr::W`](W) writer structure"]
impl crate::Writable for UartrbrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTRBR to value 0"]
impl crate::Resettable for UartrbrSpec {}
