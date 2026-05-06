#[doc = "Register `UARTRBR` reader"]
pub type R = crate::R<UartrbrSpec>;
#[doc = "Register `UARTRBR` writer"]
pub type W = crate::W<UartrbrSpec>;
#[doc = "Field `UARTRBRReceivingBufferReg` reader - UART_RBR: Receiving Buffer Register"]
pub type UartrbrreceivingBufferRegR = crate::FieldReader;
#[doc = "Field `UARTTHRTxHoldingReg` reader - UART_THR: Transmit Holding Register"]
pub type UartthrtxHoldingRegR = crate::FieldReader;
#[doc = "Field `UARTTHRTxHoldingReg` writer - UART_THR: Transmit Holding Register"]
pub type UartthrtxHoldingRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - UART_RBR: Receiving Buffer Register"]
    #[inline(always)]
    pub fn uartrbrreceiving_buffer_reg(&self) -> UartrbrreceivingBufferRegR {
        UartrbrreceivingBufferRegR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - UART_THR: Transmit Holding Register"]
    #[inline(always)]
    pub fn uartthrtx_holding_reg(&self) -> UartthrtxHoldingRegR {
        UartthrtxHoldingRegR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - UART_THR: Transmit Holding Register"]
    #[inline(always)]
    pub fn uartthrtx_holding_reg(&mut self) -> UartthrtxHoldingRegW<UartrbrSpec> {
        UartthrtxHoldingRegW::new(self, 0)
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
