#[doc = "Register `UDMA004` reader"]
pub type R = crate::R<Udma004Spec>;
#[doc = "Register `UDMA004` writer"]
pub type W = crate::W<Udma004Spec>;
#[doc = "Field `UART0RXDMAEnblReg` reader - UART0 RX DMA enable register"]
pub type Uart0rxdmaenblRegR = crate::BitReader;
#[doc = "Field `UART0RXDMAEnblReg` writer - UART0 RX DMA enable register"]
pub type Uart0rxdmaenblRegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1RXDMAEnblReg` reader - UART1 RX DMA enable register"]
pub type Uart1rxdmaenblRegR = crate::BitReader;
#[doc = "Field `UART1RXDMAEnblReg` writer - UART1 RX DMA enable register"]
pub type Uart1rxdmaenblRegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART2RXDMAEnblReg` reader - UART2 RX DMA enable register"]
pub type Uart2rxdmaenblRegR = crate::BitReader;
#[doc = "Field `UART2RXDMAEnblReg` writer - UART2 RX DMA enable register"]
pub type Uart2rxdmaenblRegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART3RXDMAEnblReg` reader - UART3 RX DMA enable register"]
pub type Uart3rxdmaenblRegR = crate::BitReader;
#[doc = "Field `UART3RXDMAEnblReg` writer - UART3 RX DMA enable register"]
pub type Uart3rxdmaenblRegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART5RXDMAEnblReg` reader - UART5 RX DMA enable register"]
pub type Uart5rxdmaenblRegR = crate::BitReader;
#[doc = "Field `UART5RXDMAEnblReg` writer - UART5 RX DMA enable register"]
pub type Uart5rxdmaenblRegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART6RXDMAEnblReg` reader - UART6 RX DMA enable register"]
pub type Uart6rxdmaenblRegR = crate::BitReader;
#[doc = "Field `UART6RXDMAEnblReg` writer - UART6 RX DMA enable register"]
pub type Uart6rxdmaenblRegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART7RXDMAEnblReg` reader - UART7 RX DMA enable register"]
pub type Uart7rxdmaenblRegR = crate::BitReader;
#[doc = "Field `UART7RXDMAEnblReg` writer - UART7 RX DMA enable register"]
pub type Uart7rxdmaenblRegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART8RXDMAEnblReg` reader - UART8 RX DMA enable register"]
pub type Uart8rxdmaenblRegR = crate::BitReader;
#[doc = "Field `UART8RXDMAEnblReg` writer - UART8 RX DMA enable register"]
pub type Uart8rxdmaenblRegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART9RXDMAEnblReg` reader - UART9 RX DMA enable register"]
pub type Uart9rxdmaenblRegR = crate::BitReader;
#[doc = "Field `UART9RXDMAEnblReg` writer - UART9 RX DMA enable register"]
pub type Uart9rxdmaenblRegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART10RXDMAEnblReg` reader - UART10 RX DMA enable register"]
pub type Uart10rxdmaenblRegR = crate::BitReader;
#[doc = "Field `UART10RXDMAEnblReg` writer - UART10 RX DMA enable register"]
pub type Uart10rxdmaenblRegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART11RXDMAEnblReg` reader - UART11 RX DMA enable register"]
pub type Uart11rxdmaenblRegR = crate::BitReader;
#[doc = "Field `UART11RXDMAEnblReg` writer - UART11 RX DMA enable register"]
pub type Uart11rxdmaenblRegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UARTBMCRXDMAEnblReg` reader - UART-BMC RX DMA enable register"]
pub type UartbmcrxdmaenblRegR = crate::BitReader;
#[doc = "Field `UARTBMCRXDMAEnblReg` writer - UART-BMC RX DMA enable register"]
pub type UartbmcrxdmaenblRegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VUART0RXDMAEnblReg` reader - VUART0 RX DMA enable register"]
pub type Vuart0rxdmaenblRegR = crate::BitReader;
#[doc = "Field `VUART0RXDMAEnblReg` writer - VUART0 RX DMA enable register"]
pub type Vuart0rxdmaenblRegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VUART1RXDMAEnblReg` reader - VUART1 RX DMA enable register"]
pub type Vuart1rxdmaenblRegR = crate::BitReader;
#[doc = "Field `VUART1RXDMAEnblReg` writer - VUART1 RX DMA enable register"]
pub type Vuart1rxdmaenblRegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VUART2RXDMAEnblReg` reader - VUART2 RX DMA enable register"]
pub type Vuart2rxdmaenblRegR = crate::BitReader;
#[doc = "Field `VUART2RXDMAEnblReg` writer - VUART2 RX DMA enable register"]
pub type Vuart2rxdmaenblRegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VUART3RXDMAEnblReg` reader - VUART3 RX DMA enable register"]
pub type Vuart3rxdmaenblRegR = crate::BitReader;
#[doc = "Field `VUART3RXDMAEnblReg` writer - VUART3 RX DMA enable register"]
pub type Vuart3rxdmaenblRegW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - UART0 RX DMA enable register"]
    #[inline(always)]
    pub fn uart0rxdmaenbl_reg(&self) -> Uart0rxdmaenblRegR {
        Uart0rxdmaenblRegR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART1 RX DMA enable register"]
    #[inline(always)]
    pub fn uart1rxdmaenbl_reg(&self) -> Uart1rxdmaenblRegR {
        Uart1rxdmaenblRegR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART2 RX DMA enable register"]
    #[inline(always)]
    pub fn uart2rxdmaenbl_reg(&self) -> Uart2rxdmaenblRegR {
        Uart2rxdmaenblRegR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART3 RX DMA enable register"]
    #[inline(always)]
    pub fn uart3rxdmaenbl_reg(&self) -> Uart3rxdmaenblRegR {
        Uart3rxdmaenblRegR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - UART5 RX DMA enable register"]
    #[inline(always)]
    pub fn uart5rxdmaenbl_reg(&self) -> Uart5rxdmaenblRegR {
        Uart5rxdmaenblRegR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART6 RX DMA enable register"]
    #[inline(always)]
    pub fn uart6rxdmaenbl_reg(&self) -> Uart6rxdmaenblRegR {
        Uart6rxdmaenblRegR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UART7 RX DMA enable register"]
    #[inline(always)]
    pub fn uart7rxdmaenbl_reg(&self) -> Uart7rxdmaenblRegR {
        Uart7rxdmaenblRegR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART8 RX DMA enable register"]
    #[inline(always)]
    pub fn uart8rxdmaenbl_reg(&self) -> Uart8rxdmaenblRegR {
        Uart8rxdmaenblRegR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - UART9 RX DMA enable register"]
    #[inline(always)]
    pub fn uart9rxdmaenbl_reg(&self) -> Uart9rxdmaenblRegR {
        Uart9rxdmaenblRegR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UART10 RX DMA enable register"]
    #[inline(always)]
    pub fn uart10rxdmaenbl_reg(&self) -> Uart10rxdmaenblRegR {
        Uart10rxdmaenblRegR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - UART11 RX DMA enable register"]
    #[inline(always)]
    pub fn uart11rxdmaenbl_reg(&self) -> Uart11rxdmaenblRegR {
        Uart11rxdmaenblRegR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - UART-BMC RX DMA enable register"]
    #[inline(always)]
    pub fn uartbmcrxdmaenbl_reg(&self) -> UartbmcrxdmaenblRegR {
        UartbmcrxdmaenblRegR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - VUART0 RX DMA enable register"]
    #[inline(always)]
    pub fn vuart0rxdmaenbl_reg(&self) -> Vuart0rxdmaenblRegR {
        Vuart0rxdmaenblRegR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - VUART1 RX DMA enable register"]
    #[inline(always)]
    pub fn vuart1rxdmaenbl_reg(&self) -> Vuart1rxdmaenblRegR {
        Vuart1rxdmaenblRegR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - VUART2 RX DMA enable register"]
    #[inline(always)]
    pub fn vuart2rxdmaenbl_reg(&self) -> Vuart2rxdmaenblRegR {
        Vuart2rxdmaenblRegR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - VUART3 RX DMA enable register"]
    #[inline(always)]
    pub fn vuart3rxdmaenbl_reg(&self) -> Vuart3rxdmaenblRegR {
        Vuart3rxdmaenblRegR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART0 RX DMA enable register"]
    #[inline(always)]
    pub fn uart0rxdmaenbl_reg(&mut self) -> Uart0rxdmaenblRegW<Udma004Spec> {
        Uart0rxdmaenblRegW::new(self, 0)
    }
    #[doc = "Bit 1 - UART1 RX DMA enable register"]
    #[inline(always)]
    pub fn uart1rxdmaenbl_reg(&mut self) -> Uart1rxdmaenblRegW<Udma004Spec> {
        Uart1rxdmaenblRegW::new(self, 1)
    }
    #[doc = "Bit 2 - UART2 RX DMA enable register"]
    #[inline(always)]
    pub fn uart2rxdmaenbl_reg(&mut self) -> Uart2rxdmaenblRegW<Udma004Spec> {
        Uart2rxdmaenblRegW::new(self, 2)
    }
    #[doc = "Bit 3 - UART3 RX DMA enable register"]
    #[inline(always)]
    pub fn uart3rxdmaenbl_reg(&mut self) -> Uart3rxdmaenblRegW<Udma004Spec> {
        Uart3rxdmaenblRegW::new(self, 3)
    }
    #[doc = "Bit 4 - UART5 RX DMA enable register"]
    #[inline(always)]
    pub fn uart5rxdmaenbl_reg(&mut self) -> Uart5rxdmaenblRegW<Udma004Spec> {
        Uart5rxdmaenblRegW::new(self, 4)
    }
    #[doc = "Bit 5 - UART6 RX DMA enable register"]
    #[inline(always)]
    pub fn uart6rxdmaenbl_reg(&mut self) -> Uart6rxdmaenblRegW<Udma004Spec> {
        Uart6rxdmaenblRegW::new(self, 5)
    }
    #[doc = "Bit 6 - UART7 RX DMA enable register"]
    #[inline(always)]
    pub fn uart7rxdmaenbl_reg(&mut self) -> Uart7rxdmaenblRegW<Udma004Spec> {
        Uart7rxdmaenblRegW::new(self, 6)
    }
    #[doc = "Bit 7 - UART8 RX DMA enable register"]
    #[inline(always)]
    pub fn uart8rxdmaenbl_reg(&mut self) -> Uart8rxdmaenblRegW<Udma004Spec> {
        Uart8rxdmaenblRegW::new(self, 7)
    }
    #[doc = "Bit 8 - UART9 RX DMA enable register"]
    #[inline(always)]
    pub fn uart9rxdmaenbl_reg(&mut self) -> Uart9rxdmaenblRegW<Udma004Spec> {
        Uart9rxdmaenblRegW::new(self, 8)
    }
    #[doc = "Bit 9 - UART10 RX DMA enable register"]
    #[inline(always)]
    pub fn uart10rxdmaenbl_reg(&mut self) -> Uart10rxdmaenblRegW<Udma004Spec> {
        Uart10rxdmaenblRegW::new(self, 9)
    }
    #[doc = "Bit 10 - UART11 RX DMA enable register"]
    #[inline(always)]
    pub fn uart11rxdmaenbl_reg(&mut self) -> Uart11rxdmaenblRegW<Udma004Spec> {
        Uart11rxdmaenblRegW::new(self, 10)
    }
    #[doc = "Bit 11 - UART-BMC RX DMA enable register"]
    #[inline(always)]
    pub fn uartbmcrxdmaenbl_reg(&mut self) -> UartbmcrxdmaenblRegW<Udma004Spec> {
        UartbmcrxdmaenblRegW::new(self, 11)
    }
    #[doc = "Bit 12 - VUART0 RX DMA enable register"]
    #[inline(always)]
    pub fn vuart0rxdmaenbl_reg(&mut self) -> Vuart0rxdmaenblRegW<Udma004Spec> {
        Vuart0rxdmaenblRegW::new(self, 12)
    }
    #[doc = "Bit 13 - VUART1 RX DMA enable register"]
    #[inline(always)]
    pub fn vuart1rxdmaenbl_reg(&mut self) -> Vuart1rxdmaenblRegW<Udma004Spec> {
        Vuart1rxdmaenblRegW::new(self, 13)
    }
    #[doc = "Bit 14 - VUART2 RX DMA enable register"]
    #[inline(always)]
    pub fn vuart2rxdmaenbl_reg(&mut self) -> Vuart2rxdmaenblRegW<Udma004Spec> {
        Vuart2rxdmaenblRegW::new(self, 14)
    }
    #[doc = "Bit 15 - VUART3 RX DMA enable register"]
    #[inline(always)]
    pub fn vuart3rxdmaenbl_reg(&mut self) -> Vuart3rxdmaenblRegW<Udma004Spec> {
        Vuart3rxdmaenblRegW::new(self, 15)
    }
}
#[doc = "UART RX DMA enable\n\nYou can [`read`](crate::Reg::read) this register and get [`udma004::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma004::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma004Spec;
impl crate::RegisterSpec for Udma004Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma004::R`](R) reader structure"]
impl crate::Readable for Udma004Spec {}
#[doc = "`write(|w| ..)` method takes [`udma004::W`](W) writer structure"]
impl crate::Writable for Udma004Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA004 to value 0"]
impl crate::Resettable for Udma004Spec {}
