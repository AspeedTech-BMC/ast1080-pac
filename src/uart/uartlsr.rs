#[doc = "Register `UARTLSR` reader"]
pub type R = crate::R<UartlsrSpec>;
#[doc = "Register `UARTLSR` writer"]
pub type W = crate::W<UartlsrSpec>;
#[doc = "Field `DRDataReady` reader - DR: Data ready"]
pub type DrdataReadyR = crate::BitReader;
#[doc = "Field `OEOverrunErrorReadClear` reader - OE: Overrun error (Read clear)"]
pub type OeoverrunErrorReadClearR = crate::BitReader;
#[doc = "Field `PEParityErrorReadClear` reader - PE: Parity error (Read clear)"]
pub type PeparityErrorReadClearR = crate::BitReader;
#[doc = "Field `FEFramingErrorReadClear` reader - FE: Framing error (Read clear)"]
pub type FeframingErrorReadClearR = crate::BitReader;
#[doc = "Field `BIBreakINTReadClear` reader - BI: Break interrupt (Read clear)"]
pub type BibreakIntreadClearR = crate::BitReader;
#[doc = "Field `THRETxterHoldingRegEmpty` reader - THRE: Transmitter holding register empty"]
pub type ThretxterHoldingRegEmptyR = crate::BitReader;
#[doc = "Field `TxterEmpty` reader - Transmitter empty"]
pub type TxterEmptyR = crate::BitReader;
#[doc = "Field `ErrInRxrFIFOReadClear` reader - Error in Receiver FIFO (Read clear)"]
pub type ErrInRxrFiforeadClearR = crate::BitReader;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - DR: Data ready"]
    #[inline(always)]
    pub fn drdata_ready(&self) -> DrdataReadyR {
        DrdataReadyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OE: Overrun error (Read clear)"]
    #[inline(always)]
    pub fn oeoverrun_error_read_clear(&self) -> OeoverrunErrorReadClearR {
        OeoverrunErrorReadClearR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PE: Parity error (Read clear)"]
    #[inline(always)]
    pub fn peparity_error_read_clear(&self) -> PeparityErrorReadClearR {
        PeparityErrorReadClearR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FE: Framing error (Read clear)"]
    #[inline(always)]
    pub fn feframing_error_read_clear(&self) -> FeframingErrorReadClearR {
        FeframingErrorReadClearR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - BI: Break interrupt (Read clear)"]
    #[inline(always)]
    pub fn bibreak_intread_clear(&self) -> BibreakIntreadClearR {
        BibreakIntreadClearR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - THRE: Transmitter holding register empty"]
    #[inline(always)]
    pub fn thretxter_holding_reg_empty(&self) -> ThretxterHoldingRegEmptyR {
        ThretxterHoldingRegEmptyR::new(((self.bits >> 5) & 1) != 0)
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
    #[doc = "Bits 8:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 8) & 0x00ff_ffff)
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
