#[doc = "Register `GPIO19C` reader"]
pub type R = crate::R<Gpio19cSpec>;
#[doc = "Register `GPIO19C` writer"]
pub type W = crate::W<Gpio19cSpec>;
#[doc = "Data register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataReg7 {
    #[doc = "0: Output 0 if Bit1 is 1."]
    Output0IfBit1Is1 = 0,
    #[doc = "1: Output 1 if Bit1 is 1."]
    Output1IfBit1Is1 = 1,
}
impl From<DataReg7> for bool {
    #[inline(always)]
    fn from(variant: DataReg7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DataReg7` reader - Data register"]
pub type DataReg7R = crate::BitReader<DataReg7>;
impl DataReg7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DataReg7 {
        match self.bits {
            false => DataReg7::Output0IfBit1Is1,
            true => DataReg7::Output1IfBit1Is1,
        }
    }
    #[doc = "Output 0 if Bit1 is 1."]
    #[inline(always)]
    pub fn is_output_0_if_bit1_is_1(&self) -> bool {
        *self == DataReg7::Output0IfBit1Is1
    }
    #[doc = "Output 1 if Bit1 is 1."]
    #[inline(always)]
    pub fn is_output_1_if_bit1_is_1(&self) -> bool {
        *self == DataReg7::Output1IfBit1Is1
    }
}
#[doc = "Field `DataReg7` writer - Data register"]
pub type DataReg7W<'a, REG> = crate::BitWriter<'a, REG, DataReg7>;
impl<'a, REG> DataReg7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output 0 if Bit1 is 1."]
    #[inline(always)]
    pub fn output_0_if_bit1_is_1(self) -> &'a mut crate::W<REG> {
        self.variant(DataReg7::Output0IfBit1Is1)
    }
    #[doc = "Output 1 if Bit1 is 1."]
    #[inline(always)]
    pub fn output_1_if_bit1_is_1(self) -> &'a mut crate::W<REG> {
        self.variant(DataReg7::Output1IfBit1Is1)
    }
}
#[doc = "Direction register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirectionReg7 {
    #[doc = "0: Input"]
    Input = 0,
    #[doc = "1: Output"]
    Output = 1,
}
impl From<DirectionReg7> for bool {
    #[inline(always)]
    fn from(variant: DirectionReg7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DirectionReg7` reader - Direction register"]
pub type DirectionReg7R = crate::BitReader<DirectionReg7>;
impl DirectionReg7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DirectionReg7 {
        match self.bits {
            false => DirectionReg7::Input,
            true => DirectionReg7::Output,
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DirectionReg7::Input
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DirectionReg7::Output
    }
}
#[doc = "Field `DirectionReg7` writer - Direction register"]
pub type DirectionReg7W<'a, REG> = crate::BitWriter<'a, REG, DirectionReg7>;
impl<'a, REG> DirectionReg7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(DirectionReg7::Input)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(DirectionReg7::Output)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Intenbl7 {
    #[doc = "0: Disable interrupt"]
    DisableInterrupt = 0,
    #[doc = "1: Enable interrupt"]
    EnableInterrupt = 1,
}
impl From<Intenbl7> for bool {
    #[inline(always)]
    fn from(variant: Intenbl7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTEnbl7` reader - Interrupt enable"]
pub type Intenbl7R = crate::BitReader<Intenbl7>;
impl Intenbl7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Intenbl7 {
        match self.bits {
            false => Intenbl7::DisableInterrupt,
            true => Intenbl7::EnableInterrupt,
        }
    }
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn is_disable_interrupt(&self) -> bool {
        *self == Intenbl7::DisableInterrupt
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn is_enable_interrupt(&self) -> bool {
        *self == Intenbl7::EnableInterrupt
    }
}
#[doc = "Field `INTEnbl7` writer - Interrupt enable"]
pub type Intenbl7W<'a, REG> = crate::BitWriter<'a, REG, Intenbl7>;
impl<'a, REG> Intenbl7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable_interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(Intenbl7::DisableInterrupt)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable_interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(Intenbl7::EnableInterrupt)
    }
}
#[doc = "Interrupt sensitivity type 0 selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntsensitivityType0sel7 {
    #[doc = "0: Select falling-edge or level-low trigger mode"]
    SelectFallingedgeOrLevellowTriggerMode = 0,
    #[doc = "1: Select rising-edge or level-high trigger mode"]
    SelectRisingedgeOrLevelhighTriggerMode = 1,
}
impl From<IntsensitivityType0sel7> for bool {
    #[inline(always)]
    fn from(variant: IntsensitivityType0sel7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTSensitivityType0Sel7` reader - Interrupt sensitivity type 0 selection"]
pub type IntsensitivityType0sel7R = crate::BitReader<IntsensitivityType0sel7>;
impl IntsensitivityType0sel7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntsensitivityType0sel7 {
        match self.bits {
            false => IntsensitivityType0sel7::SelectFallingedgeOrLevellowTriggerMode,
            true => IntsensitivityType0sel7::SelectRisingedgeOrLevelhighTriggerMode,
        }
    }
    #[doc = "Select falling-edge or level-low trigger mode"]
    #[inline(always)]
    pub fn is_select_fallingedge_or_levellow_trigger_mode(&self) -> bool {
        *self == IntsensitivityType0sel7::SelectFallingedgeOrLevellowTriggerMode
    }
    #[doc = "Select rising-edge or level-high trigger mode"]
    #[inline(always)]
    pub fn is_select_risingedge_or_levelhigh_trigger_mode(&self) -> bool {
        *self == IntsensitivityType0sel7::SelectRisingedgeOrLevelhighTriggerMode
    }
}
#[doc = "Field `INTSensitivityType0Sel7` writer - Interrupt sensitivity type 0 selection"]
pub type IntsensitivityType0sel7W<'a, REG> = crate::BitWriter<'a, REG, IntsensitivityType0sel7>;
impl<'a, REG> IntsensitivityType0sel7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select falling-edge or level-low trigger mode"]
    #[inline(always)]
    pub fn select_fallingedge_or_levellow_trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(IntsensitivityType0sel7::SelectFallingedgeOrLevellowTriggerMode)
    }
    #[doc = "Select rising-edge or level-high trigger mode"]
    #[inline(always)]
    pub fn select_risingedge_or_levelhigh_trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(IntsensitivityType0sel7::SelectRisingedgeOrLevelhighTriggerMode)
    }
}
#[doc = "Interrupt sensitivity type 1 selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntsensitivityType1sel7 {
    #[doc = "0: Select edge trigger mode"]
    SelectEdgeTriggerMode = 0,
    #[doc = "1: Select level trigger mode"]
    SelectLevelTriggerMode = 1,
}
impl From<IntsensitivityType1sel7> for bool {
    #[inline(always)]
    fn from(variant: IntsensitivityType1sel7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTSensitivityType1Sel7` reader - Interrupt sensitivity type 1 selection"]
pub type IntsensitivityType1sel7R = crate::BitReader<IntsensitivityType1sel7>;
impl IntsensitivityType1sel7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntsensitivityType1sel7 {
        match self.bits {
            false => IntsensitivityType1sel7::SelectEdgeTriggerMode,
            true => IntsensitivityType1sel7::SelectLevelTriggerMode,
        }
    }
    #[doc = "Select edge trigger mode"]
    #[inline(always)]
    pub fn is_select_edge_trigger_mode(&self) -> bool {
        *self == IntsensitivityType1sel7::SelectEdgeTriggerMode
    }
    #[doc = "Select level trigger mode"]
    #[inline(always)]
    pub fn is_select_level_trigger_mode(&self) -> bool {
        *self == IntsensitivityType1sel7::SelectLevelTriggerMode
    }
}
#[doc = "Field `INTSensitivityType1Sel7` writer - Interrupt sensitivity type 1 selection"]
pub type IntsensitivityType1sel7W<'a, REG> = crate::BitWriter<'a, REG, IntsensitivityType1sel7>;
impl<'a, REG> IntsensitivityType1sel7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select edge trigger mode"]
    #[inline(always)]
    pub fn select_edge_trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(IntsensitivityType1sel7::SelectEdgeTriggerMode)
    }
    #[doc = "Select level trigger mode"]
    #[inline(always)]
    pub fn select_level_trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(IntsensitivityType1sel7::SelectLevelTriggerMode)
    }
}
#[doc = "Interrupt sensitivity type 2 selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntsensitivityType2sel7 {
    #[doc = "0: Select edge or level trigger mode"]
    SelectEdgeOrLevelTriggerMode = 0,
    #[doc = "1: Select dual-edge trigger mode"]
    SelectDualedgeTriggerMode = 1,
}
impl From<IntsensitivityType2sel7> for bool {
    #[inline(always)]
    fn from(variant: IntsensitivityType2sel7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTSensitivityType2Sel7` reader - Interrupt sensitivity type 2 selection"]
pub type IntsensitivityType2sel7R = crate::BitReader<IntsensitivityType2sel7>;
impl IntsensitivityType2sel7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntsensitivityType2sel7 {
        match self.bits {
            false => IntsensitivityType2sel7::SelectEdgeOrLevelTriggerMode,
            true => IntsensitivityType2sel7::SelectDualedgeTriggerMode,
        }
    }
    #[doc = "Select edge or level trigger mode"]
    #[inline(always)]
    pub fn is_select_edge_or_level_trigger_mode(&self) -> bool {
        *self == IntsensitivityType2sel7::SelectEdgeOrLevelTriggerMode
    }
    #[doc = "Select dual-edge trigger mode"]
    #[inline(always)]
    pub fn is_select_dualedge_trigger_mode(&self) -> bool {
        *self == IntsensitivityType2sel7::SelectDualedgeTriggerMode
    }
}
#[doc = "Field `INTSensitivityType2Sel7` writer - Interrupt sensitivity type 2 selection"]
pub type IntsensitivityType2sel7W<'a, REG> = crate::BitWriter<'a, REG, IntsensitivityType2sel7>;
impl<'a, REG> IntsensitivityType2sel7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select edge or level trigger mode"]
    #[inline(always)]
    pub fn select_edge_or_level_trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(IntsensitivityType2sel7::SelectEdgeOrLevelTriggerMode)
    }
    #[doc = "Select dual-edge trigger mode"]
    #[inline(always)]
    pub fn select_dualedge_trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(IntsensitivityType2sel7::SelectDualedgeTriggerMode)
    }
}
#[doc = "Reset tolerance enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RstToleranceEnbl7 {
    #[doc = "0: Bit0 and Bit1 will be reset by WDT\\_SOC and/or EXTRSTN reset"]
    Bit0AndBit1WillBeResetByWdtsocAndorExtrstnReset = 0,
    #[doc = "1: Bit0 and Bit1 will not be reset by WDT\\_SOC and/or EXTRSTN"]
    Bit0AndBit1WillNotBeResetByWdtsocAndorExtrstn = 1,
}
impl From<RstToleranceEnbl7> for bool {
    #[inline(always)]
    fn from(variant: RstToleranceEnbl7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RstToleranceEnbl7` reader - Reset tolerance enable"]
pub type RstToleranceEnbl7R = crate::BitReader<RstToleranceEnbl7>;
impl RstToleranceEnbl7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RstToleranceEnbl7 {
        match self.bits {
            false => RstToleranceEnbl7::Bit0AndBit1WillBeResetByWdtsocAndorExtrstnReset,
            true => RstToleranceEnbl7::Bit0AndBit1WillNotBeResetByWdtsocAndorExtrstn,
        }
    }
    #[doc = "Bit0 and Bit1 will be reset by WDT\\_SOC and/or EXTRSTN reset"]
    #[inline(always)]
    pub fn is_bit0_and_bit1_will_be_reset_by_wdtsoc_andor_extrstn_reset(&self) -> bool {
        *self == RstToleranceEnbl7::Bit0AndBit1WillBeResetByWdtsocAndorExtrstnReset
    }
    #[doc = "Bit0 and Bit1 will not be reset by WDT\\_SOC and/or EXTRSTN"]
    #[inline(always)]
    pub fn is_bit0_and_bit1_will_not_be_reset_by_wdtsoc_andor_extrstn(&self) -> bool {
        *self == RstToleranceEnbl7::Bit0AndBit1WillNotBeResetByWdtsocAndorExtrstn
    }
}
#[doc = "Field `RstToleranceEnbl7` writer - Reset tolerance enable"]
pub type RstToleranceEnbl7W<'a, REG> = crate::BitWriter<'a, REG, RstToleranceEnbl7>;
impl<'a, REG> RstToleranceEnbl7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bit0 and Bit1 will be reset by WDT\\_SOC and/or EXTRSTN reset"]
    #[inline(always)]
    pub fn bit0_and_bit1_will_be_reset_by_wdtsoc_andor_extrstn_reset(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(RstToleranceEnbl7::Bit0AndBit1WillBeResetByWdtsocAndorExtrstnReset)
    }
    #[doc = "Bit0 and Bit1 will not be reset by WDT\\_SOC and/or EXTRSTN"]
    #[inline(always)]
    pub fn bit0_and_bit1_will_not_be_reset_by_wdtsoc_andor_extrstn(self) -> &'a mut crate::W<REG> {
        self.variant(RstToleranceEnbl7::Bit0AndBit1WillNotBeResetByWdtsocAndorExtrstn)
    }
}
#[doc = "Field `DebounceSettingReg17` reader - Debounce setting register #1"]
pub type DebounceSettingReg17R = crate::BitReader;
#[doc = "Field `DebounceSettingReg17` writer - Debounce setting register #1"]
pub type DebounceSettingReg17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DebounceSettingReg27` reader - Debounce setting register #2"]
pub type DebounceSettingReg27R = crate::BitReader;
#[doc = "Field `DebounceSettingReg27` writer - Debounce setting register #2"]
pub type DebounceSettingReg27W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Input mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputMask7 {
    #[doc = "0: Read from Bit13 will be updated."]
    ReadFromBit13WillBeUpdated = 0,
    #[doc = "1: Read from Bit13 will not be updated."]
    ReadFromBit13WillNotBeUpdated = 1,
}
impl From<InputMask7> for bool {
    #[inline(always)]
    fn from(variant: InputMask7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `InputMask7` reader - Input mask"]
pub type InputMask7R = crate::BitReader<InputMask7>;
impl InputMask7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> InputMask7 {
        match self.bits {
            false => InputMask7::ReadFromBit13WillBeUpdated,
            true => InputMask7::ReadFromBit13WillNotBeUpdated,
        }
    }
    #[doc = "Read from Bit13 will be updated."]
    #[inline(always)]
    pub fn is_read_from_bit13_will_be_updated(&self) -> bool {
        *self == InputMask7::ReadFromBit13WillBeUpdated
    }
    #[doc = "Read from Bit13 will not be updated."]
    #[inline(always)]
    pub fn is_read_from_bit13_will_not_be_updated(&self) -> bool {
        *self == InputMask7::ReadFromBit13WillNotBeUpdated
    }
}
#[doc = "Field `InputMask7` writer - Input mask"]
pub type InputMask7W<'a, REG> = crate::BitWriter<'a, REG, InputMask7>;
impl<'a, REG> InputMask7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read from Bit13 will be updated."]
    #[inline(always)]
    pub fn read_from_bit13_will_be_updated(self) -> &'a mut crate::W<REG> {
        self.variant(InputMask7::ReadFromBit13WillBeUpdated)
    }
    #[doc = "Read from Bit13 will not be updated."]
    #[inline(always)]
    pub fn read_from_bit13_will_not_be_updated(self) -> &'a mut crate::W<REG> {
        self.variant(InputMask7::ReadFromBit13WillNotBeUpdated)
    }
}
#[doc = "Field `BlinkCounterSel17` reader - Blink Counter Selection #1"]
pub type BlinkCounterSel17R = crate::BitReader;
#[doc = "Field `BlinkCounterSel17` writer - Blink Counter Selection #1"]
pub type BlinkCounterSel17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BlinkCounterSel27` reader - Blink Counter Selection #2"]
pub type BlinkCounterSel27R = crate::BitReader;
#[doc = "Field `BlinkCounterSel27` writer - Blink Counter Selection #2"]
pub type BlinkCounterSel27W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt status register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntstsReg7 {
    #[doc = "0: No interrupt pending"]
    NoInterruptPending = 0,
    #[doc = "1: interrupt pending"]
    InterruptPending = 1,
}
impl From<IntstsReg7> for bool {
    #[inline(always)]
    fn from(variant: IntstsReg7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTStsReg7` reader - Interrupt status register"]
