#[doc = "Register `GPIO448` reader"]
pub type R = crate::R<Gpio448Spec>;
#[doc = "Register `GPIO448` writer"]
pub type W = crate::W<Gpio448Spec>;
#[doc = "Data register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataReg2 {
    #[doc = "0: Output 0 if Bit1 is 1."]
    Output0IfBit1Is1 = 0,
    #[doc = "1: Output 1 if Bit1 is 1."]
    Output1IfBit1Is1 = 1,
}
impl From<DataReg2> for bool {
    #[inline(always)]
    fn from(variant: DataReg2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DataReg2` reader - Data register"]
pub type DataReg2R = crate::BitReader<DataReg2>;
impl DataReg2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DataReg2 {
        match self.bits {
            false => DataReg2::Output0IfBit1Is1,
            true => DataReg2::Output1IfBit1Is1,
        }
    }
    #[doc = "Output 0 if Bit1 is 1."]
    #[inline(always)]
    pub fn is_output_0_if_bit1_is_1(&self) -> bool {
        *self == DataReg2::Output0IfBit1Is1
    }
    #[doc = "Output 1 if Bit1 is 1."]
    #[inline(always)]
    pub fn is_output_1_if_bit1_is_1(&self) -> bool {
        *self == DataReg2::Output1IfBit1Is1
    }
}
#[doc = "Field `DataReg2` writer - Data register"]
pub type DataReg2W<'a, REG> = crate::BitWriter<'a, REG, DataReg2>;
impl<'a, REG> DataReg2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output 0 if Bit1 is 1."]
    #[inline(always)]
    pub fn output_0_if_bit1_is_1(self) -> &'a mut crate::W<REG> {
        self.variant(DataReg2::Output0IfBit1Is1)
    }
    #[doc = "Output 1 if Bit1 is 1."]
    #[inline(always)]
    pub fn output_1_if_bit1_is_1(self) -> &'a mut crate::W<REG> {
        self.variant(DataReg2::Output1IfBit1Is1)
    }
}
#[doc = "Direction register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirectionReg2 {
    #[doc = "0: Input"]
    Input = 0,
    #[doc = "1: Output"]
    Output = 1,
}
impl From<DirectionReg2> for bool {
    #[inline(always)]
    fn from(variant: DirectionReg2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DirectionReg2` reader - Direction register"]
pub type DirectionReg2R = crate::BitReader<DirectionReg2>;
impl DirectionReg2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DirectionReg2 {
        match self.bits {
            false => DirectionReg2::Input,
            true => DirectionReg2::Output,
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DirectionReg2::Input
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DirectionReg2::Output
    }
}
#[doc = "Field `DirectionReg2` writer - Direction register"]
pub type DirectionReg2W<'a, REG> = crate::BitWriter<'a, REG, DirectionReg2>;
impl<'a, REG> DirectionReg2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(DirectionReg2::Input)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(DirectionReg2::Output)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Intenbl2 {
    #[doc = "0: Disable interrupt"]
    DisableInterrupt = 0,
    #[doc = "1: Enable interrupt"]
    EnableInterrupt = 1,
}
impl From<Intenbl2> for bool {
    #[inline(always)]
    fn from(variant: Intenbl2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTEnbl2` reader - Interrupt enable"]
pub type Intenbl2R = crate::BitReader<Intenbl2>;
impl Intenbl2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Intenbl2 {
        match self.bits {
            false => Intenbl2::DisableInterrupt,
            true => Intenbl2::EnableInterrupt,
        }
    }
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn is_disable_interrupt(&self) -> bool {
        *self == Intenbl2::DisableInterrupt
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn is_enable_interrupt(&self) -> bool {
        *self == Intenbl2::EnableInterrupt
    }
}
#[doc = "Field `INTEnbl2` writer - Interrupt enable"]
pub type Intenbl2W<'a, REG> = crate::BitWriter<'a, REG, Intenbl2>;
impl<'a, REG> Intenbl2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable_interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(Intenbl2::DisableInterrupt)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable_interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(Intenbl2::EnableInterrupt)
    }
}
#[doc = "Interrupt sensitivity type 0 selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntsensitivityType0sel2 {
    #[doc = "0: Select falling-edge or level-low trigger mode"]
    SelectFallingedgeOrLevellowTriggerMode = 0,
    #[doc = "1: Select rising-edge or level-high trigger mode"]
    SelectRisingedgeOrLevelhighTriggerMode = 1,
}
impl From<IntsensitivityType0sel2> for bool {
    #[inline(always)]
    fn from(variant: IntsensitivityType0sel2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTSensitivityType0Sel2` reader - Interrupt sensitivity type 0 selection"]
pub type IntsensitivityType0sel2R = crate::BitReader<IntsensitivityType0sel2>;
impl IntsensitivityType0sel2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntsensitivityType0sel2 {
        match self.bits {
            false => IntsensitivityType0sel2::SelectFallingedgeOrLevellowTriggerMode,
            true => IntsensitivityType0sel2::SelectRisingedgeOrLevelhighTriggerMode,
        }
    }
    #[doc = "Select falling-edge or level-low trigger mode"]
    #[inline(always)]
    pub fn is_select_fallingedge_or_levellow_trigger_mode(&self) -> bool {
        *self == IntsensitivityType0sel2::SelectFallingedgeOrLevellowTriggerMode
    }
    #[doc = "Select rising-edge or level-high trigger mode"]
    #[inline(always)]
    pub fn is_select_risingedge_or_levelhigh_trigger_mode(&self) -> bool {
        *self == IntsensitivityType0sel2::SelectRisingedgeOrLevelhighTriggerMode
    }
}
#[doc = "Field `INTSensitivityType0Sel2` writer - Interrupt sensitivity type 0 selection"]
pub type IntsensitivityType0sel2W<'a, REG> = crate::BitWriter<'a, REG, IntsensitivityType0sel2>;
impl<'a, REG> IntsensitivityType0sel2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select falling-edge or level-low trigger mode"]
    #[inline(always)]
    pub fn select_fallingedge_or_levellow_trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(IntsensitivityType0sel2::SelectFallingedgeOrLevellowTriggerMode)
    }
    #[doc = "Select rising-edge or level-high trigger mode"]
    #[inline(always)]
    pub fn select_risingedge_or_levelhigh_trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(IntsensitivityType0sel2::SelectRisingedgeOrLevelhighTriggerMode)
    }
}
#[doc = "Interrupt sensitivity type 1 selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntsensitivityType1sel2 {
    #[doc = "0: Select edge trigger mode"]
    SelectEdgeTriggerMode = 0,
    #[doc = "1: Select level trigger mode"]
    SelectLevelTriggerMode = 1,
}
impl From<IntsensitivityType1sel2> for bool {
    #[inline(always)]
    fn from(variant: IntsensitivityType1sel2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTSensitivityType1Sel2` reader - Interrupt sensitivity type 1 selection"]
pub type IntsensitivityType1sel2R = crate::BitReader<IntsensitivityType1sel2>;
impl IntsensitivityType1sel2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntsensitivityType1sel2 {
        match self.bits {
            false => IntsensitivityType1sel2::SelectEdgeTriggerMode,
            true => IntsensitivityType1sel2::SelectLevelTriggerMode,
        }
    }
    #[doc = "Select edge trigger mode"]
    #[inline(always)]
    pub fn is_select_edge_trigger_mode(&self) -> bool {
        *self == IntsensitivityType1sel2::SelectEdgeTriggerMode
    }
    #[doc = "Select level trigger mode"]
    #[inline(always)]
    pub fn is_select_level_trigger_mode(&self) -> bool {
        *self == IntsensitivityType1sel2::SelectLevelTriggerMode
    }
}
#[doc = "Field `INTSensitivityType1Sel2` writer - Interrupt sensitivity type 1 selection"]
pub type IntsensitivityType1sel2W<'a, REG> = crate::BitWriter<'a, REG, IntsensitivityType1sel2>;
impl<'a, REG> IntsensitivityType1sel2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select edge trigger mode"]
    #[inline(always)]
    pub fn select_edge_trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(IntsensitivityType1sel2::SelectEdgeTriggerMode)
    }
    #[doc = "Select level trigger mode"]
    #[inline(always)]
    pub fn select_level_trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(IntsensitivityType1sel2::SelectLevelTriggerMode)
    }
}
#[doc = "Interrupt sensitivity type 2 selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntsensitivityType2sel2 {
    #[doc = "0: Select edge or level trigger mode"]
    SelectEdgeOrLevelTriggerMode = 0,
    #[doc = "1: Select dual-edge trigger mode"]
    SelectDualedgeTriggerMode = 1,
}
impl From<IntsensitivityType2sel2> for bool {
    #[inline(always)]
    fn from(variant: IntsensitivityType2sel2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTSensitivityType2Sel2` reader - Interrupt sensitivity type 2 selection"]
pub type IntsensitivityType2sel2R = crate::BitReader<IntsensitivityType2sel2>;
impl IntsensitivityType2sel2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntsensitivityType2sel2 {
        match self.bits {
            false => IntsensitivityType2sel2::SelectEdgeOrLevelTriggerMode,
            true => IntsensitivityType2sel2::SelectDualedgeTriggerMode,
        }
    }
    #[doc = "Select edge or level trigger mode"]
    #[inline(always)]
    pub fn is_select_edge_or_level_trigger_mode(&self) -> bool {
        *self == IntsensitivityType2sel2::SelectEdgeOrLevelTriggerMode
    }
    #[doc = "Select dual-edge trigger mode"]
    #[inline(always)]
    pub fn is_select_dualedge_trigger_mode(&self) -> bool {
        *self == IntsensitivityType2sel2::SelectDualedgeTriggerMode
    }
}
#[doc = "Field `INTSensitivityType2Sel2` writer - Interrupt sensitivity type 2 selection"]
pub type IntsensitivityType2sel2W<'a, REG> = crate::BitWriter<'a, REG, IntsensitivityType2sel2>;
impl<'a, REG> IntsensitivityType2sel2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select edge or level trigger mode"]
    #[inline(always)]
    pub fn select_edge_or_level_trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(IntsensitivityType2sel2::SelectEdgeOrLevelTriggerMode)
    }
    #[doc = "Select dual-edge trigger mode"]
    #[inline(always)]
    pub fn select_dualedge_trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(IntsensitivityType2sel2::SelectDualedgeTriggerMode)
    }
}
#[doc = "Reset tolerance enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RstToleranceEnbl2 {
    #[doc = "0: Bit0 and Bit1 will be reset by WDT\\_SOC and/or EXTRSTN reset"]
    Bit0AndBit1WillBeResetByWdtsocAndorExtrstnReset = 0,
    #[doc = "1: Bit0 and Bit1 will not be reset by WDT\\_SOC and/or EXTRSTN"]
    Bit0AndBit1WillNotBeResetByWdtsocAndorExtrstn = 1,
}
impl From<RstToleranceEnbl2> for bool {
    #[inline(always)]
    fn from(variant: RstToleranceEnbl2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RstToleranceEnbl2` reader - Reset tolerance enable"]
pub type RstToleranceEnbl2R = crate::BitReader<RstToleranceEnbl2>;
impl RstToleranceEnbl2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RstToleranceEnbl2 {
        match self.bits {
            false => RstToleranceEnbl2::Bit0AndBit1WillBeResetByWdtsocAndorExtrstnReset,
            true => RstToleranceEnbl2::Bit0AndBit1WillNotBeResetByWdtsocAndorExtrstn,
        }
    }
    #[doc = "Bit0 and Bit1 will be reset by WDT\\_SOC and/or EXTRSTN reset"]
    #[inline(always)]
    pub fn is_bit0_and_bit1_will_be_reset_by_wdtsoc_andor_extrstn_reset(&self) -> bool {
        *self == RstToleranceEnbl2::Bit0AndBit1WillBeResetByWdtsocAndorExtrstnReset
    }
    #[doc = "Bit0 and Bit1 will not be reset by WDT\\_SOC and/or EXTRSTN"]
    #[inline(always)]
    pub fn is_bit0_and_bit1_will_not_be_reset_by_wdtsoc_andor_extrstn(&self) -> bool {
        *self == RstToleranceEnbl2::Bit0AndBit1WillNotBeResetByWdtsocAndorExtrstn
    }
}
#[doc = "Field `RstToleranceEnbl2` writer - Reset tolerance enable"]
pub type RstToleranceEnbl2W<'a, REG> = crate::BitWriter<'a, REG, RstToleranceEnbl2>;
impl<'a, REG> RstToleranceEnbl2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bit0 and Bit1 will be reset by WDT\\_SOC and/or EXTRSTN reset"]
    #[inline(always)]
    pub fn bit0_and_bit1_will_be_reset_by_wdtsoc_andor_extrstn_reset(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(RstToleranceEnbl2::Bit0AndBit1WillBeResetByWdtsocAndorExtrstnReset)
    }
    #[doc = "Bit0 and Bit1 will not be reset by WDT\\_SOC and/or EXTRSTN"]
    #[inline(always)]
    pub fn bit0_and_bit1_will_not_be_reset_by_wdtsoc_andor_extrstn(self) -> &'a mut crate::W<REG> {
        self.variant(RstToleranceEnbl2::Bit0AndBit1WillNotBeResetByWdtsocAndorExtrstn)
    }
}
#[doc = "Field `DebounceSettingReg12` reader - Debounce setting register #1"]
pub type DebounceSettingReg12R = crate::BitReader;
#[doc = "Field `DebounceSettingReg12` writer - Debounce setting register #1"]
pub type DebounceSettingReg12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DebounceSettingReg22` reader - Debounce setting register #2"]
pub type DebounceSettingReg22R = crate::BitReader;
#[doc = "Field `DebounceSettingReg22` writer - Debounce setting register #2"]
pub type DebounceSettingReg22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Input mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputMask2 {
    #[doc = "0: Read from Bit13 will be updated."]
    ReadFromBit13WillBeUpdated = 0,
    #[doc = "1: Read from Bit13 will not be updated."]
    ReadFromBit13WillNotBeUpdated = 1,
}
impl From<InputMask2> for bool {
    #[inline(always)]
    fn from(variant: InputMask2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `InputMask2` reader - Input mask"]
pub type InputMask2R = crate::BitReader<InputMask2>;
impl InputMask2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> InputMask2 {
        match self.bits {
            false => InputMask2::ReadFromBit13WillBeUpdated,
            true => InputMask2::ReadFromBit13WillNotBeUpdated,
        }
    }
    #[doc = "Read from Bit13 will be updated."]
    #[inline(always)]
    pub fn is_read_from_bit13_will_be_updated(&self) -> bool {
        *self == InputMask2::ReadFromBit13WillBeUpdated
    }
    #[doc = "Read from Bit13 will not be updated."]
    #[inline(always)]
    pub fn is_read_from_bit13_will_not_be_updated(&self) -> bool {
        *self == InputMask2::ReadFromBit13WillNotBeUpdated
    }
}
#[doc = "Field `InputMask2` writer - Input mask"]
pub type InputMask2W<'a, REG> = crate::BitWriter<'a, REG, InputMask2>;
impl<'a, REG> InputMask2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read from Bit13 will be updated."]
    #[inline(always)]
    pub fn read_from_bit13_will_be_updated(self) -> &'a mut crate::W<REG> {
        self.variant(InputMask2::ReadFromBit13WillBeUpdated)
    }
    #[doc = "Read from Bit13 will not be updated."]
    #[inline(always)]
    pub fn read_from_bit13_will_not_be_updated(self) -> &'a mut crate::W<REG> {
        self.variant(InputMask2::ReadFromBit13WillNotBeUpdated)
    }
}
#[doc = "Field `BlinkCounterSel12` reader - Blink Counter Selection #1"]
pub type BlinkCounterSel12R = crate::BitReader;
#[doc = "Field `BlinkCounterSel12` writer - Blink Counter Selection #1"]
pub type BlinkCounterSel12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BlinkCounterSel22` reader - Blink Counter Selection #2"]
pub type BlinkCounterSel22R = crate::BitReader;
#[doc = "Field `BlinkCounterSel22` writer - Blink Counter Selection #2"]
pub type BlinkCounterSel22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt status register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntstsReg2 {
    #[doc = "0: No interrupt pending"]
    NoInterruptPending = 0,
    #[doc = "1: interrupt pending"]
    InterruptPending = 1,
}
impl From<IntstsReg2> for bool {
    #[inline(always)]
    fn from(variant: IntstsReg2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTStsReg2` reader - Interrupt status register"]
