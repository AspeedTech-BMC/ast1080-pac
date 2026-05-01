#[doc = "Register `GPIO310` reader"]
pub type R = crate::R<Gpio310Spec>;
#[doc = "Register `GPIO310` writer"]
pub type W = crate::W<Gpio310Spec>;
#[doc = "Data register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataReg4 {
    #[doc = "0: Output 0 if Bit1 is 1."]
    Output0IfBit1Is1 = 0,
    #[doc = "1: Output 1 if Bit1 is 1."]
    Output1IfBit1Is1 = 1,
}
impl From<DataReg4> for bool {
    #[inline(always)]
    fn from(variant: DataReg4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DataReg4` reader - Data register"]
pub type DataReg4R = crate::BitReader<DataReg4>;
impl DataReg4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DataReg4 {
        match self.bits {
            false => DataReg4::Output0IfBit1Is1,
            true => DataReg4::Output1IfBit1Is1,
        }
    }
    #[doc = "Output 0 if Bit1 is 1."]
    #[inline(always)]
    pub fn is_output_0_if_bit1_is_1(&self) -> bool {
        *self == DataReg4::Output0IfBit1Is1
    }
    #[doc = "Output 1 if Bit1 is 1."]
    #[inline(always)]
    pub fn is_output_1_if_bit1_is_1(&self) -> bool {
        *self == DataReg4::Output1IfBit1Is1
    }
}
#[doc = "Field `DataReg4` writer - Data register"]
pub type DataReg4W<'a, REG> = crate::BitWriter<'a, REG, DataReg4>;
impl<'a, REG> DataReg4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output 0 if Bit1 is 1."]
    #[inline(always)]
    pub fn output_0_if_bit1_is_1(self) -> &'a mut crate::W<REG> {
        self.variant(DataReg4::Output0IfBit1Is1)
    }
    #[doc = "Output 1 if Bit1 is 1."]
    #[inline(always)]
    pub fn output_1_if_bit1_is_1(self) -> &'a mut crate::W<REG> {
        self.variant(DataReg4::Output1IfBit1Is1)
    }
}
#[doc = "Direction register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirectionReg4 {
    #[doc = "0: Input"]
    Input = 0,
    #[doc = "1: Output"]
    Output = 1,
}
impl From<DirectionReg4> for bool {
    #[inline(always)]
    fn from(variant: DirectionReg4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DirectionReg4` reader - Direction register"]
pub type DirectionReg4R = crate::BitReader<DirectionReg4>;
impl DirectionReg4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DirectionReg4 {
        match self.bits {
            false => DirectionReg4::Input,
            true => DirectionReg4::Output,
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DirectionReg4::Input
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DirectionReg4::Output
    }
}
#[doc = "Field `DirectionReg4` writer - Direction register"]
pub type DirectionReg4W<'a, REG> = crate::BitWriter<'a, REG, DirectionReg4>;
impl<'a, REG> DirectionReg4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(DirectionReg4::Input)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(DirectionReg4::Output)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Intenbl4 {
    #[doc = "0: Disable interrupt"]
    DisableInterrupt = 0,
    #[doc = "1: Enable interrupt"]
    EnableInterrupt = 1,
}
impl From<Intenbl4> for bool {
    #[inline(always)]
    fn from(variant: Intenbl4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTEnbl4` reader - Interrupt enable"]
pub type Intenbl4R = crate::BitReader<Intenbl4>;
impl Intenbl4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Intenbl4 {
        match self.bits {
            false => Intenbl4::DisableInterrupt,
            true => Intenbl4::EnableInterrupt,
        }
    }
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn is_disable_interrupt(&self) -> bool {
        *self == Intenbl4::DisableInterrupt
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn is_enable_interrupt(&self) -> bool {
        *self == Intenbl4::EnableInterrupt
    }
}
#[doc = "Field `INTEnbl4` writer - Interrupt enable"]
pub type Intenbl4W<'a, REG> = crate::BitWriter<'a, REG, Intenbl4>;
impl<'a, REG> Intenbl4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable_interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(Intenbl4::DisableInterrupt)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable_interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(Intenbl4::EnableInterrupt)
    }
}
#[doc = "Interrupt sensitivity type 0 selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntsensitivityType0sel4 {
    #[doc = "0: Select falling-edge or level-low trigger mode"]
    SelectFallingedgeOrLevellowTriggerMode = 0,
    #[doc = "1: Select rising-edge or level-high trigger mode"]
    SelectRisingedgeOrLevelhighTriggerMode = 1,
}
impl From<IntsensitivityType0sel4> for bool {
    #[inline(always)]
    fn from(variant: IntsensitivityType0sel4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTSensitivityType0Sel4` reader - Interrupt sensitivity type 0 selection"]
pub type IntsensitivityType0sel4R = crate::BitReader<IntsensitivityType0sel4>;
impl IntsensitivityType0sel4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntsensitivityType0sel4 {
        match self.bits {
            false => IntsensitivityType0sel4::SelectFallingedgeOrLevellowTriggerMode,
            true => IntsensitivityType0sel4::SelectRisingedgeOrLevelhighTriggerMode,
        }
    }
    #[doc = "Select falling-edge or level-low trigger mode"]
    #[inline(always)]
    pub fn is_select_fallingedge_or_levellow_trigger_mode(&self) -> bool {
        *self == IntsensitivityType0sel4::SelectFallingedgeOrLevellowTriggerMode
    }
    #[doc = "Select rising-edge or level-high trigger mode"]
    #[inline(always)]
    pub fn is_select_risingedge_or_levelhigh_trigger_mode(&self) -> bool {
        *self == IntsensitivityType0sel4::SelectRisingedgeOrLevelhighTriggerMode
    }
}
#[doc = "Field `INTSensitivityType0Sel4` writer - Interrupt sensitivity type 0 selection"]
pub type IntsensitivityType0sel4W<'a, REG> = crate::BitWriter<'a, REG, IntsensitivityType0sel4>;
impl<'a, REG> IntsensitivityType0sel4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select falling-edge or level-low trigger mode"]
    #[inline(always)]
    pub fn select_fallingedge_or_levellow_trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(IntsensitivityType0sel4::SelectFallingedgeOrLevellowTriggerMode)
    }
    #[doc = "Select rising-edge or level-high trigger mode"]
    #[inline(always)]
    pub fn select_risingedge_or_levelhigh_trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(IntsensitivityType0sel4::SelectRisingedgeOrLevelhighTriggerMode)
    }
}
#[doc = "Interrupt sensitivity type 1 selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntsensitivityType1sel4 {
    #[doc = "0: Select edge trigger mode"]
    SelectEdgeTriggerMode = 0,
    #[doc = "1: Select level trigger mode"]
    SelectLevelTriggerMode = 1,
}
impl From<IntsensitivityType1sel4> for bool {
    #[inline(always)]
    fn from(variant: IntsensitivityType1sel4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTSensitivityType1Sel4` reader - Interrupt sensitivity type 1 selection"]
pub type IntsensitivityType1sel4R = crate::BitReader<IntsensitivityType1sel4>;
impl IntsensitivityType1sel4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntsensitivityType1sel4 {
        match self.bits {
            false => IntsensitivityType1sel4::SelectEdgeTriggerMode,
            true => IntsensitivityType1sel4::SelectLevelTriggerMode,
        }
    }
    #[doc = "Select edge trigger mode"]
    #[inline(always)]
    pub fn is_select_edge_trigger_mode(&self) -> bool {
        *self == IntsensitivityType1sel4::SelectEdgeTriggerMode
    }
    #[doc = "Select level trigger mode"]
    #[inline(always)]
    pub fn is_select_level_trigger_mode(&self) -> bool {
        *self == IntsensitivityType1sel4::SelectLevelTriggerMode
    }
}
#[doc = "Field `INTSensitivityType1Sel4` writer - Interrupt sensitivity type 1 selection"]
pub type IntsensitivityType1sel4W<'a, REG> = crate::BitWriter<'a, REG, IntsensitivityType1sel4>;
impl<'a, REG> IntsensitivityType1sel4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select edge trigger mode"]
    #[inline(always)]
    pub fn select_edge_trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(IntsensitivityType1sel4::SelectEdgeTriggerMode)
    }
    #[doc = "Select level trigger mode"]
    #[inline(always)]
    pub fn select_level_trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(IntsensitivityType1sel4::SelectLevelTriggerMode)
    }
}
#[doc = "Interrupt sensitivity type 2 selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntsensitivityType2sel4 {
    #[doc = "0: Select edge or level trigger mode"]
    SelectEdgeOrLevelTriggerMode = 0,
    #[doc = "1: Select dual-edge trigger mode"]
    SelectDualedgeTriggerMode = 1,
}
impl From<IntsensitivityType2sel4> for bool {
    #[inline(always)]
    fn from(variant: IntsensitivityType2sel4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTSensitivityType2Sel4` reader - Interrupt sensitivity type 2 selection"]
pub type IntsensitivityType2sel4R = crate::BitReader<IntsensitivityType2sel4>;
impl IntsensitivityType2sel4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntsensitivityType2sel4 {
        match self.bits {
            false => IntsensitivityType2sel4::SelectEdgeOrLevelTriggerMode,
            true => IntsensitivityType2sel4::SelectDualedgeTriggerMode,
        }
    }
    #[doc = "Select edge or level trigger mode"]
    #[inline(always)]
    pub fn is_select_edge_or_level_trigger_mode(&self) -> bool {
        *self == IntsensitivityType2sel4::SelectEdgeOrLevelTriggerMode
    }
    #[doc = "Select dual-edge trigger mode"]
    #[inline(always)]
    pub fn is_select_dualedge_trigger_mode(&self) -> bool {
        *self == IntsensitivityType2sel4::SelectDualedgeTriggerMode
    }
}
#[doc = "Field `INTSensitivityType2Sel4` writer - Interrupt sensitivity type 2 selection"]
pub type IntsensitivityType2sel4W<'a, REG> = crate::BitWriter<'a, REG, IntsensitivityType2sel4>;
impl<'a, REG> IntsensitivityType2sel4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select edge or level trigger mode"]
    #[inline(always)]
    pub fn select_edge_or_level_trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(IntsensitivityType2sel4::SelectEdgeOrLevelTriggerMode)
    }
    #[doc = "Select dual-edge trigger mode"]
    #[inline(always)]
    pub fn select_dualedge_trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(IntsensitivityType2sel4::SelectDualedgeTriggerMode)
    }
}
#[doc = "Reset tolerance enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RstToleranceEnbl4 {
    #[doc = "0: Bit0 and Bit1 will be reset by WDT\\_SOC and/or EXTRSTN reset"]
    Bit0AndBit1WillBeResetByWdtsocAndorExtrstnReset = 0,
    #[doc = "1: Bit0 and Bit1 will not be reset by WDT\\_SOC and/or EXTRSTN"]
    Bit0AndBit1WillNotBeResetByWdtsocAndorExtrstn = 1,
}
impl From<RstToleranceEnbl4> for bool {
    #[inline(always)]
    fn from(variant: RstToleranceEnbl4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RstToleranceEnbl4` reader - Reset tolerance enable"]
pub type RstToleranceEnbl4R = crate::BitReader<RstToleranceEnbl4>;
impl RstToleranceEnbl4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RstToleranceEnbl4 {
        match self.bits {
            false => RstToleranceEnbl4::Bit0AndBit1WillBeResetByWdtsocAndorExtrstnReset,
            true => RstToleranceEnbl4::Bit0AndBit1WillNotBeResetByWdtsocAndorExtrstn,
        }
    }
    #[doc = "Bit0 and Bit1 will be reset by WDT\\_SOC and/or EXTRSTN reset"]
    #[inline(always)]
    pub fn is_bit0_and_bit1_will_be_reset_by_wdtsoc_andor_extrstn_reset(&self) -> bool {
        *self == RstToleranceEnbl4::Bit0AndBit1WillBeResetByWdtsocAndorExtrstnReset
    }
    #[doc = "Bit0 and Bit1 will not be reset by WDT\\_SOC and/or EXTRSTN"]
    #[inline(always)]
    pub fn is_bit0_and_bit1_will_not_be_reset_by_wdtsoc_andor_extrstn(&self) -> bool {
        *self == RstToleranceEnbl4::Bit0AndBit1WillNotBeResetByWdtsocAndorExtrstn
    }
}
#[doc = "Field `RstToleranceEnbl4` writer - Reset tolerance enable"]
pub type RstToleranceEnbl4W<'a, REG> = crate::BitWriter<'a, REG, RstToleranceEnbl4>;
impl<'a, REG> RstToleranceEnbl4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bit0 and Bit1 will be reset by WDT\\_SOC and/or EXTRSTN reset"]
    #[inline(always)]
    pub fn bit0_and_bit1_will_be_reset_by_wdtsoc_andor_extrstn_reset(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(RstToleranceEnbl4::Bit0AndBit1WillBeResetByWdtsocAndorExtrstnReset)
    }
    #[doc = "Bit0 and Bit1 will not be reset by WDT\\_SOC and/or EXTRSTN"]
    #[inline(always)]
    pub fn bit0_and_bit1_will_not_be_reset_by_wdtsoc_andor_extrstn(self) -> &'a mut crate::W<REG> {
        self.variant(RstToleranceEnbl4::Bit0AndBit1WillNotBeResetByWdtsocAndorExtrstn)
    }
}
#[doc = "Field `DebounceSettingReg14` reader - Debounce setting register #1"]
pub type DebounceSettingReg14R = crate::BitReader;
#[doc = "Field `DebounceSettingReg14` writer - Debounce setting register #1"]
pub type DebounceSettingReg14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DebounceSettingReg24` reader - Debounce setting register #2"]
pub type DebounceSettingReg24R = crate::BitReader;
#[doc = "Field `DebounceSettingReg24` writer - Debounce setting register #2"]
pub type DebounceSettingReg24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Input mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputMask4 {
    #[doc = "0: Read from Bit13 will be updated."]
    ReadFromBit13WillBeUpdated = 0,
    #[doc = "1: Read from Bit13 will not be updated."]
    ReadFromBit13WillNotBeUpdated = 1,
}
impl From<InputMask4> for bool {
    #[inline(always)]
    fn from(variant: InputMask4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `InputMask4` reader - Input mask"]
pub type InputMask4R = crate::BitReader<InputMask4>;
impl InputMask4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> InputMask4 {
        match self.bits {
            false => InputMask4::ReadFromBit13WillBeUpdated,
            true => InputMask4::ReadFromBit13WillNotBeUpdated,
        }
    }
    #[doc = "Read from Bit13 will be updated."]
    #[inline(always)]
    pub fn is_read_from_bit13_will_be_updated(&self) -> bool {
        *self == InputMask4::ReadFromBit13WillBeUpdated
    }
    #[doc = "Read from Bit13 will not be updated."]
    #[inline(always)]
    pub fn is_read_from_bit13_will_not_be_updated(&self) -> bool {
        *self == InputMask4::ReadFromBit13WillNotBeUpdated
    }
}
#[doc = "Field `InputMask4` writer - Input mask"]
pub type InputMask4W<'a, REG> = crate::BitWriter<'a, REG, InputMask4>;
impl<'a, REG> InputMask4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read from Bit13 will be updated."]
    #[inline(always)]
    pub fn read_from_bit13_will_be_updated(self) -> &'a mut crate::W<REG> {
        self.variant(InputMask4::ReadFromBit13WillBeUpdated)
    }
    #[doc = "Read from Bit13 will not be updated."]
    #[inline(always)]
    pub fn read_from_bit13_will_not_be_updated(self) -> &'a mut crate::W<REG> {
        self.variant(InputMask4::ReadFromBit13WillNotBeUpdated)
    }
}
#[doc = "Field `BlinkCounterSel14` reader - Blink Counter Selection #1"]
pub type BlinkCounterSel14R = crate::BitReader;
#[doc = "Field `BlinkCounterSel14` writer - Blink Counter Selection #1"]
pub type BlinkCounterSel14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BlinkCounterSel24` reader - Blink Counter Selection #2"]
pub type BlinkCounterSel24R = crate::BitReader;
#[doc = "Field `BlinkCounterSel24` writer - Blink Counter Selection #2"]
pub type BlinkCounterSel24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt status register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntstsReg4 {
    #[doc = "0: No interrupt pending"]
    NoInterruptPending = 0,
    #[doc = "1: interrupt pending"]
    InterruptPending = 1,
}
impl From<IntstsReg4> for bool {
    #[inline(always)]
    fn from(variant: IntstsReg4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTStsReg4` reader - Interrupt status register"]
