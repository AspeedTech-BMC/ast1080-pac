#[doc = "Register `UARTDMA000` reader"]
pub type R = crate::R<Uartdma000Spec>;
#[doc = "Register `UARTDMA000` writer"]
pub type W = crate::W<Uartdma000Spec>;
#[doc = "Field `UART0TXDMAEnblReg` reader - UART0 TX DMA enable register"]
pub type Uart0txdmaenblRegR = crate::BitReader;
#[doc = "Field `UART0TXDMAEnblReg` writer - UART0 TX DMA enable register"]
pub type Uart0txdmaenblRegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1TXDMAEnblReg` reader - UART1 TX DMA enable register"]
pub type Uart1txdmaenblRegR = crate::BitReader;
#[doc = "Field `UART1TXDMAEnblReg` writer - UART1 TX DMA enable register"]
pub type Uart1txdmaenblRegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART2TXDMAEnblReg` reader - UART2 TX DMA enable register"]
pub type Uart2txdmaenblRegR = crate::BitReader;
#[doc = "Field `UART2TXDMAEnblReg` writer - UART2 TX DMA enable register"]
pub type Uart2txdmaenblRegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART3TXDMAEnblReg` reader - UART3 TX DMA enable register"]
pub type Uart3txdmaenblRegR = crate::BitReader;
#[doc = "Field `UART3TXDMAEnblReg` writer - UART3 TX DMA enable register"]
pub type Uart3txdmaenblRegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART5TXDMAEnblReg` reader - UART5 TX DMA enable register"]
pub type Uart5txdmaenblRegR = crate::BitReader;
#[doc = "Field `UART5TXDMAEnblReg` writer - UART5 TX DMA enable register"]
pub type Uart5txdmaenblRegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART6TXDMAEnblReg` reader - UART6 TX DMA enable register"]
pub type Uart6txdmaenblRegR = crate::BitReader;
#[doc = "Field `UART6TXDMAEnblReg` writer - UART6 TX DMA enable register"]
pub type Uart6txdmaenblRegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART7TXDMAEnblReg` reader - UART7 TX DMA enable register"]
pub type Uart7txdmaenblRegR = crate::BitReader;
#[doc = "Field `UART7TXDMAEnblReg` writer - UART7 TX DMA enable register"]
pub type Uart7txdmaenblRegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART8TXDMAEnblReg` reader - UART8 TX DMA enable register"]
pub type Uart8txdmaenblRegR = crate::BitReader;
#[doc = "Field `UART8TXDMAEnblReg` writer - UART8 TX DMA enable register"]
pub type Uart8txdmaenblRegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART9TXDMAEnblReg` reader - UART9 TX DMA enable register"]
pub type Uart9txdmaenblRegR = crate::BitReader;
#[doc = "Field `UART9TXDMAEnblReg` writer - UART9 TX DMA enable register"]
pub type Uart9txdmaenblRegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART10TXDMAEnblReg` reader - UART10 TX DMA enable register"]
pub type Uart10txdmaenblRegR = crate::BitReader;
#[doc = "Field `UART10TXDMAEnblReg` writer - UART10 TX DMA enable register"]
pub type Uart10txdmaenblRegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART11TXDMAEnblReg` reader - UART11 TX DMA enable register"]
pub type Uart11txdmaenblRegR = crate::BitReader;
#[doc = "Field `UART11TXDMAEnblReg` writer - UART11 TX DMA enable register"]
pub type Uart11txdmaenblRegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UARTBMCTXDMAEnblReg` reader - UART-BMC TX DMA enable register"]
pub type UartbmctxdmaenblRegR = crate::BitReader;
#[doc = "Field `UARTBMCTXDMAEnblReg` writer - UART-BMC TX DMA enable register"]
pub type UartbmctxdmaenblRegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VUART0TXDMAEnblReg` reader - VUART0 TX DMA enable register"]
pub type Vuart0txdmaenblRegR = crate::BitReader;
#[doc = "Field `VUART0TXDMAEnblReg` writer - VUART0 TX DMA enable register"]
pub type Vuart0txdmaenblRegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VUART1TXDMAEnblReg` reader - VUART1 TX DMA enable register"]
pub type Vuart1txdmaenblRegR = crate::BitReader;
#[doc = "Field `VUART1TXDMAEnblReg` writer - VUART1 TX DMA enable register"]
pub type Vuart1txdmaenblRegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VUART2TXDMAEnblReg` reader - VUART2 TX DMA enable register"]
pub type Vuart2txdmaenblRegR = crate::BitReader;
#[doc = "Field `VUART2TXDMAEnblReg` writer - VUART2 TX DMA enable register"]
pub type Vuart2txdmaenblRegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VUART3TXDMAEnblReg` reader - VUART3 TX DMA enable register"]
pub type Vuart3txdmaenblRegR = crate::BitReader;
#[doc = "Field `VUART3TXDMAEnblReg` writer - VUART3 TX DMA enable register"]
pub type Vuart3txdmaenblRegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved0` reader - reserved(0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - UART0 TX DMA enable register"]
    #[inline(always)]
    pub fn uart0txdmaenbl_reg(&self) -> Uart0txdmaenblRegR {
        Uart0txdmaenblRegR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART1 TX DMA enable register"]
    #[inline(always)]
    pub fn uart1txdmaenbl_reg(&self) -> Uart1txdmaenblRegR {
        Uart1txdmaenblRegR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART2 TX DMA enable register"]
    #[inline(always)]
    pub fn uart2txdmaenbl_reg(&self) -> Uart2txdmaenblRegR {
        Uart2txdmaenblRegR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART3 TX DMA enable register"]
    #[inline(always)]
    pub fn uart3txdmaenbl_reg(&self) -> Uart3txdmaenblRegR {
        Uart3txdmaenblRegR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - UART5 TX DMA enable register"]
    #[inline(always)]
    pub fn uart5txdmaenbl_reg(&self) -> Uart5txdmaenblRegR {
        Uart5txdmaenblRegR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART6 TX DMA enable register"]
    #[inline(always)]
    pub fn uart6txdmaenbl_reg(&self) -> Uart6txdmaenblRegR {
        Uart6txdmaenblRegR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UART7 TX DMA enable register"]
    #[inline(always)]
    pub fn uart7txdmaenbl_reg(&self) -> Uart7txdmaenblRegR {
        Uart7txdmaenblRegR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART8 TX DMA enable register"]
    #[inline(always)]
    pub fn uart8txdmaenbl_reg(&self) -> Uart8txdmaenblRegR {
        Uart8txdmaenblRegR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - UART9 TX DMA enable register"]
    #[inline(always)]
    pub fn uart9txdmaenbl_reg(&self) -> Uart9txdmaenblRegR {
        Uart9txdmaenblRegR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UART10 TX DMA enable register"]
    #[inline(always)]
    pub fn uart10txdmaenbl_reg(&self) -> Uart10txdmaenblRegR {
        Uart10txdmaenblRegR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - UART11 TX DMA enable register"]
    #[inline(always)]
    pub fn uart11txdmaenbl_reg(&self) -> Uart11txdmaenblRegR {
        Uart11txdmaenblRegR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - UART-BMC TX DMA enable register"]
    #[inline(always)]
    pub fn uartbmctxdmaenbl_reg(&self) -> UartbmctxdmaenblRegR {
        UartbmctxdmaenblRegR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - VUART0 TX DMA enable register"]
    #[inline(always)]
    pub fn vuart0txdmaenbl_reg(&self) -> Vuart0txdmaenblRegR {
        Vuart0txdmaenblRegR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - VUART1 TX DMA enable register"]
    #[inline(always)]
    pub fn vuart1txdmaenbl_reg(&self) -> Vuart1txdmaenblRegR {
        Vuart1txdmaenblRegR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - VUART2 TX DMA enable register"]
    #[inline(always)]
    pub fn vuart2txdmaenbl_reg(&self) -> Vuart2txdmaenblRegR {
        Vuart2txdmaenblRegR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - VUART3 TX DMA enable register"]
    #[inline(always)]
    pub fn vuart3txdmaenbl_reg(&self) -> Vuart3txdmaenblRegR {
        Vuart3txdmaenblRegR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - UART0 TX DMA enable register"]
    #[inline(always)]
    pub fn uart0txdmaenbl_reg(&mut self) -> Uart0txdmaenblRegW<Uartdma000Spec> {
        Uart0txdmaenblRegW::new(self, 0)
    }
    #[doc = "Bit 1 - UART1 TX DMA enable register"]
    #[inline(always)]
    pub fn uart1txdmaenbl_reg(&mut self) -> Uart1txdmaenblRegW<Uartdma000Spec> {
        Uart1txdmaenblRegW::new(self, 1)
    }
    #[doc = "Bit 2 - UART2 TX DMA enable register"]
    #[inline(always)]
    pub fn uart2txdmaenbl_reg(&mut self) -> Uart2txdmaenblRegW<Uartdma000Spec> {
        Uart2txdmaenblRegW::new(self, 2)
    }
    #[doc = "Bit 3 - UART3 TX DMA enable register"]
    #[inline(always)]
    pub fn uart3txdmaenbl_reg(&mut self) -> Uart3txdmaenblRegW<Uartdma000Spec> {
        Uart3txdmaenblRegW::new(self, 3)
    }
    #[doc = "Bit 4 - UART5 TX DMA enable register"]
    #[inline(always)]
    pub fn uart5txdmaenbl_reg(&mut self) -> Uart5txdmaenblRegW<Uartdma000Spec> {
        Uart5txdmaenblRegW::new(self, 4)
    }
    #[doc = "Bit 5 - UART6 TX DMA enable register"]
    #[inline(always)]
    pub fn uart6txdmaenbl_reg(&mut self) -> Uart6txdmaenblRegW<Uartdma000Spec> {
        Uart6txdmaenblRegW::new(self, 5)
    }
    #[doc = "Bit 6 - UART7 TX DMA enable register"]
    #[inline(always)]
    pub fn uart7txdmaenbl_reg(&mut self) -> Uart7txdmaenblRegW<Uartdma000Spec> {
        Uart7txdmaenblRegW::new(self, 6)
    }
    #[doc = "Bit 7 - UART8 TX DMA enable register"]
    #[inline(always)]
    pub fn uart8txdmaenbl_reg(&mut self) -> Uart8txdmaenblRegW<Uartdma000Spec> {
        Uart8txdmaenblRegW::new(self, 7)
    }
    #[doc = "Bit 8 - UART9 TX DMA enable register"]
    #[inline(always)]
    pub fn uart9txdmaenbl_reg(&mut self) -> Uart9txdmaenblRegW<Uartdma000Spec> {
        Uart9txdmaenblRegW::new(self, 8)
    }
    #[doc = "Bit 9 - UART10 TX DMA enable register"]
    #[inline(always)]
    pub fn uart10txdmaenbl_reg(&mut self) -> Uart10txdmaenblRegW<Uartdma000Spec> {
        Uart10txdmaenblRegW::new(self, 9)
    }
    #[doc = "Bit 10 - UART11 TX DMA enable register"]
    #[inline(always)]
    pub fn uart11txdmaenbl_reg(&mut self) -> Uart11txdmaenblRegW<Uartdma000Spec> {
        Uart11txdmaenblRegW::new(self, 10)
    }
    #[doc = "Bit 11 - UART-BMC TX DMA enable register"]
    #[inline(always)]
    pub fn uartbmctxdmaenbl_reg(&mut self) -> UartbmctxdmaenblRegW<Uartdma000Spec> {
        UartbmctxdmaenblRegW::new(self, 11)
    }
    #[doc = "Bit 12 - VUART0 TX DMA enable register"]
    #[inline(always)]
    pub fn vuart0txdmaenbl_reg(&mut self) -> Vuart0txdmaenblRegW<Uartdma000Spec> {
        Vuart0txdmaenblRegW::new(self, 12)
    }
    #[doc = "Bit 13 - VUART1 TX DMA enable register"]
    #[inline(always)]
    pub fn vuart1txdmaenbl_reg(&mut self) -> Vuart1txdmaenblRegW<Uartdma000Spec> {
        Vuart1txdmaenblRegW::new(self, 13)
    }
    #[doc = "Bit 14 - VUART2 TX DMA enable register"]
    #[inline(always)]
    pub fn vuart2txdmaenbl_reg(&mut self) -> Vuart2txdmaenblRegW<Uartdma000Spec> {
        Vuart2txdmaenblRegW::new(self, 14)
    }
    #[doc = "Bit 15 - VUART3 TX DMA enable register"]
    #[inline(always)]
    pub fn vuart3txdmaenbl_reg(&mut self) -> Vuart3txdmaenblRegW<Uartdma000Spec> {
        Vuart3txdmaenblRegW::new(self, 15)
    }
}
#[doc = "UART TX DMA enable\n\nYou can [`read`](crate::Reg::read) this register and get [`uartdma000::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartdma000::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uartdma000Spec;
impl crate::RegisterSpec for Uartdma000Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartdma000::R`](R) reader structure"]
impl crate::Readable for Uartdma000Spec {}
#[doc = "`write(|w| ..)` method takes [`uartdma000::W`](W) writer structure"]
impl crate::Writable for Uartdma000Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTDMA000 to value 0"]
impl crate::Resettable for Uartdma000Spec {}
