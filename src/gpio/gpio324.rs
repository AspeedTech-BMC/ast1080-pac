#[doc = "Register `GPIO324` reader"]
pub type R = crate::R<Gpio324Spec>;
#[doc = "Register `GPIO324` writer"]
pub type W = crate::W<Gpio324Spec>;
#[doc = "Data register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataReg1 {
    #[doc = "0: Output 0 if Bit1 is 1."]
    Output0IfBit1Is1 = 0,
    #[doc = "1: Output 1 if Bit1 is 1."]
    Output1IfBit1Is1 = 1,
}
impl From<DataReg1> for bool {
    #[inline(always)]
    fn from(variant: DataReg1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DataReg1` reader - Data register"]
pub type DataReg1R = crate::BitReader<DataReg1>;
impl DataReg1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DataReg1 {
        match self.bits {
            false => DataReg1::Output0IfBit1Is1,
            true => DataReg1::Output1IfBit1Is1,
        }
    }
    #[doc = "Output 0 if Bit1 is 1."]
    #[inline(always)]
    pub fn is_output_0_if_bit1_is_1(&self) -> bool {
        *self == DataReg1::Output0IfBit1Is1
    }
    #[doc = "Output 1 if Bit1 is 1."]
    #[inline(always)]
    pub fn is_output_1_if_bit1_is_1(&self) -> bool {
        *self == DataReg1::Output1IfBit1Is1
    }
}
#[doc = "Field `DataReg1` writer - Data register"]
pub type DataReg1W<'a, REG> = crate::BitWriter<'a, REG, DataReg1>;
impl<'a, REG> DataReg1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output 0 if Bit1 is 1."]
    #[inline(always)]
    pub fn output_0_if_bit1_is_1(self) -> &'a mut crate::W<REG> {
        self.variant(DataReg1::Output0IfBit1Is1)
    }
    #[doc = "Output 1 if Bit1 is 1."]
    #[inline(always)]
    pub fn output_1_if_bit1_is_1(self) -> &'a mut crate::W<REG> {
        self.variant(DataReg1::Output1IfBit1Is1)
    }
}
#[doc = "Direction register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirectionReg1 {
    #[doc = "0: Input"]
    Input = 0,
    #[doc = "1: Output"]
    Output = 1,
}
impl From<DirectionReg1> for bool {
    #[inline(always)]
    fn from(variant: DirectionReg1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DirectionReg1` reader - Direction register"]
pub type DirectionReg1R = crate::BitReader<DirectionReg1>;
impl DirectionReg1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DirectionReg1 {
        match self.bits {
            false => DirectionReg1::Input,
            true => DirectionReg1::Output,
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DirectionReg1::Input
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DirectionReg1::Output
    }
}
#[doc = "Field `DirectionReg1` writer - Direction register"]
pub type DirectionReg1W<'a, REG> = crate::BitWriter<'a, REG, DirectionReg1>;
impl<'a, REG> DirectionReg1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(DirectionReg1::Input)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(DirectionReg1::Output)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Intenbl1 {
    #[doc = "0: Disable interrupt"]
    DisableInterrupt = 0,
    #[doc = "1: Enable interrupt"]
    EnableInterrupt = 1,
}
impl From<Intenbl1> for bool {
    #[inline(always)]
    fn from(variant: Intenbl1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTEnbl1` reader - Interrupt enable"]
pub type Intenbl1R = crate::BitReader<Intenbl1>;
impl Intenbl1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Intenbl1 {
        match self.bits {
            false => Intenbl1::DisableInterrupt,
            true => Intenbl1::EnableInterrupt,
        }
    }
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn is_disable_interrupt(&self) -> bool {
        *self == Intenbl1::DisableInterrupt
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn is_enable_interrupt(&self) -> bool {
        *self == Intenbl1::EnableInterrupt
    }
}
#[doc = "Field `INTEnbl1` writer - Interrupt enable"]
pub type Intenbl1W<'a, REG> = crate::BitWriter<'a, REG, Intenbl1>;
impl<'a, REG> Intenbl1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable_interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(Intenbl1::DisableInterrupt)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable_interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(Intenbl1::EnableInterrupt)
    }
}
#[doc = "Interrupt sensitivity type 0 selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntsensitivityType0sel1 {
    #[doc = "0: Select falling-edge or level-low trigger mode"]
    SelectFallingedgeOrLevellowTriggerMode = 0,
    #[doc = "1: Select rising-edge or level-high trigger mode"]
    SelectRisingedgeOrLevelhighTriggerMode = 1,
}
impl From<IntsensitivityType0sel1> for bool {
    #[inline(always)]
    fn from(variant: IntsensitivityType0sel1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTSensitivityType0Sel1` reader - Interrupt sensitivity type 0 selection"]
pub type IntsensitivityType0sel1R = crate::BitReader<IntsensitivityType0sel1>;
impl IntsensitivityType0sel1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntsensitivityType0sel1 {
        match self.bits {
            false => IntsensitivityType0sel1::SelectFallingedgeOrLevellowTriggerMode,
            true => IntsensitivityType0sel1::SelectRisingedgeOrLevelhighTriggerMode,
        }
    }
    #[doc = "Select falling-edge or level-low trigger mode"]
    #[inline(always)]
    pub fn is_select_fallingedge_or_levellow_trigger_mode(&self) -> bool {
        *self == IntsensitivityType0sel1::SelectFallingedgeOrLevellowTriggerMode
    }
    #[doc = "Select rising-edge or level-high trigger mode"]
    #[inline(always)]
    pub fn is_select_risingedge_or_levelhigh_trigger_mode(&self) -> bool {
        *self == IntsensitivityType0sel1::SelectRisingedgeOrLevelhighTriggerMode
    }
}
#[doc = "Field `INTSensitivityType0Sel1` writer - Interrupt sensitivity type 0 selection"]
pub type IntsensitivityType0sel1W<'a, REG> = crate::BitWriter<'a, REG, IntsensitivityType0sel1>;
impl<'a, REG> IntsensitivityType0sel1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select falling-edge or level-low trigger mode"]
    #[inline(always)]
    pub fn select_fallingedge_or_levellow_trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(IntsensitivityType0sel1::SelectFallingedgeOrLevellowTriggerMode)
    }
    #[doc = "Select rising-edge or level-high trigger mode"]
    #[inline(always)]
    pub fn select_risingedge_or_levelhigh_trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(IntsensitivityType0sel1::SelectRisingedgeOrLevelhighTriggerMode)
    }
}
#[doc = "Interrupt sensitivity type 1 selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntsensitivityType1sel1 {
    #[doc = "0: Select edge trigger mode"]
    SelectEdgeTriggerMode = 0,
    #[doc = "1: Select level trigger mode"]
    SelectLevelTriggerMode = 1,
}
impl From<IntsensitivityType1sel1> for bool {
    #[inline(always)]
    fn from(variant: IntsensitivityType1sel1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTSensitivityType1Sel1` reader - Interrupt sensitivity type 1 selection"]
pub type IntsensitivityType1sel1R = crate::BitReader<IntsensitivityType1sel1>;
impl IntsensitivityType1sel1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntsensitivityType1sel1 {
        match self.bits {
            false => IntsensitivityType1sel1::SelectEdgeTriggerMode,
            true => IntsensitivityType1sel1::SelectLevelTriggerMode,
        }
    }
    #[doc = "Select edge trigger mode"]
    #[inline(always)]
    pub fn is_select_edge_trigger_mode(&self) -> bool {
        *self == IntsensitivityType1sel1::SelectEdgeTriggerMode
    }
    #[doc = "Select level trigger mode"]
    #[inline(always)]
    pub fn is_select_level_trigger_mode(&self) -> bool {
        *self == IntsensitivityType1sel1::SelectLevelTriggerMode
    }
}
#[doc = "Field `INTSensitivityType1Sel1` writer - Interrupt sensitivity type 1 selection"]
pub type IntsensitivityType1sel1W<'a, REG> = crate::BitWriter<'a, REG, IntsensitivityType1sel1>;
impl<'a, REG> IntsensitivityType1sel1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select edge trigger mode"]
    #[inline(always)]
    pub fn select_edge_trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(IntsensitivityType1sel1::SelectEdgeTriggerMode)
    }
    #[doc = "Select level trigger mode"]
    #[inline(always)]
    pub fn select_level_trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(IntsensitivityType1sel1::SelectLevelTriggerMode)
    }
}
#[doc = "Interrupt sensitivity type 2 selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntsensitivityType2sel1 {
    #[doc = "0: Select edge or level trigger mode"]
    SelectEdgeOrLevelTriggerMode = 0,
    #[doc = "1: Select dual-edge trigger mode"]
    SelectDualedgeTriggerMode = 1,
}
impl From<IntsensitivityType2sel1> for bool {
    #[inline(always)]
    fn from(variant: IntsensitivityType2sel1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTSensitivityType2Sel1` reader - Interrupt sensitivity type 2 selection"]
pub type IntsensitivityType2sel1R = crate::BitReader<IntsensitivityType2sel1>;
impl IntsensitivityType2sel1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntsensitivityType2sel1 {
        match self.bits {
            false => IntsensitivityType2sel1::SelectEdgeOrLevelTriggerMode,
            true => IntsensitivityType2sel1::SelectDualedgeTriggerMode,
        }
    }
    #[doc = "Select edge or level trigger mode"]
    #[inline(always)]
    pub fn is_select_edge_or_level_trigger_mode(&self) -> bool {
        *self == IntsensitivityType2sel1::SelectEdgeOrLevelTriggerMode
    }
    #[doc = "Select dual-edge trigger mode"]
    #[inline(always)]
    pub fn is_select_dualedge_trigger_mode(&self) -> bool {
        *self == IntsensitivityType2sel1::SelectDualedgeTriggerMode
    }
}
#[doc = "Field `INTSensitivityType2Sel1` writer - Interrupt sensitivity type 2 selection"]
pub type IntsensitivityType2sel1W<'a, REG> = crate::BitWriter<'a, REG, IntsensitivityType2sel1>;
impl<'a, REG> IntsensitivityType2sel1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select edge or level trigger mode"]
    #[inline(always)]
    pub fn select_edge_or_level_trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(IntsensitivityType2sel1::SelectEdgeOrLevelTriggerMode)
    }
    #[doc = "Select dual-edge trigger mode"]
    #[inline(always)]
    pub fn select_dualedge_trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(IntsensitivityType2sel1::SelectDualedgeTriggerMode)
    }
}
#[doc = "Reset tolerance enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RstToleranceEnbl1 {
    #[doc = "0: Bit0 and Bit1 will be reset by WDT\\_SOC and/or EXTRSTN reset"]
    Bit0AndBit1WillBeResetByWdtsocAndorExtrstnReset = 0,
    #[doc = "1: Bit0 and Bit1 will not be reset by WDT\\_SOC and/or EXTRSTN"]
    Bit0AndBit1WillNotBeResetByWdtsocAndorExtrstn = 1,
}
impl From<RstToleranceEnbl1> for bool {
    #[inline(always)]
    fn from(variant: RstToleranceEnbl1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RstToleranceEnbl1` reader - Reset tolerance enable"]
pub type RstToleranceEnbl1R = crate::BitReader<RstToleranceEnbl1>;
impl RstToleranceEnbl1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RstToleranceEnbl1 {
        match self.bits {
            false => RstToleranceEnbl1::Bit0AndBit1WillBeResetByWdtsocAndorExtrstnReset,
            true => RstToleranceEnbl1::Bit0AndBit1WillNotBeResetByWdtsocAndorExtrstn,
        }
    }
    #[doc = "Bit0 and Bit1 will be reset by WDT\\_SOC and/or EXTRSTN reset"]
    #[inline(always)]
    pub fn is_bit0_and_bit1_will_be_reset_by_wdtsoc_andor_extrstn_reset(&self) -> bool {
        *self == RstToleranceEnbl1::Bit0AndBit1WillBeResetByWdtsocAndorExtrstnReset
    }
    #[doc = "Bit0 and Bit1 will not be reset by WDT\\_SOC and/or EXTRSTN"]
    #[inline(always)]
    pub fn is_bit0_and_bit1_will_not_be_reset_by_wdtsoc_andor_extrstn(&self) -> bool {
        *self == RstToleranceEnbl1::Bit0AndBit1WillNotBeResetByWdtsocAndorExtrstn
    }
}
#[doc = "Field `RstToleranceEnbl1` writer - Reset tolerance enable"]
pub type RstToleranceEnbl1W<'a, REG> = crate::BitWriter<'a, REG, RstToleranceEnbl1>;
impl<'a, REG> RstToleranceEnbl1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bit0 and Bit1 will be reset by WDT\\_SOC and/or EXTRSTN reset"]
    #[inline(always)]
    pub fn bit0_and_bit1_will_be_reset_by_wdtsoc_andor_extrstn_reset(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(RstToleranceEnbl1::Bit0AndBit1WillBeResetByWdtsocAndorExtrstnReset)
    }
    #[doc = "Bit0 and Bit1 will not be reset by WDT\\_SOC and/or EXTRSTN"]
    #[inline(always)]
    pub fn bit0_and_bit1_will_not_be_reset_by_wdtsoc_andor_extrstn(self) -> &'a mut crate::W<REG> {
        self.variant(RstToleranceEnbl1::Bit0AndBit1WillNotBeResetByWdtsocAndorExtrstn)
    }
}
#[doc = "Field `DebounceSettingReg11` reader - Debounce setting register #1"]
pub type DebounceSettingReg11R = crate::BitReader;
#[doc = "Field `DebounceSettingReg11` writer - Debounce setting register #1"]
pub type DebounceSettingReg11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DebounceSettingReg21` reader - Debounce setting register #2"]
pub type DebounceSettingReg21R = crate::BitReader;
#[doc = "Field `DebounceSettingReg21` writer - Debounce setting register #2"]
pub type DebounceSettingReg21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Input mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputMask1 {
    #[doc = "0: Read from Bit13 will be updated."]
    ReadFromBit13WillBeUpdated = 0,
    #[doc = "1: Read from Bit13 will not be updated."]
    ReadFromBit13WillNotBeUpdated = 1,
}
impl From<InputMask1> for bool {
    #[inline(always)]
    fn from(variant: InputMask1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `InputMask1` reader - Input mask"]
pub type InputMask1R = crate::BitReader<InputMask1>;
impl InputMask1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> InputMask1 {
        match self.bits {
            false => InputMask1::ReadFromBit13WillBeUpdated,
            true => InputMask1::ReadFromBit13WillNotBeUpdated,
        }
    }
    #[doc = "Read from Bit13 will be updated."]
    #[inline(always)]
    pub fn is_read_from_bit13_will_be_updated(&self) -> bool {
        *self == InputMask1::ReadFromBit13WillBeUpdated
    }
    #[doc = "Read from Bit13 will not be updated."]
    #[inline(always)]
    pub fn is_read_from_bit13_will_not_be_updated(&self) -> bool {
        *self == InputMask1::ReadFromBit13WillNotBeUpdated
    }
}
#[doc = "Field `InputMask1` writer - Input mask"]
pub type InputMask1W<'a, REG> = crate::BitWriter<'a, REG, InputMask1>;
impl<'a, REG> InputMask1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read from Bit13 will be updated."]
    #[inline(always)]
    pub fn read_from_bit13_will_be_updated(self) -> &'a mut crate::W<REG> {
        self.variant(InputMask1::ReadFromBit13WillBeUpdated)
    }
    #[doc = "Read from Bit13 will not be updated."]
    #[inline(always)]
    pub fn read_from_bit13_will_not_be_updated(self) -> &'a mut crate::W<REG> {
        self.variant(InputMask1::ReadFromBit13WillNotBeUpdated)
    }
}
#[doc = "Field `BlinkCounterSel11` reader - Blink Counter Selection #1"]
pub type BlinkCounterSel11R = crate::BitReader;
#[doc = "Field `BlinkCounterSel11` writer - Blink Counter Selection #1"]
pub type BlinkCounterSel11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BlinkCounterSel21` reader - Blink Counter Selection #2"]
pub type BlinkCounterSel21R = crate::BitReader;
#[doc = "Field `BlinkCounterSel21` writer - Blink Counter Selection #2"]
pub type BlinkCounterSel21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt status register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntstsReg1 {
    #[doc = "0: No interrupt pending"]
    NoInterruptPending = 0,
    #[doc = "1: interrupt pending"]
    InterruptPending = 1,
}
impl From<IntstsReg1> for bool {
    #[inline(always)]
    fn from(variant: IntstsReg1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTStsReg1` reader - Interrupt status register"]
