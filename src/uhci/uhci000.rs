#[doc = "Register `UHCI000` reader"]
pub type R = crate::R<Uhci000Spec>;
#[doc = "Register `UHCI000` writer"]
pub type W = crate::W<Uhci000Spec>;
#[doc = "Run/Stop (RS)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunStopRs {
    #[doc = "0: Stop."]
    Stop = 0,
    #[doc = "1: Run."]
    Run = 1,
}
impl From<RunStopRs> for bool {
    #[inline(always)]
    fn from(variant: RunStopRs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RunStopRS` reader - Run/Stop (RS)"]
pub type RunStopRsR = crate::BitReader<RunStopRs>;
impl RunStopRsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RunStopRs {
        match self.bits {
            false => RunStopRs::Stop,
            true => RunStopRs::Run,
        }
    }
    #[doc = "Stop."]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == RunStopRs::Stop
    }
    #[doc = "Run."]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == RunStopRs::Run
    }
}
#[doc = "Field `RunStopRS` writer - Run/Stop (RS)"]
pub type RunStopRsW<'a, REG> = crate::BitWriter<'a, REG, RunStopRs>;
impl<'a, REG> RunStopRsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stop."]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(RunStopRs::Stop)
    }
    #[doc = "Run."]
    #[inline(always)]
    pub fn run(self) -> &'a mut crate::W<REG> {
        self.variant(RunStopRs::Run)
    }
}
#[doc = "Field `HostCtrlRstHCRESET` reader - Host Controller Reset (HCRESET)"]
pub type HostCtrlRstHcresetR = crate::BitReader;
#[doc = "Field `HostCtrlRstHCRESET` writer - Host Controller Reset (HCRESET)"]
pub type HostCtrlRstHcresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GlobalRstGRESET` reader - Global Reset (GRESET)"]
pub type GlobalRstGresetR = crate::BitReader;
#[doc = "Field `GlobalRstGRESET` writer - Global Reset (GRESET)"]
pub type GlobalRstGresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnterGlobalSuspendModeEGSM` reader - Enter Global Suspend Mode (EGSM)"]
pub type EnterGlobalSuspendModeEgsmR = crate::BitReader;
#[doc = "Field `EnterGlobalSuspendModeEGSM` writer - Enter Global Suspend Mode (EGSM)"]
pub type EnterGlobalSuspendModeEgsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ForceGlobalResumeFGR` reader - Force Global Resume (FGR)"]
pub type ForceGlobalResumeFgrR = crate::BitReader;
#[doc = "Field `ForceGlobalResumeFGR` writer - Force Global Resume (FGR)"]
pub type ForceGlobalResumeFgrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Software Debug (SWDBG)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwDebugSwdbg {
    #[doc = "0: Normal Mode."]
    NormalMode = 0,
    #[doc = "1: Debug mode."]
    DebugMode = 1,
}
impl From<SwDebugSwdbg> for bool {
    #[inline(always)]
    fn from(variant: SwDebugSwdbg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SwDebugSWDBG` reader - Software Debug (SWDBG)"]
pub type SwDebugSwdbgR = crate::BitReader<SwDebugSwdbg>;
impl SwDebugSwdbgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SwDebugSwdbg {
        match self.bits {
            false => SwDebugSwdbg::NormalMode,
            true => SwDebugSwdbg::DebugMode,
        }
    }
    #[doc = "Normal Mode."]
    #[inline(always)]
    pub fn is_normal_mode(&self) -> bool {
        *self == SwDebugSwdbg::NormalMode
    }
    #[doc = "Debug mode."]
    #[inline(always)]
    pub fn is_debug_mode(&self) -> bool {
        *self == SwDebugSwdbg::DebugMode
    }
}
#[doc = "Field `SwDebugSWDBG` writer - Software Debug (SWDBG)"]
pub type SwDebugSwdbgW<'a, REG> = crate::BitWriter<'a, REG, SwDebugSwdbg>;
impl<'a, REG> SwDebugSwdbgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal Mode."]
    #[inline(always)]
    pub fn normal_mode(self) -> &'a mut crate::W<REG> {
        self.variant(SwDebugSwdbg::NormalMode)
    }
    #[doc = "Debug mode."]
    #[inline(always)]
    pub fn debug_mode(self) -> &'a mut crate::W<REG> {
        self.variant(SwDebugSwdbg::DebugMode)
    }
}
#[doc = "Field `ConfigureFlagCF` reader - Configure Flag (CF)"]
pub type ConfigureFlagCfR = crate::BitReader;
#[doc = "Field `ConfigureFlagCF` writer - Configure Flag (CF)"]
pub type ConfigureFlagCfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Max Packet (MAXP)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MaxPktMaxp {
    #[doc = "0: 32 bytes."]
    _32Bytes = 0,
    #[doc = "1: 64 bytes."]
    _64Bytes = 1,
}
impl From<MaxPktMaxp> for bool {
    #[inline(always)]
    fn from(variant: MaxPktMaxp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MaxPktMAXP` reader - Max Packet (MAXP)"]
pub type MaxPktMaxpR = crate::BitReader<MaxPktMaxp>;
impl MaxPktMaxpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MaxPktMaxp {
        match self.bits {
            false => MaxPktMaxp::_32Bytes,
            true => MaxPktMaxp::_64Bytes,
        }
    }
    #[doc = "32 bytes."]
    #[inline(always)]
    pub fn is_32_bytes(&self) -> bool {
        *self == MaxPktMaxp::_32Bytes
    }
    #[doc = "64 bytes."]
    #[inline(always)]
    pub fn is_64_bytes(&self) -> bool {
        *self == MaxPktMaxp::_64Bytes
    }
}
#[doc = "Field `MaxPktMAXP` writer - Max Packet (MAXP)"]
pub type MaxPktMaxpW<'a, REG> = crate::BitWriter<'a, REG, MaxPktMaxp>;
impl<'a, REG> MaxPktMaxpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "32 bytes."]
    #[inline(always)]
    pub fn _32_bytes(self) -> &'a mut crate::W<REG> {
        self.variant(MaxPktMaxp::_32Bytes)
    }
    #[doc = "64 bytes."]
    #[inline(always)]
    pub fn _64_bytes(self) -> &'a mut crate::W<REG> {
        self.variant(MaxPktMaxp::_64Bytes)
    }
}
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Run/Stop (RS)"]
    #[inline(always)]
    pub fn run_stop_rs(&self) -> RunStopRsR {
        RunStopRsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Host Controller Reset (HCRESET)"]
    #[inline(always)]
    pub fn host_ctrl_rst_hcreset(&self) -> HostCtrlRstHcresetR {
        HostCtrlRstHcresetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Global Reset (GRESET)"]
    #[inline(always)]
    pub fn global_rst_greset(&self) -> GlobalRstGresetR {
        GlobalRstGresetR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enter Global Suspend Mode (EGSM)"]
    #[inline(always)]
    pub fn enter_global_suspend_mode_egsm(&self) -> EnterGlobalSuspendModeEgsmR {
        EnterGlobalSuspendModeEgsmR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Force Global Resume (FGR)"]
    #[inline(always)]
    pub fn force_global_resume_fgr(&self) -> ForceGlobalResumeFgrR {
        ForceGlobalResumeFgrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Software Debug (SWDBG)"]
    #[inline(always)]
    pub fn sw_debug_swdbg(&self) -> SwDebugSwdbgR {
        SwDebugSwdbgR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configure Flag (CF)"]
    #[inline(always)]
    pub fn configure_flag_cf(&self) -> ConfigureFlagCfR {
        ConfigureFlagCfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Max Packet (MAXP)"]
    #[inline(always)]
    pub fn max_pkt_maxp(&self) -> MaxPktMaxpR {
        MaxPktMaxpR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Run/Stop (RS)"]
    #[inline(always)]
    pub fn run_stop_rs(&mut self) -> RunStopRsW<Uhci000Spec> {
        RunStopRsW::new(self, 0)
    }
    #[doc = "Bit 1 - Host Controller Reset (HCRESET)"]
    #[inline(always)]
    pub fn host_ctrl_rst_hcreset(&mut self) -> HostCtrlRstHcresetW<Uhci000Spec> {
        HostCtrlRstHcresetW::new(self, 1)
    }
    #[doc = "Bit 2 - Global Reset (GRESET)"]
    #[inline(always)]
    pub fn global_rst_greset(&mut self) -> GlobalRstGresetW<Uhci000Spec> {
        GlobalRstGresetW::new(self, 2)
    }
    #[doc = "Bit 3 - Enter Global Suspend Mode (EGSM)"]
    #[inline(always)]
    pub fn enter_global_suspend_mode_egsm(&mut self) -> EnterGlobalSuspendModeEgsmW<Uhci000Spec> {
        EnterGlobalSuspendModeEgsmW::new(self, 3)
    }
    #[doc = "Bit 4 - Force Global Resume (FGR)"]
    #[inline(always)]
    pub fn force_global_resume_fgr(&mut self) -> ForceGlobalResumeFgrW<Uhci000Spec> {
        ForceGlobalResumeFgrW::new(self, 4)
    }
    #[doc = "Bit 5 - Software Debug (SWDBG)"]
    #[inline(always)]
    pub fn sw_debug_swdbg(&mut self) -> SwDebugSwdbgW<Uhci000Spec> {
        SwDebugSwdbgW::new(self, 5)
    }
    #[doc = "Bit 6 - Configure Flag (CF)"]
    #[inline(always)]
    pub fn configure_flag_cf(&mut self) -> ConfigureFlagCfW<Uhci000Spec> {
        ConfigureFlagCfW::new(self, 6)
    }
    #[doc = "Bit 7 - Max Packet (MAXP)"]
    #[inline(always)]
    pub fn max_pkt_maxp(&mut self) -> MaxPktMaxpW<Uhci000Spec> {
        MaxPktMaxpW::new(self, 7)
    }
}
#[doc = "USB Command Register (USBCMD)\n\nYou can [`read`](crate::Reg::read) this register and get [`uhci000::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhci000::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uhci000Spec;
impl crate::RegisterSpec for Uhci000Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uhci000::R`](R) reader structure"]
impl crate::Readable for Uhci000Spec {}
#[doc = "`write(|w| ..)` method takes [`uhci000::W`](W) writer structure"]
impl crate::Writable for Uhci000Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UHCI000 to value 0"]
impl crate::Resettable for Uhci000Spec {}
