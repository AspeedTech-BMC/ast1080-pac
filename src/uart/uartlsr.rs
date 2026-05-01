#[doc = "Register `UARTLSR` reader"]
pub type R = crate::R<UartlsrSpec>;
#[doc = "Register `UARTLSR` writer"]
pub type W = crate::W<UartlsrSpec>;
#[doc = "Field `DR` reader - Data ready"]
pub type DrR = crate::BitReader;
#[doc = "Field `OE` reader - Overrun error (Read clear)"]
pub type OeR = crate::BitReader;
#[doc = "Field `PE` reader - Parity error (Read clear)"]
pub type PeR = crate::BitReader;
#[doc = "Field `FE` reader - Framing error (Read clear)"]
pub type FeR = crate::BitReader;
#[doc = "Field `BI` reader - Break interrupt (Read clear)"]
pub type BiR = crate::BitReader;
#[doc = "Field `THRE` reader - Transmitter holding register empty"]
pub type ThreR = crate::BitReader;
#[doc = "Field `TxterEmpty` reader - Transmitter empty"]
pub type TxterEmptyR = crate::BitReader;
#[doc = "Field `ErrInRxrFIFOReadClear` reader - Error in Receiver FIFO (Read clear)"]
pub type ErrInRxrFiforeadClearR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Data ready"]
    #[inline(always)]
    pub fn dr(&self) -> DrR {
        DrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overrun error (Read clear)"]
    #[inline(always)]
    pub fn oe(&self) -> OeR {
        OeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Parity error (Read clear)"]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Framing error (Read clear)"]
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Break interrupt (Read clear)"]
    #[inline(always)]
    pub fn bi(&self) -> BiR {
        BiR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmitter holding register empty"]
    #[inline(always)]
    pub fn thre(&self) -> ThreR {
        ThreR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmitter empty"]
    #[inline(always)]
    pub fn txter_empty(&self) -> TxterEmptyR {
        TxterEmptyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Error in Receiver FIFO (Read clear)"]
    #[inline(always)]
    pub fn err_in_rxr_fiforead_clear(&self) -> ErrInRxrFiforeadClearR {
        ErrInRxrFiforeadClearR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {}
#[doc = "Line Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartlsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartlsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartlsrSpec;
impl crate::RegisterSpec for UartlsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartlsr::R`](R) reader structure"]
impl crate::Readable for UartlsrSpec {}
#[doc = "`write(|w| ..)` method takes [`uartlsr::W`](W) writer structure"]
impl crate::Writable for UartlsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTLSR to value 0x60"]
impl crate::Resettable for UartlsrSpec {
    const RESET_VALUE: u32 = 0x60;
}