pub type IntstsReg1R = crate::BitReader<IntstsReg1>;
impl IntstsReg1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntstsReg1 {
        match self.bits {
            false => IntstsReg1::NoInterruptPending,
            true => IntstsReg1::InterruptPending,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_no_interrupt_pending(&self) -> bool {
        *self == IntstsReg1::NoInterruptPending
    }
    #[doc = "interrupt pending"]
    #[inline(always)]
    pub fn is_interrupt_pending(&self) -> bool {
        *self == IntstsReg1::InterruptPending
    }
}
#[doc = "Field `INTStsReg1` writer - Interrupt status register"]
pub type IntstsReg1W<'a, REG> = crate::BitWriter<'a, REG, IntstsReg1>;
impl<'a, REG> IntstsReg1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn no_interrupt_pending(self) -> &'a mut crate::W<REG> {
        self.variant(IntstsReg1::NoInterruptPending)
    }
    #[doc = "interrupt pending"]
    #[inline(always)]
    pub fn interrupt_pending(self) -> &'a mut crate::W<REG> {
        self.variant(IntstsReg1::InterruptPending)
    }
}
#[doc = "Field `InputDataReg1` reader - Input data register"]
pub type InputDataReg1R = crate::BitReader;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Data register"]
    #[inline(always)]
    pub fn data_reg1(&self) -> DataReg1R {
        DataReg1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Direction register"]
    #[inline(always)]
    pub fn direction_reg1(&self) -> DirectionReg1R {
        DirectionReg1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable"]
    #[inline(always)]
    pub fn intenbl1(&self) -> Intenbl1R {
        Intenbl1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn intsensitivity_type0sel1(&self) -> IntsensitivityType0sel1R {
        IntsensitivityType0sel1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn intsensitivity_type1sel1(&self) -> IntsensitivityType1sel1R {
        IntsensitivityType1sel1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn intsensitivity_type2sel1(&self) -> IntsensitivityType2sel1R {
        IntsensitivityType2sel1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reset tolerance enable"]
    #[inline(always)]
    pub fn rst_tolerance_enbl1(&self) -> RstToleranceEnbl1R {
        RstToleranceEnbl1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Debounce setting register #1"]
    #[inline(always)]
    pub fn debounce_setting_reg11(&self) -> DebounceSettingReg11R {
        DebounceSettingReg11R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Debounce setting register #2"]
    #[inline(always)]
    pub fn debounce_setting_reg21(&self) -> DebounceSettingReg21R {
        DebounceSettingReg21R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Input mask"]
    #[inline(always)]
    pub fn input_mask1(&self) -> InputMask1R {
        InputMask1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Blink Counter Selection #1"]
    #[inline(always)]
    pub fn blink_counter_sel11(&self) -> BlinkCounterSel11R {
        BlinkCounterSel11R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Blink Counter Selection #2"]
    #[inline(always)]
    pub fn blink_counter_sel21(&self) -> BlinkCounterSel21R {
        BlinkCounterSel21R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt status register"]
    #[inline(always)]
    pub fn intsts_reg1(&self) -> IntstsReg1R {
        IntstsReg1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Input data register"]
    #[inline(always)]
    pub fn input_data_reg1(&self) -> InputDataReg1R {
        InputDataReg1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:31 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Data register"]
    #[inline(always)]
    pub fn data_reg1(&mut self) -> DataReg1W<Gpio324Spec> {
        DataReg1W::new(self, 0)
    }
    #[doc = "Bit 1 - Direction register"]
    #[inline(always)]
    pub fn direction_reg1(&mut self) -> DirectionReg1W<Gpio324Spec> {
        DirectionReg1W::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt enable"]
    #[inline(always)]
    pub fn intenbl1(&mut self) -> Intenbl1W<Gpio324Spec> {
        Intenbl1W::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn intsensitivity_type0sel1(&mut self) -> IntsensitivityType0sel1W<Gpio324Spec> {
        IntsensitivityType0sel1W::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn intsensitivity_type1sel1(&mut self) -> IntsensitivityType1sel1W<Gpio324Spec> {
        IntsensitivityType1sel1W::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn intsensitivity_type2sel1(&mut self) -> IntsensitivityType2sel1W<Gpio324Spec> {
        IntsensitivityType2sel1W::new(self, 5)
    }
    #[doc = "Bit 6 - Reset tolerance enable"]
    #[inline(always)]
    pub fn rst_tolerance_enbl1(&mut self) -> RstToleranceEnbl1W<Gpio324Spec> {
        RstToleranceEnbl1W::new(self, 6)
    }
    #[doc = "Bit 7 - Debounce setting register #1"]
    #[inline(always)]
    pub fn debounce_setting_reg11(&mut self) -> DebounceSettingReg11W<Gpio324Spec> {
        DebounceSettingReg11W::new(self, 7)
    }
    #[doc = "Bit 8 - Debounce setting register #2"]
    #[inline(always)]
    pub fn debounce_setting_reg21(&mut self) -> DebounceSettingReg21W<Gpio324Spec> {
        DebounceSettingReg21W::new(self, 8)
    }
    #[doc = "Bit 9 - Input mask"]
    #[inline(always)]
    pub fn input_mask1(&mut self) -> InputMask1W<Gpio324Spec> {
        InputMask1W::new(self, 9)
    }
    #[doc = "Bit 10 - Blink Counter Selection #1"]
    #[inline(always)]
    pub fn blink_counter_sel11(&mut self) -> BlinkCounterSel11W<Gpio324Spec> {
        BlinkCounterSel11W::new(self, 10)
    }
    #[doc = "Bit 11 - Blink Counter Selection #2"]
    #[inline(always)]
    pub fn blink_counter_sel21(&mut self) -> BlinkCounterSel21W<Gpio324Spec> {
        BlinkCounterSel21W::new(self, 11)
    }
    #[doc = "Bit 12 - Interrupt status register"]
    #[inline(always)]
    pub fn intsts_reg1(&mut self) -> IntstsReg1W<Gpio324Spec> {
        IntstsReg1W::new(self, 12)
    }
}
#[doc = "GPIO105 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio324::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio324::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio324Spec;
impl crate::RegisterSpec for Gpio324Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio324::R`](R) reader structure"]
impl crate::Readable for Gpio324Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio324::W`](W) writer structure"]
impl crate::Writable for Gpio324Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO324 to value 0"]
impl crate::Resettable for Gpio324Spec {}
