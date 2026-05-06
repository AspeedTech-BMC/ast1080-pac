#[doc = "Register `UARTDMA03C` reader"]
pub type R = crate::R<Uartdma03cSpec>;
#[doc = "Register `UARTDMA03C` writer"]
pub type W = crate::W<Uartdma03cSpec>;
#[doc = "Field `UART0RXDMAINTSts` reader - UART0 RX DMA interrupt status"]
pub type Uart0rxdmaintstsR = crate::BitReader;
#[doc = "Field `UART0RXDMAINTSts` writer - UART0 RX DMA interrupt status"]
pub type Uart0rxdmaintstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1RXDMAINTSts` reader - UART1 RX DMA interrupt status"]
pub type Uart1rxdmaintstsR = crate::BitReader;
#[doc = "Field `UART1RXDMAINTSts` writer - UART1 RX DMA interrupt status"]
pub type Uart1rxdmaintstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART2RXDMAINTSts` reader - UART2 RX DMA interrupt status"]
pub type Uart2rxdmaintstsR = crate::BitReader;
#[doc = "Field `UART2RXDMAINTSts` writer - UART2 RX DMA interrupt status"]
pub type Uart2rxdmaintstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART3RXDMAINTSts` reader - UART3 RX DMA interrupt status"]
pub type Uart3rxdmaintstsR = crate::BitReader;
#[doc = "Field `UART3RXDMAINTSts` writer - UART3 RX DMA interrupt status"]
pub type Uart3rxdmaintstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART5RXDMAINTSts` reader - UART5 RX DMA interrupt status"]
pub type Uart5rxdmaintstsR = crate::BitReader;
#[doc = "Field `UART5RXDMAINTSts` writer - UART5 RX DMA interrupt status"]
pub type Uart5rxdmaintstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART6RXDMAINTSts` reader - UART6 RX DMA interrupt status"]
pub type Uart6rxdmaintstsR = crate::BitReader;
#[doc = "Field `UART6RXDMAINTSts` writer - UART6 RX DMA interrupt status"]
pub type Uart6rxdmaintstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART7RXDMAINTSts` reader - UART7 RX DMA interrupt status"]
pub type Uart7rxdmaintstsR = crate::BitReader;
#[doc = "Field `UART7RXDMAINTSts` writer - UART7 RX DMA interrupt status"]
pub type Uart7rxdmaintstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART8RXDMAINTSts` reader - UART8 RX DMA interrupt status"]
pub type Uart8rxdmaintstsR = crate::BitReader;
#[doc = "Field `UART8RXDMAINTSts` writer - UART8 RX DMA interrupt status"]
pub type Uart8rxdmaintstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART9RXDMAINTSts` reader - UART9 RX DMA interrupt status"]
pub type Uart9rxdmaintstsR = crate::BitReader;
#[doc = "Field `UART9RXDMAINTSts` writer - UART9 RX DMA interrupt status"]
pub type Uart9rxdmaintstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART10RXDMAINTSts` reader - UART10 RX DMA interrupt status"]
pub type Uart10rxdmaintstsR = crate::BitReader;
#[doc = "Field `UART10RXDMAINTSts` writer - UART10 RX DMA interrupt status"]
pub type Uart10rxdmaintstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART11RXDMAINTSts` reader - UART11 RX DMA interrupt status"]
pub type Uart11rxdmaintstsR = crate::BitReader;
#[doc = "Field `UART11RXDMAINTSts` writer - UART11 RX DMA interrupt status"]
pub type Uart11rxdmaintstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UARTBMCRXDMAINTSts` reader - UART-BMC RX DMA interrupt status"]
pub type UartbmcrxdmaintstsR = crate::BitReader;
#[doc = "Field `UARTBMCRXDMAINTSts` writer - UART-BMC RX DMA interrupt status"]
pub type UartbmcrxdmaintstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VUART0RXDMAINTSts` reader - VUART0 RX DMA interrupt status"]
pub type Vuart0rxdmaintstsR = crate::BitReader;
#[doc = "Field `VUART0RXDMAINTSts` writer - VUART0 RX DMA interrupt status"]
pub type Vuart0rxdmaintstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VUART1RXDMAINTSts` reader - VUART1 RX DMA interrupt status"]
pub type Vuart1rxdmaintstsR = crate::BitReader;
#[doc = "Field `VUART1RXDMAINTSts` writer - VUART1 RX DMA interrupt status"]
pub type Vuart1rxdmaintstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VUART2RXDMAINTSts` reader - VUART2 RX DMA interrupt status"]
pub type Vuart2rxdmaintstsR = crate::BitReader;
#[doc = "Field `VUART2RXDMAINTSts` writer - VUART2 RX DMA interrupt status"]
pub type Vuart2rxdmaintstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VUART3RXDMAINTSts` reader - VUART3 RX DMA interrupt status"]
pub type Vuart3rxdmaintstsR = crate::BitReader;
#[doc = "Field `VUART3RXDMAINTSts` writer - VUART3 RX DMA interrupt status"]
pub type Vuart3rxdmaintstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - UART0 RX DMA interrupt status"]
    #[inline(always)]
    pub fn uart0rxdmaintsts(&self) -> Uart0rxdmaintstsR {
        Uart0rxdmaintstsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART1 RX DMA interrupt status"]
    #[inline(always)]
    pub fn uart1rxdmaintsts(&self) -> Uart1rxdmaintstsR {
        Uart1rxdmaintstsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART2 RX DMA interrupt status"]
    #[inline(always)]
    pub fn uart2rxdmaintsts(&self) -> Uart2rxdmaintstsR {
        Uart2rxdmaintstsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART3 RX DMA interrupt status"]
    #[inline(always)]
    pub fn uart3rxdmaintsts(&self) -> Uart3rxdmaintstsR {
        Uart3rxdmaintstsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - UART5 RX DMA interrupt status"]
    #[inline(always)]
    pub fn uart5rxdmaintsts(&self) -> Uart5rxdmaintstsR {
        Uart5rxdmaintstsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART6 RX DMA interrupt status"]
    #[inline(always)]
    pub fn uart6rxdmaintsts(&self) -> Uart6rxdmaintstsR {
        Uart6rxdmaintstsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UART7 RX DMA interrupt status"]
    #[inline(always)]
    pub fn uart7rxdmaintsts(&self) -> Uart7rxdmaintstsR {
        Uart7rxdmaintstsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART8 RX DMA interrupt status"]
    #[inline(always)]
    pub fn uart8rxdmaintsts(&self) -> Uart8rxdmaintstsR {
        Uart8rxdmaintstsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - UART9 RX DMA interrupt status"]
    #[inline(always)]
    pub fn uart9rxdmaintsts(&self) -> Uart9rxdmaintstsR {
        Uart9rxdmaintstsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UART10 RX DMA interrupt status"]
    #[inline(always)]
    pub fn uart10rxdmaintsts(&self) -> Uart10rxdmaintstsR {
        Uart10rxdmaintstsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - UART11 RX DMA interrupt status"]
    #[inline(always)]
    pub fn uart11rxdmaintsts(&self) -> Uart11rxdmaintstsR {
        Uart11rxdmaintstsR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - UART-BMC RX DMA interrupt status"]
    #[inline(always)]
    pub fn uartbmcrxdmaintsts(&self) -> UartbmcrxdmaintstsR {
        UartbmcrxdmaintstsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - VUART0 RX DMA interrupt status"]
    #[inline(always)]
    pub fn vuart0rxdmaintsts(&self) -> Vuart0rxdmaintstsR {
        Vuart0rxdmaintstsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - VUART1 RX DMA interrupt status"]
    #[inline(always)]
    pub fn vuart1rxdmaintsts(&self) -> Vuart1rxdmaintstsR {
        Vuart1rxdmaintstsR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - VUART2 RX DMA interrupt status"]
    #[inline(always)]
    pub fn vuart2rxdmaintsts(&self) -> Vuart2rxdmaintstsR {
        Vuart2rxdmaintstsR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - VUART3 RX DMA interrupt status"]
    #[inline(always)]
    pub fn vuart3rxdmaintsts(&self) -> Vuart3rxdmaintstsR {
        Vuart3rxdmaintstsR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - UART0 RX DMA interrupt status"]
    #[inline(always)]
    pub fn uart0rxdmaintsts(&mut self) -> Uart0rxdmaintstsW<Uartdma03cSpec> {
        Uart0rxdmaintstsW::new(self, 0)
    }
    #[doc = "Bit 1 - UART1 RX DMA interrupt status"]
    #[inline(always)]
    pub fn uart1rxdmaintsts(&mut self) -> Uart1rxdmaintstsW<Uartdma03cSpec> {
        Uart1rxdmaintstsW::new(self, 1)
    }
    #[doc = "Bit 2 - UART2 RX DMA interrupt status"]
    #[inline(always)]
    pub fn uart2rxdmaintsts(&mut self) -> Uart2rxdmaintstsW<Uartdma03cSpec> {
        Uart2rxdmaintstsW::new(self, 2)
    }
    #[doc = "Bit 3 - UART3 RX DMA interrupt status"]
    #[inline(always)]
    pub fn uart3rxdmaintsts(&mut self) -> Uart3rxdmaintstsW<Uartdma03cSpec> {
        Uart3rxdmaintstsW::new(self, 3)
    }
    #[doc = "Bit 4 - UART5 RX DMA interrupt status"]
    #[inline(always)]
    pub fn uart5rxdmaintsts(&mut self) -> Uart5rxdmaintstsW<Uartdma03cSpec> {
        Uart5rxdmaintstsW::new(self, 4)
    }
    #[doc = "Bit 5 - UART6 RX DMA interrupt status"]
    #[inline(always)]
    pub fn uart6rxdmaintsts(&mut self) -> Uart6rxdmaintstsW<Uartdma03cSpec> {
        Uart6rxdmaintstsW::new(self, 5)
    }
    #[doc = "Bit 6 - UART7 RX DMA interrupt status"]
    #[inline(always)]
    pub fn uart7rxdmaintsts(&mut self) -> Uart7rxdmaintstsW<Uartdma03cSpec> {
        Uart7rxdmaintstsW::new(self, 6)
    }
    #[doc = "Bit 7 - UART8 RX DMA interrupt status"]
    #[inline(always)]
    pub fn uart8rxdmaintsts(&mut self) -> Uart8rxdmaintstsW<Uartdma03cSpec> {
        Uart8rxdmaintstsW::new(self, 7)
    }
    #[doc = "Bit 8 - UART9 RX DMA interrupt status"]
    #[inline(always)]
    pub fn uart9rxdmaintsts(&mut self) -> Uart9rxdmaintstsW<Uartdma03cSpec> {
        Uart9rxdmaintstsW::new(self, 8)
    }
    #[doc = "Bit 9 - UART10 RX DMA interrupt status"]
    #[inline(always)]
    pub fn uart10rxdmaintsts(&mut self) -> Uart10rxdmaintstsW<Uartdma03cSpec> {
        Uart10rxdmaintstsW::new(self, 9)
    }
    #[doc = "Bit 10 - UART11 RX DMA interrupt status"]
    #[inline(always)]
    pub fn uart11rxdmaintsts(&mut self) -> Uart11rxdmaintstsW<Uartdma03cSpec> {
        Uart11rxdmaintstsW::new(self, 10)
    }
    #[doc = "Bit 11 - UART-BMC RX DMA interrupt status"]
    #[inline(always)]
    pub fn uartbmcrxdmaintsts(&mut self) -> UartbmcrxdmaintstsW<Uartdma03cSpec> {
        UartbmcrxdmaintstsW::new(self, 11)
    }
    #[doc = "Bit 12 - VUART0 RX DMA interrupt status"]
    #[inline(always)]
    pub fn vuart0rxdmaintsts(&mut self) -> Vuart0rxdmaintstsW<Uartdma03cSpec> {
        Vuart0rxdmaintstsW::new(self, 12)
    }
    #[doc = "Bit 13 - VUART1 RX DMA interrupt status"]
    #[inline(always)]
    pub fn vuart1rxdmaintsts(&mut self) -> Vuart1rxdmaintstsW<Uartdma03cSpec> {
        Vuart1rxdmaintstsW::new(self, 13)
    }
    #[doc = "Bit 14 - VUART2 RX DMA interrupt status"]
    #[inline(always)]
    pub fn vuart2rxdmaintsts(&mut self) -> Vuart2rxdmaintstsW<Uartdma03cSpec> {
        Vuart2rxdmaintstsW::new(self, 14)
    }
    #[doc = "Bit 15 - VUART3 RX DMA interrupt status"]
    #[inline(always)]
    pub fn vuart3rxdmaintsts(&mut self) -> Vuart3rxdmaintstsW<Uartdma03cSpec> {
        Vuart3rxdmaintstsW::new(self, 15)
    }
}
#[doc = "UART RX DMA interrrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma03c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma03c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma03cSpec;
impl crate::RegisterSpec for Uartdma03cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma03c::R`](R) reader structure"]
impl crate::Readable for Uartdma03cSpec {}
#[doc = "`write(|w| ..)` method takes [`uartdma03c::W`](W) writer structure"]
impl crate::Writable for Uartdma03cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA03C to value 0"]
impl crate::Resettable for Uartdma03cSpec {}
