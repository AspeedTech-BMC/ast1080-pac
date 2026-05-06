#[doc = "Register `UARTTHR` reader"]
pub type R = crate::R<UartthrSpec>;
#[doc = "Register `UARTTHR` writer"]
pub type W = crate::W<UartthrSpec>;
#[doc = "Field `UARTRBRReceivingBufferReg1` reader - UART_RBR: Receiving Buffer Register"]
pub type UartrbrreceivingBufferReg1R = crate::FieldReader;
#[doc = "Field `UARTTHRTxHoldingReg1` reader - UART_THR: Transmit Holding Register"]
pub type UartthrtxHoldingReg1R = crate::FieldReader;
#[doc = "Field `UARTTHRTxHoldingReg1` writer - UART_THR: Transmit Holding Register"]
pub type UartthrtxHoldingReg1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Reserved01` reader - Reserved (0)"]
pub type Reserved01R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - UART_RBR: Receiving Buffer Register"]
    #[inline(always)]
    pub fn uartrbrreceiving_buffer_reg1(&self) -> UartrbrreceivingBufferReg1R {
        UartrbrreceivingBufferReg1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - UART_THR: Transmit Holding Register"]
    #[inline(always)]
    pub fn uartthrtx_holding_reg1(&self) -> UartthrtxHoldingReg1R {
        UartthrtxHoldingReg1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - UART_THR: Transmit Holding Register"]
    #[inline(always)]
    pub fn uartthrtx_holding_reg1(&mut self) -> UartthrtxHoldingReg1W<UartthrSpec> {
        UartthrtxHoldingReg1W::new(self, 0)
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
