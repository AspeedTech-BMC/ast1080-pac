#[doc = "Register `GPIO2A0` reader"]
pub type R = crate::R<Gpio2a0Spec>;
#[doc = "Register `GPIO2A0` writer"]
pub type W = crate::W<Gpio2a0Spec>;
#[doc = "Data register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataReg {
    #[doc = "0: Output 0 if Bit1 is 1."]
    Output0IfBit1Is1 = 0,
    #[doc = "1: Output 1 if Bit1 is 1."]
    Output1IfBit1Is1 = 1,
}
impl From<DataReg> for bool {
    #[inline(always)]
    fn from(variant: DataReg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DataReg` reader - Data register"]
pub type DataRegR = crate::BitReader<DataReg>;
impl DataRegR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DataReg {
        match self.bits {
            false => DataReg::Output0IfBit1Is1,
            true => DataReg::Output1IfBit1Is1,
        }
    }
    #[doc = "Output 0 if Bit1 is 1."]
    #[inline(always)]
    pub fn is_output_0_if_bit1_is_1(&self) -> bool {
        *self == DataReg::Output0IfBit1Is1
    }
    #[doc = "Output 1 if Bit1 is 1."]
    #[inline(always)]
    pub fn is_output_1_if_bit1_is_1(&self) -> bool {
        *self == DataReg::Output1IfBit1Is1
    }
}
#[doc = "Field `DataReg` writer - Data register"]
pub type DataRegW<'a, REG> = crate::BitWriter<'a, REG, DataReg>;
impl<'a, REG> DataRegW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output 0 if Bit1 is 1."]
    #[inline(always)]
    pub fn output_0_if_bit1_is_1(self) -> &'a mut crate::W<REG> {
        self.variant(DataReg::Output0IfBit1Is1)
    }
    #[doc = "Output 1 if Bit1 is 1."]
    #[inline(always)]
    pub fn output_1_if_bit1_is_1(self) -> &'a mut crate::W<REG> {
        self.variant(DataReg::Output1IfBit1Is1)
    }
}
#[doc = "Direction register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirectionReg {
    #[doc = "0: Input"]
    Input = 0,
    #[doc = "1: Output"]
    Output = 1,
}
impl From<DirectionReg> for bool {
    #[inline(always)]
    fn from(variant: DirectionReg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DirectionReg` reader - Direction register"]
pub type DirectionRegR = crate::BitReader<DirectionReg>;
impl DirectionRegR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DirectionReg {
        match self.bits {
            false => DirectionReg::Input,
            true => DirectionReg::Output,
        }
    }
    #[doc = "Input"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DirectionReg::Input
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DirectionReg::Output
    }
}
#[doc = "Field `DirectionReg` writer - Direction register"]
pub type DirectionRegW<'a, REG> = crate::BitWriter<'a, REG, DirectionReg>;
impl<'a, REG> DirectionRegW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(DirectionReg::Input)
    }
    #[doc = "Output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(DirectionReg::Output)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Intenbl {
    #[doc = "0: Disable interrupt"]
    DisableInterrupt = 0,
    #[doc = "1: Enable interrupt"]
    EnableInterrupt = 1,
}
impl From<Intenbl> for bool {
    #[inline(always)]
    fn from(variant: Intenbl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTEnbl` reader - Interrupt enable"]
pub type IntenblR = crate::BitReader<Intenbl>;
impl IntenblR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Intenbl {
        match self.bits {
            false => Intenbl::DisableInterrupt,
            true => Intenbl::EnableInterrupt,
        }
    }
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn is_disable_interrupt(&self) -> bool {
        *self == Intenbl::DisableInterrupt
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn is_enable_interrupt(&self) -> bool {
        *self == Intenbl::EnableInterrupt
    }
}
#[doc = "Field `INTEnbl` writer - Interrupt enable"]
pub type IntenblW<'a, REG> = crate::BitWriter<'a, REG, Intenbl>;
impl<'a, REG> IntenblW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable_interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(Intenbl::DisableInterrupt)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable_interrupt(self) -> &'a mut crate::W<REG> {
        self.variant(Intenbl::EnableInterrupt)
    }
}
#[doc = "Interrupt sensitivity type 0 selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntsensitivityType0sel {
    #[doc = "0: Select falling-edge or level-low trigger mode"]
    SelectFallingedgeOrLevellowTriggerMode = 0,
    #[doc = "1: Select rising-edge or level-high trigger mode"]
    SelectRisingedgeOrLevelhighTriggerMode = 1,
}
impl From<IntsensitivityType0sel> for bool {
    #[inline(always)]
    fn from(variant: IntsensitivityType0sel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTSensitivityType0Sel` reader - Interrupt sensitivity type 0 selection"]
pub type IntsensitivityType0selR = crate::BitReader<IntsensitivityType0sel>;
impl IntsensitivityType0selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntsensitivityType0sel {
        match self.bits {
            false => IntsensitivityType0sel::SelectFallingedgeOrLevellowTriggerMode,
            true => IntsensitivityType0sel::SelectRisingedgeOrLevelhighTriggerMode,
        }
    }
    #[doc = "Select falling-edge or level-low trigger mode"]
    #[inline(always)]
    pub fn is_select_fallingedge_or_levellow_trigger_mode(&self) -> bool {
        *self == IntsensitivityType0sel::SelectFallingedgeOrLevellowTriggerMode
    }
    #[doc = "Select rising-edge or level-high trigger mode"]
    #[inline(always)]
    pub fn is_select_risingedge_or_levelhigh_trigger_mode(&self) -> bool {
        *self == IntsensitivityType0sel::SelectRisingedgeOrLevelhighTriggerMode
    }
}
#[doc = "Field `INTSensitivityType0Sel` writer - Interrupt sensitivity type 0 selection"]
pub type IntsensitivityType0selW<'a, REG> = crate::BitWriter<'a, REG, IntsensitivityType0sel>;
impl<'a, REG> IntsensitivityType0selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select falling-edge or level-low trigger mode"]
    #[inline(always)]
    pub fn select_fallingedge_or_levellow_trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(IntsensitivityType0sel::SelectFallingedgeOrLevellowTriggerMode)
    }
    #[doc = "Select rising-edge or level-high trigger mode"]
    #[inline(always)]
    pub fn select_risingedge_or_levelhigh_trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(IntsensitivityType0sel::SelectRisingedgeOrLevelhighTriggerMode)
    }
}
#[doc = "Interrupt sensitivity type 1 selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntsensitivityType1sel {
    #[doc = "0: Select edge trigger mode"]
    SelectEdgeTriggerMode = 0,
    #[doc = "1: Select level trigger mode"]
    SelectLevelTriggerMode = 1,
}
impl From<IntsensitivityType1sel> for bool {
    #[inline(always)]
    fn from(variant: IntsensitivityType1sel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTSensitivityType1Sel` reader - Interrupt sensitivity type 1 selection"]
pub type IntsensitivityType1selR = crate::BitReader<IntsensitivityType1sel>;
impl IntsensitivityType1selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntsensitivityType1sel {
        match self.bits {
            false => IntsensitivityType1sel::SelectEdgeTriggerMode,
            true => IntsensitivityType1sel::SelectLevelTriggerMode,
        }
    }
    #[doc = "Select edge trigger mode"]
    #[inline(always)]
    pub fn is_select_edge_trigger_mode(&self) -> bool {
        *self == IntsensitivityType1sel::SelectEdgeTriggerMode
    }
    #[doc = "Select level trigger mode"]
    #[inline(always)]
    pub fn is_select_level_trigger_mode(&self) -> bool {
        *self == IntsensitivityType1sel::SelectLevelTriggerMode
    }
}
#[doc = "Field `INTSensitivityType1Sel` writer - Interrupt sensitivity type 1 selection"]
pub type IntsensitivityType1selW<'a, REG> = crate::BitWriter<'a, REG, IntsensitivityType1sel>;
impl<'a, REG> IntsensitivityType1selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select edge trigger mode"]
    #[inline(always)]
    pub fn select_edge_trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(IntsensitivityType1sel::SelectEdgeTriggerMode)
    }
    #[doc = "Select level trigger mode"]
    #[inline(always)]
    pub fn select_level_trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(IntsensitivityType1sel::SelectLevelTriggerMode)
    }
}
#[doc = "Interrupt sensitivity type 2 selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntsensitivityType2sel {
    #[doc = "0: Select edge or level trigger mode"]
    SelectEdgeOrLevelTriggerMode = 0,
    #[doc = "1: Select dual-edge trigger mode"]
    SelectDualedgeTriggerMode = 1,
}
impl From<IntsensitivityType2sel> for bool {
    #[inline(always)]
    fn from(variant: IntsensitivityType2sel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTSensitivityType2Sel` reader - Interrupt sensitivity type 2 selection"]
pub type IntsensitivityType2selR = crate::BitReader<IntsensitivityType2sel>;
impl IntsensitivityType2selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntsensitivityType2sel {
        match self.bits {
            false => IntsensitivityType2sel::SelectEdgeOrLevelTriggerMode,
            true => IntsensitivityType2sel::SelectDualedgeTriggerMode,
        }
    }
    #[doc = "Select edge or level trigger mode"]
    #[inline(always)]
    pub fn is_select_edge_or_level_trigger_mode(&self) -> bool {
        *self == IntsensitivityType2sel::SelectEdgeOrLevelTriggerMode
    }
    #[doc = "Select dual-edge trigger mode"]
    #[inline(always)]
    pub fn is_select_dualedge_trigger_mode(&self) -> bool {
        *self == IntsensitivityType2sel::SelectDualedgeTriggerMode
    }
}
#[doc = "Field `INTSensitivityType2Sel` writer - Interrupt sensitivity type 2 selection"]
pub type IntsensitivityType2selW<'a, REG> = crate::BitWriter<'a, REG, IntsensitivityType2sel>;
impl<'a, REG> IntsensitivityType2selW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select edge or level trigger mode"]
    #[inline(always)]
    pub fn select_edge_or_level_trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(IntsensitivityType2sel::SelectEdgeOrLevelTriggerMode)
    }
    #[doc = "Select dual-edge trigger mode"]
    #[inline(always)]
    pub fn select_dualedge_trigger_mode(self) -> &'a mut crate::W<REG> {
        self.variant(IntsensitivityType2sel::SelectDualedgeTriggerMode)
    }
}
#[doc = "Reset tolerance enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RstToleranceEnbl {
    #[doc = "0: Bit0 and Bit1 will be reset by WDT\\_SOC and/or EXTRSTN reset"]
    Bit0AndBit1WillBeResetByWdtsocAndorExtrstnReset = 0,
    #[doc = "1: Bit0 and Bit1 will not be reset by WDT\\_SOC and/or EXTRSTN"]
    Bit0AndBit1WillNotBeResetByWdtsocAndorExtrstn = 1,
}
impl From<RstToleranceEnbl> for bool {
    #[inline(always)]
    fn from(variant: RstToleranceEnbl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RstToleranceEnbl` reader - Reset tolerance enable"]
pub type RstToleranceEnblR = crate::BitReader<RstToleranceEnbl>;
impl RstToleranceEnblR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RstToleranceEnbl {
        match self.bits {
            false => RstToleranceEnbl::Bit0AndBit1WillBeResetByWdtsocAndorExtrstnReset,
            true => RstToleranceEnbl::Bit0AndBit1WillNotBeResetByWdtsocAndorExtrstn,
        }
    }
    #[doc = "Bit0 and Bit1 will be reset by WDT\\_SOC and/or EXTRSTN reset"]
    #[inline(always)]
    pub fn is_bit0_and_bit1_will_be_reset_by_wdtsoc_andor_extrstn_reset(&self) -> bool {
        *self == RstToleranceEnbl::Bit0AndBit1WillBeResetByWdtsocAndorExtrstnReset
    }
    #[doc = "Bit0 and Bit1 will not be reset by WDT\\_SOC and/or EXTRSTN"]
    #[inline(always)]
    pub fn is_bit0_and_bit1_will_not_be_reset_by_wdtsoc_andor_extrstn(&self) -> bool {
        *self == RstToleranceEnbl::Bit0AndBit1WillNotBeResetByWdtsocAndorExtrstn
    }
}
#[doc = "Field `RstToleranceEnbl` writer - Reset tolerance enable"]
pub type RstToleranceEnblW<'a, REG> = crate::BitWriter<'a, REG, RstToleranceEnbl>;
impl<'a, REG> RstToleranceEnblW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bit0 and Bit1 will be reset by WDT\\_SOC and/or EXTRSTN reset"]
    #[inline(always)]
    pub fn bit0_and_bit1_will_be_reset_by_wdtsoc_andor_extrstn_reset(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(RstToleranceEnbl::Bit0AndBit1WillBeResetByWdtsocAndorExtrstnReset)
    }
    #[doc = "Bit0 and Bit1 will not be reset by WDT\\_SOC and/or EXTRSTN"]
    #[inline(always)]
    pub fn bit0_and_bit1_will_not_be_reset_by_wdtsoc_andor_extrstn(self) -> &'a mut crate::W<REG> {
        self.variant(RstToleranceEnbl::Bit0AndBit1WillNotBeResetByWdtsocAndorExtrstn)
    }
}
#[doc = "Field `DebounceSettingReg1` reader - Debounce setting register #1"]
pub type DebounceSettingReg1R = crate::BitReader;
#[doc = "Field `DebounceSettingReg1` writer - Debounce setting register #1"]
pub type DebounceSettingReg1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DebounceSettingReg2` reader - Debounce setting register #2"]
pub type DebounceSettingReg2R = crate::BitReader;
#[doc = "Field `DebounceSettingReg2` writer - Debounce setting register #2"]
pub type DebounceSettingReg2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Input mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputMask {
    #[doc = "0: Read from Bit13 will be updated."]
    ReadFromBit13WillBeUpdated = 0,
    #[doc = "1: Read from Bit13 will not be updated."]
    ReadFromBit13WillNotBeUpdated = 1,
}
impl From<InputMask> for bool {
    #[inline(always)]
    fn from(variant: InputMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `InputMask` reader - Input mask"]
pub type InputMaskR = crate::BitReader<InputMask>;
impl InputMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> InputMask {
        match self.bits {
            false => InputMask::ReadFromBit13WillBeUpdated,
            true => InputMask::ReadFromBit13WillNotBeUpdated,
        }
    }
    #[doc = "Read from Bit13 will be updated."]
    #[inline(always)]
    pub fn is_read_from_bit13_will_be_updated(&self) -> bool {
        *self == InputMask::ReadFromBit13WillBeUpdated
    }
    #[doc = "Read from Bit13 will not be updated."]
    #[inline(always)]
    pub fn is_read_from_bit13_will_not_be_updated(&self) -> bool {
        *self == InputMask::ReadFromBit13WillNotBeUpdated
    }
}
#[doc = "Field `InputMask` writer - Input mask"]
pub type InputMaskW<'a, REG> = crate::BitWriter<'a, REG, InputMask>;
impl<'a, REG> InputMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read from Bit13 will be updated."]
    #[inline(always)]
    pub fn read_from_bit13_will_be_updated(self) -> &'a mut crate::W<REG> {
        self.variant(InputMask::ReadFromBit13WillBeUpdated)
    }
    #[doc = "Read from Bit13 will not be updated."]
    #[inline(always)]
    pub fn read_from_bit13_will_not_be_updated(self) -> &'a mut crate::W<REG> {
        self.variant(InputMask::ReadFromBit13WillNotBeUpdated)
    }
}
#[doc = "Field `BlinkCounterSel1` reader - Blink Counter Selection #1"]
pub type BlinkCounterSel1R = crate::BitReader;
#[doc = "Field `BlinkCounterSel1` writer - Blink Counter Selection #1"]
pub type BlinkCounterSel1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BlinkCounterSel2` reader - Blink Counter Selection #2"]
pub type BlinkCounterSel2R = crate::BitReader;
#[doc = "Field `BlinkCounterSel2` writer - Blink Counter Selection #2"]
pub type BlinkCounterSel2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt status register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntstsReg {
    #[doc = "0: No interrupt pending"]
    NoInterruptPending = 0,
    #[doc = "1: interrupt pending"]
    InterruptPending = 1,
}
impl From<IntstsReg> for bool {
    #[inline(always)]
    fn from(variant: IntstsReg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTStsReg` reader - Interrupt status register"]
