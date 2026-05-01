#[doc = "Register `UHCI040` reader"]
pub type R = crate::R<Uhci040Spec>;
#[doc = "Register `UHCI040` writer"]
pub type W = crate::W<Uhci040Spec>;
#[doc = "USB Test Mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UsbtestModeSel {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable Test J"]
    EnableTestJ = 1,
    #[doc = "2: Enable Test K"]
    EnableTestK = 2,
    #[doc = "4: Enable Test Packet"]
    EnableTestPacket = 4,
    #[doc = "5: Enable Test SE0"]
    EnableTestSe0 = 5,
    #[doc = "6: Enable Test SE1"]
    EnableTestSe1 = 6,
    #[doc = "7: Enable Test Loop Back \\regdebug"]
    EnableTestLoopBackRegdebug = 7,
}
impl From<UsbtestModeSel> for u8 {
    #[inline(always)]
    fn from(variant: UsbtestModeSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UsbtestModeSel {
    type Ux = u8;
}
impl crate::IsEnum for UsbtestModeSel {}
#[doc = "Field `USBTestModeSel` reader - USB Test Mode selection"]
pub type UsbtestModeSelR = crate::FieldReader<UsbtestModeSel>;
impl UsbtestModeSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UsbtestModeSel {
        match self.bits {
            0 => UsbtestModeSel::Disable,
            1 => UsbtestModeSel::EnableTestJ,
            2 => UsbtestModeSel::EnableTestK,
            4 => UsbtestModeSel::EnableTestPacket,
            5 => UsbtestModeSel::EnableTestSe0,
            6 => UsbtestModeSel::EnableTestSe1,
            7 => UsbtestModeSel::EnableTestLoopBackRegdebug,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == UsbtestModeSel::Disable
    }
    #[doc = "Enable Test J"]
    #[inline(always)]
    pub fn is_enable_test_j(&self) -> bool {
        *self == UsbtestModeSel::EnableTestJ
    }
    #[doc = "Enable Test K"]
    #[inline(always)]
    pub fn is_enable_test_k(&self) -> bool {
        *self == UsbtestModeSel::EnableTestK
    }
    #[doc = "Enable Test Packet"]
    #[inline(always)]
    pub fn is_enable_test_packet(&self) -> bool {
        *self == UsbtestModeSel::EnableTestPacket
    }
    #[doc = "Enable Test SE0"]
    #[inline(always)]
    pub fn is_enable_test_se0(&self) -> bool {
        *self == UsbtestModeSel::EnableTestSe0
    }
    #[doc = "Enable Test SE1"]
    #[inline(always)]
    pub fn is_enable_test_se1(&self) -> bool {
        *self == UsbtestModeSel::EnableTestSe1
    }
    #[doc = "Enable Test Loop Back \\regdebug"]
    #[inline(always)]
    pub fn is_enable_test_loop_back_regdebug(&self) -> bool {
        *self == UsbtestModeSel::EnableTestLoopBackRegdebug
    }
}
#[doc = "Field `USBTestModeSel` writer - USB Test Mode selection"]
pub type UsbtestModeSelW<'a, REG> = crate::FieldWriter<'a, REG, 3, UsbtestModeSel>;
impl<'a, REG> UsbtestModeSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(UsbtestModeSel::Disable)
    }
    #[doc = "Enable Test J"]
    #[inline(always)]
    pub fn enable_test_j(self) -> &'a mut crate::W<REG> {
        self.variant(UsbtestModeSel::EnableTestJ)
    }
    #[doc = "Enable Test K"]
    #[inline(always)]
    pub fn enable_test_k(self) -> &'a mut crate::W<REG> {
        self.variant(UsbtestModeSel::EnableTestK)
    }
    #[doc = "Enable Test Packet"]
    #[inline(always)]
    pub fn enable_test_packet(self) -> &'a mut crate::W<REG> {
        self.variant(UsbtestModeSel::EnableTestPacket)
    }
    #[doc = "Enable Test SE0"]
    #[inline(always)]
    pub fn enable_test_se0(self) -> &'a mut crate::W<REG> {
        self.variant(UsbtestModeSel::EnableTestSe0)
    }
    #[doc = "Enable Test SE1"]
    #[inline(always)]
    pub fn enable_test_se1(self) -> &'a mut crate::W<REG> {
        self.variant(UsbtestModeSel::EnableTestSe1)
    }
    #[doc = "Enable Test Loop Back \\regdebug"]
    #[inline(always)]
    pub fn enable_test_loop_back_regdebug(self) -> &'a mut crate::W<REG> {
        self.variant(UsbtestModeSel::EnableTestLoopBackRegdebug)
    }
}
#[doc = "Test Mode speed selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TestModeSpeedSel {
    #[doc = "0: Full speed"]
    FullSpeed = 0,
    #[doc = "1: Low speed"]
    LowSpeed = 1,
}
impl From<TestModeSpeedSel> for bool {
    #[inline(always)]
    fn from(variant: TestModeSpeedSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TestModeSpeedSel` reader - Test Mode speed selection"]
pub type TestModeSpeedSelR = crate::BitReader<TestModeSpeedSel>;
impl TestModeSpeedSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TestModeSpeedSel {
        match self.bits {
            false => TestModeSpeedSel::FullSpeed,
            true => TestModeSpeedSel::LowSpeed,
        }
    }
    #[doc = "Full speed"]
    #[inline(always)]
    pub fn is_full_speed(&self) -> bool {
        *self == TestModeSpeedSel::FullSpeed
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn is_low_speed(&self) -> bool {
        *self == TestModeSpeedSel::LowSpeed
    }
}
#[doc = "Field `TestModeSpeedSel` writer - Test Mode speed selection"]
pub type TestModeSpeedSelW<'a, REG> = crate::BitWriter<'a, REG, TestModeSpeedSel>;
impl<'a, REG> TestModeSpeedSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Full speed"]
    #[inline(always)]
    pub fn full_speed(self) -> &'a mut crate::W<REG> {
        self.variant(TestModeSpeedSel::FullSpeed)
    }
    #[doc = "Low speed"]
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut crate::W<REG> {
        self.variant(TestModeSpeedSel::LowSpeed)
    }
}
#[doc = "SOF Test mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SoftestMode {
    #[doc = "0: 1 ms, Normal mode"]
    _1MsNormalMode = 0,
    #[doc = "1: 64 us"]
    _64Us = 1,
    #[doc = "2: 128 us"]
    _128Us = 2,
    #[doc = "3: 256 us"]
    _256Us = 3,
}
impl From<SoftestMode> for u8 {
    #[inline(always)]
    fn from(variant: SoftestMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SoftestMode {
    type Ux = u8;
}
impl crate::IsEnum for SoftestMode {}
#[doc = "Field `SOFTestMode` reader - SOF Test mode"]
pub type SoftestModeR = crate::FieldReader<SoftestMode>;
impl SoftestModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SoftestMode {
        match self.bits {
            0 => SoftestMode::_1MsNormalMode,
            1 => SoftestMode::_64Us,
            2 => SoftestMode::_128Us,
            3 => SoftestMode::_256Us,
            _ => unreachable!(),
        }
    }
    #[doc = "1 ms, Normal mode"]
    #[inline(always)]
    pub fn is_1_ms_normal_mode(&self) -> bool {
        *self == SoftestMode::_1MsNormalMode
    }
    #[doc = "64 us"]
    #[inline(always)]
    pub fn is_64_us(&self) -> bool {
        *self == SoftestMode::_64Us
    }
    #[doc = "128 us"]
    #[inline(always)]
    pub fn is_128_us(&self) -> bool {
        *self == SoftestMode::_128Us
    }
    #[doc = "256 us"]
    #[inline(always)]
    pub fn is_256_us(&self) -> bool {
        *self == SoftestMode::_256Us
    }
}
#[doc = "Field `SOFTestMode` writer - SOF Test mode"]
pub type SoftestModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, SoftestMode, crate::Safe>;
impl<'a, REG> SoftestModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 ms, Normal mode"]
    #[inline(always)]
    pub fn _1_ms_normal_mode(self) -> &'a mut crate::W<REG> {
        self.variant(SoftestMode::_1MsNormalMode)
    }
    #[doc = "64 us"]
    #[inline(always)]
    pub fn _64_us(self) -> &'a mut crate::W<REG> {
        self.variant(SoftestMode::_64Us)
    }
    #[doc = "128 us"]
    #[inline(always)]
    pub fn _128_us(self) -> &'a mut crate::W<REG> {
        self.variant(SoftestMode::_128Us)
    }
    #[doc = "256 us"]
    #[inline(always)]
    pub fn _256_us(self) -> &'a mut crate::W<REG> {
        self.variant(SoftestMode::_256Us)
    }
}
#[doc = "Port LineState sampling mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortLineStateSamplingMode {
    #[doc = "0: Normal, sample at EOF2"]
    NormalSampleAtEof2 = 0,
    #[doc = "1: Real time"]
    RealTime = 1,
}
impl From<PortLineStateSamplingMode> for bool {
    #[inline(always)]
    fn from(variant: PortLineStateSamplingMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PortLineStateSamplingMode` reader - Port LineState sampling mode"]
pub type PortLineStateSamplingModeR = crate::BitReader<PortLineStateSamplingMode>;
impl PortLineStateSamplingModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PortLineStateSamplingMode {
        match self.bits {
            false => PortLineStateSamplingMode::NormalSampleAtEof2,
            true => PortLineStateSamplingMode::RealTime,
        }
    }
    #[doc = "Normal, sample at EOF2"]
    #[inline(always)]
    pub fn is_normal_sample_at_eof2(&self) -> bool {
        *self == PortLineStateSamplingMode::NormalSampleAtEof2
    }
    #[doc = "Real time"]
    #[inline(always)]
    pub fn is_real_time(&self) -> bool {
        *self == PortLineStateSamplingMode::RealTime
    }
}
#[doc = "Field `PortLineStateSamplingMode` writer - Port LineState sampling mode"]
pub type PortLineStateSamplingModeW<'a, REG> = crate::BitWriter<'a, REG, PortLineStateSamplingMode>;
impl<'a, REG> PortLineStateSamplingModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal, sample at EOF2"]
    #[inline(always)]
    pub fn normal_sample_at_eof2(self) -> &'a mut crate::W<REG> {
        self.variant(PortLineStateSamplingMode::NormalSampleAtEof2)
    }
    #[doc = "Real time"]
    #[inline(always)]
    pub fn real_time(self) -> &'a mut crate::W<REG> {
        self.variant(PortLineStateSamplingMode::RealTime)
    }
}
#[doc = "Transmit FIFO threshold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxFifothreshold {
    #[doc = "0: 32 bytes"]
    _32Bytes = 0,
    #[doc = "1: 64 bytes"]
    _64Bytes = 1,
}
impl From<TxFifothreshold> for bool {
    #[inline(always)]
    fn from(variant: TxFifothreshold) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TxFIFOThreshold` reader - Transmit FIFO threshold"]
pub type TxFifothresholdR = crate::BitReader<TxFifothreshold>;
impl TxFifothresholdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TxFifothreshold {
        match self.bits {
            false => TxFifothreshold::_32Bytes,
            true => TxFifothreshold::_64Bytes,
        }
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn is_32_bytes(&self) -> bool {
        *self == TxFifothreshold::_32Bytes
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn is_64_bytes(&self) -> bool {
        *self == TxFifothreshold::_64Bytes
    }
}
#[doc = "Field `TxFIFOThreshold` writer - Transmit FIFO threshold"]
pub type TxFifothresholdW<'a, REG> = crate::BitWriter<'a, REG, TxFifothreshold>;
impl<'a, REG> TxFifothresholdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn _32_bytes(self) -> &'a mut crate::W<REG> {
        self.variant(TxFifothreshold::_32Bytes)
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn _64_bytes(self) -> &'a mut crate::W<REG> {
        self.variant(TxFifothreshold::_64Bytes)
    }
}
#[doc = "Loop Back test status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoopBackTestSts {
    #[doc = "0: Not finish"]
    NotFinish = 0,
    #[doc = "1: Finished"]
    Finished = 1,
}
impl From<LoopBackTestSts> for bool {
    #[inline(always)]
    fn from(variant: LoopBackTestSts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LoopBackTestSts` reader - Loop Back test status"]
pub type LoopBackTestStsR = crate::BitReader<LoopBackTestSts>;
impl LoopBackTestStsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LoopBackTestSts {
        match self.bits {
            false => LoopBackTestSts::NotFinish,
            true => LoopBackTestSts::Finished,
        }
    }
    #[doc = "Not finish"]
    #[inline(always)]
    pub fn is_not_finish(&self) -> bool {
        *self == LoopBackTestSts::NotFinish
    }
    #[doc = "Finished"]
    #[inline(always)]
    pub fn is_finished(&self) -> bool {
        *self == LoopBackTestSts::Finished
    }
}
#[doc = "Loop Back test result\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoopBackTestResult {
    #[doc = "0: Fail"]
    Fail = 0,
    #[doc = "1: Pass"]
    Pass = 1,
}
impl From<LoopBackTestResult> for bool {
    #[inline(always)]
    fn from(variant: LoopBackTestResult) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LoopBackTestResult` reader - Loop Back test result"]
pub type LoopBackTestResultR = crate::BitReader<LoopBackTestResult>;
impl LoopBackTestResultR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LoopBackTestResult {
        match self.bits {
            false => LoopBackTestResult::Fail,
            true => LoopBackTestResult::Pass,
        }
    }
    #[doc = "Fail"]
    #[inline(always)]
    pub fn is_fail(&self) -> bool {
        *self == LoopBackTestResult::Fail
    }
    #[doc = "Pass"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        *self == LoopBackTestResult::Pass
    }
}
#[doc = "Field `Reserved01` reader - Reserved (0)"]
pub type Reserved01R = crate::FieldReader;
#[doc = "Delay List operation 8 clock cycles\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DelayListOp8clkCycles {
    #[doc = "0: No delay"]
    NoDelay = 0,
    #[doc = "1: Delay 8 cycles"]
    Delay8Cycles = 1,
}
impl From<DelayListOp8clkCycles> for bool {
    #[inline(always)]
    fn from(variant: DelayListOp8clkCycles) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DelayListOp8ClkCycles` reader - Delay List operation 8 clock cycles"]
pub type DelayListOp8clkCyclesR = crate::BitReader<DelayListOp8clkCycles>;
impl DelayListOp8clkCyclesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DelayListOp8clkCycles {
        match self.bits {
            false => DelayListOp8clkCycles::NoDelay,
            true => DelayListOp8clkCycles::Delay8Cycles,
        }
    }
    #[doc = "No delay"]
    #[inline(always)]
    pub fn is_no_delay(&self) -> bool {
        *self == DelayListOp8clkCycles::NoDelay
    }
    #[doc = "Delay 8 cycles"]
    #[inline(always)]
    pub fn is_delay_8_cycles(&self) -> bool {
        *self == DelayListOp8clkCycles::Delay8Cycles
    }
}
#[doc = "Field `DelayListOp8ClkCycles` writer - Delay List operation 8 clock cycles"]
pub type DelayListOp8clkCyclesW<'a, REG> = crate::BitWriter<'a, REG, DelayListOp8clkCycles>;
impl<'a, REG> DelayListOp8clkCyclesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No delay"]
    #[inline(always)]
    pub fn no_delay(self) -> &'a mut crate::W<REG> {
        self.variant(DelayListOp8clkCycles::NoDelay)
    }
    #[doc = "Delay 8 cycles"]
    #[inline(always)]
    pub fn delay_8_cycles(self) -> &'a mut crate::W<REG> {
        self.variant(DelayListOp8clkCycles::Delay8Cycles)
    }
}
#[doc = "Enable FIFO Auto Power Down\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblFifoautoPowerDown {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<EnblFifoautoPowerDown> for bool {
    #[inline(always)]
    fn from(variant: EnblFifoautoPowerDown) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblFIFOAutoPowerDown` reader - Enable FIFO Auto Power Down"]
pub type EnblFifoautoPowerDownR = crate::BitReader<EnblFifoautoPowerDown>;
impl EnblFifoautoPowerDownR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblFifoautoPowerDown {
        match self.bits {
            false => EnblFifoautoPowerDown::Disable,
            true => EnblFifoautoPowerDown::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EnblFifoautoPowerDown::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EnblFifoautoPowerDown::Enable
    }
}
#[doc = "Field `EnblFIFOAutoPowerDown` writer - Enable FIFO Auto Power Down"]
pub type EnblFifoautoPowerDownW<'a, REG> = crate::BitWriter<'a, REG, EnblFifoautoPowerDown>;
impl<'a, REG> EnblFifoautoPowerDownW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(EnblFifoautoPowerDown::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(EnblFifoautoPowerDown::Enable)
    }
}
#[doc = "Field `AHBBusCtrlSel` reader - AHB Bus control selection"]
pub type AhbbusCtrlSelR = crate::BitReader;
#[doc = "Field `AHBBusCtrlSel` writer - AHB Bus control selection"]
pub type AhbbusCtrlSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:2 - USB Test Mode selection"]
    #[inline(always)]
    pub fn usbtest_mode_sel(&self) -> UsbtestModeSelR {
        UsbtestModeSelR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Test Mode speed selection"]
    #[inline(always)]
    pub fn test_mode_speed_sel(&self) -> TestModeSpeedSelR {
        TestModeSpeedSelR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - SOF Test mode"]
    #[inline(always)]
    pub fn softest_mode(&self) -> SoftestModeR {
        SoftestModeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Port LineState sampling mode"]
    #[inline(always)]
    pub fn port_line_state_sampling_mode(&self) -> PortLineStateSamplingModeR {
        PortLineStateSamplingModeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit FIFO threshold"]
    #[inline(always)]
    pub fn tx_fifothreshold(&self) -> TxFifothresholdR {
        TxFifothresholdR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Loop Back test status"]
    #[inline(always)]
    pub fn loop_back_test_sts(&self) -> LoopBackTestStsR {
        LoopBackTestStsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Loop Back test result"]
    #[inline(always)]
    pub fn loop_back_test_result(&self) -> LoopBackTestResultR {
        LoopBackTestResultR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Delay List operation 8 clock cycles"]
    #[inline(always)]
    pub fn delay_list_op8clk_cycles(&self) -> DelayListOp8clkCyclesR {
        DelayListOp8clkCyclesR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable FIFO Auto Power Down"]
    #[inline(always)]
    pub fn enbl_fifoauto_power_down(&self) -> EnblFifoautoPowerDownR {
        EnblFifoautoPowerDownR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - AHB Bus control selection"]
    #[inline(always)]
    pub fn ahbbus_ctrl_sel(&self) -> AhbbusCtrlSelR {
        AhbbusCtrlSelR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 15) & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - USB Test Mode selection"]
    #[inline(always)]
    pub fn usbtest_mode_sel(&mut self) -> UsbtestModeSelW<Uhci040Spec> {
        UsbtestModeSelW::new(self, 0)
    }
    #[doc = "Bit 3 - Test Mode speed selection"]
    #[inline(always)]
    pub fn test_mode_speed_sel(&mut self) -> TestModeSpeedSelW<Uhci040Spec> {
        TestModeSpeedSelW::new(self, 3)
    }
    #[doc = "Bits 4:5 - SOF Test mode"]
    #[inline(always)]
    pub fn softest_mode(&mut self) -> SoftestModeW<Uhci040Spec> {
        SoftestModeW::new(self, 4)
    }
    #[doc = "Bit 6 - Port LineState sampling mode"]
    #[inline(always)]
    pub fn port_line_state_sampling_mode(&mut self) -> PortLineStateSamplingModeW<Uhci040Spec> {
        PortLineStateSamplingModeW::new(self, 6)
    }
    #[doc = "Bit 7 - Transmit FIFO threshold"]
    #[inline(always)]
    pub fn tx_fifothreshold(&mut self) -> TxFifothresholdW<Uhci040Spec> {
        TxFifothresholdW::new(self, 7)
    }
    #[doc = "Bit 12 - Delay List operation 8 clock cycles"]
    #[inline(always)]
    pub fn delay_list_op8clk_cycles(&mut self) -> DelayListOp8clkCyclesW<Uhci040Spec> {
        DelayListOp8clkCyclesW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable FIFO Auto Power Down"]
    #[inline(always)]
    pub fn enbl_fifoauto_power_down(&mut self) -> EnblFifoautoPowerDownW<Uhci040Spec> {
        EnblFifoautoPowerDownW::new(self, 13)
    }
    #[doc = "Bit 14 - AHB Bus control selection"]
    #[inline(always)]
    pub fn ahbbus_ctrl_sel(&mut self) -> AhbbusCtrlSelW<Uhci040Spec> {
        AhbbusCtrlSelW::new(self, 14)
    }
}
#[doc = "Test Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uhci040::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhci040::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uhci040Spec;
impl crate::RegisterSpec for Uhci040Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uhci040::R`](R) reader structure"]
impl crate::Readable for Uhci040Spec {}
#[doc = "`write(|w| ..)` method takes [`uhci040::W`](W) writer structure"]
impl crate::Writable for Uhci040Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UHCI040 to value 0"]
impl crate::Resettable for Uhci040Spec {}
