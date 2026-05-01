#[doc = "Register `GPIO2D4` reader"]
pub type R = crate::R<Gpio2d4Spec>;
#[doc = "Register `GPIO2D4` writer"]
pub type W = crate::W<Gpio2d4Spec>;
#[doc = "Data register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataReg5 {
    #[doc = "0: Output 0 if Bit1 is 1."]
    Output0IfBit1Is1 = 0,
    #[doc = "1: Output 1 if Bit1 is 1."]
    Output1IfBit1Is1 = 1,
}
impl From<DataReg5> for bool {
    #[inline(always)]
    fn from(variant: DataReg5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DataReg5` reader - Data register"]
pub type DataReg5R = crate::BitReader<DataReg5>;
impl DataReg5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DataReg5 {
        match self.bits {
            false => DataReg5::Output0IfBit1Is1,
            true => DataReg5::Output1IfBit1Is1,
        }
    }
    #[doc = "Output 0 if Bit1 is 1."]
    #[inline(always)]
    pub fn is_output_0_if_bit1_is_1(&self) -> bool {
        *self == DataReg5::Output0IfBit1Is1
    }
    #[doc = "Output 1 if Bit1 is 1."]
    #[inline(always)]
    pub fn is_output_1_if_bit1_is_1(&self) -> bool {
        *self == DataReg5::Output1IfBit1Is1
    }
}
#[doc = "Field `DataReg5` writer - Data register"]
pub type DataReg5W<'a, REG> = crate::BitWriter<'a, REG, DataReg5>;
impl<'a, REG> DataReg5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output 0 if Bit1 is 1."]
    #[inline(always)]
    pub fn output_0_if_bit1_is_1(self) -> &'a mut crate::W<REG> {
        self.variant(DataReg5::Output0IfBit1Is1)
    }
    #[doc = "Output 1 if Bit1 is 1."]
    #[inline(always)]
    pub fn output_1_if_bit1_is_1(self) -> &'a mut crate::W<REG> {
        self.variant(DataReg5::Output1IfBit1Is1)
    }
}
#[doc = "Direction register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirectionReg5 {
    #[doc = "0: Input"]
    Input = 0,
    #[doc = "1: Output"]
    Output = 1,
}
impl From<DirectionReg5> for bool {
    #[inline(always)]
    fn from(variant: DirectionReg5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DirectionReg5` reader - Direction register"]
pub type DirectionReg5R = crate::BitReader<DirectionReg5>;
impl DirectionReg5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DirectionReg5 {
        match self.bits {
            false => DirectionReg5::Input,
            true => DirectionReg5::Output,
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DirectionReg5::Input
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DirectionReg5::Output
    }
}
#[doc = "Field `DirectionReg5` writer - Direction register"]
pub type DirectionReg5W<'a, REG> = crate::BitWriter<'a, REG, DirectionReg5>;
impl<'a, REG> DirectionReg5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(DirectionReg5::Input)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(DirectionReg5::Output)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Intenbl5 {
    #[doc = "0: Disable interrupt"]
    DisableInterrupt = 0,
    #[doc = "1: Enable interrupt"]
    EnableInterrupt = 1,
}
impl From<Intenbl5> for bool {
    #[inline(always)]
    fn from(variant: Intenbl5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTEnbl5` reader - Interrupt enable"]
pub type Intenbl5R = crate::BitReader<Intenbl5>;
impl Intenbl5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Intenbl5 {
        match self.bits {
            false => Intenbl5::DisableInterrupt,
            true => Intenbl5::EnableInterrupt,
        }
    }
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn is_disable_interrupt(&self) -> bool {
        *self == Intenbl5::DisableInterrupt
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn is_enable_interrupt(&self) -> bool {
        *self == Intenbl5::EnableInterrupt
    }
}
#[doc = "Field `INTEnbl5` writer - Interrupt enable"]
pub type Intenbl5W<'a, REG> = crate::BitWriter<'a, REG, Intenbl5>;
impl<'a, REG> Intenbl5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable_interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(Intenbl5::DisableInterrupt)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable_interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(Intenbl5::EnableInterrupt)
    }
}
#[doc = "Interrupt sensitivity type 0 selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntsensitivityType0sel5 {
    #[doc = "0: Select falling-edge or level-low trigger mode"]
    SelectFallingedgeOrLevellowTriggerMode = 0,
    #[doc = "1: Select rising-edge or level-high trigger mode"]
    SelectRisingedgeOrLevelhighTriggerMode = 1,
}
impl From<IntsensitivityType0sel5> for bool {
    #[inline(always)]
    fn from(variant: IntsensitivityType0sel5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTSensitivityType0Sel5` reader - Interrupt sensitivity type 0 selection"]
pub type IntsensitivityType0sel5R = crate::BitReader<IntsensitivityType0sel5>;
impl IntsensitivityType0sel5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntsensitivityType0sel5 {
        match self.bits {
            false => IntsensitivityType0sel5::SelectFallingedgeOrLevellowTriggerMode,
            true => IntsensitivityType0sel5::SelectRisingedgeOrLevelhighTriggerMode,
        }
    }
    #[doc = "Select falling-edge or level-low trigger mode"]
    #[inline(always)]
    pub fn is_select_fallingedge_or_levellow_trigger_mode(&self) -> bool {
        *self == IntsensitivityType0sel5::SelectFallingedgeOrLevellowTriggerMode
    }
    #[doc = "Select rising-edge or level-high trigger mode"]
    #[inline(always)]
    pub fn is_select_risingedge_or_levelhigh_trigger_mode(&self) -> bool {
        *self == IntsensitivityType0sel5::SelectRisingedgeOrLevelhighTriggerMode
    }
}
#[doc = "Field `INTSensitivityType0Sel5` writer - Interrupt sensitivity type 0 selection"]
pub type IntsensitivityType0sel5W<'a, REG> = crate::BitWriter<'a, REG, IntsensitivityType0sel5>;
impl<'a, REG> IntsensitivityType0sel5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select falling-edge or level-low trigger mode"]
    #[inline(always)]
    pub fn select_fallingedge_or_levellow_trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(IntsensitivityType0sel5::SelectFallingedgeOrLevellowTriggerMode)
    }
    #[doc = "Select rising-edge or level-high trigger mode"]
    #[inline(always)]
    pub fn select_risingedge_or_levelhigh_trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(IntsensitivityType0sel5::SelectRisingedgeOrLevelhighTriggerMode)
    }
}
#[doc = "Interrupt sensitivity type 1 selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntsensitivityType1sel5 {
    #[doc = "0: Select edge trigger mode"]
    SelectEdgeTriggerMode = 0,
    #[doc = "1: Select level trigger mode"]
    SelectLevelTriggerMode = 1,
}
impl From<IntsensitivityType1sel5> for bool {
    #[inline(always)]
    fn from(variant: IntsensitivityType1sel5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTSensitivityType1Sel5` reader - Interrupt sensitivity type 1 selection"]
pub type IntsensitivityType1sel5R = crate::BitReader<IntsensitivityType1sel5>;
impl IntsensitivityType1sel5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntsensitivityType1sel5 {
        match self.bits {
            false => IntsensitivityType1sel5::SelectEdgeTriggerMode,
            true => IntsensitivityType1sel5::SelectLevelTriggerMode,
        }
    }
    #[doc = "Select edge trigger mode"]
    #[inline(always)]
    pub fn is_select_edge_trigger_mode(&self) -> bool {
        *self == IntsensitivityType1sel5::SelectEdgeTriggerMode
    }
    #[doc = "Select level trigger mode"]
    #[inline(always)]
    pub fn is_select_level_trigger_mode(&self) -> bool {
        *self == IntsensitivityType1sel5::SelectLevelTriggerMode
    }
}
#[doc = "Field `INTSensitivityType1Sel5` writer - Interrupt sensitivity type 1 selection"]
pub type IntsensitivityType1sel5W<'a, REG> = crate::BitWriter<'a, REG, IntsensitivityType1sel5>;
impl<'a, REG> IntsensitivityType1sel5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select edge trigger mode"]
    #[inline(always)]
    pub fn select_edge_trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(IntsensitivityType1sel5::SelectEdgeTriggerMode)
    }
    #[doc = "Select level trigger mode"]
    #[inline(always)]
    pub fn select_level_trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(IntsensitivityType1sel5::SelectLevelTriggerMode)
    }
}
#[doc = "Interrupt sensitivity type 2 selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntsensitivityType2sel5 {
    #[doc = "0: Select edge or level trigger mode"]
    SelectEdgeOrLevelTriggerMode = 0,
    #[doc = "1: Select dual-edge trigger mode"]
    SelectDualedgeTriggerMode = 1,
}
impl From<IntsensitivityType2sel5> for bool {
    #[inline(always)]
    fn from(variant: IntsensitivityType2sel5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTSensitivityType2Sel5` reader - Interrupt sensitivity type 2 selection"]
pub type IntsensitivityType2sel5R = crate::BitReader<IntsensitivityType2sel5>;
impl IntsensitivityType2sel5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntsensitivityType2sel5 {
        match self.bits {
            false => IntsensitivityType2sel5::SelectEdgeOrLevelTriggerMode,
            true => IntsensitivityType2sel5::SelectDualedgeTriggerMode,
        }
    }
    #[doc = "Select edge or level trigger mode"]
    #[inline(always)]
    pub fn is_select_edge_or_level_trigger_mode(&self) -> bool {
        *self == IntsensitivityType2sel5::SelectEdgeOrLevelTriggerMode
    }
    #[doc = "Select dual-edge trigger mode"]
    #[inline(always)]
    pub fn is_select_dualedge_trigger_mode(&self) -> bool {
        *self == IntsensitivityType2sel5::SelectDualedgeTriggerMode
    }
}
#[doc = "Field `INTSensitivityType2Sel5` writer - Interrupt sensitivity type 2 selection"]
pub type IntsensitivityType2sel5W<'a, REG> = crate::BitWriter<'a, REG, IntsensitivityType2sel5>;
impl<'a, REG> IntsensitivityType2sel5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select edge or level trigger mode"]
    #[inline(always)]
    pub fn select_edge_or_level_trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(IntsensitivityType2sel5::SelectEdgeOrLevelTriggerMode)
    }
    #[doc = "Select dual-edge trigger mode"]
    #[inline(always)]
    pub fn select_dualedge_trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(IntsensitivityType2sel5::SelectDualedgeTriggerMode)
    }
}
#[doc = "Reset tolerance enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RstToleranceEnbl5 {
    #[doc = "0: Bit0 and Bit1 will be reset by WDT\\_SOC and/or EXTRSTN reset"]
    Bit0AndBit1WillBeResetByWdtsocAndorExtrstnReset = 0,
    #[doc = "1: Bit0 and Bit1 will not be reset by WDT\\_SOC and/or EXTRSTN"]
    Bit0AndBit1WillNotBeResetByWdtsocAndorExtrstn = 1,
}
impl From<RstToleranceEnbl5> for bool {
    #[inline(always)]
    fn from(variant: RstToleranceEnbl5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RstToleranceEnbl5` reader - Reset tolerance enable"]
pub type RstToleranceEnbl5R = crate::BitReader<RstToleranceEnbl5>;
impl RstToleranceEnbl5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RstToleranceEnbl5 {
        match self.bits {
            false => RstToleranceEnbl5::Bit0AndBit1WillBeResetByWdtsocAndorExtrstnReset,
            true => RstToleranceEnbl5::Bit0AndBit1WillNotBeResetByWdtsocAndorExtrstn,
        }
    }
    #[doc = "Bit0 and Bit1 will be reset by WDT\\_SOC and/or EXTRSTN reset"]
    #[inline(always)]
    pub fn is_bit0_and_bit1_will_be_reset_by_wdtsoc_andor_extrstn_reset(&self) -> bool {
        *self == RstToleranceEnbl5::Bit0AndBit1WillBeResetByWdtsocAndorExtrstnReset
    }
    #[doc = "Bit0 and Bit1 will not be reset by WDT\\_SOC and/or EXTRSTN"]
    #[inline(always)]
    pub fn is_bit0_and_bit1_will_not_be_reset_by_wdtsoc_andor_extrstn(&self) -> bool {
        *self == RstToleranceEnbl5::Bit0AndBit1WillNotBeResetByWdtsocAndorExtrstn
    }
}
#[doc = "Field `RstToleranceEnbl5` writer - Reset tolerance enable"]
pub type RstToleranceEnbl5W<'a, REG> = crate::BitWriter<'a, REG, RstToleranceEnbl5>;
impl<'a, REG> RstToleranceEnbl5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bit0 and Bit1 will be reset by WDT\\_SOC and/or EXTRSTN reset"]
    #[inline(always)]
    pub fn bit0_and_bit1_will_be_reset_by_wdtsoc_andor_extrstn_reset(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(RstToleranceEnbl5::Bit0AndBit1WillBeResetByWdtsocAndorExtrstnReset)
    }
    #[doc = "Bit0 and Bit1 will not be reset by WDT\\_SOC and/or EXTRSTN"]
    #[inline(always)]
    pub fn bit0_and_bit1_will_not_be_reset_by_wdtsoc_andor_extrstn(self) -> &'a mut crate::W<REG> {
        self.variant(RstToleranceEnbl5::Bit0AndBit1WillNotBeResetByWdtsocAndorExtrstn)
    }
}
#[doc = "Field `DebounceSettingReg15` reader - Debounce setting register #1"]
pub type DebounceSettingReg15R = crate::BitReader;
#[doc = "Field `DebounceSettingReg15` writer - Debounce setting register #1"]
pub type DebounceSettingReg15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DebounceSettingReg25` reader - Debounce setting register #2"]
pub type DebounceSettingReg25R = crate::BitReader;
#[doc = "Field `DebounceSettingReg25` writer - Debounce setting register #2"]
pub type DebounceSettingReg25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Input mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputMask5 {
    #[doc = "0: Read from Bit13 will be updated."]
    ReadFromBit13WillBeUpdated = 0,
    #[doc = "1: Read from Bit13 will not be updated."]
    ReadFromBit13WillNotBeUpdated = 1,
}
impl From<InputMask5> for bool {
    #[inline(always)]
    fn from(variant: InputMask5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `InputMask5` reader - Input mask"]
pub type InputMask5R = crate::BitReader<InputMask5>;
impl InputMask5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> InputMask5 {
        match self.bits {
            false => InputMask5::ReadFromBit13WillBeUpdated,
            true => InputMask5::ReadFromBit13WillNotBeUpdated,
        }
    }
    #[doc = "Read from Bit13 will be updated."]
    #[inline(always)]
    pub fn is_read_from_bit13_will_be_updated(&self) -> bool {
        *self == InputMask5::ReadFromBit13WillBeUpdated
    }
    #[doc = "Read from Bit13 will not be updated."]
    #[inline(always)]
    pub fn is_read_from_bit13_will_not_be_updated(&self) -> bool {
        *self == InputMask5::ReadFromBit13WillNotBeUpdated
    }
}
#[doc = "Field `InputMask5` writer - Input mask"]
pub type InputMask5W<'a, REG> = crate::BitWriter<'a, REG, InputMask5>;
impl<'a, REG> InputMask5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read from Bit13 will be updated."]
    #[inline(always)]
    pub fn read_from_bit13_will_be_updated(self) -> &'a mut crate::W<REG> {
        self.variant(InputMask5::ReadFromBit13WillBeUpdated)
    }
    #[doc = "Read from Bit13 will not be updated."]
    #[inline(always)]
    pub fn read_from_bit13_will_not_be_updated(self) -> &'a mut crate::W<REG> {
        self.variant(InputMask5::ReadFromBit13WillNotBeUpdated)
    }
}
#[doc = "Field `BlinkCounterSel15` reader - Blink Counter Selection #1"]
pub type BlinkCounterSel15R = crate::BitReader;
#[doc = "Field `BlinkCounterSel15` writer - Blink Counter Selection #1"]
pub type BlinkCounterSel15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BlinkCounterSel25` reader - Blink Counter Selection #2"]
pub type BlinkCounterSel25R = crate::BitReader;
#[doc = "Field `BlinkCounterSel25` writer - Blink Counter Selection #2"]
pub type BlinkCounterSel25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt status register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntstsReg5 {
    #[doc = "0: No interrupt pending"]
    NoInterruptPending = 0,
    #[doc = "1: interrupt pending"]
    InterruptPending = 1,
}
impl From<IntstsReg5> for bool {
    #[inline(always)]
    fn from(variant: IntstsReg5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTStsReg5` reader - Interrupt status register"]
