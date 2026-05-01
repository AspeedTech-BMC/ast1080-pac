#[doc = "Register `UDMA038` reader"]
pub type R = crate::R<Udma038Spec>;
#[doc = "Register `UDMA038` writer"]
pub type W = crate::W<Udma038Spec>;
#[doc = "Field `UART0RXDMAINTEnbl` reader - UART0 RX DMA interrupt enable"]
pub type Uart0rxdmaintenblR = crate::BitReader;
#[doc = "Field `UART0RXDMAINTEnbl` writer - UART0 RX DMA interrupt enable"]
pub type Uart0rxdmaintenblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1RXDMAINTEnbl` reader - UART1 RX DMA interrupt enable"]
pub type Uart1rxdmaintenblR = crate::BitReader;
#[doc = "Field `UART1RXDMAINTEnbl` writer - UART1 RX DMA interrupt enable"]
pub type Uart1rxdmaintenblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART2RXDMAINTEnbl` reader - UART2 RX DMA interrupt enable"]
pub type Uart2rxdmaintenblR = crate::BitReader;
#[doc = "Field `UART2RXDMAINTEnbl` writer - UART2 RX DMA interrupt enable"]
pub type Uart2rxdmaintenblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART3RXDMAINTEnbl` reader - UART3 RX DMA interrupt enable"]
pub type Uart3rxdmaintenblR = crate::BitReader;
#[doc = "Field `UART3RXDMAINTEnbl` writer - UART3 RX DMA interrupt enable"]
pub type Uart3rxdmaintenblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART5RXDMAINTEnbl` reader - UART5 RX DMA interrupt enable"]
pub type Uart5rxdmaintenblR = crate::BitReader;
#[doc = "Field `UART5RXDMAINTEnbl` writer - UART5 RX DMA interrupt enable"]
pub type Uart5rxdmaintenblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART6RXDMAINTEnbl` reader - UART6 RX DMA interrupt enable"]
pub type Uart6rxdmaintenblR = crate::BitReader;
#[doc = "Field `UART6RXDMAINTEnbl` writer - UART6 RX DMA interrupt enable"]
pub type Uart6rxdmaintenblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART7RXDMAINTEnbl` reader - UART7 RX DMA interrupt enable"]
pub type Uart7rxdmaintenblR = crate::BitReader;
#[doc = "Field `UART7RXDMAINTEnbl` writer - UART7 RX DMA interrupt enable"]
pub type Uart7rxdmaintenblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART8RXDMAINTEnbl` reader - UART8 RX DMA interrupt enable"]
pub type Uart8rxdmaintenblR = crate::BitReader;
#[doc = "Field `UART8RXDMAINTEnbl` writer - UART8 RX DMA interrupt enable"]
pub type Uart8rxdmaintenblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART9RXDMAINTEnbl` reader - UART9 RX DMA interrupt enable"]
pub type Uart9rxdmaintenblR = crate::BitReader;
#[doc = "Field `UART9RXDMAINTEnbl` writer - UART9 RX DMA interrupt enable"]
pub type Uart9rxdmaintenblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART10RXDMAINTEnbl` reader - UART10 RX DMA interrupt enable"]
pub type Uart10rxdmaintenblR = crate::BitReader;
#[doc = "Field `UART10RXDMAINTEnbl` writer - UART10 RX DMA interrupt enable"]
pub type Uart10rxdmaintenblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART11RXDMAINTEnbl` reader - UART11 RX DMA interrupt enable"]
pub type Uart11rxdmaintenblR = crate::BitReader;
#[doc = "Field `UART11RXDMAINTEnbl` writer - UART11 RX DMA interrupt enable"]
pub type Uart11rxdmaintenblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UARTBMCRXDMAINTEnbl` reader - UART-BMC RX DMA interrupt enable"]
pub type UartbmcrxdmaintenblR = crate::BitReader;
#[doc = "Field `UARTBMCRXDMAINTEnbl` writer - UART-BMC RX DMA interrupt enable"]
pub type UartbmcrxdmaintenblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VUART0RXDMAINTEnbl` reader - VUART0 RX DMA interrupt enable"]
pub type Vuart0rxdmaintenblR = crate::BitReader;
#[doc = "Field `VUART0RXDMAINTEnbl` writer - VUART0 RX DMA interrupt enable"]
pub type Vuart0rxdmaintenblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VUART1RXDMAINTEnbl` reader - VUART1 RX DMA interrupt enable"]
pub type Vuart1rxdmaintenblR = crate::BitReader;
#[doc = "Field `VUART1RXDMAINTEnbl` writer - VUART1 RX DMA interrupt enable"]
pub type Vuart1rxdmaintenblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VUART2RXDMAINTEnbl` reader - VUART2 RX DMA interrupt enable"]
pub type Vuart2rxdmaintenblR = crate::BitReader;
#[doc = "Field `VUART2RXDMAINTEnbl` writer - VUART2 RX DMA interrupt enable"]
pub type Vuart2rxdmaintenblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VUART3RXDMAINTEnbl` reader - VUART3 RX DMA interrupt enable"]
pub type Vuart3rxdmaintenblR = crate::BitReader;
#[doc = "Field `VUART3RXDMAINTEnbl` writer - VUART3 RX DMA interrupt enable"]
pub type Vuart3rxdmaintenblW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - UART0 RX DMA interrupt enable"]
    #[inline(always)]
    pub fn uart0rxdmaintenbl(&self) -> Uart0rxdmaintenblR {
        Uart0rxdmaintenblR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART1 RX DMA interrupt enable"]
    #[inline(always)]
    pub fn uart1rxdmaintenbl(&self) -> Uart1rxdmaintenblR {
        Uart1rxdmaintenblR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART2 RX DMA interrupt enable"]
    #[inline(always)]
    pub fn uart2rxdmaintenbl(&self) -> Uart2rxdmaintenblR {
        Uart2rxdmaintenblR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART3 RX DMA interrupt enable"]
    #[inline(always)]
    pub fn uart3rxdmaintenbl(&self) -> Uart3rxdmaintenblR {
        Uart3rxdmaintenblR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - UART5 RX DMA interrupt enable"]
    #[inline(always)]
    pub fn uart5rxdmaintenbl(&self) -> Uart5rxdmaintenblR {
        Uart5rxdmaintenblR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART6 RX DMA interrupt enable"]
    #[inline(always)]
    pub fn uart6rxdmaintenbl(&self) -> Uart6rxdmaintenblR {
        Uart6rxdmaintenblR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UART7 RX DMA interrupt enable"]
    #[inline(always)]
    pub fn uart7rxdmaintenbl(&self) -> Uart7rxdmaintenblR {
        Uart7rxdmaintenblR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART8 RX DMA interrupt enable"]
    #[inline(always)]
    pub fn uart8rxdmaintenbl(&self) -> Uart8rxdmaintenblR {
        Uart8rxdmaintenblR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - UART9 RX DMA interrupt enable"]
    #[inline(always)]
    pub fn uart9rxdmaintenbl(&self) -> Uart9rxdmaintenblR {
        Uart9rxdmaintenblR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UART10 RX DMA interrupt enable"]
    #[inline(always)]
    pub fn uart10rxdmaintenbl(&self) -> Uart10rxdmaintenblR {
        Uart10rxdmaintenblR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - UART11 RX DMA interrupt enable"]
    #[inline(always)]
    pub fn uart11rxdmaintenbl(&self) -> Uart11rxdmaintenblR {
        Uart11rxdmaintenblR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - UART-BMC RX DMA interrupt enable"]
    #[inline(always)]
    pub fn uartbmcrxdmaintenbl(&self) -> UartbmcrxdmaintenblR {
        UartbmcrxdmaintenblR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - VUART0 RX DMA interrupt enable"]
    #[inline(always)]
    pub fn vuart0rxdmaintenbl(&self) -> Vuart0rxdmaintenblR {
        Vuart0rxdmaintenblR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - VUART1 RX DMA interrupt enable"]
    #[inline(always)]
    pub fn vuart1rxdmaintenbl(&self) -> Vuart1rxdmaintenblR {
        Vuart1rxdmaintenblR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - VUART2 RX DMA interrupt enable"]
    #[inline(always)]
    pub fn vuart2rxdmaintenbl(&self) -> Vuart2rxdmaintenblR {
        Vuart2rxdmaintenblR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - VUART3 RX DMA interrupt enable"]
    #[inline(always)]
    pub fn vuart3rxdmaintenbl(&self) -> Vuart3rxdmaintenblR {
        Vuart3rxdmaintenblR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART0 RX DMA interrupt enable"]
    #[inline(always)]
    pub fn uart0rxdmaintenbl(&mut self) -> Uart0rxdmaintenblW<Udma038Spec> {
        Uart0rxdmaintenblW::new(self, 0)
    }
    #[doc = "Bit 1 - UART1 RX DMA interrupt enable"]
    #[inline(always)]
    pub fn uart1rxdmaintenbl(&mut self) -> Uart1rxdmaintenblW<Udma038Spec> {
        Uart1rxdmaintenblW::new(self, 1)
    }
    #[doc = "Bit 2 - UART2 RX DMA interrupt enable"]
    #[inline(always)]
    pub fn uart2rxdmaintenbl(&mut self) -> Uart2rxdmaintenblW<Udma038Spec> {
        Uart2rxdmaintenblW::new(self, 2)
    }
    #[doc = "Bit 3 - UART3 RX DMA interrupt enable"]
    #[inline(always)]
    pub fn uart3rxdmaintenbl(&mut self) -> Uart3rxdmaintenblW<Udma038Spec> {
        Uart3rxdmaintenblW::new(self, 3)
    }
    #[doc = "Bit 4 - UART5 RX DMA interrupt enable"]
    #[inline(always)]
    pub fn uart5rxdmaintenbl(&mut self) -> Uart5rxdmaintenblW<Udma038Spec> {
        Uart5rxdmaintenblW::new(self, 4)
    }
    #[doc = "Bit 5 - UART6 RX DMA interrupt enable"]
    #[inline(always)]
    pub fn uart6rxdmaintenbl(&mut self) -> Uart6rxdmaintenblW<Udma038Spec> {
        Uart6rxdmaintenblW::new(self, 5)
    }
    #[doc = "Bit 6 - UART7 RX DMA interrupt enable"]
    #[inline(always)]
    pub fn uart7rxdmaintenbl(&mut self) -> Uart7rxdmaintenblW<Udma038Spec> {
        Uart7rxdmaintenblW::new(self, 6)
    }
    #[doc = "Bit 7 - UART8 RX DMA interrupt enable"]
    #[inline(always)]
    pub fn uart8rxdmaintenbl(&mut self) -> Uart8rxdmaintenblW<Udma038Spec> {
        Uart8rxdmaintenblW::new(self, 7)
    }
    #[doc = "Bit 8 - UART9 RX DMA interrupt enable"]
    #[inline(always)]
    pub fn uart9rxdmaintenbl(&mut self) -> Uart9rxdmaintenblW<Udma038Spec> {
        Uart9rxdmaintenblW::new(self, 8)
    }
    #[doc = "Bit 9 - UART10 RX DMA interrupt enable"]
    #[inline(always)]
    pub fn uart10rxdmaintenbl(&mut self) -> Uart10rxdmaintenblW<Udma038Spec> {
        Uart10rxdmaintenblW::new(self, 9)
    }
    #[doc = "Bit 10 - UART11 RX DMA interrupt enable"]
    #[inline(always)]
    pub fn uart11rxdmaintenbl(&mut self) -> Uart11rxdmaintenblW<Udma038Spec> {
        Uart11rxdmaintenblW::new(self, 10)
    }
    #[doc = "Bit 11 - UART-BMC RX DMA interrupt enable"]
    #[inline(always)]
    pub fn uartbmcrxdmaintenbl(&mut self) -> UartbmcrxdmaintenblW<Udma038Spec> {
        UartbmcrxdmaintenblW::new(self, 11)
    }
    #[doc = "Bit 12 - VUART0 RX DMA interrupt enable"]
    #[inline(always)]
    pub fn vuart0rxdmaintenbl(&mut self) -> Vuart0rxdmaintenblW<Udma038Spec> {
        Vuart0rxdmaintenblW::new(self, 12)
    }
    #[doc = "Bit 13 - VUART1 RX DMA interrupt enable"]
    #[inline(always)]
    pub fn vuart1rxdmaintenbl(&mut self) -> Vuart1rxdmaintenblW<Udma038Spec> {
        Vuart1rxdmaintenblW::new(self, 13)
    }
    #[doc = "Bit 14 - VUART2 RX DMA interrupt enable"]
    #[inline(always)]
    pub fn vuart2rxdmaintenbl(&mut self) -> Vuart2rxdmaintenblW<Udma038Spec> {
        Vuart2rxdmaintenblW::new(self, 14)
    }
    #[doc = "Bit 15 - VUART3 RX DMA interrupt enable"]
    #[inline(always)]
    pub fn vuart3rxdmaintenbl(&mut self) -> Vuart3rxdmaintenblW<Udma038Spec> {
        Vuart3rxdmaintenblW::new(self, 15)
    }
}
#[doc = "UART RX DMA interrrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`udma038::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma038::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma038Spec;
impl crate::RegisterSpec for Udma038Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma038::R`](R) reader structure"]
impl crate::Readable for Udma038Spec {}
#[doc = "`write(|w| ..)` method takes [`udma038::W`](W) writer structure"]
impl crate::Writable for Udma038Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA038 to value 0"]
impl crate::Resettable for Udma038Spec {}
