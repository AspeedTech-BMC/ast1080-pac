#[doc = "Register `UHCI08C` reader"]
pub type R = crate::R<Uhci08cSpec>;
#[doc = "Register `UHCI08C` writer"]
pub type W = crate::W<Uhci08cSpec>;
#[doc = "Current Connect Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CurConnectStatus1 {
    #[doc = "0: No device is present."]
    NoDeviceIsPresent = 0,
    #[doc = "1: Device is present on port."]
    DeviceIsPresentOnPort = 1,
}
impl From<CurConnectStatus1> for bool {
    #[inline(always)]
    fn from(variant: CurConnectStatus1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CurConnectStatus1` reader - Current Connect Status"]
pub type CurConnectStatus1R = crate::BitReader<CurConnectStatus1>;
impl CurConnectStatus1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CurConnectStatus1 {
        match self.bits {
            false => CurConnectStatus1::NoDeviceIsPresent,
            true => CurConnectStatus1::DeviceIsPresentOnPort,
        }
    }
    #[doc = "No device is present."]
    #[inline(always)]
    pub fn is_no_device_is_present(&self) -> bool {
        *self == CurConnectStatus1::NoDeviceIsPresent
    }
    #[doc = "Device is present on port."]
    #[inline(always)]
    pub fn is_device_is_present_on_port(&self) -> bool {
        *self == CurConnectStatus1::DeviceIsPresentOnPort
    }
}
#[doc = "Connect Status Change (WC)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConnectStatusChangeWc1 {
    #[doc = "0: No change."]
    NoChange = 0,
    #[doc = "1: Change in Current Connect Status."]
    ChangeInCurrentConnectStatus = 1,
}
impl From<ConnectStatusChangeWc1> for bool {
    #[inline(always)]
    fn from(variant: ConnectStatusChangeWc1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ConnectStatusChangeWC1` reader - Connect Status Change (WC)"]
pub type ConnectStatusChangeWc1R = crate::BitReader<ConnectStatusChangeWc1>;
impl ConnectStatusChangeWc1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ConnectStatusChangeWc1 {
        match self.bits {
            false => ConnectStatusChangeWc1::NoChange,
            true => ConnectStatusChangeWc1::ChangeInCurrentConnectStatus,
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == ConnectStatusChangeWc1::NoChange
    }
    #[doc = "Change in Current Connect Status."]
    #[inline(always)]
    pub fn is_change_in_current_connect_status(&self) -> bool {
        *self == ConnectStatusChangeWc1::ChangeInCurrentConnectStatus
    }
}
#[doc = "Field `ConnectStatusChangeWC1` writer - Connect Status Change (WC)"]
pub type ConnectStatusChangeWc1W<'a, REG> = crate::BitWriter<'a, REG, ConnectStatusChangeWc1>;
impl<'a, REG> ConnectStatusChangeWc1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut crate::W<REG> {
        self.variant(ConnectStatusChangeWc1::NoChange)
    }
    #[doc = "Change in Current Connect Status."]
    #[inline(always)]
    pub fn change_in_current_connect_status(self) -> &'a mut crate::W<REG> {
        self.variant(ConnectStatusChangeWc1::ChangeInCurrentConnectStatus)
    }
}
#[doc = "Port Enabled/Disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortEnbldDisd1 {
    #[doc = "0: Disable."]
    Disable = 0,
    #[doc = "1: Enable."]
    Enable = 1,
}
impl From<PortEnbldDisd1> for bool {
    #[inline(always)]
    fn from(variant: PortEnbldDisd1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PortEnbldDisd1` reader - Port Enabled/Disabled"]
pub type PortEnbldDisd1R = crate::BitReader<PortEnbldDisd1>;
impl PortEnbldDisd1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PortEnbldDisd1 {
        match self.bits {
            false => PortEnbldDisd1::Disable,
            true => PortEnbldDisd1::Enable,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PortEnbldDisd1::Disable
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PortEnbldDisd1::Enable
    }
}
#[doc = "Field `PortEnbldDisd1` writer - Port Enabled/Disabled"]
pub type PortEnbldDisd1W<'a, REG> = crate::BitWriter<'a, REG, PortEnbldDisd1>;
impl<'a, REG> PortEnbldDisd1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(PortEnbldDisd1::Disable)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(PortEnbldDisd1::Enable)
    }
}
#[doc = "Port Enable/Disable Change (WC)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortEnblDisChangeWc1 {
    #[doc = "0: No change."]
    NoChange = 0,
    #[doc = "1: Port enabled/disabled status has changed."]
    PortEnableddisabledStatusHasChanged = 1,
}
impl From<PortEnblDisChangeWc1> for bool {
    #[inline(always)]
    fn from(variant: PortEnblDisChangeWc1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PortEnblDisChangeWC1` reader - Port Enable/Disable Change (WC)"]
pub type PortEnblDisChangeWc1R = crate::BitReader<PortEnblDisChangeWc1>;
impl PortEnblDisChangeWc1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PortEnblDisChangeWc1 {
        match self.bits {
            false => PortEnblDisChangeWc1::NoChange,
            true => PortEnblDisChangeWc1::PortEnableddisabledStatusHasChanged,
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == PortEnblDisChangeWc1::NoChange
    }
    #[doc = "Port enabled/disabled status has changed."]
    #[inline(always)]
    pub fn is_port_enableddisabled_status_has_changed(&self) -> bool {
        *self == PortEnblDisChangeWc1::PortEnableddisabledStatusHasChanged
    }
}
#[doc = "Field `PortEnblDisChangeWC1` writer - Port Enable/Disable Change (WC)"]
pub type PortEnblDisChangeWc1W<'a, REG> = crate::BitWriter<'a, REG, PortEnblDisChangeWc1>;
impl<'a, REG> PortEnblDisChangeWc1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut crate::W<REG> {
        self.variant(PortEnblDisChangeWc1::NoChange)
    }
    #[doc = "Port enabled/disabled status has changed."]
    #[inline(always)]
    pub fn port_enableddisabled_status_has_changed(self) -> &'a mut crate::W<REG> {
        self.variant(PortEnblDisChangeWc1::PortEnableddisabledStatusHasChanged)
    }
}
#[doc = "Field `LineStatus1` reader - Line Status"]
pub type LineStatus1R = crate::FieldReader;
#[doc = "Resume Detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResumeDetect1 {
    #[doc = "0: No resume (K-state) detected/driven on port."]
    NoResumeKstateDetecteddrivenOnPort = 0,
    #[doc = "1: Resume detected/driven on port."]
    ResumeDetecteddrivenOnPort = 1,
}
impl From<ResumeDetect1> for bool {
    #[inline(always)]
    fn from(variant: ResumeDetect1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ResumeDetect1` reader - Resume Detect"]
pub type ResumeDetect1R = crate::BitReader<ResumeDetect1>;
impl ResumeDetect1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ResumeDetect1 {
        match self.bits {
            false => ResumeDetect1::NoResumeKstateDetecteddrivenOnPort,
            true => ResumeDetect1::ResumeDetecteddrivenOnPort,
        }
    }
    #[doc = "No resume (K-state) detected/driven on port."]
    #[inline(always)]
    pub fn is_no_resume_kstate_detecteddriven_on_port(&self) -> bool {
        *self == ResumeDetect1::NoResumeKstateDetecteddrivenOnPort
    }
    #[doc = "Resume detected/driven on port."]
    #[inline(always)]
    pub fn is_resume_detecteddriven_on_port(&self) -> bool {
        *self == ResumeDetect1::ResumeDetecteddrivenOnPort
    }
}
#[doc = "Field `ResumeDetect1` writer - Resume Detect"]
pub type ResumeDetect1W<'a, REG> = crate::BitWriter<'a, REG, ResumeDetect1>;
impl<'a, REG> ResumeDetect1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No resume (K-state) detected/driven on port."]
    #[inline(always)]
    pub fn no_resume_kstate_detecteddriven_on_port(self) -> &'a mut crate::W<REG> {
        self.variant(ResumeDetect1::NoResumeKstateDetecteddrivenOnPort)
    }
    #[doc = "Resume detected/driven on port."]
    #[inline(always)]
    pub fn resume_detecteddriven_on_port(self) -> &'a mut crate::W<REG> {
        self.variant(ResumeDetect1::ResumeDetecteddrivenOnPort)
    }
}
#[doc = "Field `Reserved11` reader - Reserved (1)"]
pub type Reserved11R = crate::BitReader;
#[doc = "Low Speed Device Attached\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LowSpeedDevAttached1 {
    #[doc = "0: Full speed device."]
    FullSpeedDevice = 0,
    #[doc = "1: Low speed device is attached to this port."]
    LowSpeedDeviceIsAttachedToThisPort = 1,
}
impl From<LowSpeedDevAttached1> for bool {
    #[inline(always)]
    fn from(variant: LowSpeedDevAttached1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LowSpeedDevAttached1` reader - Low Speed Device Attached"]
pub type LowSpeedDevAttached1R = crate::BitReader<LowSpeedDevAttached1>;
impl LowSpeedDevAttached1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LowSpeedDevAttached1 {
        match self.bits {
            false => LowSpeedDevAttached1::FullSpeedDevice,
            true => LowSpeedDevAttached1::LowSpeedDeviceIsAttachedToThisPort,
        }
    }
    #[doc = "Full speed device."]
    #[inline(always)]
    pub fn is_full_speed_device(&self) -> bool {
        *self == LowSpeedDevAttached1::FullSpeedDevice
    }
    #[doc = "Low speed device is attached to this port."]
    #[inline(always)]
    pub fn is_low_speed_device_is_attached_to_this_port(&self) -> bool {
        *self == LowSpeedDevAttached1::LowSpeedDeviceIsAttachedToThisPort
    }
}
#[doc = "Port Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortRst1 {
    #[doc = "0: Port is not in Reset."]
    PortIsNotInReset = 0,
    #[doc = "1: Port is in Reset."]
    PortIsInReset = 1,
}
impl From<PortRst1> for bool {
    #[inline(always)]
    fn from(variant: PortRst1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PortRst1` reader - Port Reset"]
pub type PortRst1R = crate::BitReader<PortRst1>;
impl PortRst1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PortRst1 {
        match self.bits {
            false => PortRst1::PortIsNotInReset,
            true => PortRst1::PortIsInReset,
        }
    }
    #[doc = "Port is not in Reset."]
    #[inline(always)]
    pub fn is_port_is_not_in_reset(&self) -> bool {
        *self == PortRst1::PortIsNotInReset
    }
    #[doc = "Port is in Reset."]
    #[inline(always)]
    pub fn is_port_is_in_reset(&self) -> bool {
        *self == PortRst1::PortIsInReset
    }
}
#[doc = "Field `PortRst1` writer - Port Reset"]
pub type PortRst1W<'a, REG> = crate::BitWriter<'a, REG, PortRst1>;
impl<'a, REG> PortRst1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port is not in Reset."]
    #[inline(always)]
    pub fn port_is_not_in_reset(self) -> &'a mut crate::W<REG> {
        self.variant(PortRst1::PortIsNotInReset)
    }
    #[doc = "Port is in Reset."]
    #[inline(always)]
    pub fn port_is_in_reset(self) -> &'a mut crate::W<REG> {
        self.variant(PortRst1::PortIsInReset)
    }
}
#[doc = "Field `Reserved03` reader - Reserved (0)"]
pub type Reserved03R = crate::FieldReader;
#[doc = "Suspend\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Suspend1 {
    #[doc = "0: Port not in suspend state."]
    PortNotInSuspendState = 0,
    #[doc = "1: Port in suspend state."]
    PortInSuspendState = 1,
}
impl From<Suspend1> for bool {
    #[inline(always)]
    fn from(variant: Suspend1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Suspend1` reader - Suspend"]
pub type Suspend1R = crate::BitReader<Suspend1>;
impl Suspend1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Suspend1 {
        match self.bits {
            false => Suspend1::PortNotInSuspendState,
            true => Suspend1::PortInSuspendState,
        }
    }
    #[doc = "Port not in suspend state."]
    #[inline(always)]
    pub fn is_port_not_in_suspend_state(&self) -> bool {
        *self == Suspend1::PortNotInSuspendState
    }
    #[doc = "Port in suspend state."]
    #[inline(always)]
    pub fn is_port_in_suspend_state(&self) -> bool {
        *self == Suspend1::PortInSuspendState
    }
}
#[doc = "Field `Suspend1` writer - Suspend"]
pub type Suspend1W<'a, REG> = crate::BitWriter<'a, REG, Suspend1>;
impl<'a, REG> Suspend1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port not in suspend state."]
    #[inline(always)]
    pub fn port_not_in_suspend_state(self) -> &'a mut crate::W<REG> {
        self.variant(Suspend1::PortNotInSuspendState)
    }
    #[doc = "Port in suspend state."]
    #[inline(always)]
    pub fn port_in_suspend_state(self) -> &'a mut crate::W<REG> {
        self.variant(Suspend1::PortInSuspendState)
    }
}
#[doc = "Field `Reserved01` reader - Reserved (0)"]
pub type Reserved01R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Current Connect Status"]
    #[inline(always)]
    pub fn cur_connect_status1(&self) -> CurConnectStatus1R {
        CurConnectStatus1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Connect Status Change (WC)"]
    #[inline(always)]
    pub fn connect_status_change_wc1(&self) -> ConnectStatusChangeWc1R {
        ConnectStatusChangeWc1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port Enabled/Disabled"]
    #[inline(always)]
    pub fn port_enbld_disd1(&self) -> PortEnbldDisd1R {
        PortEnbldDisd1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port Enable/Disable Change (WC)"]
    #[inline(always)]
    pub fn port_enbl_dis_change_wc1(&self) -> PortEnblDisChangeWc1R {
        PortEnblDisChangeWc1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Line Status"]
    #[inline(always)]
    pub fn line_status1(&self) -> LineStatus1R {
        LineStatus1R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Resume Detect"]
    #[inline(always)]
    pub fn resume_detect1(&self) -> ResumeDetect1R {
        ResumeDetect1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reserved (1)"]
    #[inline(always)]
    pub fn reserved11(&self) -> Reserved11R {
        Reserved11R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Low Speed Device Attached"]
    #[inline(always)]
    pub fn low_speed_dev_attached1(&self) -> LowSpeedDevAttached1R {
        LowSpeedDevAttached1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port Reset"]
    #[inline(always)]
    pub fn port_rst1(&self) -> PortRst1R {
        PortRst1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved03(&self) -> Reserved03R {
        Reserved03R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Suspend"]
    #[inline(always)]
    pub fn suspend1(&self) -> Suspend1R {
        Suspend1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new((self.bits >> 13) & 0x0007_ffff)
    }
}
impl W {
    #[doc = "Bit 1 - Connect Status Change (WC)"]
    #[inline(always)]
    pub fn connect_status_change_wc1(&mut self) -> ConnectStatusChangeWc1W<Uhci08cSpec> {
        ConnectStatusChangeWc1W::new(self, 1)
    }
    #[doc = "Bit 2 - Port Enabled/Disabled"]
    #[inline(always)]
    pub fn port_enbld_disd1(&mut self) -> PortEnbldDisd1W<Uhci08cSpec> {
        PortEnbldDisd1W::new(self, 2)
    }
    #[doc = "Bit 3 - Port Enable/Disable Change (WC)"]
    #[inline(always)]
    pub fn port_enbl_dis_change_wc1(&mut self) -> PortEnblDisChangeWc1W<Uhci08cSpec> {
        PortEnblDisChangeWc1W::new(self, 3)
    }
    #[doc = "Bit 6 - Resume Detect"]
    #[inline(always)]
    pub fn resume_detect1(&mut self) -> ResumeDetect1W<Uhci08cSpec> {
        ResumeDetect1W::new(self, 6)
    }
    #[doc = "Bit 9 - Port Reset"]
    #[inline(always)]
    pub fn port_rst1(&mut self) -> PortRst1W<Uhci08cSpec> {
        PortRst1W::new(self, 9)
    }
    #[doc = "Bit 12 - Suspend"]
    #[inline(always)]
    pub fn suspend1(&mut self) -> Suspend1W<Uhci08cSpec> {
        Suspend1W::new(self, 12)
    }
}
#[doc = "Port2 Status/Control Register (PORTSC2)\n\nYou can [`read`](crate::Reg::read) this register and get [`uhci08c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhci08c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uhci08cSpec;
impl crate::RegisterSpec for Uhci08cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uhci08c::R`](R) reader structure"]
impl crate::Readable for Uhci08cSpec {}
#[doc = "`write(|w| ..)` method takes [`uhci08c::W`](W) writer structure"]
impl crate::Writable for Uhci08cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UHCI08C to value 0x80"]
impl crate::Resettable for Uhci08cSpec {
    const RESET_VALUE: u32 = 0x80;
}
