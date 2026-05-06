#[doc = "Register `UARTIIR` reader"]
pub type R = crate::R<UartiirSpec>;
#[doc = "Register `UARTIIR` writer"]
pub type W = crate::W<UartiirSpec>;
#[doc = "Field `IndicatesThatAnINTIsPendingWhenItsLogic0` reader - Indicates that an interrupt is pending when it's logic \"0\"."]
pub type IndicatesThatAnIntisPendingWhenItsLogic0R = crate::BitReader;
#[doc = "Interrupt Decoding Table\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IntdecodingTable {
    #[doc = "0: Modem Status Changed"]
    ModemStatusChanged = 0,
    #[doc = "1: UART\\_THR empty"]
    UartthrEmpty = 1,
    #[doc = "2: Received Data Available"]
    ReceivedDataAvailable = 2,
    #[doc = "3: Receiver Status"]
    ReceiverStatus = 3,
    #[doc = "6: Character Time Out"]
    CharacterTimeOut = 6,
}
impl From<IntdecodingTable> for u8 {
    #[inline(always)]
    fn from(variant: IntdecodingTable) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IntdecodingTable {
    type Ux = u8;
}
impl crate::IsEnum for IntdecodingTable {}
#[doc = "Field `INTDecodingTable` reader - Interrupt Decoding Table"]
pub type IntdecodingTableR = crate::FieldReader<IntdecodingTable>;
impl IntdecodingTableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IntdecodingTable> {
        match self.bits {
            0 => Some(IntdecodingTable::ModemStatusChanged),
            1 => Some(IntdecodingTable::UartthrEmpty),
            2 => Some(IntdecodingTable::ReceivedDataAvailable),
            3 => Some(IntdecodingTable::ReceiverStatus),
            6 => Some(IntdecodingTable::CharacterTimeOut),
            _ => None,
        }
    }
    #[doc = "Modem Status Changed"]
    #[inline(always)]
    pub fn is_modem_status_changed(&self) -> bool {
        *self == IntdecodingTable::ModemStatusChanged
    }
    #[doc = "UART\\_THR empty"]
    #[inline(always)]
    pub fn is_uartthr_empty(&self) -> bool {
        *self == IntdecodingTable::UartthrEmpty
    }
    #[doc = "Received Data Available"]
    #[inline(always)]
    pub fn is_received_data_available(&self) -> bool {
        *self == IntdecodingTable::ReceivedDataAvailable
    }
    #[doc = "Receiver Status"]
    #[inline(always)]
    pub fn is_receiver_status(&self) -> bool {
        *self == IntdecodingTable::ReceiverStatus
    }
    #[doc = "Character Time Out"]
    #[inline(always)]
    pub fn is_character_time_out(&self) -> bool {
        *self == IntdecodingTable::CharacterTimeOut
    }
}
#[doc = "Field `Reserved01` reader - Reserved (0)"]
pub type Reserved01R = crate::FieldReader;
#[doc = "FIFO-Enabled Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FifoenbldBits {
    #[doc = "0: FIFOs disabled"]
    FifosDisabled = 0,
    #[doc = "3: FIFOs enabled"]
    FifosEnabled = 3,
}
impl From<FifoenbldBits> for u8 {
    #[inline(always)]
    fn from(variant: FifoenbldBits) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FifoenbldBits {
    type Ux = u8;
}
impl crate::IsEnum for FifoenbldBits {}
#[doc = "Field `FIFOEnbldBits` reader - FIFO-Enabled Bits"]
pub type FifoenbldBitsR = crate::FieldReader<FifoenbldBits>;
impl FifoenbldBitsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<FifoenbldBits> {
        match self.bits {
            0 => Some(FifoenbldBits::FifosDisabled),
            3 => Some(FifoenbldBits::FifosEnabled),
            _ => None,
        }
    }
    #[doc = "FIFOs disabled"]
    #[inline(always)]
    pub fn is_fifos_disabled(&self) -> bool {
        *self == FifoenbldBits::FifosDisabled
    }
    #[doc = "FIFOs enabled"]
    #[inline(always)]
    pub fn is_fifos_enabled(&self) -> bool {
        *self == FifoenbldBits::FifosEnabled
    }
}
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Indicates that an interrupt is pending when it's logic \"0\"."]
    #[inline(always)]
    pub fn indicates_that_an_intis_pending_when_its_logic0(
        &self,
    ) -> IndicatesThatAnIntisPendingWhenItsLogic0R {
        IndicatesThatAnIntisPendingWhenItsLogic0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Interrupt Decoding Table"]
    #[inline(always)]
    pub fn intdecoding_table(&self) -> IntdecodingTableR {
        IntdecodingTableR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:5 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - FIFO-Enabled Bits"]
    #[inline(always)]
    pub fn fifoenbld_bits(&self) -> FifoenbldBitsR {
        FifoenbldBitsR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {}
#[doc = "Interrupt Identity Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartiir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartiir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartiirSpec;
impl crate::RegisterSpec for UartiirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartiir::R`](R) reader structure"]
impl crate::Readable for UartiirSpec {}
#[doc = "`write(|w| ..)` method takes [`uartiir::W`](W) writer structure"]
impl crate::Writable for UartiirSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTIIR to value 0x01"]
impl crate::Resettable for UartiirSpec {
    const RESET_VALUE: u32 = 0x01;
}
