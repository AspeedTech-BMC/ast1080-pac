#[doc = "Register `UARTDLL` reader"]
pub type R = crate::R<UartdllSpec>;
#[doc = "Register `UARTDLL` writer"]
pub type W = crate::W<UartdllSpec>;
#[doc = "Field `TheLSBOfTheBdDivisorLatch` reader - The LSB of the baud-rate divisor latch."]
pub type TheLsbofTheBdDivisorLatchR = crate::FieldReader;
#[doc = "Field `TheLSBOfTheBdDivisorLatch` writer - The LSB of the baud-rate divisor latch."]
pub type TheLsbofTheBdDivisorLatchW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - The LSB of the baud-rate divisor latch."]
    #[inline(always)]
    pub fn the_lsbof_the_bd_divisor_latch(&self) -> TheLsbofTheBdDivisorLatchR {
        TheLsbofTheBdDivisorLatchR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The LSB of the baud-rate divisor latch."]
    #[inline(always)]
    pub fn the_lsbof_the_bd_divisor_latch(&mut self) -> TheLsbofTheBdDivisorLatchW<UartdllSpec> {
        TheLsbofTheBdDivisorLatchW::new(self, 0)
    }
}
#[doc = "Divisor Latch Low Register : (DLAB = 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdll::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdll::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartdllSpec;
impl crate::RegisterSpec for UartdllSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdll::R`](R) reader structure"]
impl crate::Readable for UartdllSpec {}
#[doc = "`write(|w| ..)` method takes [`uartdll::W`](W) writer structure"]
impl crate::Writable for UartdllSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDLL to value 0"]
impl crate::Resettable for UartdllSpec {}
