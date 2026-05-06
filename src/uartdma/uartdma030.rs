#[doc = "Register `UARTDMA030` reader"]
pub type R = crate::R<Uartdma030Spec>;
#[doc = "Register `UARTDMA030` writer"]
pub type W = crate::W<Uartdma030Spec>;
#[doc = "Field `UART0TXDMAINTEnbl` reader - UART0 TX DMA interrupt enable"]
pub type Uart0txdmaintenblR = crate::BitReader;
#[doc = "Field `UART0TXDMAINTEnbl` writer - UART0 TX DMA interrupt enable"]
pub type Uart0txdmaintenblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1TXDMAINTEnbl` reader - UART1 TX DMA interrupt enable"]
pub type Uart1txdmaintenblR = crate::BitReader;
#[doc = "Field `UART1TXDMAINTEnbl` writer - UART1 TX DMA interrupt enable"]
pub type Uart1txdmaintenblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART2TXDMAINTEnbl` reader - UART2 TX DMA interrupt enable"]
pub type Uart2txdmaintenblR = crate::BitReader;
#[doc = "Field `UART2TXDMAINTEnbl` writer - UART2 TX DMA interrupt enable"]
pub type Uart2txdmaintenblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART3TXDMAINTEnbl` reader - UART3 TX DMA interrupt enable"]
pub type Uart3txdmaintenblR = crate::BitReader;
#[doc = "Field `UART3TXDMAINTEnbl` writer - UART3 TX DMA interrupt enable"]
pub type Uart3txdmaintenblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART5TXDMAINTEnbl` reader - UART5 TX DMA interrupt enable"]
pub type Uart5txdmaintenblR = crate::BitReader;
#[doc = "Field `UART5TXDMAINTEnbl` writer - UART5 TX DMA interrupt enable"]
pub type Uart5txdmaintenblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART6TXDMAINTEnbl` reader - UART6 TX DMA interrupt enable"]
pub type Uart6txdmaintenblR = crate::BitReader;
#[doc = "Field `UART6TXDMAINTEnbl` writer - UART6 TX DMA interrupt enable"]
pub type Uart6txdmaintenblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART7TXDMAINTEnbl` reader - UART7 TX DMA interrupt enable"]
pub type Uart7txdmaintenblR = crate::BitReader;
#[doc = "Field `UART7TXDMAINTEnbl` writer - UART7 TX DMA interrupt enable"]
pub type Uart7txdmaintenblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART8TXDMAINTEnbl` reader - UART8 TX DMA interrupt enable"]
pub type Uart8txdmaintenblR = crate::BitReader;
#[doc = "Field `UART8TXDMAINTEnbl` writer - UART8 TX DMA interrupt enable"]
pub type Uart8txdmaintenblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART9TXDMAINTEnbl` reader - UART9 TX DMA interrupt enable"]
pub type Uart9txdmaintenblR = crate::BitReader;
#[doc = "Field `UART9TXDMAINTEnbl` writer - UART9 TX DMA interrupt enable"]
pub type Uart9txdmaintenblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART10TXDMAINTEnbl` reader - UART10 TX DMA interrupt enable"]
pub type Uart10txdmaintenblR = crate::BitReader;
#[doc = "Field `UART10TXDMAINTEnbl` writer - UART10 TX DMA interrupt enable"]
pub type Uart10txdmaintenblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART11TXDMAINTEnbl` reader - UART11 TX DMA interrupt enable"]
pub type Uart11txdmaintenblR = crate::BitReader;
#[doc = "Field `UART11TXDMAINTEnbl` writer - UART11 TX DMA interrupt enable"]
pub type Uart11txdmaintenblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UARTBMCTXDMAINTEnbl` reader - UART-BMC TX DMA interrupt enable"]
pub type UartbmctxdmaintenblR = crate::BitReader;
#[doc = "Field `UARTBMCTXDMAINTEnbl` writer - UART-BMC TX DMA interrupt enable"]
pub type UartbmctxdmaintenblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VUART0TXDMAINTEnbl` reader - VUART0 TX DMA interrupt enable"]
pub type Vuart0txdmaintenblR = crate::BitReader;
#[doc = "Field `VUART0TXDMAINTEnbl` writer - VUART0 TX DMA interrupt enable"]
pub type Vuart0txdmaintenblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VUART1TXDMAINTEnbl` reader - VUART1 TX DMA interrupt enable"]
pub type Vuart1txdmaintenblR = crate::BitReader;
#[doc = "Field `VUART1TXDMAINTEnbl` writer - VUART1 TX DMA interrupt enable"]
pub type Vuart1txdmaintenblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VUART2TXDMAINTEnbl` reader - VUART2 TX DMA interrupt enable"]
pub type Vuart2txdmaintenblR = crate::BitReader;
#[doc = "Field `VUART2TXDMAINTEnbl` writer - VUART2 TX DMA interrupt enable"]
pub type Vuart2txdmaintenblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VUART3TXDMAINTEnbl` reader - VUART3 TX DMA interrupt enable"]
pub type Vuart3txdmaintenblR = crate::BitReader;
#[doc = "Field `VUART3TXDMAINTEnbl` writer - VUART3 TX DMA interrupt enable"]
pub type Vuart3txdmaintenblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - UART0 TX DMA interrupt enable"]
    #[inline(always)]
    pub fn uart0txdmaintenbl(&self) -> Uart0txdmaintenblR {
        Uart0txdmaintenblR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART1 TX DMA interrupt enable"]
    #[inline(always)]
    pub fn uart1txdmaintenbl(&self) -> Uart1txdmaintenblR {
        Uart1txdmaintenblR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART2 TX DMA interrupt enable"]
    #[inline(always)]
    pub fn uart2txdmaintenbl(&self) -> Uart2txdmaintenblR {
        Uart2txdmaintenblR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART3 TX DMA interrupt enable"]
    #[inline(always)]
    pub fn uart3txdmaintenbl(&self) -> Uart3txdmaintenblR {
        Uart3txdmaintenblR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - UART5 TX DMA interrupt enable"]
    #[inline(always)]
    pub fn uart5txdmaintenbl(&self) -> Uart5txdmaintenblR {
        Uart5txdmaintenblR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART6 TX DMA interrupt enable"]
    #[inline(always)]
    pub fn uart6txdmaintenbl(&self) -> Uart6txdmaintenblR {
        Uart6txdmaintenblR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UART7 TX DMA interrupt enable"]
    #[inline(always)]
    pub fn uart7txdmaintenbl(&self) -> Uart7txdmaintenblR {
        Uart7txdmaintenblR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART8 TX DMA interrupt enable"]
    #[inline(always)]
    pub fn uart8txdmaintenbl(&self) -> Uart8txdmaintenblR {
        Uart8txdmaintenblR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - UART9 TX DMA interrupt enable"]
    #[inline(always)]
    pub fn uart9txdmaintenbl(&self) -> Uart9txdmaintenblR {
        Uart9txdmaintenblR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UART10 TX DMA interrupt enable"]
    #[inline(always)]
    pub fn uart10txdmaintenbl(&self) -> Uart10txdmaintenblR {
        Uart10txdmaintenblR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - UART11 TX DMA interrupt enable"]
    #[inline(always)]
    pub fn uart11txdmaintenbl(&self) -> Uart11txdmaintenblR {
        Uart11txdmaintenblR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - UART-BMC TX DMA interrupt enable"]
    #[inline(always)]
    pub fn uartbmctxdmaintenbl(&self) -> UartbmctxdmaintenblR {
        UartbmctxdmaintenblR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - VUART0 TX DMA interrupt enable"]
    #[inline(always)]
    pub fn vuart0txdmaintenbl(&self) -> Vuart0txdmaintenblR {
        Vuart0txdmaintenblR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - VUART1 TX DMA interrupt enable"]
    #[inline(always)]
    pub fn vuart1txdmaintenbl(&self) -> Vuart1txdmaintenblR {
        Vuart1txdmaintenblR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - VUART2 TX DMA interrupt enable"]
    #[inline(always)]
    pub fn vuart2txdmaintenbl(&self) -> Vuart2txdmaintenblR {
        Vuart2txdmaintenblR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - VUART3 TX DMA interrupt enable"]
    #[inline(always)]
    pub fn vuart3txdmaintenbl(&self) -> Vuart3txdmaintenblR {
        Vuart3txdmaintenblR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - UART0 TX DMA interrupt enable"]
    #[inline(always)]
    pub fn uart0txdmaintenbl(&mut self) -> Uart0txdmaintenblW<Uartdma030Spec> {
        Uart0txdmaintenblW::new(self, 0)
    }
    #[doc = "Bit 1 - UART1 TX DMA interrupt enable"]
    #[inline(always)]
    pub fn uart1txdmaintenbl(&mut self) -> Uart1txdmaintenblW<Uartdma030Spec> {
        Uart1txdmaintenblW::new(self, 1)
    }
    #[doc = "Bit 2 - UART2 TX DMA interrupt enable"]
    #[inline(always)]
    pub fn uart2txdmaintenbl(&mut self) -> Uart2txdmaintenblW<Uartdma030Spec> {
        Uart2txdmaintenblW::new(self, 2)
    }
    #[doc = "Bit 3 - UART3 TX DMA interrupt enable"]
    #[inline(always)]
    pub fn uart3txdmaintenbl(&mut self) -> Uart3txdmaintenblW<Uartdma030Spec> {
        Uart3txdmaintenblW::new(self, 3)
    }
    #[doc = "Bit 4 - UART5 TX DMA interrupt enable"]
    #[inline(always)]
    pub fn uart5txdmaintenbl(&mut self) -> Uart5txdmaintenblW<Uartdma030Spec> {
        Uart5txdmaintenblW::new(self, 4)
    }
    #[doc = "Bit 5 - UART6 TX DMA interrupt enable"]
    #[inline(always)]
    pub fn uart6txdmaintenbl(&mut self) -> Uart6txdmaintenblW<Uartdma030Spec> {
        Uart6txdmaintenblW::new(self, 5)
    }
    #[doc = "Bit 6 - UART7 TX DMA interrupt enable"]
    #[inline(always)]
    pub fn uart7txdmaintenbl(&mut self) -> Uart7txdmaintenblW<Uartdma030Spec> {
        Uart7txdmaintenblW::new(self, 6)
    }
    #[doc = "Bit 7 - UART8 TX DMA interrupt enable"]
    #[inline(always)]
    pub fn uart8txdmaintenbl(&mut self) -> Uart8txdmaintenblW<Uartdma030Spec> {
        Uart8txdmaintenblW::new(self, 7)
    }
    #[doc = "Bit 8 - UART9 TX DMA interrupt enable"]
    #[inline(always)]
    pub fn uart9txdmaintenbl(&mut self) -> Uart9txdmaintenblW<Uartdma030Spec> {
        Uart9txdmaintenblW::new(self, 8)
    }
    #[doc = "Bit 9 - UART10 TX DMA interrupt enable"]
    #[inline(always)]
    pub fn uart10txdmaintenbl(&mut self) -> Uart10txdmaintenblW<Uartdma030Spec> {
        Uart10txdmaintenblW::new(self, 9)
    }
    #[doc = "Bit 10 - UART11 TX DMA interrupt enable"]
    #[inline(always)]
    pub fn uart11txdmaintenbl(&mut self) -> Uart11txdmaintenblW<Uartdma030Spec> {
        Uart11txdmaintenblW::new(self, 10)
    }
    #[doc = "Bit 11 - UART-BMC TX DMA interrupt enable"]
    #[inline(always)]
    pub fn uartbmctxdmaintenbl(&mut self) -> UartbmctxdmaintenblW<Uartdma030Spec> {
        UartbmctxdmaintenblW::new(self, 11)
    }
    #[doc = "Bit 12 - VUART0 TX DMA interrupt enable"]
    #[inline(always)]
    pub fn vuart0txdmaintenbl(&mut self) -> Vuart0txdmaintenblW<Uartdma030Spec> {
        Vuart0txdmaintenblW::new(self, 12)
    }
    #[doc = "Bit 13 - VUART1 TX DMA interrupt enable"]
    #[inline(always)]
    pub fn vuart1txdmaintenbl(&mut self) -> Vuart1txdmaintenblW<Uartdma030Spec> {
        Vuart1txdmaintenblW::new(self, 13)
    }
    #[doc = "Bit 14 - VUART2 TX DMA interrupt enable"]
    #[inline(always)]
    pub fn vuart2txdmaintenbl(&mut self) -> Vuart2txdmaintenblW<Uartdma030Spec> {
        Vuart2txdmaintenblW::new(self, 14)
    }
    #[doc = "Bit 15 - VUART3 TX DMA interrupt enable"]
    #[inline(always)]
    pub fn vuart3txdmaintenbl(&mut self) -> Vuart3txdmaintenblW<Uartdma030Spec> {
        Vuart3txdmaintenblW::new(self, 15)
    }
}
#[doc = "UART TX DMA interrrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma030::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma030::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma030Spec;
impl crate::RegisterSpec for Uartdma030Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma030::R`](R) reader structure"]
impl crate::Readable for Uartdma030Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma030::W`](W) writer structure"]
impl crate::Writable for Uartdma030Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA030 to value 0"]
impl crate::Resettable for Uartdma030Spec {}
