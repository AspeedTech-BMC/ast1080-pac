#[doc = "Register `UARTFCR` reader"]
pub type R = crate::R<UartfcrSpec>;
#[doc = "Register `UARTFCR` writer"]
pub type W = crate::W<UartfcrSpec>;
#[doc = "Field `EnblUARTFIFO` reader - Enable UART FIFO"]
pub type EnblUartfifoR = crate::BitReader;
#[doc = "Field `EnblUARTFIFO` writer - Enable UART FIFO"]
pub type EnblUartfifoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RxFIFORst` reader - Receive FIFO Reset"]
pub type RxFiforstR = crate::BitReader;
#[doc = "Field `RxFIFORst` writer - Receive FIFO Reset"]
pub type RxFiforstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TxFIFORst` reader - Transmit FIFO Reset"]
pub type TxFiforstR = crate::BitReader;
#[doc = "Field `TxFIFORst` writer - Transmit FIFO Reset"]
pub type TxFiforstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved (0)"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `DefineTheRxrFIFOINTTriggerLevel` reader - Define the Receiver FIFO Interrupt trigger level."]
pub type DefineTheRxrFifointtriggerLevelR = crate::FieldReader;
#[doc = "Field `DefineTheRxrFIFOINTTriggerLevel` writer - Define the Receiver FIFO Interrupt trigger level."]
pub type DefineTheRxrFifointtriggerLevelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Enable UART FIFO"]
    #[inline(always)]
    pub fn enbl_uartfifo(&self) -> EnblUartfifoR {
        EnblUartfifoR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive FIFO Reset"]
    #[inline(always)]
    pub fn rx_fiforst(&self) -> RxFiforstR {
        RxFiforstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit FIFO Reset"]
    #[inline(always)]
    pub fn tx_fiforst(&self) -> TxFiforstR {
        TxFiforstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - Define the Receiver FIFO Interrupt trigger level."]
    #[inline(always)]
    pub fn define_the_rxr_fifointtrigger_level(&self) -> DefineTheRxrFifointtriggerLevelR {
        DefineTheRxrFifointtriggerLevelR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable UART FIFO"]
    #[inline(always)]
    pub fn enbl_uartfifo(&mut self) -> EnblUartfifoW<UartfcrSpec> {
        EnblUartfifoW::new(self, 0)
    }
    #[doc = "Bit 1 - Receive FIFO Reset"]
    #[inline(always)]
    pub fn rx_fiforst(&mut self) -> RxFiforstW<UartfcrSpec> {
        RxFiforstW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit FIFO Reset"]
    #[inline(always)]
    pub fn tx_fiforst(&mut self) -> TxFiforstW<UartfcrSpec> {
        TxFiforstW::new(self, 2)
    }
    #[doc = "Bits 6:7 - Define the Receiver FIFO Interrupt trigger level."]
    #[inline(always)]
    pub fn define_the_rxr_fifointtrigger_level(
        &mut self,
    ) -> DefineTheRxrFifointtriggerLevelW<UartfcrSpec> {
        DefineTheRxrFifointtriggerLevelW::new(self, 6)
    }
}
#[doc = "FIFO Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartfcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartfcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartfcrSpec;
impl crate::RegisterSpec for UartfcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartfcr::R`](R) reader structure"]
impl crate::Readable for UartfcrSpec {}
#[doc = "`write(|w| ..)` method takes [`uartfcr::W`](W) writer structure"]
impl crate::Writable for UartfcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTFCR to value 0"]
impl crate::Resettable for UartfcrSpec {}
