#[doc = "Register `UARTLSR` reader"]
pub type R = crate::R<UartlsrSpec>;
#[doc = "Register `UARTLSR` writer"]
pub type W = crate::W<UartlsrSpec>;
#[doc = "DR: Data ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrdataReady {
    #[doc = "1: The receiver contains at least one character in the UART\\_RBR or the receiver FIFO."]
    TheReceiverContainsAtLeastOneCharacterInTheUartrbrOrTheReceiverFifo = 1,
    #[doc = "0: The UART\\_RBR is read or the receiver FIFO is empty."]
    TheUartrbrIsReadOrTheReceiverFifoIsEmpty = 0,
}
impl From<DrdataReady> for bool {
    #[inline(always)]
    fn from(variant: DrdataReady) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DRDataReady` reader - DR: Data ready"]
pub type DrdataReadyR = crate::BitReader<DrdataReady>;
impl DrdataReadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DrdataReady {
        match self.bits {
            true => {
                DrdataReady::TheReceiverContainsAtLeastOneCharacterInTheUartrbrOrTheReceiverFifo
            }
            false => DrdataReady::TheUartrbrIsReadOrTheReceiverFifoIsEmpty,
        }
    }
    #[doc = "The receiver contains at least one character in the UART\\_RBR or the receiver FIFO."]
    #[inline(always)]
    pub fn is_the_receiver_contains_at_least_one_character_in_the_uartrbr_or_the_receiver_fifo(
        &self,
    ) -> bool {
        *self == DrdataReady::TheReceiverContainsAtLeastOneCharacterInTheUartrbrOrTheReceiverFifo
    }
    #[doc = "The UART\\_RBR is read or the receiver FIFO is empty."]
    #[inline(always)]
    pub fn is_the_uartrbr_is_read_or_the_receiver_fifo_is_empty(&self) -> bool {
        *self == DrdataReady::TheUartrbrIsReadOrTheReceiverFifoIsEmpty
    }
}
#[doc = "OE: Overrun error (Read clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OeoverrunErrorReadClear {
    #[doc = "1: An overrun error has occurred because a new data character was received before"]
    AnOverrunErrorHasOccurredBecauseANewDataCharacterWasReceivedBefore = 1,
    #[doc = "0: No overrun state."]
    NoOverrunState = 0,
}
impl From<OeoverrunErrorReadClear> for bool {
    #[inline(always)]
    fn from(variant: OeoverrunErrorReadClear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OEOverrunErrorReadClear` reader - OE: Overrun error (Read clear)"]
pub type OeoverrunErrorReadClearR = crate::BitReader<OeoverrunErrorReadClear>;
impl OeoverrunErrorReadClearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OeoverrunErrorReadClear {
        match self . bits { true => OeoverrunErrorReadClear :: AnOverrunErrorHasOccurredBecauseANewDataCharacterWasReceivedBefore , false => OeoverrunErrorReadClear :: NoOverrunState , }
    }
    #[doc = "An overrun error has occurred because a new data character was received before"]
    #[inline(always)]
    pub fn is_an_overrun_error_has_occurred_because_a_new_data_character_was_received_before(
        &self,
    ) -> bool {
        * self == OeoverrunErrorReadClear :: AnOverrunErrorHasOccurredBecauseANewDataCharacterWasReceivedBefore
    }
    #[doc = "No overrun state."]
    #[inline(always)]
    pub fn is_no_overrun_state(&self) -> bool {
        *self == OeoverrunErrorReadClear::NoOverrunState
    }
}
#[doc = "PE: Parity error (Read clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PeparityErrorReadClear {
    #[doc = "1: There is a parity error in the receiver if the Parity Enable is set."]
    ThereIsAParityErrorInTheReceiverIfTheParityEnableIsSet = 1,
    #[doc = "0: No parity error in the current character."]
    NoParityErrorInTheCurrentCharacter = 0,
}
impl From<PeparityErrorReadClear> for bool {
    #[inline(always)]
    fn from(variant: PeparityErrorReadClear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEParityErrorReadClear` reader - PE: Parity error (Read clear)"]
pub type PeparityErrorReadClearR = crate::BitReader<PeparityErrorReadClear>;
impl PeparityErrorReadClearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PeparityErrorReadClear {
        match self.bits {
            true => PeparityErrorReadClear::ThereIsAParityErrorInTheReceiverIfTheParityEnableIsSet,
            false => PeparityErrorReadClear::NoParityErrorInTheCurrentCharacter,
        }
    }
    #[doc = "There is a parity error in the receiver if the Parity Enable is set."]
    #[inline(always)]
    pub fn is_there_is_a_parity_error_in_the_receiver_if_the_parity_enable_is_set(&self) -> bool {
        *self == PeparityErrorReadClear::ThereIsAParityErrorInTheReceiverIfTheParityEnableIsSet
    }
    #[doc = "No parity error in the current character."]
    #[inline(always)]
    pub fn is_no_parity_error_in_the_current_character(&self) -> bool {
        *self == PeparityErrorReadClear::NoParityErrorInTheCurrentCharacter
    }
}
#[doc = "FE: Framing error (Read clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FeframingErrorReadClear {
    #[doc = "1: There is a framing error in the receiver. A framing error occurs when the receiver"]
    ThereIsAFramingErrorInTheReceiverAFramingErrorOccursWhenTheReceiver = 1,
    #[doc = "0: No framing error in the current character."]
    NoFramingErrorInTheCurrentCharacter = 0,
}
impl From<FeframingErrorReadClear> for bool {
    #[inline(always)]
    fn from(variant: FeframingErrorReadClear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEFramingErrorReadClear` reader - FE: Framing error (Read clear)"]
pub type FeframingErrorReadClearR = crate::BitReader<FeframingErrorReadClear>;
impl FeframingErrorReadClearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FeframingErrorReadClear {
        match self . bits { true => FeframingErrorReadClear :: ThereIsAFramingErrorInTheReceiverAFramingErrorOccursWhenTheReceiver , false => FeframingErrorReadClear :: NoFramingErrorInTheCurrentCharacter , }
    }
    #[doc = "There is a framing error in the receiver. A framing error occurs when the receiver"]
    #[inline(always)]
    pub fn is_there_is_a_framing_error_in_the_receiver_a_framing_error_occurs_when_the_receiver(
        &self,
    ) -> bool {
        * self == FeframingErrorReadClear :: ThereIsAFramingErrorInTheReceiverAFramingErrorOccursWhenTheReceiver
    }
    #[doc = "No framing error in the current character."]
    #[inline(always)]
    pub fn is_no_framing_error_in_the_current_character(&self) -> bool {
        *self == FeframingErrorReadClear::NoFramingErrorInTheCurrentCharacter
    }
}
#[doc = "BI: Break interrupt (Read clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BibreakIntreadClear {
    #[doc = "1: The serial input is held in a logic \"0\" state for longer than the sum"]
    TheSerialInputIsHeldInALogic0StateForLongerThanTheSum = 1,
    #[doc = "0: No break condition in the current character."]
    NoBreakConditionInTheCurrentCharacter = 0,
}
impl From<BibreakIntreadClear> for bool {
    #[inline(always)]
    fn from(variant: BibreakIntreadClear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIBreakINTReadClear` reader - BI: Break interrupt (Read clear)"]
pub type BibreakIntreadClearR = crate::BitReader<BibreakIntreadClear>;
impl BibreakIntreadClearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BibreakIntreadClear {
        match self.bits {
            true => BibreakIntreadClear::TheSerialInputIsHeldInALogic0StateForLongerThanTheSum,
            false => BibreakIntreadClear::NoBreakConditionInTheCurrentCharacter,
        }
    }
    #[doc = "The serial input is held in a logic \"0\" state for longer than the sum"]
    #[inline(always)]
    pub fn is_the_serial_input_is_held_in_a_logic_0_state_for_longer_than_the_sum(&self) -> bool {
        *self == BibreakIntreadClear::TheSerialInputIsHeldInALogic0StateForLongerThanTheSum
    }
    #[doc = "No break condition in the current character."]
    #[inline(always)]
    pub fn is_no_break_condition_in_the_current_character(&self) -> bool {
        *self == BibreakIntreadClear::NoBreakConditionInTheCurrentCharacter
    }
}
#[doc = "Field `THRETxterHoldingRegEmpty` reader - THRE: Transmitter holding register empty"]
pub type ThretxterHoldingRegEmptyR = crate::BitReader;
#[doc = "Field `TxterEmpty` reader - Transmitter empty"]
pub type TxterEmptyR = crate::BitReader;
#[doc = "Error in Receiver FIFO (Read clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrInRxrFiforeadClear {
    #[doc = "1: There is at least one parity error, framing error, or break indication in the FIFO."]
    ThereIsAtLeastOneParityErrorFramingErrorOrBreakIndicationInTheFifo = 1,
    #[doc = "0: Otherwise."]
    Otherwise = 0,
}
impl From<ErrInRxrFiforeadClear> for bool {
    #[inline(always)]
    fn from(variant: ErrInRxrFiforeadClear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ErrInRxrFIFOReadClear` reader - Error in Receiver FIFO (Read clear)"]
pub type ErrInRxrFiforeadClearR = crate::BitReader<ErrInRxrFiforeadClear>;
impl ErrInRxrFiforeadClearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ErrInRxrFiforeadClear {
        match self . bits { true => ErrInRxrFiforeadClear :: ThereIsAtLeastOneParityErrorFramingErrorOrBreakIndicationInTheFifo , false => ErrInRxrFiforeadClear :: Otherwise , }
    }
    #[doc = "There is at least one parity error, framing error, or break indication in the FIFO."]
    #[inline(always)]
    pub fn is_there_is_at_least_one_parity_error_framing_error_or_break_indication_in_the_fifo(
        &self,
    ) -> bool {
        * self == ErrInRxrFiforeadClear :: ThereIsAtLeastOneParityErrorFramingErrorOrBreakIndicationInTheFifo
    }
    #[doc = "Otherwise."]
    #[inline(always)]
    pub fn is_otherwise(&self) -> bool {
        *self == ErrInRxrFiforeadClear::Otherwise
    }
}
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
