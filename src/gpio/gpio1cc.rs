#[doc = "Register `GPIO1CC` reader"]
pub type R = crate::R<Gpio1ccSpec>;
#[doc = "Register `GPIO1CC` writer"]
pub type W = crate::W<Gpio1ccSpec>;
#[doc = "Data register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataReg3 {
    #[doc = "0: Output 0 if Bit1 is 1."]
    Output0IfBit1Is1 = 0,
    #[doc = "1: Output 1 if Bit1 is 1."]
    Output1IfBit1Is1 = 1,
}
impl From<DataReg3> for bool {
    #[inline(always)]
    fn from(variant: DataReg3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DataReg3` reader - Data register"]
pub type DataReg3R = crate::BitReader<DataReg3>;
impl DataReg3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DataReg3 {
        match self.bits {
            false => DataReg3::Output0IfBit1Is1,
            true => DataReg3::Output1IfBit1Is1,
        }
    }
    #[doc = "Output 0 if Bit1 is 1."]
    #[inline(always)]
    pub fn is_output_0_if_bit1_is_1(&self) -> bool {
        *self == DataReg3::Output0IfBit1Is1
    }
    #[doc = "Output 1 if Bit1 is 1."]
    #[inline(always)]
    pub fn is_output_1_if_bit1_is_1(&self) -> bool {
        *self == DataReg3::Output1IfBit1Is1
    }
}
#[doc = "Field `DataReg3` writer - Data register"]
pub type DataReg3W<'a, REG> = crate::BitWriter<'a, REG, DataReg3>;
impl<'a, REG> DataReg3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output 0 if Bit1 is 1."]
    #[inline(always)]
    pub fn output_0_if_bit1_is_1(self) -> &'a mut crate::W<REG> {
        self.variant(DataReg3::Output0IfBit1Is1)
    }
    #[doc = "Output 1 if Bit1 is 1."]
    #[inline(always)]
    pub fn output_1_if_bit1_is_1(self) -> &'a mut crate::W<REG> {
        self.variant(DataReg3::Output1IfBit1Is1)
    }
}
#[doc = "Direction register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirectionReg3 {
    #[doc = "0: Input"]
    Input = 0,
    #[doc = "1: Output"]
    Output = 1,
}
impl From<DirectionReg3> for bool {
    #[inline(always)]
    fn from(variant: DirectionReg3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DirectionReg3` reader - Direction register"]
pub type DirectionReg3R = crate::BitReader<DirectionReg3>;
impl DirectionReg3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DirectionReg3 {
        match self.bits {
            false => DirectionReg3::Input,
            true => DirectionReg3::Output,
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DirectionReg3::Input
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DirectionReg3::Output
    }
}
#[doc = "Field `DirectionReg3` writer - Direction register"]
pub type DirectionReg3W<'a, REG> = crate::BitWriter<'a, REG, DirectionReg3>;
impl<'a, REG> DirectionReg3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(DirectionReg3::Input)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(DirectionReg3::Output)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Intenbl3 {
    #[doc = "0: Disable interrupt"]
    DisableInterrupt = 0,
    #[doc = "1: Enable interrupt"]
    EnableInterrupt = 1,
}
impl From<Intenbl3> for bool {
    #[inline(always)]
    fn from(variant: Intenbl3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTEnbl3` reader - Interrupt enable"]
pub type Intenbl3R = crate::BitReader<Intenbl3>;
impl Intenbl3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Intenbl3 {
        match self.bits {
            false => Intenbl3::DisableInterrupt,
            true => Intenbl3::EnableInterrupt,
        }
    }
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn is_disable_interrupt(&self) -> bool {
        *self == Intenbl3::DisableInterrupt
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn is_enable_interrupt(&self) -> bool {
        *self == Intenbl3::EnableInterrupt
    }
}
#[doc = "Field `INTEnbl3` writer - Interrupt enable"]
pub type Intenbl3W<'a, REG> = crate::BitWriter<'a, REG, Intenbl3>;
impl<'a, REG> Intenbl3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable_interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(Intenbl3::DisableInterrupt)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable_interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(Intenbl3::EnableInterrupt)
    }
}
#[doc = "Interrupt sensitivity type 0 selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntsensitivityType0sel3 {
    #[doc = "0: Select falling-edge or level-low trigger mode"]
    SelectFallingedgeOrLevellowTriggerMode = 0,
    #[doc = "1: Select rising-edge or level-high trigger mode"]
    SelectRisingedgeOrLevelhighTriggerMode = 1,
}
impl From<IntsensitivityType0sel3> for bool {
    #[inline(always)]
    fn from(variant: IntsensitivityType0sel3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTSensitivityType0Sel3` reader - Interrupt sensitivity type 0 selection"]
pub type IntsensitivityType0sel3R = crate::BitReader<IntsensitivityType0sel3>;
impl IntsensitivityType0sel3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntsensitivityType0sel3 {
        match self.bits {
            false => IntsensitivityType0sel3::SelectFallingedgeOrLevellowTriggerMode,
            true => IntsensitivityType0sel3::SelectRisingedgeOrLevelhighTriggerMode,
        }
    }
    #[doc = "Select falling-edge or level-low trigger mode"]
    #[inline(always)]
    pub fn is_select_fallingedge_or_levellow_trigger_mode(&self) -> bool {
        *self == IntsensitivityType0sel3::SelectFallingedgeOrLevellowTriggerMode
    }
    #[doc = "Select rising-edge or level-high trigger mode"]
    #[inline(always)]
    pub fn is_select_risingedge_or_levelhigh_trigger_mode(&self) -> bool {
        *self == IntsensitivityType0sel3::SelectRisingedgeOrLevelhighTriggerMode
    }
}
#[doc = "Field `INTSensitivityType0Sel3` writer - Interrupt sensitivity type 0 selection"]
pub type IntsensitivityType0sel3W<'a, REG> = crate::BitWriter<'a, REG, IntsensitivityType0sel3>;
impl<'a, REG> IntsensitivityType0sel3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select falling-edge or level-low trigger mode"]
    #[inline(always)]
    pub fn select_fallingedge_or_levellow_trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(IntsensitivityType0sel3::SelectFallingedgeOrLevellowTriggerMode)
    }
    #[doc = "Select rising-edge or level-high trigger mode"]
    #[inline(always)]
    pub fn select_risingedge_or_levelhigh_trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(IntsensitivityType0sel3::SelectRisingedgeOrLevelhighTriggerMode)
    }
}
#[doc = "Interrupt sensitivity type 1 selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntsensitivityType1sel3 {
    #[doc = "0: Select edge trigger mode"]
    SelectEdgeTriggerMode = 0,
    #[doc = "1: Select level trigger mode"]
    SelectLevelTriggerMode = 1,
}
impl From<IntsensitivityType1sel3> for bool {
    #[inline(always)]
    fn from(variant: IntsensitivityType1sel3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTSensitivityType1Sel3` reader - Interrupt sensitivity type 1 selection"]
pub type IntsensitivityType1sel3R = crate::BitReader<IntsensitivityType1sel3>;
impl IntsensitivityType1sel3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntsensitivityType1sel3 {
        match self.bits {
            false => IntsensitivityType1sel3::SelectEdgeTriggerMode,
            true => IntsensitivityType1sel3::SelectLevelTriggerMode,
        }
    }
    #[doc = "Select edge trigger mode"]
    #[inline(always)]
    pub fn is_select_edge_trigger_mode(&self) -> bool {
        *self == IntsensitivityType1sel3::SelectEdgeTriggerMode
    }
    #[doc = "Select level trigger mode"]
    #[inline(always)]
    pub fn is_select_level_trigger_mode(&self) -> bool {
        *self == IntsensitivityType1sel3::SelectLevelTriggerMode
    }
}
#[doc = "Field `INTSensitivityType1Sel3` writer - Interrupt sensitivity type 1 selection"]
pub type IntsensitivityType1sel3W<'a, REG> = crate::BitWriter<'a, REG, IntsensitivityType1sel3>;
impl<'a, REG> IntsensitivityType1sel3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select edge trigger mode"]
    #[inline(always)]
    pub fn select_edge_trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(IntsensitivityType1sel3::SelectEdgeTriggerMode)
    }
    #[doc = "Select level trigger mode"]
    #[inline(always)]
    pub fn select_level_trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(IntsensitivityType1sel3::SelectLevelTriggerMode)
    }
}
#[doc = "Interrupt sensitivity type 2 selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntsensitivityType2sel3 {
    #[doc = "0: Select edge or level trigger mode"]
    SelectEdgeOrLevelTriggerMode = 0,
    #[doc = "1: Select dual-edge trigger mode"]
    SelectDualedgeTriggerMode = 1,
}
impl From<IntsensitivityType2sel3> for bool {
    #[inline(always)]
    fn from(variant: IntsensitivityType2sel3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTSensitivityType2Sel3` reader - Interrupt sensitivity type 2 selection"]
pub type IntsensitivityType2sel3R = crate::BitReader<IntsensitivityType2sel3>;
impl IntsensitivityType2sel3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntsensitivityType2sel3 {
        match self.bits {
            false => IntsensitivityType2sel3::SelectEdgeOrLevelTriggerMode,
            true => IntsensitivityType2sel3::SelectDualedgeTriggerMode,
        }
    }
    #[doc = "Select edge or level trigger mode"]
    #[inline(always)]
    pub fn is_select_edge_or_level_trigger_mode(&self) -> bool {
        *self == IntsensitivityType2sel3::SelectEdgeOrLevelTriggerMode
    }
    #[doc = "Select dual-edge trigger mode"]
    #[inline(always)]
    pub fn is_select_dualedge_trigger_mode(&self) -> bool {
        *self == IntsensitivityType2sel3::SelectDualedgeTriggerMode
    }
}
#[doc = "Field `INTSensitivityType2Sel3` writer - Interrupt sensitivity type 2 selection"]
pub type IntsensitivityType2sel3W<'a, REG> = crate::BitWriter<'a, REG, IntsensitivityType2sel3>;
impl<'a, REG> IntsensitivityType2sel3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select edge or level trigger mode"]
    #[inline(always)]
    pub fn select_edge_or_level_trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(IntsensitivityType2sel3::SelectEdgeOrLevelTriggerMode)
    }
    #[doc = "Select dual-edge trigger mode"]
    #[inline(always)]
    pub fn select_dualedge_trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(IntsensitivityType2sel3::SelectDualedgeTriggerMode)
    }
}
#[doc = "Reset tolerance enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RstToleranceEnbl3 {
    #[doc = "0: Bit0 and Bit1 will be reset by WDT\\_SOC and/or EXTRSTN reset"]
    Bit0AndBit1WillBeResetByWdtsocAndorExtrstnReset = 0,
    #[doc = "1: Bit0 and Bit1 will not be reset by WDT\\_SOC and/or EXTRSTN"]
    Bit0AndBit1WillNotBeResetByWdtsocAndorExtrstn = 1,
}
impl From<RstToleranceEnbl3> for bool {
    #[inline(always)]
    fn from(variant: RstToleranceEnbl3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RstToleranceEnbl3` reader - Reset tolerance enable"]
pub type RstToleranceEnbl3R = crate::BitReader<RstToleranceEnbl3>;
impl RstToleranceEnbl3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RstToleranceEnbl3 {
        match self.bits {
            false => RstToleranceEnbl3::Bit0AndBit1WillBeResetByWdtsocAndorExtrstnReset,
            true => RstToleranceEnbl3::Bit0AndBit1WillNotBeResetByWdtsocAndorExtrstn,
        }
    }
    #[doc = "Bit0 and Bit1 will be reset by WDT\\_SOC and/or EXTRSTN reset"]
    #[inline(always)]
    pub fn is_bit0_and_bit1_will_be_reset_by_wdtsoc_andor_extrstn_reset(&self) -> bool {
        *self == RstToleranceEnbl3::Bit0AndBit1WillBeResetByWdtsocAndorExtrstnReset
    }
    #[doc = "Bit0 and Bit1 will not be reset by WDT\\_SOC and/or EXTRSTN"]
    #[inline(always)]
    pub fn is_bit0_and_bit1_will_not_be_reset_by_wdtsoc_andor_extrstn(&self) -> bool {
        *self == RstToleranceEnbl3::Bit0AndBit1WillNotBeResetByWdtsocAndorExtrstn
    }
}
#[doc = "Field `RstToleranceEnbl3` writer - Reset tolerance enable"]
pub type RstToleranceEnbl3W<'a, REG> = crate::BitWriter<'a, REG, RstToleranceEnbl3>;
impl<'a, REG> RstToleranceEnbl3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bit0 and Bit1 will be reset by WDT\\_SOC and/or EXTRSTN reset"]
    #[inline(always)]
    pub fn bit0_and_bit1_will_be_reset_by_wdtsoc_andor_extrstn_reset(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(RstToleranceEnbl3::Bit0AndBit1WillBeResetByWdtsocAndorExtrstnReset)
    }
    #[doc = "Bit0 and Bit1 will not be reset by WDT\\_SOC and/or EXTRSTN"]
    #[inline(always)]
    pub fn bit0_and_bit1_will_not_be_reset_by_wdtsoc_andor_extrstn(self) -> &'a mut crate::W<REG> {
        self.variant(RstToleranceEnbl3::Bit0AndBit1WillNotBeResetByWdtsocAndorExtrstn)
    }
}
#[doc = "Field `DebounceSettingReg13` reader - Debounce setting register #1"]
pub type DebounceSettingReg13R = crate::BitReader;
#[doc = "Field `DebounceSettingReg13` writer - Debounce setting register #1"]
pub type DebounceSettingReg13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DebounceSettingReg23` reader - Debounce setting register #2"]
pub type DebounceSettingReg23R = crate::BitReader;
#[doc = "Field `DebounceSettingReg23` writer - Debounce setting register #2"]
pub type DebounceSettingReg23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Input mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputMask3 {
    #[doc = "0: Read from Bit13 will be updated."]
    ReadFromBit13WillBeUpdated = 0,
    #[doc = "1: Read from Bit13 will not be updated."]
    ReadFromBit13WillNotBeUpdated = 1,
}
impl From<InputMask3> for bool {
    #[inline(always)]
    fn from(variant: InputMask3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `InputMask3` reader - Input mask"]
pub type InputMask3R = crate::BitReader<InputMask3>;
impl InputMask3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> InputMask3 {
        match self.bits {
            false => InputMask3::ReadFromBit13WillBeUpdated,
            true => InputMask3::ReadFromBit13WillNotBeUpdated,
        }
    }
    #[doc = "Read from Bit13 will be updated."]
    #[inline(always)]
    pub fn is_read_from_bit13_will_be_updated(&self) -> bool {
        *self == InputMask3::ReadFromBit13WillBeUpdated
    }
    #[doc = "Read from Bit13 will not be updated."]
    #[inline(always)]
    pub fn is_read_from_bit13_will_not_be_updated(&self) -> bool {
        *self == InputMask3::ReadFromBit13WillNotBeUpdated
    }
}
#[doc = "Field `InputMask3` writer - Input mask"]
pub type InputMask3W<'a, REG> = crate::BitWriter<'a, REG, InputMask3>;
impl<'a, REG> InputMask3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read from Bit13 will be updated."]
    #[inline(always)]
    pub fn read_from_bit13_will_be_updated(self) -> &'a mut crate::W<REG> {
        self.variant(InputMask3::ReadFromBit13WillBeUpdated)
    }
    #[doc = "Read from Bit13 will not be updated."]
    #[inline(always)]
    pub fn read_from_bit13_will_not_be_updated(self) -> &'a mut crate::W<REG> {
        self.variant(InputMask3::ReadFromBit13WillNotBeUpdated)
    }
}
#[doc = "Field `BlinkCounterSel13` reader - Blink Counter Selection #1"]
pub type BlinkCounterSel13R = crate::BitReader;
#[doc = "Field `BlinkCounterSel13` writer - Blink Counter Selection #1"]
pub type BlinkCounterSel13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BlinkCounterSel23` reader - Blink Counter Selection #2"]
pub type BlinkCounterSel23R = crate::BitReader;
#[doc = "Field `BlinkCounterSel23` writer - Blink Counter Selection #2"]
pub type BlinkCounterSel23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt status register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntstsReg3 {
    #[doc = "0: No interrupt pending"]
    NoInterruptPending = 0,
    #[doc = "1: interrupt pending"]
    InterruptPending = 1,
}
impl From<IntstsReg3> for bool {
    #[inline(always)]
    fn from(variant: IntstsReg3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTStsReg3` reader - Interrupt status register"]
