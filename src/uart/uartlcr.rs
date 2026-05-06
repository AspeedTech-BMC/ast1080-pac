#[doc = "Register `UARTLCR` reader"]
pub type R = crate::R<UartlcrSpec>;
#[doc = "Register `UARTLCR` writer"]
pub type W = crate::W<UartlcrSpec>;
#[doc = "CLS: Select number of bits per character\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClsselectNumberOfBitsPerCharacter {
    #[doc = "0: 5 bits."]
    _5Bits = 0,
    #[doc = "1: 6 bits."]
    _6Bits = 1,
    #[doc = "2: 7 bits."]
    _7Bits = 2,
    #[doc = "3: 8 bits."]
    _8Bits = 3,
}
impl From<ClsselectNumberOfBitsPerCharacter> for u8 {
    #[inline(always)]
    fn from(variant: ClsselectNumberOfBitsPerCharacter) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClsselectNumberOfBitsPerCharacter {
    type Ux = u8;
}
impl crate::IsEnum for ClsselectNumberOfBitsPerCharacter {}
#[doc = "Field `CLSSelectNumberOfBitsPerCharacter` reader - CLS: Select number of bits per character"]
pub type ClsselectNumberOfBitsPerCharacterR = crate::FieldReader<ClsselectNumberOfBitsPerCharacter>;
impl ClsselectNumberOfBitsPerCharacterR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClsselectNumberOfBitsPerCharacter {
        match self.bits {
            0 => ClsselectNumberOfBitsPerCharacter::_5Bits,
            1 => ClsselectNumberOfBitsPerCharacter::_6Bits,
            2 => ClsselectNumberOfBitsPerCharacter::_7Bits,
            3 => ClsselectNumberOfBitsPerCharacter::_8Bits,
            _ => unreachable!(),
        }
    }
    #[doc = "5 bits."]
    #[inline(always)]
    pub fn is_5_bits(&self) -> bool {
        *self == ClsselectNumberOfBitsPerCharacter::_5Bits
    }
    #[doc = "6 bits."]
    #[inline(always)]
    pub fn is_6_bits(&self) -> bool {
        *self == ClsselectNumberOfBitsPerCharacter::_6Bits
    }
    #[doc = "7 bits."]
    #[inline(always)]
    pub fn is_7_bits(&self) -> bool {
        *self == ClsselectNumberOfBitsPerCharacter::_7Bits
    }
    #[doc = "8 bits."]
    #[inline(always)]
    pub fn is_8_bits(&self) -> bool {
        *self == ClsselectNumberOfBitsPerCharacter::_8Bits
    }
}
#[doc = "Field `CLSSelectNumberOfBitsPerCharacter` writer - CLS: Select number of bits per character"]
pub type ClsselectNumberOfBitsPerCharacterW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, ClsselectNumberOfBitsPerCharacter, crate::Safe>;
impl<'a, REG> ClsselectNumberOfBitsPerCharacterW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "5 bits."]
    #[inline(always)]
    pub fn _5_bits(self) -> &'a mut crate::W<REG> {
        self.variant(ClsselectNumberOfBitsPerCharacter::_5Bits)
    }
    #[doc = "6 bits."]
    #[inline(always)]
    pub fn _6_bits(self) -> &'a mut crate::W<REG> {
        self.variant(ClsselectNumberOfBitsPerCharacter::_6Bits)
    }
    #[doc = "7 bits."]
    #[inline(always)]
    pub fn _7_bits(self) -> &'a mut crate::W<REG> {
        self.variant(ClsselectNumberOfBitsPerCharacter::_7Bits)
    }
    #[doc = "8 bits."]
    #[inline(always)]
    pub fn _8_bits(self) -> &'a mut crate::W<REG> {
        self.variant(ClsselectNumberOfBitsPerCharacter::_8Bits)
    }
}
#[doc = "STOP: Number of stop bits transmitted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StopnumberOfStopBitsTxted {
    #[doc = "0: 1 stop bit."]
    _1StopBit = 0,
    #[doc = "1: 1.5 stop bits when 5-bit character length selected and 2 bits otherwise"]
    _15StopBitsWhen5bitCharacterLengthSelectedAnd2BitsOtherwise = 1,
}
impl From<StopnumberOfStopBitsTxted> for bool {
    #[inline(always)]
    fn from(variant: StopnumberOfStopBitsTxted) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPNumberOfStopBitsTxted` reader - STOP: Number of stop bits transmitted"]
pub type StopnumberOfStopBitsTxtedR = crate::BitReader<StopnumberOfStopBitsTxted>;
impl StopnumberOfStopBitsTxtedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StopnumberOfStopBitsTxted {
        match self . bits { false => StopnumberOfStopBitsTxted :: _1StopBit , true => StopnumberOfStopBitsTxted :: _15StopBitsWhen5bitCharacterLengthSelectedAnd2BitsOtherwise , }
    }
    #[doc = "1 stop bit."]
    #[inline(always)]
    pub fn is_1_stop_bit(&self) -> bool {
        *self == StopnumberOfStopBitsTxted::_1StopBit
    }
    #[doc = "1.5 stop bits when 5-bit character length selected and 2 bits otherwise"]
    #[inline(always)]
    pub fn is_15_stop_bits_when_5bit_character_length_selected_and_2_bits_otherwise(&self) -> bool {
        * self == StopnumberOfStopBitsTxted :: _15StopBitsWhen5bitCharacterLengthSelectedAnd2BitsOtherwise
    }
}
#[doc = "Field `STOPNumberOfStopBitsTxted` writer - STOP: Number of stop bits transmitted"]
pub type StopnumberOfStopBitsTxtedW<'a, REG> = crate::BitWriter<'a, REG, StopnumberOfStopBitsTxted>;
impl<'a, REG> StopnumberOfStopBitsTxtedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "1 stop bit."]
    #[inline(always)]
    pub fn _1_stop_bit(self) -> &'a mut crate::W<REG> {
        self.variant(StopnumberOfStopBitsTxted::_1StopBit)
    }
    #[doc = "1.5 stop bits when 5-bit character length selected and 2 bits otherwise"]
    #[inline(always)]
    pub fn _15_stop_bits_when_5bit_character_length_selected_and_2_bits_otherwise(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(
            StopnumberOfStopBitsTxted::_15StopBitsWhen5bitCharacterLengthSelectedAnd2BitsOtherwise,
        )
    }
}
#[doc = "PEN: Enable parity bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PenenblParityBit {
    #[doc = "0: Disable parity bit"]
    DisableParityBit = 0,
    #[doc = "1: Enable parity bit"]
    EnableParityBit = 1,
}
impl From<PenenblParityBit> for bool {
    #[inline(always)]
    fn from(variant: PenenblParityBit) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PENEnblParityBit` reader - PEN: Enable parity bit"]
pub type PenenblParityBitR = crate::BitReader<PenenblParityBit>;
impl PenenblParityBitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PenenblParityBit {
        match self.bits {
            false => PenenblParityBit::DisableParityBit,
            true => PenenblParityBit::EnableParityBit,
        }
    }
    #[doc = "Disable parity bit"]
    #[inline(always)]
    pub fn is_disable_parity_bit(&self) -> bool {
        *self == PenenblParityBit::DisableParityBit
    }
    #[doc = "Enable parity bit"]
    #[inline(always)]
    pub fn is_enable_parity_bit(&self) -> bool {
        *self == PenenblParityBit::EnableParityBit
    }
}
#[doc = "Field `PENEnblParityBit` writer - PEN: Enable parity bit"]
pub type PenenblParityBitW<'a, REG> = crate::BitWriter<'a, REG, PenenblParityBit>;
impl<'a, REG> PenenblParityBitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable parity bit"]
    #[inline(always)]
    pub fn disable_parity_bit(self) -> &'a mut crate::W<REG> {
        self.variant(PenenblParityBit::DisableParityBit)
    }
    #[doc = "Enable parity bit"]
    #[inline(always)]
    pub fn enable_parity_bit(self) -> &'a mut crate::W<REG> {
        self.variant(PenenblParityBit::EnableParityBit)
    }
}
#[doc = "EPS: Parity mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EpsparityModeSel {
    #[doc = "0: Select odd parity mode (odd number of \"1\" for data and parity combined)"]
    SelectOddParityModeOddNumberOf1ForDataAndParityCombined = 0,
    #[doc = "1: Select even parity mode (even number of \"1\" for data and parity combined)"]
    SelectEvenParityModeEvenNumberOf1ForDataAndParityCombined = 1,
}
impl From<EpsparityModeSel> for bool {
    #[inline(always)]
    fn from(variant: EpsparityModeSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPSParityModeSel` reader - EPS: Parity mode selection"]
pub type EpsparityModeSelR = crate::BitReader<EpsparityModeSel>;
impl EpsparityModeSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EpsparityModeSel {
        match self.bits {
            false => EpsparityModeSel::SelectOddParityModeOddNumberOf1ForDataAndParityCombined,
            true => EpsparityModeSel::SelectEvenParityModeEvenNumberOf1ForDataAndParityCombined,
        }
    }
    #[doc = "Select odd parity mode (odd number of \"1\" for data and parity combined)"]
    #[inline(always)]
    pub fn is_select_odd_parity_mode_odd_number_of_1_for_data_and_parity_combined(&self) -> bool {
        *self == EpsparityModeSel::SelectOddParityModeOddNumberOf1ForDataAndParityCombined
    }
    #[doc = "Select even parity mode (even number of \"1\" for data and parity combined)"]
    #[inline(always)]
    pub fn is_select_even_parity_mode_even_number_of_1_for_data_and_parity_combined(&self) -> bool {
        *self == EpsparityModeSel::SelectEvenParityModeEvenNumberOf1ForDataAndParityCombined
    }
}
#[doc = "Field `EPSParityModeSel` writer - EPS: Parity mode selection"]
pub type EpsparityModeSelW<'a, REG> = crate::BitWriter<'a, REG, EpsparityModeSel>;
impl<'a, REG> EpsparityModeSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select odd parity mode (odd number of \"1\" for data and parity combined)"]
    #[inline(always)]
    pub fn select_odd_parity_mode_odd_number_of_1_for_data_and_parity_combined(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(EpsparityModeSel::SelectOddParityModeOddNumberOf1ForDataAndParityCombined)
    }
    #[doc = "Select even parity mode (even number of \"1\" for data and parity combined)"]
    #[inline(always)]
    pub fn select_even_parity_mode_even_number_of_1_for_data_and_parity_combined(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(EpsparityModeSel::SelectEvenParityModeEvenNumberOf1ForDataAndParityCombined)
    }
}
#[doc = "Break Control bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BreakCtrlBit {
    #[doc = "0: break is disabled."]
    BreakIsDisabled = 0,
    #[doc = "1: When not in Loopback Mode, the serial out is forced into logic '0' (break state)."]
    WhenNotInLoopbackModeTheSerialOutIsForcedIntoLogic0BreakState = 1,
}
impl From<BreakCtrlBit> for bool {
    #[inline(always)]
    fn from(variant: BreakCtrlBit) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BreakCtrlBit` reader - Break Control bit."]
pub type BreakCtrlBitR = crate::BitReader<BreakCtrlBit>;
impl BreakCtrlBitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BreakCtrlBit {
        match self.bits {
            false => BreakCtrlBit::BreakIsDisabled,
            true => BreakCtrlBit::WhenNotInLoopbackModeTheSerialOutIsForcedIntoLogic0BreakState,
        }
    }
    #[doc = "break is disabled."]
    #[inline(always)]
    pub fn is_break_is_disabled(&self) -> bool {
        *self == BreakCtrlBit::BreakIsDisabled
    }
    #[doc = "When not in Loopback Mode, the serial out is forced into logic '0' (break state)."]
    #[inline(always)]
    pub fn is_when_not_in_loopback_mode_the_serial_out_is_forced_into_logic_0_break_state(
        &self,
    ) -> bool {
        *self == BreakCtrlBit::WhenNotInLoopbackModeTheSerialOutIsForcedIntoLogic0BreakState
    }
}
#[doc = "Field `BreakCtrlBit` writer - Break Control bit."]
pub type BreakCtrlBitW<'a, REG> = crate::BitWriter<'a, REG, BreakCtrlBit>;
impl<'a, REG> BreakCtrlBitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "break is disabled."]
    #[inline(always)]
    pub fn break_is_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BreakCtrlBit::BreakIsDisabled)
    }
    #[doc = "When not in Loopback Mode, the serial out is forced into logic '0' (break state)."]
    #[inline(always)]
    pub fn when_not_in_loopback_mode_the_serial_out_is_forced_into_logic_0_break_state(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(BreakCtrlBit::WhenNotInLoopbackModeTheSerialOutIsForcedIntoLogic0BreakState)
    }
}
#[doc = "DLAB: Divisor latch access bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DlabdivisorLatchAccessBit {
    #[doc = "0: The normal registers are accessed."]
    TheNormalRegistersAreAccessed = 0,
    #[doc = "1: The divisor latches can be accessed."]
    TheDivisorLatchesCanBeAccessed = 1,
}
impl From<DlabdivisorLatchAccessBit> for bool {
    #[inline(always)]
    fn from(variant: DlabdivisorLatchAccessBit) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DLABDivisorLatchAccessBit` reader - DLAB: Divisor latch access bit"]
pub type DlabdivisorLatchAccessBitR = crate::BitReader<DlabdivisorLatchAccessBit>;
impl DlabdivisorLatchAccessBitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DlabdivisorLatchAccessBit {
        match self.bits {
            false => DlabdivisorLatchAccessBit::TheNormalRegistersAreAccessed,
            true => DlabdivisorLatchAccessBit::TheDivisorLatchesCanBeAccessed,
        }
    }
    #[doc = "The normal registers are accessed."]
    #[inline(always)]
    pub fn is_the_normal_registers_are_accessed(&self) -> bool {
        *self == DlabdivisorLatchAccessBit::TheNormalRegistersAreAccessed
    }
    #[doc = "The divisor latches can be accessed."]
    #[inline(always)]
    pub fn is_the_divisor_latches_can_be_accessed(&self) -> bool {
        *self == DlabdivisorLatchAccessBit::TheDivisorLatchesCanBeAccessed
    }
}
#[doc = "Field `DLABDivisorLatchAccessBit` writer - DLAB: Divisor latch access bit"]
pub type DlabdivisorLatchAccessBitW<'a, REG> = crate::BitWriter<'a, REG, DlabdivisorLatchAccessBit>;
impl<'a, REG> DlabdivisorLatchAccessBitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The normal registers are accessed."]
    #[inline(always)]
    pub fn the_normal_registers_are_accessed(self) -> &'a mut crate::W<REG> {
        self.variant(DlabdivisorLatchAccessBit::TheNormalRegistersAreAccessed)
    }
    #[doc = "The divisor latches can be accessed."]
    #[inline(always)]
    pub fn the_divisor_latches_can_be_accessed(self) -> &'a mut crate::W<REG> {
        self.variant(DlabdivisorLatchAccessBit::TheDivisorLatchesCanBeAccessed)
    }
}
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:1 - CLS: Select number of bits per character"]
    #[inline(always)]
    pub fn clsselect_number_of_bits_per_character(&self) -> ClsselectNumberOfBitsPerCharacterR {
        ClsselectNumberOfBitsPerCharacterR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - STOP: Number of stop bits transmitted"]
    #[inline(always)]
    pub fn stopnumber_of_stop_bits_txted(&self) -> StopnumberOfStopBitsTxtedR {
        StopnumberOfStopBitsTxtedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PEN: Enable parity bit"]
    #[inline(always)]
    pub fn penenbl_parity_bit(&self) -> PenenblParityBitR {
        PenenblParityBitR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EPS: Parity mode selection"]
    #[inline(always)]
    pub fn epsparity_mode_sel(&self) -> EpsparityModeSelR {
        EpsparityModeSelR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Break Control bit."]
    #[inline(always)]
    pub fn break_ctrl_bit(&self) -> BreakCtrlBitR {
        BreakCtrlBitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DLAB: Divisor latch access bit"]
    #[inline(always)]
    pub fn dlabdivisor_latch_access_bit(&self) -> DlabdivisorLatchAccessBitR {
        DlabdivisorLatchAccessBitR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - CLS: Select number of bits per character"]
    #[inline(always)]
    pub fn clsselect_number_of_bits_per_character(
        &mut self,
    ) -> ClsselectNumberOfBitsPerCharacterW<UartlcrSpec> {
        ClsselectNumberOfBitsPerCharacterW::new(self, 0)
    }
    #[doc = "Bit 2 - STOP: Number of stop bits transmitted"]
    #[inline(always)]
    pub fn stopnumber_of_stop_bits_txted(&mut self) -> StopnumberOfStopBitsTxtedW<UartlcrSpec> {
        StopnumberOfStopBitsTxtedW::new(self, 2)
    }
    #[doc = "Bit 3 - PEN: Enable parity bit"]
    #[inline(always)]
    pub fn penenbl_parity_bit(&mut self) -> PenenblParityBitW<UartlcrSpec> {
        PenenblParityBitW::new(self, 3)
    }
    #[doc = "Bit 4 - EPS: Parity mode selection"]
    #[inline(always)]
    pub fn epsparity_mode_sel(&mut self) -> EpsparityModeSelW<UartlcrSpec> {
        EpsparityModeSelW::new(self, 4)
    }
    #[doc = "Bit 6 - Break Control bit."]
    #[inline(always)]
    pub fn break_ctrl_bit(&mut self) -> BreakCtrlBitW<UartlcrSpec> {
        BreakCtrlBitW::new(self, 6)
    }
    #[doc = "Bit 7 - DLAB: Divisor latch access bit"]
    #[inline(always)]
    pub fn dlabdivisor_latch_access_bit(&mut self) -> DlabdivisorLatchAccessBitW<UartlcrSpec> {
        DlabdivisorLatchAccessBitW::new(self, 7)
    }
}
#[doc = "Line Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uartlcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartlcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartlcrSpec;
impl crate::RegisterSpec for UartlcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartlcr::R`](R) reader structure"]
impl crate::Readable for UartlcrSpec {}
#[doc = "`write(|w| ..)` method takes [`uartlcr::W`](W) writer structure"]
impl crate::Writable for UartlcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UARTLCR to value 0"]
impl crate::Resettable for UartlcrSpec {}