pub type IntstsReg7R = crate::BitReader<IntstsReg7>;
impl IntstsReg7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntstsReg7 {
        match self.bits {
            false => IntstsReg7::NoInterruptPending,
            true => IntstsReg7::InterruptPending,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_no_interrupt_pending(&self) -> bool {
        *self == IntstsReg7::NoInterruptPending
    }
    #[doc = "interrupt pending"]
    #[inline(always)]
    pub fn is_interrupt_pending(&self) -> bool {
        *self == IntstsReg7::InterruptPending
    }
}
#[doc = "Field `INTStsReg7` writer - Interrupt status register"]
pub type IntstsReg7W<'a, REG> = crate::BitWriter<'a, REG, IntstsReg7>;
impl<'a, REG> IntstsReg7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn no_interrupt_pending(self) -> &'a mut crate::W<REG> {
        self.variant(IntstsReg7::NoInterruptPending)
    }
    #[doc = "interrupt pending"]
    #[inline(always)]
    pub fn interrupt_pending(self) -> &'a mut crate::W<REG> {
        self.variant(IntstsReg7::InterruptPending)
    }
}
#[doc = "Field `InputDataReg7` reader - Input data register"]
pub type InputDataReg7R = crate::BitReader;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Data register"]
    #[inline(always)]
    pub fn data_reg7(&self) -> DataReg7R {
        DataReg7R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Direction register"]
    #[inline(always)]
    pub fn direction_reg7(&self) -> DirectionReg7R {
        DirectionReg7R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable"]
    #[inline(always)]
    pub fn intenbl7(&self) -> Intenbl7R {
        Intenbl7R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn intsensitivity_type0sel7(&self) -> IntsensitivityType0sel7R {
        IntsensitivityType0sel7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn intsensitivity_type1sel7(&self) -> IntsensitivityType1sel7R {
        IntsensitivityType1sel7R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn intsensitivity_type2sel7(&self) -> IntsensitivityType2sel7R {
        IntsensitivityType2sel7R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reset tolerance enable"]
    #[inline(always)]
    pub fn rst_tolerance_enbl7(&self) -> RstToleranceEnbl7R {
        RstToleranceEnbl7R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Debounce setting register #1"]
    #[inline(always)]
    pub fn debounce_setting_reg17(&self) -> DebounceSettingReg17R {
        DebounceSettingReg17R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Debounce setting register #2"]
    #[inline(always)]
    pub fn debounce_setting_reg27(&self) -> DebounceSettingReg27R {
        DebounceSettingReg27R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Input mask"]
    #[inline(always)]
    pub fn input_mask7(&self) -> InputMask7R {
        InputMask7R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Blink Counter Selection #1"]
    #[inline(always)]
    pub fn blink_counter_sel17(&self) -> BlinkCounterSel17R {
        BlinkCounterSel17R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Blink Counter Selection #2"]
    #[inline(always)]
    pub fn blink_counter_sel27(&self) -> BlinkCounterSel27R {
        BlinkCounterSel27R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt status register"]
    #[inline(always)]
    pub fn intsts_reg7(&self) -> IntstsReg7R {
        IntstsReg7R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Input data register"]
    #[inline(always)]
    pub fn input_data_reg7(&self) -> InputDataReg7R {
        InputDataReg7R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:31 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Data register"]
    #[inline(always)]
    pub fn data_reg7(&mut self) -> DataReg7W<Gpio19cSpec> {
        DataReg7W::new(self, 0)
    }
    #[doc = "Bit 1 - Direction register"]
    #[inline(always)]
    pub fn direction_reg7(&mut self) -> DirectionReg7W<Gpio19cSpec> {
        DirectionReg7W::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt enable"]
    #[inline(always)]
    pub fn intenbl7(&mut self) -> Intenbl7W<Gpio19cSpec> {
        Intenbl7W::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn intsensitivity_type0sel7(&mut self) -> IntsensitivityType0sel7W<Gpio19cSpec> {
        IntsensitivityType0sel7W::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn intsensitivity_type1sel7(&mut self) -> IntsensitivityType1sel7W<Gpio19cSpec> {
        IntsensitivityType1sel7W::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn intsensitivity_type2sel7(&mut self) -> IntsensitivityType2sel7W<Gpio19cSpec> {
        IntsensitivityType2sel7W::new(self, 5)
    }
    #[doc = "Bit 6 - Reset tolerance enable"]
    #[inline(always)]
    pub fn rst_tolerance_enbl7(&mut self) -> RstToleranceEnbl7W<Gpio19cSpec> {
        RstToleranceEnbl7W::new(self, 6)
    }
    #[doc = "Bit 7 - Debounce setting register #1"]
    #[inline(always)]
    pub fn debounce_setting_reg17(&mut self) -> DebounceSettingReg17W<Gpio19cSpec> {
        DebounceSettingReg17W::new(self, 7)
    }
    #[doc = "Bit 8 - Debounce setting register #2"]
    #[inline(always)]
    pub fn debounce_setting_reg27(&mut self) -> DebounceSettingReg27W<Gpio19cSpec> {
        DebounceSettingReg27W::new(self, 8)
    }
    #[doc = "Bit 9 - Input mask"]
    #[inline(always)]
    pub fn input_mask7(&mut self) -> InputMask7W<Gpio19cSpec> {
        InputMask7W::new(self, 9)
    }
    #[doc = "Bit 10 - Blink Counter Selection #1"]
    #[inline(always)]
    pub fn blink_counter_sel17(&mut self) -> BlinkCounterSel17W<Gpio19cSpec> {
        BlinkCounterSel17W::new(self, 10)
    }
    #[doc = "Bit 11 - Blink Counter Selection #2"]
    #[inline(always)]
    pub fn blink_counter_sel27(&mut self) -> BlinkCounterSel27W<Gpio19cSpec> {
        BlinkCounterSel27W::new(self, 11)
    }
    #[doc = "Bit 12 - Interrupt status register"]
    #[inline(always)]
    pub fn intsts_reg7(&mut self) -> IntstsReg7W<Gpio19cSpec> {
        IntstsReg7W::new(self, 12)
    }
}
#[doc = "GPIO007 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio19c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio19c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio19cSpec;
impl crate::RegisterSpec for Gpio19cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio19c::R`](R) reader structure"]
impl crate::Readable for Gpio19cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio19c::W`](W) writer structure"]
impl crate::Writable for Gpio19cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO19C to value 0"]
impl crate::Resettable for Gpio19cSpec {}
