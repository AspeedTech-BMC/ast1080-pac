#[doc = "Register `UARTDMA00C` reader"]
pub type R = crate::R<Uartdma00cSpec>;
#[doc = "Register `UARTDMA00C` writer"]
pub type W = crate::W<Uartdma00cSpec>;
#[doc = "Field `UARTDMATimeOutTimer` reader - UART DMA time out timer"]
pub type UartdmatimeOutTimerR = crate::FieldReader<u16>;
#[doc = "Field `UARTDMATimeOutTimer` writer - UART DMA time out timer"]
pub type UartdmatimeOutTimerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - UART DMA time out timer"]
    #[inline(always)]
    pub fn uartdmatime_out_timer(&self) -> UartdmatimeOutTimerR {
        UartdmatimeOutTimerR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - UART DMA time out timer"]
    #[inline(always)]
    pub fn uartdmatime_out_timer(&mut self) -> UartdmatimeOutTimerW<Uartdma00cSpec> {
        UartdmatimeOutTimerW::new(self, 0)
    }
}
#[doc = "UART DMA time out timer\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma00c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma00c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma00cSpec;
impl crate::RegisterSpec for Uartdma00cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma00c::R`](R) reader structure"]
impl crate::Readable for Uartdma00cSpec {}
#[doc = "`write(|w| ..)` method takes [`uartdma00c::W`](W) writer structure"]
impl crate::Writable for Uartdma00cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA00C to value 0"]
impl crate::Resettable for Uartdma00cSpec {}