pub type IntstsReg3R = crate::BitReader<IntstsReg3>;
impl IntstsReg3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntstsReg3 {
        match self.bits {
            false => IntstsReg3::NoInterruptPending,
            true => IntstsReg3::InterruptPending,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_no_interrupt_pending(&self) -> bool {
        *self == IntstsReg3::NoInterruptPending
    }
    #[doc = "interrupt pending"]
    #[inline(always)]
    pub fn is_interrupt_pending(&self) -> bool {
        *self == IntstsReg3::InterruptPending
    }
}
#[doc = "Field `INTStsReg3` writer - Interrupt status register"]
pub type IntstsReg3W<'a, REG> = crate::BitWriter<'a, REG, IntstsReg3>;
impl<'a, REG> IntstsReg3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn no_interrupt_pending(self) -> &'a mut crate::W<REG> {
        self.variant(IntstsReg3::NoInterruptPending)
    }
    #[doc = "interrupt pending"]
    #[inline(always)]
    pub fn interrupt_pending(self) -> &'a mut crate::W<REG> {
        self.variant(IntstsReg3::InterruptPending)
    }
}
#[doc = "Field `InputDataReg3` reader - Input data register"]
pub type InputDataReg3R = crate::BitReader;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Data register"]
    #[inline(always)]
    pub fn data_reg3(&self) -> DataReg3R {
        DataReg3R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Direction register"]
    #[inline(always)]
    pub fn direction_reg3(&self) -> DirectionReg3R {
        DirectionReg3R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable"]
    #[inline(always)]
    pub fn intenbl3(&self) -> Intenbl3R {
        Intenbl3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn intsensitivity_type0sel3(&self) -> IntsensitivityType0sel3R {
        IntsensitivityType0sel3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn intsensitivity_type1sel3(&self) -> IntsensitivityType1sel3R {
        IntsensitivityType1sel3R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn intsensitivity_type2sel3(&self) -> IntsensitivityType2sel3R {
        IntsensitivityType2sel3R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reset tolerance enable"]
    #[inline(always)]
    pub fn rst_tolerance_enbl3(&self) -> RstToleranceEnbl3R {
        RstToleranceEnbl3R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Debounce setting register #1"]
    #[inline(always)]
    pub fn debounce_setting_reg13(&self) -> DebounceSettingReg13R {
        DebounceSettingReg13R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Debounce setting register #2"]
    #[inline(always)]
    pub fn debounce_setting_reg23(&self) -> DebounceSettingReg23R {
        DebounceSettingReg23R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Input mask"]
    #[inline(always)]
    pub fn input_mask3(&self) -> InputMask3R {
        InputMask3R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Blink Counter Selection #1"]
    #[inline(always)]
    pub fn blink_counter_sel13(&self) -> BlinkCounterSel13R {
        BlinkCounterSel13R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Blink Counter Selection #2"]
    #[inline(always)]
    pub fn blink_counter_sel23(&self) -> BlinkCounterSel23R {
        BlinkCounterSel23R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt status register"]
    #[inline(always)]
    pub fn intsts_reg3(&self) -> IntstsReg3R {
        IntstsReg3R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Input data register"]
    #[inline(always)]
    pub fn input_data_reg3(&self) -> InputDataReg3R {
        InputDataReg3R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:31 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Data register"]
    #[inline(always)]
    pub fn data_reg3(&mut self) -> DataReg3W<Gpio1ccSpec> {
        DataReg3W::new(self, 0)
    }
    #[doc = "Bit 1 - Direction register"]
    #[inline(always)]
    pub fn direction_reg3(&mut self) -> DirectionReg3W<Gpio1ccSpec> {
        DirectionReg3W::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt enable"]
    #[inline(always)]
    pub fn intenbl3(&mut self) -> Intenbl3W<Gpio1ccSpec> {
        Intenbl3W::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn intsensitivity_type0sel3(&mut self) -> IntsensitivityType0sel3W<Gpio1ccSpec> {
        IntsensitivityType0sel3W::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn intsensitivity_type1sel3(&mut self) -> IntsensitivityType1sel3W<Gpio1ccSpec> {
        IntsensitivityType1sel3W::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn intsensitivity_type2sel3(&mut self) -> IntsensitivityType2sel3W<Gpio1ccSpec> {
        IntsensitivityType2sel3W::new(self, 5)
    }
    #[doc = "Bit 6 - Reset tolerance enable"]
    #[inline(always)]
    pub fn rst_tolerance_enbl3(&mut self) -> RstToleranceEnbl3W<Gpio1ccSpec> {
        RstToleranceEnbl3W::new(self, 6)
    }
    #[doc = "Bit 7 - Debounce setting register #1"]
    #[inline(always)]
    pub fn debounce_setting_reg13(&mut self) -> DebounceSettingReg13W<Gpio1ccSpec> {
        DebounceSettingReg13W::new(self, 7)
    }
    #[doc = "Bit 8 - Debounce setting register #2"]
    #[inline(always)]
    pub fn debounce_setting_reg23(&mut self) -> DebounceSettingReg23W<Gpio1ccSpec> {
        DebounceSettingReg23W::new(self, 8)
    }
    #[doc = "Bit 9 - Input mask"]
    #[inline(always)]
    pub fn input_mask3(&mut self) -> InputMask3W<Gpio1ccSpec> {
        InputMask3W::new(self, 9)
    }
    #[doc = "Bit 10 - Blink Counter Selection #1"]
    #[inline(always)]
    pub fn blink_counter_sel13(&mut self) -> BlinkCounterSel13W<Gpio1ccSpec> {
        BlinkCounterSel13W::new(self, 10)
    }
    #[doc = "Bit 11 - Blink Counter Selection #2"]
    #[inline(always)]
    pub fn blink_counter_sel23(&mut self) -> BlinkCounterSel23W<Gpio1ccSpec> {
        BlinkCounterSel23W::new(self, 11)
    }
    #[doc = "Bit 12 - Interrupt status register"]
    #[inline(always)]
    pub fn intsts_reg3(&mut self) -> IntstsReg3W<Gpio1ccSpec> {
        IntstsReg3W::new(self, 12)
    }
}
#[doc = "GPIO019 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio1cc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio1cc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio1ccSpec;
impl crate::RegisterSpec for Gpio1ccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio1cc::R`](R) reader structure"]
impl crate::Readable for Gpio1ccSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio1cc::W`](W) writer structure"]
impl crate::Writable for Gpio1ccSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO1CC to value 0"]
impl crate::Resettable for Gpio1ccSpec {}