pub type IntstsReg2R = crate::BitReader<IntstsReg2>;
impl IntstsReg2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntstsReg2 {
        match self.bits {
            false => IntstsReg2::NoInterruptPending,
            true => IntstsReg2::InterruptPending,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_no_interrupt_pending(&self) -> bool {
        *self == IntstsReg2::NoInterruptPending
    }
    #[doc = "interrupt pending"]
    #[inline(always)]
    pub fn is_interrupt_pending(&self) -> bool {
        *self == IntstsReg2::InterruptPending
    }
}
#[doc = "Field `INTStsReg2` writer - Interrupt status register"]
pub type IntstsReg2W<'a, REG> = crate::BitWriter<'a, REG, IntstsReg2>;
impl<'a, REG> IntstsReg2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn no_interrupt_pending(self) -> &'a mut crate::W<REG> {
        self.variant(IntstsReg2::NoInterruptPending)
    }
    #[doc = "interrupt pending"]
    #[inline(always)]
    pub fn interrupt_pending(self) -> &'a mut crate::W<REG> {
        self.variant(IntstsReg2::InterruptPending)
    }
}
#[doc = "Field `InputDataReg2` reader - Input data register"]
pub type InputDataReg2R = crate::BitReader;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Data register"]
    #[inline(always)]
    pub fn data_reg2(&self) -> DataReg2R {
        DataReg2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Direction register"]
    #[inline(always)]
    pub fn direction_reg2(&self) -> DirectionReg2R {
        DirectionReg2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable"]
    #[inline(always)]
    pub fn intenbl2(&self) -> Intenbl2R {
        Intenbl2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn intsensitivity_type0sel2(&self) -> IntsensitivityType0sel2R {
        IntsensitivityType0sel2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn intsensitivity_type1sel2(&self) -> IntsensitivityType1sel2R {
        IntsensitivityType1sel2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn intsensitivity_type2sel2(&self) -> IntsensitivityType2sel2R {
        IntsensitivityType2sel2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reset tolerance enable"]
    #[inline(always)]
    pub fn rst_tolerance_enbl2(&self) -> RstToleranceEnbl2R {
        RstToleranceEnbl2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Debounce setting register #1"]
    #[inline(always)]
    pub fn debounce_setting_reg12(&self) -> DebounceSettingReg12R {
        DebounceSettingReg12R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Debounce setting register #2"]
    #[inline(always)]
    pub fn debounce_setting_reg22(&self) -> DebounceSettingReg22R {
        DebounceSettingReg22R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Input mask"]
    #[inline(always)]
    pub fn input_mask2(&self) -> InputMask2R {
        InputMask2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Blink Counter Selection #1"]
    #[inline(always)]
    pub fn blink_counter_sel12(&self) -> BlinkCounterSel12R {
        BlinkCounterSel12R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Blink Counter Selection #2"]
    #[inline(always)]
    pub fn blink_counter_sel22(&self) -> BlinkCounterSel22R {
        BlinkCounterSel22R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt status register"]
    #[inline(always)]
    pub fn intsts_reg2(&self) -> IntstsReg2R {
        IntstsReg2R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Input data register"]
    #[inline(always)]
    pub fn input_data_reg2(&self) -> InputDataReg2R {
        InputDataReg2R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:31 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Data register"]
    #[inline(always)]
    pub fn data_reg2(&mut self) -> DataReg2W<Gpio448Spec> {
        DataReg2W::new(self, 0)
    }
    #[doc = "Bit 1 - Direction register"]
    #[inline(always)]
    pub fn direction_reg2(&mut self) -> DirectionReg2W<Gpio448Spec> {
        DirectionReg2W::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt enable"]
    #[inline(always)]
    pub fn intenbl2(&mut self) -> Intenbl2W<Gpio448Spec> {
        Intenbl2W::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn intsensitivity_type0sel2(&mut self) -> IntsensitivityType0sel2W<Gpio448Spec> {
        IntsensitivityType0sel2W::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn intsensitivity_type1sel2(&mut self) -> IntsensitivityType1sel2W<Gpio448Spec> {
        IntsensitivityType1sel2W::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn intsensitivity_type2sel2(&mut self) -> IntsensitivityType2sel2W<Gpio448Spec> {
        IntsensitivityType2sel2W::new(self, 5)
    }
    #[doc = "Bit 6 - Reset tolerance enable"]
    #[inline(always)]
    pub fn rst_tolerance_enbl2(&mut self) -> RstToleranceEnbl2W<Gpio448Spec> {
        RstToleranceEnbl2W::new(self, 6)
    }
    #[doc = "Bit 7 - Debounce setting register #1"]
    #[inline(always)]
    pub fn debounce_setting_reg12(&mut self) -> DebounceSettingReg12W<Gpio448Spec> {
        DebounceSettingReg12W::new(self, 7)
    }
    #[doc = "Bit 8 - Debounce setting register #2"]
    #[inline(always)]
    pub fn debounce_setting_reg22(&mut self) -> DebounceSettingReg22W<Gpio448Spec> {
        DebounceSettingReg22W::new(self, 8)
    }
    #[doc = "Bit 9 - Input mask"]
    #[inline(always)]
    pub fn input_mask2(&mut self) -> InputMask2W<Gpio448Spec> {
        InputMask2W::new(self, 9)
    }
    #[doc = "Bit 10 - Blink Counter Selection #1"]
    #[inline(always)]
    pub fn blink_counter_sel12(&mut self) -> BlinkCounterSel12W<Gpio448Spec> {
        BlinkCounterSel12W::new(self, 10)
    }
    #[doc = "Bit 11 - Blink Counter Selection #2"]
    #[inline(always)]
    pub fn blink_counter_sel22(&mut self) -> BlinkCounterSel22W<Gpio448Spec> {
        BlinkCounterSel22W::new(self, 11)
    }
    #[doc = "Bit 12 - Interrupt status register"]
    #[inline(always)]
    pub fn intsts_reg2(&mut self) -> IntstsReg2W<Gpio448Spec> {
        IntstsReg2W::new(self, 12)
    }
}
#[doc = "GPIO178 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio448::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio448::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio448Spec;
impl crate::RegisterSpec for Gpio448Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio448::R`](R) reader structure"]
impl crate::Readable for Gpio448Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio448::W`](W) writer structure"]
impl crate::Writable for Gpio448Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO448 to value 0"]
impl crate::Resettable for Gpio448Spec {}
