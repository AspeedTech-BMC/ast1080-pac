#[doc = "Register `UDMA00C` reader"]
pub type R = crate::R<Udma00cSpec>;
#[doc = "Register `UDMA00C` writer"]
pub type W = crate::W<Udma00cSpec>;
#[doc = "Field `UARTDMATimeOutTimer` reader - UART DMA time out timer"]
pub type UartdmatimeOutTimerR = crate::FieldReader<u16>;
#[doc = "Field `UARTDMATimeOutTimer` writer - UART DMA time out timer"]
pub type UartdmatimeOutTimerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - UART DMA time out timer"]
    #[inline(always)]
    pub fn uartdmatime_out_timer(&self) -> UartdmatimeOutTimerR {
        UartdmatimeOutTimerR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - UART DMA time out timer"]
    #[inline(always)]
    pub fn uartdmatime_out_timer(&mut self) -> UartdmatimeOutTimerW<Udma00cSpec> {
        UartdmatimeOutTimerW::new(self, 0)
    }
}
#[doc = "UART DMA time out timer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma00c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma00c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma00cSpec;
impl crate::RegisterSpec for Udma00cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma00c::R`](R) reader structure"]
impl crate::Readable for Udma00cSpec {}
#[doc = "`write(|w| ..)` method takes [`udma00c::W`](W) writer structure"]
impl crate::Writable for Udma00cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA00C to value 0"]
impl crate::Resettable for Udma00cSpec {}