pub type IntstsReg5R = crate::BitReader<IntstsReg5>;
impl IntstsReg5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntstsReg5 {
        match self.bits {
            false => IntstsReg5::NoInterruptPending,
            true => IntstsReg5::InterruptPending,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_no_interrupt_pending(&self) -> bool {
        *self == IntstsReg5::NoInterruptPending
    }
    #[doc = "interrupt pending"]
    #[inline(always)]
    pub fn is_interrupt_pending(&self) -> bool {
        *self == IntstsReg5::InterruptPending
    }
}
#[doc = "Field `INTStsReg5` writer - Interrupt status register"]
pub type IntstsReg5W<'a, REG> = crate::BitWriter<'a, REG, IntstsReg5>;
impl<'a, REG> IntstsReg5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn no_interrupt_pending(self) -> &'a mut crate::W<REG> {
        self.variant(IntstsReg5::NoInterruptPending)
    }
    #[doc = "interrupt pending"]
    #[inline(always)]
    pub fn interrupt_pending(self) -> &'a mut crate::W<REG> {
        self.variant(IntstsReg5::InterruptPending)
    }
}
#[doc = "Field `InputDataReg5` reader - Input data register"]
pub type InputDataReg5R = crate::BitReader;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Data register"]
    #[inline(always)]
    pub fn data_reg5(&self) -> DataReg5R {
        DataReg5R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Direction register"]
    #[inline(always)]
    pub fn direction_reg5(&self) -> DirectionReg5R {
        DirectionReg5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable"]
    #[inline(always)]
    pub fn intenbl5(&self) -> Intenbl5R {
        Intenbl5R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn intsensitivity_type0sel5(&self) -> IntsensitivityType0sel5R {
        IntsensitivityType0sel5R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn intsensitivity_type1sel5(&self) -> IntsensitivityType1sel5R {
        IntsensitivityType1sel5R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn intsensitivity_type2sel5(&self) -> IntsensitivityType2sel5R {
        IntsensitivityType2sel5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reset tolerance enable"]
    #[inline(always)]
    pub fn rst_tolerance_enbl5(&self) -> RstToleranceEnbl5R {
        RstToleranceEnbl5R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Debounce setting register #1"]
    #[inline(always)]
    pub fn debounce_setting_reg15(&self) -> DebounceSettingReg15R {
        DebounceSettingReg15R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Debounce setting register #2"]
    #[inline(always)]
    pub fn debounce_setting_reg25(&self) -> DebounceSettingReg25R {
        DebounceSettingReg25R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Input mask"]
    #[inline(always)]
    pub fn input_mask5(&self) -> InputMask5R {
        InputMask5R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Blink Counter Selection #1"]
    #[inline(always)]
    pub fn blink_counter_sel15(&self) -> BlinkCounterSel15R {
        BlinkCounterSel15R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Blink Counter Selection #2"]
    #[inline(always)]
    pub fn blink_counter_sel25(&self) -> BlinkCounterSel25R {
        BlinkCounterSel25R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt status register"]
    #[inline(always)]
    pub fn intsts_reg5(&self) -> IntstsReg5R {
        IntstsReg5R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Input data register"]
    #[inline(always)]
    pub fn input_data_reg5(&self) -> InputDataReg5R {
        InputDataReg5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:31 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Data register"]
    #[inline(always)]
    pub fn data_reg5(&mut self) -> DataReg5W<Gpio2d4Spec> {
        DataReg5W::new(self, 0)
    }
    #[doc = "Bit 1 - Direction register"]
    #[inline(always)]
    pub fn direction_reg5(&mut self) -> DirectionReg5W<Gpio2d4Spec> {
        DirectionReg5W::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt enable"]
    #[inline(always)]
    pub fn intenbl5(&mut self) -> Intenbl5W<Gpio2d4Spec> {
        Intenbl5W::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn intsensitivity_type0sel5(&mut self) -> IntsensitivityType0sel5W<Gpio2d4Spec> {
        IntsensitivityType0sel5W::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn intsensitivity_type1sel5(&mut self) -> IntsensitivityType1sel5W<Gpio2d4Spec> {
        IntsensitivityType1sel5W::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn intsensitivity_type2sel5(&mut self) -> IntsensitivityType2sel5W<Gpio2d4Spec> {
        IntsensitivityType2sel5W::new(self, 5)
    }
    #[doc = "Bit 6 - Reset tolerance enable"]
    #[inline(always)]
    pub fn rst_tolerance_enbl5(&mut self) -> RstToleranceEnbl5W<Gpio2d4Spec> {
        RstToleranceEnbl5W::new(self, 6)
    }
    #[doc = "Bit 7 - Debounce setting register #1"]
    #[inline(always)]
    pub fn debounce_setting_reg15(&mut self) -> DebounceSettingReg15W<Gpio2d4Spec> {
        DebounceSettingReg15W::new(self, 7)
    }
    #[doc = "Bit 8 - Debounce setting register #2"]
    #[inline(always)]
    pub fn debounce_setting_reg25(&mut self) -> DebounceSettingReg25W<Gpio2d4Spec> {
        DebounceSettingReg25W::new(self, 8)
    }
    #[doc = "Bit 9 - Input mask"]
    #[inline(always)]
    pub fn input_mask5(&mut self) -> InputMask5W<Gpio2d4Spec> {
        InputMask5W::new(self, 9)
    }
    #[doc = "Bit 10 - Blink Counter Selection #1"]
    #[inline(always)]
    pub fn blink_counter_sel15(&mut self) -> BlinkCounterSel15W<Gpio2d4Spec> {
        BlinkCounterSel15W::new(self, 10)
    }
    #[doc = "Bit 11 - Blink Counter Selection #2"]
    #[inline(always)]
    pub fn blink_counter_sel25(&mut self) -> BlinkCounterSel25W<Gpio2d4Spec> {
        BlinkCounterSel25W::new(self, 11)
    }
    #[doc = "Bit 12 - Interrupt status register"]
    #[inline(always)]
    pub fn intsts_reg5(&mut self) -> IntstsReg5W<Gpio2d4Spec> {
        IntstsReg5W::new(self, 12)
    }
}
#[doc = "GPIO085 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio2d4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio2d4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio2d4Spec;
impl crate::RegisterSpec for Gpio2d4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio2d4::R`](R) reader structure"]
impl crate::Readable for Gpio2d4Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio2d4::W`](W) writer structure"]
impl crate::Writable for Gpio2d4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO2D4 to value 0"]
impl crate::Resettable for Gpio2d4Spec {}
