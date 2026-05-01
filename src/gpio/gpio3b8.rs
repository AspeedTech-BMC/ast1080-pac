#[doc = "Register `GPIO3B8` reader"]
pub type R = crate::R<Gpio3b8Spec>;
#[doc = "Register `GPIO3B8` writer"]
pub type W = crate::W<Gpio3b8Spec>;
#[doc = "Data register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataReg6 {
    #[doc = "0: Output 0 if Bit1 is 1."]
    Output0IfBit1Is1 = 0,
    #[doc = "1: Output 1 if Bit1 is 1."]
    Output1IfBit1Is1 = 1,
}
impl From<DataReg6> for bool {
    #[inline(always)]
    fn from(variant: DataReg6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DataReg6` reader - Data register"]
pub type DataReg6R = crate::BitReader<DataReg6>;
impl DataReg6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DataReg6 {
        match self.bits {
            false => DataReg6::Output0IfBit1Is1,
            true => DataReg6::Output1IfBit1Is1,
        }
    }
    #[doc = "Output 0 if Bit1 is 1."]
    #[inline(always)]
    pub fn is_output_0_if_bit1_is_1(&self) -> bool {
        *self == DataReg6::Output0IfBit1Is1
    }
    #[doc = "Output 1 if Bit1 is 1."]
    #[inline(always)]
    pub fn is_output_1_if_bit1_is_1(&self) -> bool {
        *self == DataReg6::Output1IfBit1Is1
    }
}
#[doc = "Field `DataReg6` writer - Data register"]
pub type DataReg6W<'a, REG> = crate::BitWriter<'a, REG, DataReg6>;
impl<'a, REG> DataReg6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output 0 if Bit1 is 1."]
    #[inline(always)]
    pub fn output_0_if_bit1_is_1(self) -> &'a mut crate::W<REG> {
        self.variant(DataReg6::Output0IfBit1Is1)
    }
    #[doc = "Output 1 if Bit1 is 1."]
    #[inline(always)]
    pub fn output_1_if_bit1_is_1(self) -> &'a mut crate::W<REG> {
        self.variant(DataReg6::Output1IfBit1Is1)
    }
}
#[doc = "Direction register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirectionReg6 {
    #[doc = "0: Input"]
    Input = 0,
    #[doc = "1: Output"]
    Output = 1,
}
impl From<DirectionReg6> for bool {
    #[inline(always)]
    fn from(variant: DirectionReg6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DirectionReg6` reader - Direction register"]
pub type DirectionReg6R = crate::BitReader<DirectionReg6>;
impl DirectionReg6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DirectionReg6 {
        match self.bits {
            false => DirectionReg6::Input,
            true => DirectionReg6::Output,
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DirectionReg6::Input
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DirectionReg6::Output
    }
}
#[doc = "Field `DirectionReg6` writer - Direction register"]
pub type DirectionReg6W<'a, REG> = crate::BitWriter<'a, REG, DirectionReg6>;
impl<'a, REG> DirectionReg6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(DirectionReg6::Input)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(DirectionReg6::Output)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Intenbl6 {
    #[doc = "0: Disable interrupt"]
    DisableInterrupt = 0,
    #[doc = "1: Enable interrupt"]
    EnableInterrupt = 1,
}
impl From<Intenbl6> for bool {
    #[inline(always)]
    fn from(variant: Intenbl6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTEnbl6` reader - Interrupt enable"]
pub type Intenbl6R = crate::BitReader<Intenbl6>;
impl Intenbl6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Intenbl6 {
        match self.bits {
            false => Intenbl6::DisableInterrupt,
            true => Intenbl6::EnableInterrupt,
        }
    }
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn is_disable_interrupt(&self) -> bool {
        *self == Intenbl6::DisableInterrupt
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn is_enable_interrupt(&self) -> bool {
        *self == Intenbl6::EnableInterrupt
    }
}
#[doc = "Field `INTEnbl6` writer - Interrupt enable"]
pub type Intenbl6W<'a, REG> = crate::BitWriter<'a, REG, Intenbl6>;
impl<'a, REG> Intenbl6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable_interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(Intenbl6::DisableInterrupt)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable_interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(Intenbl6::EnableInterrupt)
    }
}
#[doc = "Interrupt sensitivity type 0 selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntsensitivityType0sel6 {
    #[doc = "0: Select falling-edge or level-low trigger mode"]
    SelectFallingedgeOrLevellowTriggerMode = 0,
    #[doc = "1: Select rising-edge or level-high trigger mode"]
    SelectRisingedgeOrLevelhighTriggerMode = 1,
}
impl From<IntsensitivityType0sel6> for bool {
    #[inline(always)]
    fn from(variant: IntsensitivityType0sel6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTSensitivityType0Sel6` reader - Interrupt sensitivity type 0 selection"]
pub type IntsensitivityType0sel6R = crate::BitReader<IntsensitivityType0sel6>;
impl IntsensitivityType0sel6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntsensitivityType0sel6 {
        match self.bits {
            false => IntsensitivityType0sel6::SelectFallingedgeOrLevellowTriggerMode,
            true => IntsensitivityType0sel6::SelectRisingedgeOrLevelhighTriggerMode,
        }
    }
    #[doc = "Select falling-edge or level-low trigger mode"]
    #[inline(always)]
    pub fn is_select_fallingedge_or_levellow_trigger_mode(&self) -> bool {
        *self == IntsensitivityType0sel6::SelectFallingedgeOrLevellowTriggerMode
    }
    #[doc = "Select rising-edge or level-high trigger mode"]
    #[inline(always)]
    pub fn is_select_risingedge_or_levelhigh_trigger_mode(&self) -> bool {
        *self == IntsensitivityType0sel6::SelectRisingedgeOrLevelhighTriggerMode
    }
}
#[doc = "Field `INTSensitivityType0Sel6` writer - Interrupt sensitivity type 0 selection"]
pub type IntsensitivityType0sel6W<'a, REG> = crate::BitWriter<'a, REG, IntsensitivityType0sel6>;
impl<'a, REG> IntsensitivityType0sel6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select falling-edge or level-low trigger mode"]
    #[inline(always)]
    pub fn select_fallingedge_or_levellow_trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(IntsensitivityType0sel6::SelectFallingedgeOrLevellowTriggerMode)
    }
    #[doc = "Select rising-edge or level-high trigger mode"]
    #[inline(always)]
    pub fn select_risingedge_or_levelhigh_trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(IntsensitivityType0sel6::SelectRisingedgeOrLevelhighTriggerMode)
    }
}
#[doc = "Interrupt sensitivity type 1 selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntsensitivityType1sel6 {
    #[doc = "0: Select edge trigger mode"]
    SelectEdgeTriggerMode = 0,
    #[doc = "1: Select level trigger mode"]
    SelectLevelTriggerMode = 1,
}
impl From<IntsensitivityType1sel6> for bool {
    #[inline(always)]
    fn from(variant: IntsensitivityType1sel6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTSensitivityType1Sel6` reader - Interrupt sensitivity type 1 selection"]
pub type IntsensitivityType1sel6R = crate::BitReader<IntsensitivityType1sel6>;
impl IntsensitivityType1sel6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntsensitivityType1sel6 {
        match self.bits {
            false => IntsensitivityType1sel6::SelectEdgeTriggerMode,
            true => IntsensitivityType1sel6::SelectLevelTriggerMode,
        }
    }
    #[doc = "Select edge trigger mode"]
    #[inline(always)]
    pub fn is_select_edge_trigger_mode(&self) -> bool {
        *self == IntsensitivityType1sel6::SelectEdgeTriggerMode
    }
    #[doc = "Select level trigger mode"]
    #[inline(always)]
    pub fn is_select_level_trigger_mode(&self) -> bool {
        *self == IntsensitivityType1sel6::SelectLevelTriggerMode
    }
}
#[doc = "Field `INTSensitivityType1Sel6` writer - Interrupt sensitivity type 1 selection"]
pub type IntsensitivityType1sel6W<'a, REG> = crate::BitWriter<'a, REG, IntsensitivityType1sel6>;
impl<'a, REG> IntsensitivityType1sel6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select edge trigger mode"]
    #[inline(always)]
    pub fn select_edge_trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(IntsensitivityType1sel6::SelectEdgeTriggerMode)
    }
    #[doc = "Select level trigger mode"]
    #[inline(always)]
    pub fn select_level_trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(IntsensitivityType1sel6::SelectLevelTriggerMode)
    }
}
#[doc = "Interrupt sensitivity type 2 selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntsensitivityType2sel6 {
    #[doc = "0: Select edge or level trigger mode"]
    SelectEdgeOrLevelTriggerMode = 0,
    #[doc = "1: Select dual-edge trigger mode"]
    SelectDualedgeTriggerMode = 1,
}
impl From<IntsensitivityType2sel6> for bool {
    #[inline(always)]
    fn from(variant: IntsensitivityType2sel6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTSensitivityType2Sel6` reader - Interrupt sensitivity type 2 selection"]
pub type IntsensitivityType2sel6R = crate::BitReader<IntsensitivityType2sel6>;
impl IntsensitivityType2sel6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntsensitivityType2sel6 {
        match self.bits {
            false => IntsensitivityType2sel6::SelectEdgeOrLevelTriggerMode,
            true => IntsensitivityType2sel6::SelectDualedgeTriggerMode,
        }
    }
    #[doc = "Select edge or level trigger mode"]
    #[inline(always)]
    pub fn is_select_edge_or_level_trigger_mode(&self) -> bool {
        *self == IntsensitivityType2sel6::SelectEdgeOrLevelTriggerMode
    }
    #[doc = "Select dual-edge trigger mode"]
    #[inline(always)]
    pub fn is_select_dualedge_trigger_mode(&self) -> bool {
        *self == IntsensitivityType2sel6::SelectDualedgeTriggerMode
    }
}
#[doc = "Field `INTSensitivityType2Sel6` writer - Interrupt sensitivity type 2 selection"]
pub type IntsensitivityType2sel6W<'a, REG> = crate::BitWriter<'a, REG, IntsensitivityType2sel6>;
impl<'a, REG> IntsensitivityType2sel6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select edge or level trigger mode"]
    #[inline(always)]
    pub fn select_edge_or_level_trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(IntsensitivityType2sel6::SelectEdgeOrLevelTriggerMode)
    }
    #[doc = "Select dual-edge trigger mode"]
    #[inline(always)]
    pub fn select_dualedge_trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(IntsensitivityType2sel6::SelectDualedgeTriggerMode)
    }
}
#[doc = "Reset tolerance enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RstToleranceEnbl6 {
    #[doc = "0: Bit0 and Bit1 will be reset by WDT\\_SOC and/or EXTRSTN reset"]
    Bit0AndBit1WillBeResetByWdtsocAndorExtrstnReset = 0,
    #[doc = "1: Bit0 and Bit1 will not be reset by WDT\\_SOC and/or EXTRSTN"]
    Bit0AndBit1WillNotBeResetByWdtsocAndorExtrstn = 1,
}
impl From<RstToleranceEnbl6> for bool {
    #[inline(always)]
    fn from(variant: RstToleranceEnbl6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RstToleranceEnbl6` reader - Reset tolerance enable"]
pub type RstToleranceEnbl6R = crate::BitReader<RstToleranceEnbl6>;
impl RstToleranceEnbl6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RstToleranceEnbl6 {
        match self.bits {
            false => RstToleranceEnbl6::Bit0AndBit1WillBeResetByWdtsocAndorExtrstnReset,
            true => RstToleranceEnbl6::Bit0AndBit1WillNotBeResetByWdtsocAndorExtrstn,
        }
    }
    #[doc = "Bit0 and Bit1 will be reset by WDT\\_SOC and/or EXTRSTN reset"]
    #[inline(always)]
    pub fn is_bit0_and_bit1_will_be_reset_by_wdtsoc_andor_extrstn_reset(&self) -> bool {
        *self == RstToleranceEnbl6::Bit0AndBit1WillBeResetByWdtsocAndorExtrstnReset
    }
    #[doc = "Bit0 and Bit1 will not be reset by WDT\\_SOC and/or EXTRSTN"]
    #[inline(always)]
    pub fn is_bit0_and_bit1_will_not_be_reset_by_wdtsoc_andor_extrstn(&self) -> bool {
        *self == RstToleranceEnbl6::Bit0AndBit1WillNotBeResetByWdtsocAndorExtrstn
    }
}
#[doc = "Field `RstToleranceEnbl6` writer - Reset tolerance enable"]
pub type RstToleranceEnbl6W<'a, REG> = crate::BitWriter<'a, REG, RstToleranceEnbl6>;
impl<'a, REG> RstToleranceEnbl6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bit0 and Bit1 will be reset by WDT\\_SOC and/or EXTRSTN reset"]
    #[inline(always)]
    pub fn bit0_and_bit1_will_be_reset_by_wdtsoc_andor_extrstn_reset(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(RstToleranceEnbl6::Bit0AndBit1WillBeResetByWdtsocAndorExtrstnReset)
    }
    #[doc = "Bit0 and Bit1 will not be reset by WDT\\_SOC and/or EXTRSTN"]
    #[inline(always)]
    pub fn bit0_and_bit1_will_not_be_reset_by_wdtsoc_andor_extrstn(self) -> &'a mut crate::W<REG> {
        self.variant(RstToleranceEnbl6::Bit0AndBit1WillNotBeResetByWdtsocAndorExtrstn)
    }
}
#[doc = "Field `DebounceSettingReg16` reader - Debounce setting register #1"]
pub type DebounceSettingReg16R = crate::BitReader;
#[doc = "Field `DebounceSettingReg16` writer - Debounce setting register #1"]
pub type DebounceSettingReg16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DebounceSettingReg26` reader - Debounce setting register #2"]
pub type DebounceSettingReg26R = crate::BitReader;
#[doc = "Field `DebounceSettingReg26` writer - Debounce setting register #2"]
pub type DebounceSettingReg26W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Input mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputMask6 {
    #[doc = "0: Read from Bit13 will be updated."]
    ReadFromBit13WillBeUpdated = 0,
    #[doc = "1: Read from Bit13 will not be updated."]
    ReadFromBit13WillNotBeUpdated = 1,
}
impl From<InputMask6> for bool {
    #[inline(always)]
    fn from(variant: InputMask6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `InputMask6` reader - Input mask"]
pub type InputMask6R = crate::BitReader<InputMask6>;
impl InputMask6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> InputMask6 {
        match self.bits {
            false => InputMask6::ReadFromBit13WillBeUpdated,
            true => InputMask6::ReadFromBit13WillNotBeUpdated,
        }
    }
    #[doc = "Read from Bit13 will be updated."]
    #[inline(always)]
    pub fn is_read_from_bit13_will_be_updated(&self) -> bool {
        *self == InputMask6::ReadFromBit13WillBeUpdated
    }
    #[doc = "Read from Bit13 will not be updated."]
    #[inline(always)]
    pub fn is_read_from_bit13_will_not_be_updated(&self) -> bool {
        *self == InputMask6::ReadFromBit13WillNotBeUpdated
    }
}
#[doc = "Field `InputMask6` writer - Input mask"]
pub type InputMask6W<'a, REG> = crate::BitWriter<'a, REG, InputMask6>;
impl<'a, REG> InputMask6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read from Bit13 will be updated."]
    #[inline(always)]
    pub fn read_from_bit13_will_be_updated(self) -> &'a mut crate::W<REG> {
        self.variant(InputMask6::ReadFromBit13WillBeUpdated)
    }
    #[doc = "Read from Bit13 will not be updated."]
    #[inline(always)]
    pub fn read_from_bit13_will_not_be_updated(self) -> &'a mut crate::W<REG> {
        self.variant(InputMask6::ReadFromBit13WillNotBeUpdated)
    }
}
#[doc = "Field `BlinkCounterSel16` reader - Blink Counter Selection #1"]
pub type BlinkCounterSel16R = crate::BitReader;
#[doc = "Field `BlinkCounterSel16` writer - Blink Counter Selection #1"]
pub type BlinkCounterSel16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BlinkCounterSel26` reader - Blink Counter Selection #2"]
pub type BlinkCounterSel26R = crate::BitReader;
#[doc = "Field `BlinkCounterSel26` writer - Blink Counter Selection #2"]
pub type BlinkCounterSel26W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt status register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntstsReg6 {
    #[doc = "0: No interrupt pending"]
    NoInterruptPending = 0,
    #[doc = "1: interrupt pending"]
    InterruptPending = 1,
}
impl From<IntstsReg6> for bool {
    #[inline(always)]
    fn from(variant: IntstsReg6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTStsReg6` reader - Interrupt status register"]