pub type IntstsRegR = crate::BitReader<IntstsReg>;
impl IntstsRegR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntstsReg {
        match self.bits {
            false => IntstsReg::NoInterruptPending,
            true => IntstsReg::InterruptPending,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_no_interrupt_pending(&self) -> bool {
        *self == IntstsReg::NoInterruptPending
    }
    #[doc = "interrupt pending"]
    #[inline(always)]
    pub fn is_interrupt_pending(&self) -> bool {
        *self == IntstsReg::InterruptPending
    }
}
#[doc = "Field `INTStsReg` writer - Interrupt status register"]
pub type IntstsRegW<'a, REG> = crate::BitWriter<'a, REG, IntstsReg>;
impl<'a, REG> IntstsRegW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn no_interrupt_pending(self) -> &'a mut crate::W<REG> {
        self.variant(IntstsReg::NoInterruptPending)
    }
    #[doc = "interrupt pending"]
    #[inline(always)]
    pub fn interrupt_pending(self) -> &'a mut crate::W<REG> {
        self.variant(IntstsReg::InterruptPending)
    }
}
#[doc = "Field `InputDataReg` reader - Input data register"]
pub type InputDataRegR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Data register"]
    #[inline(always)]
    pub fn data_reg(&self) -> DataRegR {
        DataRegR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Direction register"]
    #[inline(always)]
    pub fn direction_reg(&self) -> DirectionRegR {
        DirectionRegR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable"]
    #[inline(always)]
    pub fn intenbl(&self) -> IntenblR {
        IntenblR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn intsensitivity_type0sel(&self) -> IntsensitivityType0selR {
        IntsensitivityType0selR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn intsensitivity_type1sel(&self) -> IntsensitivityType1selR {
        IntsensitivityType1selR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn intsensitivity_type2sel(&self) -> IntsensitivityType2selR {
        IntsensitivityType2selR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reset tolerance enable"]
    #[inline(always)]
    pub fn rst_tolerance_enbl(&self) -> RstToleranceEnblR {
        RstToleranceEnblR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Debounce setting register #1"]
    #[inline(always)]
    pub fn debounce_setting_reg1(&self) -> DebounceSettingReg1R {
        DebounceSettingReg1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Debounce setting register #2"]
    #[inline(always)]
    pub fn debounce_setting_reg2(&self) -> DebounceSettingReg2R {
        DebounceSettingReg2R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Input mask"]
    #[inline(always)]
    pub fn input_mask(&self) -> InputMaskR {
        InputMaskR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Blink Counter Selection #1"]
    #[inline(always)]
    pub fn blink_counter_sel1(&self) -> BlinkCounterSel1R {
        BlinkCounterSel1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Blink Counter Selection #2"]
    #[inline(always)]
    pub fn blink_counter_sel2(&self) -> BlinkCounterSel2R {
        BlinkCounterSel2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt status register"]
    #[inline(always)]
    pub fn intsts_reg(&self) -> IntstsRegR {
        IntstsRegR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Input data register"]
    #[inline(always)]
    pub fn input_data_reg(&self) -> InputDataRegR {
        InputDataRegR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data register"]
    #[inline(always)]
    pub fn data_reg(&mut self) -> DataRegW<Gpio2a0Spec> {
        DataRegW::new(self, 0)
    }
    #[doc = "Bit 1 - Direction register"]
    #[inline(always)]
    pub fn direction_reg(&mut self) -> DirectionRegW<Gpio2a0Spec> {
        DirectionRegW::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt enable"]
    #[inline(always)]
    pub fn intenbl(&mut self) -> IntenblW<Gpio2a0Spec> {
        IntenblW::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt sensitivity type 0 selection"]
    #[inline(always)]
    pub fn intsensitivity_type0sel(&mut self) -> IntsensitivityType0selW<Gpio2a0Spec> {
        IntsensitivityType0selW::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn intsensitivity_type1sel(&mut self) -> IntsensitivityType1selW<Gpio2a0Spec> {
        IntsensitivityType1selW::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn intsensitivity_type2sel(&mut self) -> IntsensitivityType2selW<Gpio2a0Spec> {
        IntsensitivityType2selW::new(self, 5)
    }
    #[doc = "Bit 6 - Reset tolerance enable"]
    #[inline(always)]
    pub fn rst_tolerance_enbl(&mut self) -> RstToleranceEnblW<Gpio2a0Spec> {
        RstToleranceEnblW::new(self, 6)
    }
    #[doc = "Bit 7 - Debounce setting register #1"]
    #[inline(always)]
    pub fn debounce_setting_reg1(&mut self) -> DebounceSettingReg1W<Gpio2a0Spec> {
        DebounceSettingReg1W::new(self, 7)
    }
    #[doc = "Bit 8 - Debounce setting register #2"]
    #[inline(always)]
    pub fn debounce_setting_reg2(&mut self) -> DebounceSettingReg2W<Gpio2a0Spec> {
        DebounceSettingReg2W::new(self, 8)
    }
    #[doc = "Bit 9 - Input mask"]
    #[inline(always)]
    pub fn input_mask(&mut self) -> InputMaskW<Gpio2a0Spec> {
        InputMaskW::new(self, 9)
    }
    #[doc = "Bit 10 - Blink Counter Selection #1"]
    #[inline(always)]
    pub fn blink_counter_sel1(&mut self) -> BlinkCounterSel1W<Gpio2a0Spec> {
        BlinkCounterSel1W::new(self, 10)
    }
    #[doc = "Bit 11 - Blink Counter Selection #2"]
    #[inline(always)]
    pub fn blink_counter_sel2(&mut self) -> BlinkCounterSel2W<Gpio2a0Spec> {
        BlinkCounterSel2W::new(self, 11)
    }
    #[doc = "Bit 12 - Interrupt status register"]
    #[inline(always)]
    pub fn intsts_reg(&mut self) -> IntstsRegW<Gpio2a0Spec> {
        IntstsRegW::new(self, 12)
    }
}
#[doc = "GPIO072 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio2a0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio2a0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio2a0Spec;
impl crate::RegisterSpec for Gpio2a0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio2a0::R`](R) reader structure"]
impl crate::Readable for Gpio2a0Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio2a0::W`](W) writer structure"]
impl crate::Writable for Gpio2a0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO2A0 to value 0"]
impl crate::Resettable for Gpio2a0Spec {}
