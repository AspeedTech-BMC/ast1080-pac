#[doc = "Register `UARTFCR` reader"]
pub type R = crate::R<UartfcrSpec>;
#[doc = "Register `UARTFCR` writer"]
pub type W = crate::W<UartfcrSpec>;
#[doc = "Enable UART FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblUartfifo {
    #[doc = "0: Disable FIFO"]
    DisableFifo = 0,
    #[doc = "1: Enable FIFO"]
    EnableFifo = 1,
}
impl From<EnblUartfifo> for bool {
    #[inline(always)]
    fn from(variant: EnblUartfifo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblUARTFIFO` reader - Enable UART FIFO"]
pub type EnblUartfifoR = crate::BitReader<EnblUartfifo>;
impl EnblUartfifoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblUartfifo {
        match self.bits {
            false => EnblUartfifo::DisableFifo,
            true => EnblUartfifo::EnableFifo,
        }
    }
    #[doc = "Disable FIFO"]
    #[inline(always)]
    pub fn is_disable_fifo(&self) -> bool {
        *self == EnblUartfifo::DisableFifo
    }
    #[doc = "Enable FIFO"]
    #[inline(always)]
    pub fn is_enable_fifo(&self) -> bool {
        *self == EnblUartfifo::EnableFifo
    }
}
#[doc = "Field `EnblUARTFIFO` writer - Enable UART FIFO"]
pub type EnblUartfifoW<'a, REG> = crate::BitWriter<'a, REG, EnblUartfifo>;
impl<'a, REG> EnblUartfifoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable FIFO"]
    #[inline(always)]
    pub fn disable_fifo(self) -> &'a mut crate::W<REG> {
        self.variant(EnblUartfifo::DisableFifo)
    }
    #[doc = "Enable FIFO"]
    #[inline(always)]
    pub fn enable_fifo(self) -> &'a mut crate::W<REG> {
        self.variant(EnblUartfifo::EnableFifo)
    }
}
#[doc = "Field `RxFIFORst` reader - Receive FIFO Reset"]
pub type RxFiforstR = crate::BitReader;
#[doc = "Field `RxFIFORst` writer - Receive FIFO Reset"]
pub type RxFiforstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TxFIFORst` reader - Transmit FIFO Reset"]
pub type TxFiforstR = crate::BitReader;
#[doc = "Field `TxFIFORst` writer - Transmit FIFO Reset"]
pub type TxFiforstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved01` reader - Reserved (0)"]
pub type Reserved01R = crate::FieldReader;
#[doc = "Define the Receiver FIFO Interrupt trigger level.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DefineTheRxrFifointtriggerLevel {
    #[doc = "0: 1~~ byte received"]
    _1Byte_Received = 0,
    #[doc = "1: 4~~ bytes received"]
    _4BytesReceived = 1,
    #[doc = "2: 8~~ bytes received"]
    _8BytesReceived = 2,
    #[doc = "3: 14 bytes received"]
    _14BytesReceived = 3,
}
impl From<DefineTheRxrFifointtriggerLevel> for u8 {
    #[inline(always)]
    fn from(variant: DefineTheRxrFifointtriggerLevel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DefineTheRxrFifointtriggerLevel {
    type Ux = u8;
}
impl crate::IsEnum for DefineTheRxrFifointtriggerLevel {}
#[doc = "Field `DefineTheRxrFIFOINTTriggerLevel` reader - Define the Receiver FIFO Interrupt trigger level."]
pub type DefineTheRxrFifointtriggerLevelR = crate::FieldReader<DefineTheRxrFifointtriggerLevel>;
impl DefineTheRxrFifointtriggerLevelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DefineTheRxrFifointtriggerLevel {
        match self.bits {
            0 => DefineTheRxrFifointtriggerLevel::_1Byte_Received,
            1 => DefineTheRxrFifointtriggerLevel::_4BytesReceived,
            2 => DefineTheRxrFifointtriggerLevel::_8BytesReceived,
            3 => DefineTheRxrFifointtriggerLevel::_14BytesReceived,
            _ => unreachable!(),
        }
    }
    #[doc = "1~~ byte received"]
    #[inline(always)]
    pub fn is_1_byte__received(&self) -> bool {
        *self == DefineTheRxrFifointtriggerLevel::_1Byte_Received
    }
    #[doc = "4~~ bytes received"]
    #[inline(always)]
    pub fn is_4_bytes_received(&self) -> bool {
        *self == DefineTheRxrFifointtriggerLevel::_4BytesReceived
    }
    #[doc = "8~~ bytes received"]
    #[inline(always)]
    pub fn is_8_bytes_received(&self) -> bool {
        *self == DefineTheRxrFifointtriggerLevel::_8BytesReceived
    }
    #[doc = "14 bytes received"]
    #[inline(always)]
    pub fn is_14_bytes_received(&self) -> bool {
        *self == DefineTheRxrFifointtriggerLevel::_14BytesReceived
    }
}
#[doc = "Field `DefineTheRxrFIFOINTTriggerLevel` writer - Define the Receiver FIFO Interrupt trigger level."]
pub type DefineTheRxrFifointtriggerLevelW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, DefineTheRxrFifointtriggerLevel, crate::Safe>;
impl<'a, REG> DefineTheRxrFifointtriggerLevelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1~~ byte received"]
    #[inline(always)]
    pub fn _1_byte__received(self) -> &'a mut crate::W<REG> {
        self.variant(DefineTheRxrFifointtriggerLevel::_1Byte_Received)
    }
    #[doc = "4~~ bytes received"]
    #[inline(always)]
    pub fn _4_bytes_received(self) -> &'a mut crate::W<REG> {
        self.variant(DefineTheRxrFifointtriggerLevel::_4BytesReceived)
    }
    #[doc = "8~~ bytes received"]
    #[inline(always)]
    pub fn _8_bytes_received(self) -> &'a mut crate::W<REG> {
        self.variant(DefineTheRxrFifointtriggerLevel::_8BytesReceived)
    }
    #[doc = "14 bytes received"]
    #[inline(always)]
    pub fn _14_bytes_received(self) -> &'a mut crate::W<REG> {
        self.variant(DefineTheRxrFifointtriggerLevel::_14BytesReceived)
    }
}
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u32>;
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
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - Define the Receiver FIFO Interrupt trigger level."]
    #[inline(always)]
    pub fn define_the_rxr_fifointtrigger_level(&self) -> DefineTheRxrFifointtriggerLevelR {
        DefineTheRxrFifointtriggerLevelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 8) & 0x00ff_ffff)
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