pub type IntstsReg6R = crate::BitReader<IntstsReg6>;
impl IntstsReg6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntstsReg6 {
        match self.bits {
            false => IntstsReg6::NoInterruptPending,
            true => IntstsReg6::InterruptPending,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_no_interrupt_pending(&self) -> bool {
        *self == IntstsReg6::NoInterruptPending
    }
    #[doc = "interrupt pending"]
    #[inline(always)]
    pub fn is_interrupt_pending(&self) -> bool {
        *self == IntstsReg6::InterruptPending
    }
}
#[doc = "Field `INTStsReg6` writer - Interrupt status register"]
pub type IntstsReg6W<'a, REG> = crate::BitWriter<'a, REG, IntstsReg6>;
impl<'a, REG> IntstsReg6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn no_interrupt_pending(self) -> &'a mut crate::W<REG> {
        self.variant(IntstsReg6::NoInterruptPending)
    }
    #[doc = "interrupt pending"]
    #[inline(always)]
    pub fn interrupt_pending(self) -> &'a mut crate::W<REG> {
        self.variant(IntstsReg6::InterruptPending)
    }
}
#[doc = "Field `InputDataReg6` reader - Input data register"]
pub type InputDataReg6R = crate::BitReader;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Data register"]
    #[inline(always)]
    pub fn data_reg6(&self) -> DataReg6R {
        DataReg6R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Direction register"]
    #[inline(always)]
    pub fn direction_reg6(&self) -> DirectionReg6R {
        DirectionReg6R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable"]
    #[inline(always)]
    pub fn intenbl6(&self) -> Intenbl6R {
        Intenbl6R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn intsensitivity_type0sel6(&self) -> IntsensitivityType0sel6R {
        IntsensitivityType0sel6R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn intsensitivity_type1sel6(&self) -> IntsensitivityType1sel6R {
        IntsensitivityType1sel6R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn intsensitivity_type2sel6(&self) -> IntsensitivityType2sel6R {
        IntsensitivityType2sel6R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reset tolerance enable"]
    #[inline(always)]
    pub fn rst_tolerance_enbl6(&self) -> RstToleranceEnbl6R {
        RstToleranceEnbl6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Debounce setting register #1"]
    #[inline(always)]
    pub fn debounce_setting_reg16(&self) -> DebounceSettingReg16R {
        DebounceSettingReg16R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Debounce setting register #2"]
    #[inline(always)]
    pub fn debounce_setting_reg26(&self) -> DebounceSettingReg26R {
        DebounceSettingReg26R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Input mask"]
    #[inline(always)]
    pub fn input_mask6(&self) -> InputMask6R {
        InputMask6R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Blink Counter Selection #1"]
    #[inline(always)]
    pub fn blink_counter_sel16(&self) -> BlinkCounterSel16R {
        BlinkCounterSel16R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Blink Counter Selection #2"]
    #[inline(always)]
    pub fn blink_counter_sel26(&self) -> BlinkCounterSel26R {
        BlinkCounterSel26R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt status register"]
    #[inline(always)]
    pub fn intsts_reg6(&self) -> IntstsReg6R {
        IntstsReg6R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Input data register"]
    #[inline(always)]
    pub fn input_data_reg6(&self) -> InputDataReg6R {
        InputDataReg6R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:31 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Data register"]
    #[inline(always)]
    pub fn data_reg6(&mut self) -> DataReg6W<Gpio3b8Spec> {
        DataReg6W::new(self, 0)
    }
    #[doc = "Bit 1 - Direction register"]
    #[inline(always)]
    pub fn direction_reg6(&mut self) -> DirectionReg6W<Gpio3b8Spec> {
        DirectionReg6W::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt enable"]
    #[inline(always)]
    pub fn intenbl6(&mut self) -> Intenbl6W<Gpio3b8Spec> {
        Intenbl6W::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn intsensitivity_type0sel6(&mut self) -> IntsensitivityType0sel6W<Gpio3b8Spec> {
        IntsensitivityType0sel6W::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn intsensitivity_type1sel6(&mut self) -> IntsensitivityType1sel6W<Gpio3b8Spec> {
        IntsensitivityType1sel6W::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn intsensitivity_type2sel6(&mut self) -> IntsensitivityType2sel6W<Gpio3b8Spec> {
        IntsensitivityType2sel6W::new(self, 5)
    }
    #[doc = "Bit 6 - Reset tolerance enable"]
    #[inline(always)]
    pub fn rst_tolerance_enbl6(&mut self) -> RstToleranceEnbl6W<Gpio3b8Spec> {
        RstToleranceEnbl6W::new(self, 6)
    }
    #[doc = "Bit 7 - Debounce setting register #1"]
    #[inline(always)]
    pub fn debounce_setting_reg16(&mut self) -> DebounceSettingReg16W<Gpio3b8Spec> {
        DebounceSettingReg16W::new(self, 7)
    }
    #[doc = "Bit 8 - Debounce setting register #2"]
    #[inline(always)]
    pub fn debounce_setting_reg26(&mut self) -> DebounceSettingReg26W<Gpio3b8Spec> {
        DebounceSettingReg26W::new(self, 8)
    }
    #[doc = "Bit 9 - Input mask"]
    #[inline(always)]
    pub fn input_mask6(&mut self) -> InputMask6W<Gpio3b8Spec> {
        InputMask6W::new(self, 9)
    }
    #[doc = "Bit 10 - Blink Counter Selection #1"]
    #[inline(always)]
    pub fn blink_counter_sel16(&mut self) -> BlinkCounterSel16W<Gpio3b8Spec> {
        BlinkCounterSel16W::new(self, 10)
    }
    #[doc = "Bit 11 - Blink Counter Selection #2"]
    #[inline(always)]
    pub fn blink_counter_sel26(&mut self) -> BlinkCounterSel26W<Gpio3b8Spec> {
        BlinkCounterSel26W::new(self, 11)
    }
    #[doc = "Bit 12 - Interrupt status register"]
    #[inline(always)]
    pub fn intsts_reg6(&mut self) -> IntstsReg6W<Gpio3b8Spec> {
        IntstsReg6W::new(self, 12)
    }
}
#[doc = "GPIO142 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio3b8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio3b8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio3b8Spec;
impl crate::RegisterSpec for Gpio3b8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio3b8::R`](R) reader structure"]
impl crate::Readable for Gpio3b8Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio3b8::W`](W) writer structure"]
impl crate::Writable for Gpio3b8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO3B8 to value 0"]
impl crate::Resettable for Gpio3b8Spec {}
