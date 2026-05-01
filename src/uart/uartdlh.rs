#[doc = "Register `UARTDLH` reader"]
pub type R = crate::R<UartdlhSpec>;
#[doc = "Register `UARTDLH` writer"]
pub type W = crate::W<UartdlhSpec>;
#[doc = "Field `TheMSBOfTheBdDivisorLatch` reader - The MSB of the baud-rate divisor latch."]
pub type TheMsbofTheBdDivisorLatchR = crate::FieldReader;
#[doc = "Field `TheMSBOfTheBdDivisorLatch` writer - The MSB of the baud-rate divisor latch."]
pub type TheMsbofTheBdDivisorLatchW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - The MSB of the baud-rate divisor latch."]
    #[inline(always)]
    pub fn the_msbof_the_bd_divisor_latch(&self) -> TheMsbofTheBdDivisorLatchR {
        TheMsbofTheBdDivisorLatchR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The MSB of the baud-rate divisor latch."]
    #[inline(always)]
    pub fn the_msbof_the_bd_divisor_latch(&mut self) -> TheMsbofTheBdDivisorLatchW<UartdlhSpec> {
        TheMsbofTheBdDivisorLatchW::new(self, 0)
    }
}
#[doc = "Divisor Latch High Register : (DLAB = 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdlh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdlh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartdlhSpec;
impl crate::RegisterSpec for UartdlhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdlh::R`](R) reader structure"]
impl crate::Readable for UartdlhSpec {}
#[doc = "`write(|w| ..)` method takes [`uartdlh::W`](W) writer structure"]
impl crate::Writable for UartdlhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDLH to value 0"]
impl crate::Resettable for UartdlhSpec {}
