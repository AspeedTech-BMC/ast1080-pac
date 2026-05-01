#[doc = "Register `UDMA024` reader"]
pub type R = crate::R<Udma024Spec>;
#[doc = "Register `UDMA024` writer"]
pub type W = crate::W<Udma024Spec>;
#[doc = "Field `UART0RXDMARst` reader - UART0 RX DMA reset"]
pub type Uart0rxdmarstR = crate::BitReader;
#[doc = "Field `UART0RXDMARst` writer - UART0 RX DMA reset"]
pub type Uart0rxdmarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1RXDMARst` reader - UART1 RX DMA reset"]
pub type Uart1rxdmarstR = crate::BitReader;
#[doc = "Field `UART1RXDMARst` writer - UART1 RX DMA reset"]
pub type Uart1rxdmarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART2RXDMARst` reader - UART2 RX DMA reset"]
pub type Uart2rxdmarstR = crate::BitReader;
#[doc = "Field `UART2RXDMARst` writer - UART2 RX DMA reset"]
pub type Uart2rxdmarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART3RXDMARst` reader - UART3 RX DMA reset"]
pub type Uart3rxdmarstR = crate::BitReader;
#[doc = "Field `UART3RXDMARst` writer - UART3 RX DMA reset"]
pub type Uart3rxdmarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART5RXDMARst` reader - UART5 RX DMA reset"]
pub type Uart5rxdmarstR = crate::BitReader;
#[doc = "Field `UART5RXDMARst` writer - UART5 RX DMA reset"]
pub type Uart5rxdmarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART6RXDMARst` reader - UART6 RX DMA reset"]
pub type Uart6rxdmarstR = crate::BitReader;
#[doc = "Field `UART6RXDMARst` writer - UART6 RX DMA reset"]
pub type Uart6rxdmarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART7RXDMARst` reader - UART7 RX DMA reset"]
pub type Uart7rxdmarstR = crate::BitReader;
#[doc = "Field `UART7RXDMARst` writer - UART7 RX DMA reset"]
pub type Uart7rxdmarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART8RXDMARst` reader - UART8 RX DMA reset"]
pub type Uart8rxdmarstR = crate::BitReader;
#[doc = "Field `UART8RXDMARst` writer - UART8 RX DMA reset"]
pub type Uart8rxdmarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART9RXDMARst` reader - UART9 RX DMA reset"]
pub type Uart9rxdmarstR = crate::BitReader;
#[doc = "Field `UART9RXDMARst` writer - UART9 RX DMA reset"]
pub type Uart9rxdmarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART10RXDMARst` reader - UART10 RX DMA reset"]
pub type Uart10rxdmarstR = crate::BitReader;
#[doc = "Field `UART10RXDMARst` writer - UART10 RX DMA reset"]
pub type Uart10rxdmarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART11RXDMARst` reader - UART11 RX DMA reset"]
pub type Uart11rxdmarstR = crate::BitReader;
#[doc = "Field `UART11RXDMARst` writer - UART11 RX DMA reset"]
pub type Uart11rxdmarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UARTBMCRXDMARst` reader - UART-BMC RX DMA reset"]
pub type UartbmcrxdmarstR = crate::BitReader;
#[doc = "Field `UARTBMCRXDMARst` writer - UART-BMC RX DMA reset"]
pub type UartbmcrxdmarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VUART0RXDMARst` reader - VUART0 RX DMA reset"]
pub type Vuart0rxdmarstR = crate::BitReader;
#[doc = "Field `VUART0RXDMARst` writer - VUART0 RX DMA reset"]
pub type Vuart0rxdmarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VUART1RXDMARst` reader - VUART1 RX DMA reset"]
pub type Vuart1rxdmarstR = crate::BitReader;
#[doc = "Field `VUART1RXDMARst` writer - VUART1 RX DMA reset"]
pub type Vuart1rxdmarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VUART2RXDMARst` reader - VUART2 RX DMA reset"]
pub type Vuart2rxdmarstR = crate::BitReader;
#[doc = "Field `VUART2RXDMARst` writer - VUART2 RX DMA reset"]
pub type Vuart2rxdmarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VUART3RXDMARst` reader - VUART3 RX DMA reset"]
pub type Vuart3rxdmarstR = crate::BitReader;
#[doc = "Field `VUART3RXDMARst` writer - VUART3 RX DMA reset"]
pub type Vuart3rxdmarstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - UART0 RX DMA reset"]
    #[inline(always)]
    pub fn uart0rxdmarst(&self) -> Uart0rxdmarstR {
        Uart0rxdmarstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART1 RX DMA reset"]
    #[inline(always)]
    pub fn uart1rxdmarst(&self) -> Uart1rxdmarstR {
        Uart1rxdmarstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART2 RX DMA reset"]
    #[inline(always)]
    pub fn uart2rxdmarst(&self) -> Uart2rxdmarstR {
        Uart2rxdmarstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART3 RX DMA reset"]
    #[inline(always)]
    pub fn uart3rxdmarst(&self) -> Uart3rxdmarstR {
        Uart3rxdmarstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - UART5 RX DMA reset"]
    #[inline(always)]
    pub fn uart5rxdmarst(&self) -> Uart5rxdmarstR {
        Uart5rxdmarstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART6 RX DMA reset"]
    #[inline(always)]
    pub fn uart6rxdmarst(&self) -> Uart6rxdmarstR {
        Uart6rxdmarstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UART7 RX DMA reset"]
    #[inline(always)]
    pub fn uart7rxdmarst(&self) -> Uart7rxdmarstR {
        Uart7rxdmarstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART8 RX DMA reset"]
    #[inline(always)]
    pub fn uart8rxdmarst(&self) -> Uart8rxdmarstR {
        Uart8rxdmarstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - UART9 RX DMA reset"]
    #[inline(always)]
    pub fn uart9rxdmarst(&self) -> Uart9rxdmarstR {
        Uart9rxdmarstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UART10 RX DMA reset"]
    #[inline(always)]
    pub fn uart10rxdmarst(&self) -> Uart10rxdmarstR {
        Uart10rxdmarstR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - UART11 RX DMA reset"]
    #[inline(always)]
    pub fn uart11rxdmarst(&self) -> Uart11rxdmarstR {
        Uart11rxdmarstR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - UART-BMC RX DMA reset"]
    #[inline(always)]
    pub fn uartbmcrxdmarst(&self) -> UartbmcrxdmarstR {
        UartbmcrxdmarstR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - VUART0 RX DMA reset"]
    #[inline(always)]
    pub fn vuart0rxdmarst(&self) -> Vuart0rxdmarstR {
        Vuart0rxdmarstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - VUART1 RX DMA reset"]
    #[inline(always)]
    pub fn vuart1rxdmarst(&self) -> Vuart1rxdmarstR {
        Vuart1rxdmarstR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - VUART2 RX DMA reset"]
    #[inline(always)]
    pub fn vuart2rxdmarst(&self) -> Vuart2rxdmarstR {
        Vuart2rxdmarstR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - VUART3 RX DMA reset"]
    #[inline(always)]
    pub fn vuart3rxdmarst(&self) -> Vuart3rxdmarstR {
        Vuart3rxdmarstR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART0 RX DMA reset"]
    #[inline(always)]
    pub fn uart0rxdmarst(&mut self) -> Uart0rxdmarstW<Udma024Spec> {
        Uart0rxdmarstW::new(self, 0)
    }
    #[doc = "Bit 1 - UART1 RX DMA reset"]
    #[inline(always)]
    pub fn uart1rxdmarst(&mut self) -> Uart1rxdmarstW<Udma024Spec> {
        Uart1rxdmarstW::new(self, 1)
    }
    #[doc = "Bit 2 - UART2 RX DMA reset"]
    #[inline(always)]
    pub fn uart2rxdmarst(&mut self) -> Uart2rxdmarstW<Udma024Spec> {
        Uart2rxdmarstW::new(self, 2)
    }
    #[doc = "Bit 3 - UART3 RX DMA reset"]
    #[inline(always)]
    pub fn uart3rxdmarst(&mut self) -> Uart3rxdmarstW<Udma024Spec> {
        Uart3rxdmarstW::new(self, 3)
    }
    #[doc = "Bit 4 - UART5 RX DMA reset"]
    #[inline(always)]
    pub fn uart5rxdmarst(&mut self) -> Uart5rxdmarstW<Udma024Spec> {
        Uart5rxdmarstW::new(self, 4)
    }
    #[doc = "Bit 5 - UART6 RX DMA reset"]
    #[inline(always)]
    pub fn uart6rxdmarst(&mut self) -> Uart6rxdmarstW<Udma024Spec> {
        Uart6rxdmarstW::new(self, 5)
    }
    #[doc = "Bit 6 - UART7 RX DMA reset"]
    #[inline(always)]
    pub fn uart7rxdmarst(&mut self) -> Uart7rxdmarstW<Udma024Spec> {
        Uart7rxdmarstW::new(self, 6)
    }
    #[doc = "Bit 7 - UART8 RX DMA reset"]
    #[inline(always)]
    pub fn uart8rxdmarst(&mut self) -> Uart8rxdmarstW<Udma024Spec> {
        Uart8rxdmarstW::new(self, 7)
    }
    #[doc = "Bit 8 - UART9 RX DMA reset"]
    #[inline(always)]
    pub fn uart9rxdmarst(&mut self) -> Uart9rxdmarstW<Udma024Spec> {
        Uart9rxdmarstW::new(self, 8)
    }
    #[doc = "Bit 9 - UART10 RX DMA reset"]
    #[inline(always)]
    pub fn uart10rxdmarst(&mut self) -> Uart10rxdmarstW<Udma024Spec> {
        Uart10rxdmarstW::new(self, 9)
    }
    #[doc = "Bit 10 - UART11 RX DMA reset"]
    #[inline(always)]
    pub fn uart11rxdmarst(&mut self) -> Uart11rxdmarstW<Udma024Spec> {
        Uart11rxdmarstW::new(self, 10)
    }
    #[doc = "Bit 11 - UART-BMC RX DMA reset"]
    #[inline(always)]
    pub fn uartbmcrxdmarst(&mut self) -> UartbmcrxdmarstW<Udma024Spec> {
        UartbmcrxdmarstW::new(self, 11)
    }
    #[doc = "Bit 12 - VUART0 RX DMA reset"]
    #[inline(always)]
    pub fn vuart0rxdmarst(&mut self) -> Vuart0rxdmarstW<Udma024Spec> {
        Vuart0rxdmarstW::new(self, 12)
    }
    #[doc = "Bit 13 - VUART1 RX DMA reset"]
    #[inline(always)]
    pub fn vuart1rxdmarst(&mut self) -> Vuart1rxdmarstW<Udma024Spec> {
        Vuart1rxdmarstW::new(self, 13)
    }
    #[doc = "Bit 14 - VUART2 RX DMA reset"]
    #[inline(always)]
    pub fn vuart2rxdmarst(&mut self) -> Vuart2rxdmarstW<Udma024Spec> {
        Vuart2rxdmarstW::new(self, 14)
    }
    #[doc = "Bit 15 - VUART3 RX DMA reset"]
    #[inline(always)]
    pub fn vuart3rxdmarst(&mut self) -> Vuart3rxdmarstW<Udma024Spec> {
        Vuart3rxdmarstW::new(self, 15)
    }
}
#[doc = "UART RX DMA reset\n\nYou can [`read`](crate::Reg::read) this register and get [`udma024::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma024::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma024Spec;
impl crate::RegisterSpec for Udma024Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma024::R`](R) reader structure"]
impl crate::Readable for Udma024Spec {}
#[doc = "`write(|w| ..)` method takes [`udma024::W`](W) writer structure"]
impl crate::Writable for Udma024Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA024 to value 0"]
impl crate::Resettable for Udma024Spec {}
