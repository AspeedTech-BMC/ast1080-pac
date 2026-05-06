#[doc = "Register `UARTDMA020` reader"]
pub type R = crate::R<Uartdma020Spec>;
#[doc = "Register `UARTDMA020` writer"]
pub type W = crate::W<Uartdma020Spec>;
#[doc = "Field `UART0TXDMARst` reader - UART0 TX DMA reset"]
pub type Uart0txdmarstR = crate::BitReader;
#[doc = "Field `UART0TXDMARst` writer - UART0 TX DMA reset"]
pub type Uart0txdmarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1TXDMARst` reader - UART1 TX DMA reset"]
pub type Uart1txdmarstR = crate::BitReader;
#[doc = "Field `UART1TXDMARst` writer - UART1 TX DMA reset"]
pub type Uart1txdmarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART2TXDMARst` reader - UART2 TX DMA reset"]
pub type Uart2txdmarstR = crate::BitReader;
#[doc = "Field `UART2TXDMARst` writer - UART2 TX DMA reset"]
pub type Uart2txdmarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART3TXDMARst` reader - UART3 TX DMA reset"]
pub type Uart3txdmarstR = crate::BitReader;
#[doc = "Field `UART3TXDMARst` writer - UART3 TX DMA reset"]
pub type Uart3txdmarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART5TXDMARst` reader - UART5 TX DMA reset"]
pub type Uart5txdmarstR = crate::BitReader;
#[doc = "Field `UART5TXDMARst` writer - UART5 TX DMA reset"]
pub type Uart5txdmarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART6TXDMARst` reader - UART6 TX DMA reset"]
pub type Uart6txdmarstR = crate::BitReader;
#[doc = "Field `UART6TXDMARst` writer - UART6 TX DMA reset"]
pub type Uart6txdmarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART7TXDMARst` reader - UART7 TX DMA reset"]
pub type Uart7txdmarstR = crate::BitReader;
#[doc = "Field `UART7TXDMARst` writer - UART7 TX DMA reset"]
pub type Uart7txdmarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART8TXDMARst` reader - UART8 TX DMA reset"]
pub type Uart8txdmarstR = crate::BitReader;
#[doc = "Field `UART8TXDMARst` writer - UART8 TX DMA reset"]
pub type Uart8txdmarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART9TXDMARst` reader - UART9 TX DMA reset"]
pub type Uart9txdmarstR = crate::BitReader;
#[doc = "Field `UART9TXDMARst` writer - UART9 TX DMA reset"]
pub type Uart9txdmarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART10TXDMARst` reader - UART10 TX DMA reset"]
pub type Uart10txdmarstR = crate::BitReader;
#[doc = "Field `UART10TXDMARst` writer - UART10 TX DMA reset"]
pub type Uart10txdmarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART11TXDMARst` reader - UART11 TX DMA reset"]
pub type Uart11txdmarstR = crate::BitReader;
#[doc = "Field `UART11TXDMARst` writer - UART11 TX DMA reset"]
pub type Uart11txdmarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UARTBMCTXDMARst` reader - UART-BMC TX DMA reset"]
pub type UartbmctxdmarstR = crate::BitReader;
#[doc = "Field `UARTBMCTXDMARst` writer - UART-BMC TX DMA reset"]
pub type UartbmctxdmarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VUART0TXDMARst` reader - VUART0 TX DMA reset"]
pub type Vuart0txdmarstR = crate::BitReader;
#[doc = "Field `VUART0TXDMARst` writer - VUART0 TX DMA reset"]
pub type Vuart0txdmarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VUART1TXDMARst` reader - VUART1 TX DMA reset"]
pub type Vuart1txdmarstR = crate::BitReader;
#[doc = "Field `VUART1TXDMARst` writer - VUART1 TX DMA reset"]
pub type Vuart1txdmarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VUART2TXDMARst` reader - VUART2 TX DMA reset"]
pub type Vuart2txdmarstR = crate::BitReader;
#[doc = "Field `VUART2TXDMARst` writer - VUART2 TX DMA reset"]
pub type Vuart2txdmarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VUART3TXDMARst` reader - VUART3 TX DMA reset"]
pub type Vuart3txdmarstR = crate::BitReader;
#[doc = "Field `VUART3TXDMARst` writer - VUART3 TX DMA reset"]
pub type Vuart3txdmarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - UART0 TX DMA reset"]
    #[inline(always)]
    pub fn uart0txdmarst(&self) -> Uart0txdmarstR {
        Uart0txdmarstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART1 TX DMA reset"]
    #[inline(always)]
    pub fn uart1txdmarst(&self) -> Uart1txdmarstR {
        Uart1txdmarstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART2 TX DMA reset"]
    #[inline(always)]
    pub fn uart2txdmarst(&self) -> Uart2txdmarstR {
        Uart2txdmarstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART3 TX DMA reset"]
    #[inline(always)]
    pub fn uart3txdmarst(&self) -> Uart3txdmarstR {
        Uart3txdmarstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - UART5 TX DMA reset"]
    #[inline(always)]
    pub fn uart5txdmarst(&self) -> Uart5txdmarstR {
        Uart5txdmarstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART6 TX DMA reset"]
    #[inline(always)]
    pub fn uart6txdmarst(&self) -> Uart6txdmarstR {
        Uart6txdmarstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UART7 TX DMA reset"]
    #[inline(always)]
    pub fn uart7txdmarst(&self) -> Uart7txdmarstR {
        Uart7txdmarstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART8 TX DMA reset"]
    #[inline(always)]
    pub fn uart8txdmarst(&self) -> Uart8txdmarstR {
        Uart8txdmarstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - UART9 TX DMA reset"]
    #[inline(always)]
    pub fn uart9txdmarst(&self) -> Uart9txdmarstR {
        Uart9txdmarstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UART10 TX DMA reset"]
    #[inline(always)]
    pub fn uart10txdmarst(&self) -> Uart10txdmarstR {
        Uart10txdmarstR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - UART11 TX DMA reset"]
    #[inline(always)]
    pub fn uart11txdmarst(&self) -> Uart11txdmarstR {
        Uart11txdmarstR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - UART-BMC TX DMA reset"]
    #[inline(always)]
    pub fn uartbmctxdmarst(&self) -> UartbmctxdmarstR {
        UartbmctxdmarstR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - VUART0 TX DMA reset"]
    #[inline(always)]
    pub fn vuart0txdmarst(&self) -> Vuart0txdmarstR {
        Vuart0txdmarstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - VUART1 TX DMA reset"]
    #[inline(always)]
    pub fn vuart1txdmarst(&self) -> Vuart1txdmarstR {
        Vuart1txdmarstR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - VUART2 TX DMA reset"]
    #[inline(always)]
    pub fn vuart2txdmarst(&self) -> Vuart2txdmarstR {
        Vuart2txdmarstR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - VUART3 TX DMA reset"]
    #[inline(always)]
    pub fn vuart3txdmarst(&self) -> Vuart3txdmarstR {
        Vuart3txdmarstR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - UART0 TX DMA reset"]
    #[inline(always)]
    pub fn uart0txdmarst(&mut self) -> Uart0txdmarstW<Uartdma020Spec> {
        Uart0txdmarstW::new(self, 0)
    }
    #[doc = "Bit 1 - UART1 TX DMA reset"]
    #[inline(always)]
    pub fn uart1txdmarst(&mut self) -> Uart1txdmarstW<Uartdma020Spec> {
        Uart1txdmarstW::new(self, 1)
    }
    #[doc = "Bit 2 - UART2 TX DMA reset"]
    #[inline(always)]
    pub fn uart2txdmarst(&mut self) -> Uart2txdmarstW<Uartdma020Spec> {
        Uart2txdmarstW::new(self, 2)
    }
    #[doc = "Bit 3 - UART3 TX DMA reset"]
    #[inline(always)]
    pub fn uart3txdmarst(&mut self) -> Uart3txdmarstW<Uartdma020Spec> {
        Uart3txdmarstW::new(self, 3)
    }
    #[doc = "Bit 4 - UART5 TX DMA reset"]
    #[inline(always)]
    pub fn uart5txdmarst(&mut self) -> Uart5txdmarstW<Uartdma020Spec> {
        Uart5txdmarstW::new(self, 4)
    }
    #[doc = "Bit 5 - UART6 TX DMA reset"]
    #[inline(always)]
    pub fn uart6txdmarst(&mut self) -> Uart6txdmarstW<Uartdma020Spec> {
        Uart6txdmarstW::new(self, 5)
    }
    #[doc = "Bit 6 - UART7 TX DMA reset"]
    #[inline(always)]
    pub fn uart7txdmarst(&mut self) -> Uart7txdmarstW<Uartdma020Spec> {
        Uart7txdmarstW::new(self, 6)
    }
    #[doc = "Bit 7 - UART8 TX DMA reset"]
    #[inline(always)]
    pub fn uart8txdmarst(&mut self) -> Uart8txdmarstW<Uartdma020Spec> {
        Uart8txdmarstW::new(self, 7)
    }
    #[doc = "Bit 8 - UART9 TX DMA reset"]
    #[inline(always)]
    pub fn uart9txdmarst(&mut self) -> Uart9txdmarstW<Uartdma020Spec> {
        Uart9txdmarstW::new(self, 8)
    }
    #[doc = "Bit 9 - UART10 TX DMA reset"]
    #[inline(always)]
    pub fn uart10txdmarst(&mut self) -> Uart10txdmarstW<Uartdma020Spec> {
        Uart10txdmarstW::new(self, 9)
    }
    #[doc = "Bit 10 - UART11 TX DMA reset"]
    #[inline(always)]
    pub fn uart11txdmarst(&mut self) -> Uart11txdmarstW<Uartdma020Spec> {
        Uart11txdmarstW::new(self, 10)
    }
    #[doc = "Bit 11 - UART-BMC TX DMA reset"]
    #[inline(always)]
    pub fn uartbmctxdmarst(&mut self) -> UartbmctxdmarstW<Uartdma020Spec> {
        UartbmctxdmarstW::new(self, 11)
    }
    #[doc = "Bit 12 - VUART0 TX DMA reset"]
    #[inline(always)]
    pub fn vuart0txdmarst(&mut self) -> Vuart0txdmarstW<Uartdma020Spec> {
        Vuart0txdmarstW::new(self, 12)
    }
    #[doc = "Bit 13 - VUART1 TX DMA reset"]
    #[inline(always)]
    pub fn vuart1txdmarst(&mut self) -> Vuart1txdmarstW<Uartdma020Spec> {
        Vuart1txdmarstW::new(self, 13)
    }
    #[doc = "Bit 14 - VUART2 TX DMA reset"]
    #[inline(always)]
    pub fn vuart2txdmarst(&mut self) -> Vuart2txdmarstW<Uartdma020Spec> {
        Vuart2txdmarstW::new(self, 14)
    }
    #[doc = "Bit 15 - VUART3 TX DMA reset"]
    #[inline(always)]
    pub fn vuart3txdmarst(&mut self) -> Vuart3txdmarstW<Uartdma020Spec> {
        Vuart3txdmarstW::new(self, 15)
    }
}
#[doc = "UART TX DMA reset\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma020::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma020::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma020Spec;
impl crate::RegisterSpec for Uartdma020Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma020::R`](R) reader structure"]
impl crate::Readable for Uartdma020Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma020::W`](W) writer structure"]
impl crate::Writable for Uartdma020Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA020 to value 0"]
impl crate::Resettable for Uartdma020Spec {}
