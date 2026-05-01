#[doc = "Register `UDMA034` reader"]
pub type R = crate::R<Udma034Spec>;
#[doc = "Register `UDMA034` writer"]
pub type W = crate::W<Udma034Spec>;
#[doc = "Field `UART0TXDMAINTSts` reader - UART0 TX DMA interrupt status"]
pub type Uart0txdmaintstsR = crate::BitReader;
#[doc = "Field `UART0TXDMAINTSts` writer - UART0 TX DMA interrupt status"]
pub type Uart0txdmaintstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1TXDMAINTSts` reader - UART1 TX DMA interrupt status"]
pub type Uart1txdmaintstsR = crate::BitReader;
#[doc = "Field `UART1TXDMAINTSts` writer - UART1 TX DMA interrupt status"]
pub type Uart1txdmaintstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART2TXDMAINTSts` reader - UART2 TX DMA interrupt status"]
pub type Uart2txdmaintstsR = crate::BitReader;
#[doc = "Field `UART2TXDMAINTSts` writer - UART2 TX DMA interrupt status"]
pub type Uart2txdmaintstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART3TXDMAINTSts` reader - UART3 TX DMA interrupt status"]
pub type Uart3txdmaintstsR = crate::BitReader;
#[doc = "Field `UART3TXDMAINTSts` writer - UART3 TX DMA interrupt status"]
pub type Uart3txdmaintstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART5TXDMAINTSts` reader - UART5 TX DMA interrupt status"]
pub type Uart5txdmaintstsR = crate::BitReader;
#[doc = "Field `UART5TXDMAINTSts` writer - UART5 TX DMA interrupt status"]
pub type Uart5txdmaintstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART6TXDMAINTSts` reader - UART6 TX DMA interrupt status"]
pub type Uart6txdmaintstsR = crate::BitReader;
#[doc = "Field `UART6TXDMAINTSts` writer - UART6 TX DMA interrupt status"]
pub type Uart6txdmaintstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART7TXDMAINTSts` reader - UART7 TX DMA interrupt status"]
pub type Uart7txdmaintstsR = crate::BitReader;
#[doc = "Field `UART7TXDMAINTSts` writer - UART7 TX DMA interrupt status"]
pub type Uart7txdmaintstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART8TXDMAINTSts` reader - UART8 TX DMA interrupt status"]
pub type Uart8txdmaintstsR = crate::BitReader;
#[doc = "Field `UART8TXDMAINTSts` writer - UART8 TX DMA interrupt status"]
pub type Uart8txdmaintstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART9TXDMAINTSts` reader - UART9 TX DMA interrupt status"]
pub type Uart9txdmaintstsR = crate::BitReader;
#[doc = "Field `UART9TXDMAINTSts` writer - UART9 TX DMA interrupt status"]
pub type Uart9txdmaintstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART10TXDMAINTSts` reader - UART10 TX DMA interrupt status"]
pub type Uart10txdmaintstsR = crate::BitReader;
#[doc = "Field `UART10TXDMAINTSts` writer - UART10 TX DMA interrupt status"]
pub type Uart10txdmaintstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART11TXDMAINTSts` reader - UART11 TX DMA interrupt status"]
pub type Uart11txdmaintstsR = crate::BitReader;
#[doc = "Field `UART11TXDMAINTSts` writer - UART11 TX DMA interrupt status"]
pub type Uart11txdmaintstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UARTBMCTXDMAINTSts` reader - UART-BMC TX DMA interrupt status"]
pub type UartbmctxdmaintstsR = crate::BitReader;
#[doc = "Field `UARTBMCTXDMAINTSts` writer - UART-BMC TX DMA interrupt status"]
pub type UartbmctxdmaintstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VUART0TXDMAINTSts` reader - VUART0 TX DMA interrupt status"]
pub type Vuart0txdmaintstsR = crate::BitReader;
#[doc = "Field `VUART0TXDMAINTSts` writer - VUART0 TX DMA interrupt status"]
pub type Vuart0txdmaintstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VUART1TXDMAINTSts` reader - VUART1 TX DMA interrupt status"]
pub type Vuart1txdmaintstsR = crate::BitReader;
#[doc = "Field `VUART1TXDMAINTSts` writer - VUART1 TX DMA interrupt status"]
pub type Vuart1txdmaintstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VUART2TXDMAINTSts` reader - VUART2 TX DMA interrupt status"]
pub type Vuart2txdmaintstsR = crate::BitReader;
#[doc = "Field `VUART2TXDMAINTSts` writer - VUART2 TX DMA interrupt status"]
pub type Vuart2txdmaintstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VUART3TXDMAINTSts` reader - VUART3 TX DMA interrupt status"]
pub type Vuart3txdmaintstsR = crate::BitReader;
#[doc = "Field `VUART3TXDMAINTSts` writer - VUART3 TX DMA interrupt status"]
pub type Vuart3txdmaintstsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - UART0 TX DMA interrupt status"]
    #[inline(always)]
    pub fn uart0txdmaintsts(&self) -> Uart0txdmaintstsR {
        Uart0txdmaintstsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART1 TX DMA interrupt status"]
    #[inline(always)]
    pub fn uart1txdmaintsts(&self) -> Uart1txdmaintstsR {
        Uart1txdmaintstsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART2 TX DMA interrupt status"]
    #[inline(always)]
    pub fn uart2txdmaintsts(&self) -> Uart2txdmaintstsR {
        Uart2txdmaintstsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART3 TX DMA interrupt status"]
    #[inline(always)]
    pub fn uart3txdmaintsts(&self) -> Uart3txdmaintstsR {
        Uart3txdmaintstsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - UART5 TX DMA interrupt status"]
    #[inline(always)]
    pub fn uart5txdmaintsts(&self) -> Uart5txdmaintstsR {
        Uart5txdmaintstsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART6 TX DMA interrupt status"]
    #[inline(always)]
    pub fn uart6txdmaintsts(&self) -> Uart6txdmaintstsR {
        Uart6txdmaintstsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UART7 TX DMA interrupt status"]
    #[inline(always)]
    pub fn uart7txdmaintsts(&self) -> Uart7txdmaintstsR {
        Uart7txdmaintstsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART8 TX DMA interrupt status"]
    #[inline(always)]
    pub fn uart8txdmaintsts(&self) -> Uart8txdmaintstsR {
        Uart8txdmaintstsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - UART9 TX DMA interrupt status"]
    #[inline(always)]
    pub fn uart9txdmaintsts(&self) -> Uart9txdmaintstsR {
        Uart9txdmaintstsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UART10 TX DMA interrupt status"]
    #[inline(always)]
    pub fn uart10txdmaintsts(&self) -> Uart10txdmaintstsR {
        Uart10txdmaintstsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - UART11 TX DMA interrupt status"]
    #[inline(always)]
    pub fn uart11txdmaintsts(&self) -> Uart11txdmaintstsR {
        Uart11txdmaintstsR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - UART-BMC TX DMA interrupt status"]
    #[inline(always)]
    pub fn uartbmctxdmaintsts(&self) -> UartbmctxdmaintstsR {
        UartbmctxdmaintstsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - VUART0 TX DMA interrupt status"]
    #[inline(always)]
    pub fn vuart0txdmaintsts(&self) -> Vuart0txdmaintstsR {
        Vuart0txdmaintstsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - VUART1 TX DMA interrupt status"]
    #[inline(always)]
    pub fn vuart1txdmaintsts(&self) -> Vuart1txdmaintstsR {
        Vuart1txdmaintstsR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - VUART2 TX DMA interrupt status"]
    #[inline(always)]
    pub fn vuart2txdmaintsts(&self) -> Vuart2txdmaintstsR {
        Vuart2txdmaintstsR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - VUART3 TX DMA interrupt status"]
    #[inline(always)]
    pub fn vuart3txdmaintsts(&self) -> Vuart3txdmaintstsR {
        Vuart3txdmaintstsR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART0 TX DMA interrupt status"]
    #[inline(always)]
    pub fn uart0txdmaintsts(&mut self) -> Uart0txdmaintstsW<Udma034Spec> {
        Uart0txdmaintstsW::new(self, 0)
    }
    #[doc = "Bit 1 - UART1 TX DMA interrupt status"]
    #[inline(always)]
    pub fn uart1txdmaintsts(&mut self) -> Uart1txdmaintstsW<Udma034Spec> {
        Uart1txdmaintstsW::new(self, 1)
    }
    #[doc = "Bit 2 - UART2 TX DMA interrupt status"]
    #[inline(always)]
    pub fn uart2txdmaintsts(&mut self) -> Uart2txdmaintstsW<Udma034Spec> {
        Uart2txdmaintstsW::new(self, 2)
    }
    #[doc = "Bit 3 - UART3 TX DMA interrupt status"]
    #[inline(always)]
    pub fn uart3txdmaintsts(&mut self) -> Uart3txdmaintstsW<Udma034Spec> {
        Uart3txdmaintstsW::new(self, 3)
    }
    #[doc = "Bit 4 - UART5 TX DMA interrupt status"]
    #[inline(always)]
    pub fn uart5txdmaintsts(&mut self) -> Uart5txdmaintstsW<Udma034Spec> {
        Uart5txdmaintstsW::new(self, 4)
    }
    #[doc = "Bit 5 - UART6 TX DMA interrupt status"]
    #[inline(always)]
    pub fn uart6txdmaintsts(&mut self) -> Uart6txdmaintstsW<Udma034Spec> {
        Uart6txdmaintstsW::new(self, 5)
    }
    #[doc = "Bit 6 - UART7 TX DMA interrupt status"]
    #[inline(always)]
    pub fn uart7txdmaintsts(&mut self) -> Uart7txdmaintstsW<Udma034Spec> {
        Uart7txdmaintstsW::new(self, 6)
    }
    #[doc = "Bit 7 - UART8 TX DMA interrupt status"]
    #[inline(always)]
    pub fn uart8txdmaintsts(&mut self) -> Uart8txdmaintstsW<Udma034Spec> {
        Uart8txdmaintstsW::new(self, 7)
    }
    #[doc = "Bit 8 - UART9 TX DMA interrupt status"]
    #[inline(always)]
    pub fn uart9txdmaintsts(&mut self) -> Uart9txdmaintstsW<Udma034Spec> {
        Uart9txdmaintstsW::new(self, 8)
    }
    #[doc = "Bit 9 - UART10 TX DMA interrupt status"]
    #[inline(always)]
    pub fn uart10txdmaintsts(&mut self) -> Uart10txdmaintstsW<Udma034Spec> {
        Uart10txdmaintstsW::new(self, 9)
    }
    #[doc = "Bit 10 - UART11 TX DMA interrupt status"]
    #[inline(always)]
    pub fn uart11txdmaintsts(&mut self) -> Uart11txdmaintstsW<Udma034Spec> {
        Uart11txdmaintstsW::new(self, 10)
    }
    #[doc = "Bit 11 - UART-BMC TX DMA interrupt status"]
    #[inline(always)]
    pub fn uartbmctxdmaintsts(&mut self) -> UartbmctxdmaintstsW<Udma034Spec> {
        UartbmctxdmaintstsW::new(self, 11)
    }
    #[doc = "Bit 12 - VUART0 TX DMA interrupt status"]
    #[inline(always)]
    pub fn vuart0txdmaintsts(&mut self) -> Vuart0txdmaintstsW<Udma034Spec> {
        Vuart0txdmaintstsW::new(self, 12)
    }
    #[doc = "Bit 13 - VUART1 TX DMA interrupt status"]
    #[inline(always)]
    pub fn vuart1txdmaintsts(&mut self) -> Vuart1txdmaintstsW<Udma034Spec> {
        Vuart1txdmaintstsW::new(self, 13)
    }
    #[doc = "Bit 14 - VUART2 TX DMA interrupt status"]
    #[inline(always)]
    pub fn vuart2txdmaintsts(&mut self) -> Vuart2txdmaintstsW<Udma034Spec> {
        Vuart2txdmaintstsW::new(self, 14)
    }
    #[doc = "Bit 15 - VUART3 TX DMA interrupt status"]
    #[inline(always)]
    pub fn vuart3txdmaintsts(&mut self) -> Vuart3txdmaintstsW<Udma034Spec> {
        Vuart3txdmaintstsW::new(self, 15)
    }
}
#[doc = "UART TX DMA interrrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`udma034::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma034::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma034Spec;
impl crate::RegisterSpec for Udma034Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma034::R`](R) reader structure"]
impl crate::Readable for Udma034Spec {}
#[doc = "`write(|w| ..)` method takes [`udma034::W`](W) writer structure"]
impl crate::Writable for Udma034Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA034 to value 0"]
impl crate::Resettable for Udma034Spec {}