pub type IntstsReg4R = crate::BitReader<IntstsReg4>;
impl IntstsReg4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntstsReg4 {
        match self.bits {
            false => IntstsReg4::NoInterruptPending,
            true => IntstsReg4::InterruptPending,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_no_interrupt_pending(&self) -> bool {
        *self == IntstsReg4::NoInterruptPending
    }
    #[doc = "interrupt pending"]
    #[inline(always)]
    pub fn is_interrupt_pending(&self) -> bool {
        *self == IntstsReg4::InterruptPending
    }
}
#[doc = "Field `INTStsReg4` writer - Interrupt status register"]
pub type IntstsReg4W<'a, REG> = crate::BitWriter<'a, REG, IntstsReg4>;
impl<'a, REG> IntstsReg4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn no_interrupt_pending(self) -> &'a mut crate::W<REG> {
        self.variant(IntstsReg4::NoInterruptPending)
    }
    #[doc = "interrupt pending"]
    #[inline(always)]
    pub fn interrupt_pending(self) -> &'a mut crate::W<REG> {
        self.variant(IntstsReg4::InterruptPending)
    }
}
#[doc = "Field `InputDataReg4` reader - Input data register"]
pub type InputDataReg4R = crate::BitReader;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Data register"]
    #[inline(always)]
    pub fn data_reg4(&self) -> DataReg4R {
        DataReg4R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Direction register"]
    #[inline(always)]
    pub fn direction_reg4(&self) -> DirectionReg4R {
        DirectionReg4R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable"]
    #[inline(always)]
    pub fn intenbl4(&self) -> Intenbl4R {
        Intenbl4R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn intsensitivity_type0sel4(&self) -> IntsensitivityType0sel4R {
        IntsensitivityType0sel4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn intsensitivity_type1sel4(&self) -> IntsensitivityType1sel4R {
        IntsensitivityType1sel4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn intsensitivity_type2sel4(&self) -> IntsensitivityType2sel4R {
        IntsensitivityType2sel4R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reset tolerance enable"]
    #[inline(always)]
    pub fn rst_tolerance_enbl4(&self) -> RstToleranceEnbl4R {
        RstToleranceEnbl4R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Debounce setting register #1"]
    #[inline(always)]
    pub fn debounce_setting_reg14(&self) -> DebounceSettingReg14R {
        DebounceSettingReg14R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Debounce setting register #2"]
    #[inline(always)]
    pub fn debounce_setting_reg24(&self) -> DebounceSettingReg24R {
        DebounceSettingReg24R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Input mask"]
    #[inline(always)]
    pub fn input_mask4(&self) -> InputMask4R {
        InputMask4R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Blink Counter Selection #1"]
    #[inline(always)]
    pub fn blink_counter_sel14(&self) -> BlinkCounterSel14R {
        BlinkCounterSel14R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Blink Counter Selection #2"]
    #[inline(always)]
    pub fn blink_counter_sel24(&self) -> BlinkCounterSel24R {
        BlinkCounterSel24R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt status register"]
    #[inline(always)]
    pub fn intsts_reg4(&self) -> IntstsReg4R {
        IntstsReg4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Input data register"]
    #[inline(always)]
    pub fn input_data_reg4(&self) -> InputDataReg4R {
        InputDataReg4R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:31 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Data register"]
    #[inline(always)]
    pub fn data_reg4(&mut self) -> DataReg4W<Gpio310Spec> {
        DataReg4W::new(self, 0)
    }
    #[doc = "Bit 1 - Direction register"]
    #[inline(always)]
    pub fn direction_reg4(&mut self) -> DirectionReg4W<Gpio310Spec> {
        DirectionReg4W::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt enable"]
    #[inline(always)]
    pub fn intenbl4(&mut self) -> Intenbl4W<Gpio310Spec> {
        Intenbl4W::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn intsensitivity_type0sel4(&mut self) -> IntsensitivityType0sel4W<Gpio310Spec> {
        IntsensitivityType0sel4W::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn intsensitivity_type1sel4(&mut self) -> IntsensitivityType1sel4W<Gpio310Spec> {
        IntsensitivityType1sel4W::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn intsensitivity_type2sel4(&mut self) -> IntsensitivityType2sel4W<Gpio310Spec> {
        IntsensitivityType2sel4W::new(self, 5)
    }
    #[doc = "Bit 6 - Reset tolerance enable"]
    #[inline(always)]
    pub fn rst_tolerance_enbl4(&mut self) -> RstToleranceEnbl4W<Gpio310Spec> {
        RstToleranceEnbl4W::new(self, 6)
    }
    #[doc = "Bit 7 - Debounce setting register #1"]
    #[inline(always)]
    pub fn debounce_setting_reg14(&mut self) -> DebounceSettingReg14W<Gpio310Spec> {
        DebounceSettingReg14W::new(self, 7)
    }
    #[doc = "Bit 8 - Debounce setting register #2"]
    #[inline(always)]
    pub fn debounce_setting_reg24(&mut self) -> DebounceSettingReg24W<Gpio310Spec> {
        DebounceSettingReg24W::new(self, 8)
    }
    #[doc = "Bit 9 - Input mask"]
    #[inline(always)]
    pub fn input_mask4(&mut self) -> InputMask4W<Gpio310Spec> {
        InputMask4W::new(self, 9)
    }
    #[doc = "Bit 10 - Blink Counter Selection #1"]
    #[inline(always)]
    pub fn blink_counter_sel14(&mut self) -> BlinkCounterSel14W<Gpio310Spec> {
        BlinkCounterSel14W::new(self, 10)
    }
    #[doc = "Bit 11 - Blink Counter Selection #2"]
    #[inline(always)]
    pub fn blink_counter_sel24(&mut self) -> BlinkCounterSel24W<Gpio310Spec> {
        BlinkCounterSel24W::new(self, 11)
    }
    #[doc = "Bit 12 - Interrupt status register"]
    #[inline(always)]
    pub fn intsts_reg4(&mut self) -> IntstsReg4W<Gpio310Spec> {
        IntstsReg4W::new(self, 12)
    }
}
#[doc = "GPIO100 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio310::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio310::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio310Spec;
impl crate::RegisterSpec for Gpio310Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio310::R`](R) reader structure"]
impl crate::Readable for Gpio310Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio310::W`](W) writer structure"]
impl crate::Writable for Gpio310Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO310 to value 0"]
impl crate::Resettable for Gpio310Spec {}
