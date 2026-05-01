#[doc = "Register `EHCI064` reader"]
pub type R = crate::R<Ehci064Spec>;
#[doc = "Register `EHCI064` writer"]
pub type W = crate::W<Ehci064Spec>;
#[doc = "Current Connect Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CurConnectStatus {
    #[doc = "0: No device is present."]
    NoDeviceIsPresent = 0,
    #[doc = "1: Device is present on port."]
    DeviceIsPresentOnPort = 1,
}
impl From<CurConnectStatus> for bool {
    #[inline(always)]
    fn from(variant: CurConnectStatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CurConnectStatus` reader - Current Connect Status"]
pub type CurConnectStatusR = crate::BitReader<CurConnectStatus>;
impl CurConnectStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CurConnectStatus {
        match self.bits {
            false => CurConnectStatus::NoDeviceIsPresent,
            true => CurConnectStatus::DeviceIsPresentOnPort,
        }
    }
    #[doc = "No device is present."]
    #[inline(always)]
    pub fn is_no_device_is_present(&self) -> bool {
        *self == CurConnectStatus::NoDeviceIsPresent
    }
    #[doc = "Device is present on port."]
    #[inline(always)]
    pub fn is_device_is_present_on_port(&self) -> bool {
        *self == CurConnectStatus::DeviceIsPresentOnPort
    }
}
#[doc = "Connect Status Change (WC)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConnectStatusChangeWc {
    #[doc = "0: No change."]
    NoChange = 0,
    #[doc = "1: Change in Current Connect Status."]
    ChangeInCurrentConnectStatus = 1,
}
impl From<ConnectStatusChangeWc> for bool {
    #[inline(always)]
    fn from(variant: ConnectStatusChangeWc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ConnectStatusChangeWC` reader - Connect Status Change (WC)"]
pub type ConnectStatusChangeWcR = crate::BitReader<ConnectStatusChangeWc>;
impl ConnectStatusChangeWcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ConnectStatusChangeWc {
        match self.bits {
            false => ConnectStatusChangeWc::NoChange,
            true => ConnectStatusChangeWc::ChangeInCurrentConnectStatus,
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == ConnectStatusChangeWc::NoChange
    }
    #[doc = "Change in Current Connect Status."]
    #[inline(always)]
    pub fn is_change_in_current_connect_status(&self) -> bool {
        *self == ConnectStatusChangeWc::ChangeInCurrentConnectStatus
    }
}
#[doc = "Field `ConnectStatusChangeWC` writer - Connect Status Change (WC)"]
pub type ConnectStatusChangeWcW<'a, REG> = crate::BitWriter<'a, REG, ConnectStatusChangeWc>;
impl<'a, REG> ConnectStatusChangeWcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut crate::W<REG> {
        self.variant(ConnectStatusChangeWc::NoChange)
    }
    #[doc = "Change in Current Connect Status."]
    #[inline(always)]
    pub fn change_in_current_connect_status(self) -> &'a mut crate::W<REG> {
        self.variant(ConnectStatusChangeWc::ChangeInCurrentConnectStatus)
    }
}
#[doc = "Port Enabled/Disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortEnbldDisd {
    #[doc = "0: Disable."]
    Disable = 0,
    #[doc = "1: Enable."]
    Enable = 1,
}
impl From<PortEnbldDisd> for bool {
    #[inline(always)]
    fn from(variant: PortEnbldDisd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PortEnbldDisd` reader - Port Enabled/Disabled"]
pub type PortEnbldDisdR = crate::BitReader<PortEnbldDisd>;
impl PortEnbldDisdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PortEnbldDisd {
        match self.bits {
            false => PortEnbldDisd::Disable,
            true => PortEnbldDisd::Enable,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PortEnbldDisd::Disable
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PortEnbldDisd::Enable
    }
}
#[doc = "Field `PortEnbldDisd` writer - Port Enabled/Disabled"]
pub type PortEnbldDisdW<'a, REG> = crate::BitWriter<'a, REG, PortEnbldDisd>;
impl<'a, REG> PortEnbldDisdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(PortEnbldDisd::Disable)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(PortEnbldDisd::Enable)
    }
}
#[doc = "Port Enable/Disable Change (WC)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortEnblDisChangeWc {
    #[doc = "0: No change."]
    NoChange = 0,
    #[doc = "1: Port enabled/disabled status has changed."]
    PortEnableddisabledStatusHasChanged = 1,
}
impl From<PortEnblDisChangeWc> for bool {
    #[inline(always)]
    fn from(variant: PortEnblDisChangeWc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PortEnblDisChangeWC` reader - Port Enable/Disable Change (WC)"]
pub type PortEnblDisChangeWcR = crate::BitReader<PortEnblDisChangeWc>;
impl PortEnblDisChangeWcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PortEnblDisChangeWc {
        match self.bits {
            false => PortEnblDisChangeWc::NoChange,
            true => PortEnblDisChangeWc::PortEnableddisabledStatusHasChanged,
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn is_no_change(&self) -> bool {
        *self == PortEnblDisChangeWc::NoChange
    }
    #[doc = "Port enabled/disabled status has changed."]
    #[inline(always)]
    pub fn is_port_enableddisabled_status_has_changed(&self) -> bool {
        *self == PortEnblDisChangeWc::PortEnableddisabledStatusHasChanged
    }
}
#[doc = "Field `PortEnblDisChangeWC` writer - Port Enable/Disable Change (WC)"]
pub type PortEnblDisChangeWcW<'a, REG> = crate::BitWriter<'a, REG, PortEnblDisChangeWc>;
impl<'a, REG> PortEnblDisChangeWcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No change."]
    #[inline(always)]
    pub fn no_change(self) -> &'a mut crate::W<REG> {
        self.variant(PortEnblDisChangeWc::NoChange)
    }
    #[doc = "Port enabled/disabled status has changed."]
    #[inline(always)]
    pub fn port_enableddisabled_status_has_changed(self) -> &'a mut crate::W<REG> {
        self.variant(PortEnblDisChangeWc::PortEnableddisabledStatusHasChanged)
    }
}
#[doc = "Field `Reserved03` reader - Reserved (0)"]
pub type Reserved03R = crate::FieldReader;
#[doc = "Force Port Resume\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ForcePortResume {
    #[doc = "0: No resume (K-state) detected/driven on port."]
    NoResumeKstateDetecteddrivenOnPort = 0,
    #[doc = "1: Resume detected/driven on port."]
    ResumeDetecteddrivenOnPort = 1,
}
impl From<ForcePortResume> for bool {
    #[inline(always)]
    fn from(variant: ForcePortResume) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ForcePortResume` reader - Force Port Resume"]
pub type ForcePortResumeR = crate::BitReader<ForcePortResume>;
impl ForcePortResumeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ForcePortResume {
        match self.bits {
            false => ForcePortResume::NoResumeKstateDetecteddrivenOnPort,
            true => ForcePortResume::ResumeDetecteddrivenOnPort,
        }
    }
    #[doc = "No resume (K-state) detected/driven on port."]
    #[inline(always)]
    pub fn is_no_resume_kstate_detecteddriven_on_port(&self) -> bool {
        *self == ForcePortResume::NoResumeKstateDetecteddrivenOnPort
    }
    #[doc = "Resume detected/driven on port."]
    #[inline(always)]
    pub fn is_resume_detecteddriven_on_port(&self) -> bool {
        *self == ForcePortResume::ResumeDetecteddrivenOnPort
    }
}
#[doc = "Field `ForcePortResume` writer - Force Port Resume"]
pub type ForcePortResumeW<'a, REG> = crate::BitWriter<'a, REG, ForcePortResume>;
impl<'a, REG> ForcePortResumeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No resume (K-state) detected/driven on port."]
    #[inline(always)]
    pub fn no_resume_kstate_detecteddriven_on_port(self) -> &'a mut crate::W<REG> {
        self.variant(ForcePortResume::NoResumeKstateDetecteddrivenOnPort)
    }
    #[doc = "Resume detected/driven on port."]
    #[inline(always)]
    pub fn resume_detecteddriven_on_port(self) -> &'a mut crate::W<REG> {
        self.variant(ForcePortResume::ResumeDetecteddrivenOnPort)
    }
}
#[doc = "Suspend\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Suspend {
    #[doc = "0: Port not in suspend state."]
    PortNotInSuspendState = 0,
    #[doc = "1: Port in suspend state."]
    PortInSuspendState = 1,
}
impl From<Suspend> for bool {
    #[inline(always)]
    fn from(variant: Suspend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Suspend` reader - Suspend"]
pub type SuspendR = crate::BitReader<Suspend>;
impl SuspendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Suspend {
        match self.bits {
            false => Suspend::PortNotInSuspendState,
            true => Suspend::PortInSuspendState,
        }
    }
    #[doc = "Port not in suspend state."]
    #[inline(always)]
    pub fn is_port_not_in_suspend_state(&self) -> bool {
        *self == Suspend::PortNotInSuspendState
    }
    #[doc = "Port in suspend state."]
    #[inline(always)]
    pub fn is_port_in_suspend_state(&self) -> bool {
        *self == Suspend::PortInSuspendState
    }
}
#[doc = "Field `Suspend` writer - Suspend"]
pub type SuspendW<'a, REG> = crate::BitWriter<'a, REG, Suspend>;
impl<'a, REG> SuspendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port not in suspend state."]
    #[inline(always)]
    pub fn port_not_in_suspend_state(self) -> &'a mut crate::W<REG> {
        self.variant(Suspend::PortNotInSuspendState)
    }
    #[doc = "Port in suspend state."]
    #[inline(always)]
    pub fn port_in_suspend_state(self) -> &'a mut crate::W<REG> {
        self.variant(Suspend::PortInSuspendState)
    }
}
#[doc = "Port Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortRst {
    #[doc = "0: Port is not in Reset."]
    PortIsNotInReset = 0,
    #[doc = "1: Port is in Reset."]
    PortIsInReset = 1,
}
impl From<PortRst> for bool {
    #[inline(always)]
    fn from(variant: PortRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PortRst` reader - Port Reset"]
pub type PortRstR = crate::BitReader<PortRst>;
impl PortRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PortRst {
        match self.bits {
            false => PortRst::PortIsNotInReset,
            true => PortRst::PortIsInReset,
        }
    }
    #[doc = "Port is not in Reset."]
    #[inline(always)]
    pub fn is_port_is_not_in_reset(&self) -> bool {
        *self == PortRst::PortIsNotInReset
    }
    #[doc = "Port is in Reset."]
    #[inline(always)]
    pub fn is_port_is_in_reset(&self) -> bool {
        *self == PortRst::PortIsInReset
    }
}
#[doc = "Field `PortRst` writer - Port Reset"]
pub type PortRstW<'a, REG> = crate::BitWriter<'a, REG, PortRst>;
impl<'a, REG> PortRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port is not in Reset."]
    #[inline(always)]
    pub fn port_is_not_in_reset(self) -> &'a mut crate::W<REG> {
        self.variant(PortRst::PortIsNotInReset)
    }
    #[doc = "Port is in Reset."]
    #[inline(always)]
    pub fn port_is_in_reset(self) -> &'a mut crate::W<REG> {
        self.variant(PortRst::PortIsInReset)
    }
}
#[doc = "Field `Reserved02` reader - Reserved (0)"]
pub type Reserved02R = crate::BitReader;
#[doc = "Line Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LineStatus {
    #[doc = "3: 10] USB State Interpretation \\\\"]
    _10_UsbState_Interpretation_ = 3,
}
impl From<LineStatus> for u8 {
    #[inline(always)]
    fn from(variant: LineStatus) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LineStatus {
    type Ux = u8;
}
impl crate::IsEnum for LineStatus {}
#[doc = "Field `LineStatus` reader - Line Status"]
pub type LineStatusR = crate::FieldReader<LineStatus>;
impl LineStatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LineStatus> {
        match self.bits {
            3 => Some(LineStatus::_10_UsbState_Interpretation_),
            _ => None,
        }
    }
    #[doc = "10] USB State Interpretation \\\\"]
    #[inline(always)]
    pub fn is_10__usb_state__interpretation______________________________(&self) -> bool {
        *self == LineStatus::_10_UsbState_Interpretation_
    }
}
#[doc = "Field `Reserved1` reader - Reserved (1)"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `PortOwner` reader - Port Owner"]
pub type PortOwnerR = crate::BitReader;
#[doc = "Field `PortOwner` writer - Port Owner"]
pub type PortOwnerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved01` reader - Reserved (0)"]
pub type Reserved01R = crate::FieldReader;
#[doc = "Field `WakeOnConnectEnblWKCNNTE` reader - Wake on Connect Enable (WKCNNT_E)"]
pub type WakeOnConnectEnblWkcnnteR = crate::BitReader;
#[doc = "Field `WakeOnConnectEnblWKCNNTE` writer - Wake on Connect Enable (WKCNNT_E)"]
pub type WakeOnConnectEnblWkcnnteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WakeOnDisconnectEnblWKDSCNNTE` reader - Wake on Disconnect Enable (WKDSCNNT_E)"]
pub type WakeOnDisconnectEnblWkdscnnteR = crate::BitReader;
#[doc = "Field `WakeOnDisconnectEnblWKDSCNNTE` writer - Wake on Disconnect Enable (WKDSCNNT_E)"]
pub type WakeOnDisconnectEnblWkdscnnteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Current Connect Status"]
    #[inline(always)]
    pub fn cur_connect_status(&self) -> CurConnectStatusR {
        CurConnectStatusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Connect Status Change (WC)"]
    #[inline(always)]
    pub fn connect_status_change_wc(&self) -> ConnectStatusChangeWcR {
        ConnectStatusChangeWcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port Enabled/Disabled"]
    #[inline(always)]
    pub fn port_enbld_disd(&self) -> PortEnbldDisdR {
        PortEnbldDisdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port Enable/Disable Change (WC)"]
    #[inline(always)]
    pub fn port_enbl_dis_change_wc(&self) -> PortEnblDisChangeWcR {
        PortEnblDisChangeWcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved03(&self) -> Reserved03R {
        Reserved03R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Force Port Resume"]
    #[inline(always)]
    pub fn force_port_resume(&self) -> ForcePortResumeR {
        ForcePortResumeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Suspend"]
    #[inline(always)]
    pub fn suspend(&self) -> SuspendR {
        SuspendR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port Reset"]
    #[inline(always)]
    pub fn port_rst(&self) -> PortRstR {
        PortRstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved02(&self) -> Reserved02R {
        Reserved02R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Line Status"]
    #[inline(always)]
    pub fn line_status(&self) -> LineStatusR {
        LineStatusR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Reserved (1)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port Owner"]
    #[inline(always)]
    pub fn port_owner(&self) -> PortOwnerR {
        PortOwnerR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:19 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new(((self.bits >> 14) & 0x3f) as u8)
    }
    #[doc = "Bit 20 - Wake on Connect Enable (WKCNNT_E)"]
    #[inline(always)]
    pub fn wake_on_connect_enbl_wkcnnte(&self) -> WakeOnConnectEnblWkcnnteR {
        WakeOnConnectEnblWkcnnteR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Wake on Disconnect Enable (WKDSCNNT_E)"]
    #[inline(always)]
    pub fn wake_on_disconnect_enbl_wkdscnnte(&self) -> WakeOnDisconnectEnblWkdscnnteR {
        WakeOnDisconnectEnblWkdscnnteR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 1 - Connect Status Change (WC)"]
    #[inline(always)]
    pub fn connect_status_change_wc(&mut self) -> ConnectStatusChangeWcW<Ehci064Spec> {
        ConnectStatusChangeWcW::new(self, 1)
    }
    #[doc = "Bit 2 - Port Enabled/Disabled"]
    #[inline(always)]
    pub fn port_enbld_disd(&mut self) -> PortEnbldDisdW<Ehci064Spec> {
        PortEnbldDisdW::new(self, 2)
    }
    #[doc = "Bit 3 - Port Enable/Disable Change (WC)"]
    #[inline(always)]
    pub fn port_enbl_dis_change_wc(&mut self) -> PortEnblDisChangeWcW<Ehci064Spec> {
        PortEnblDisChangeWcW::new(self, 3)
    }
    #[doc = "Bit 6 - Force Port Resume"]
    #[inline(always)]
    pub fn force_port_resume(&mut self) -> ForcePortResumeW<Ehci064Spec> {
        ForcePortResumeW::new(self, 6)
    }
    #[doc = "Bit 7 - Suspend"]
    #[inline(always)]
    pub fn suspend(&mut self) -> SuspendW<Ehci064Spec> {
        SuspendW::new(self, 7)
    }
    #[doc = "Bit 8 - Port Reset"]
    #[inline(always)]
    pub fn port_rst(&mut self) -> PortRstW<Ehci064Spec> {
        PortRstW::new(self, 8)
    }
    #[doc = "Bit 13 - Port Owner"]
    #[inline(always)]
    pub fn port_owner(&mut self) -> PortOwnerW<Ehci064Spec> {
        PortOwnerW::new(self, 13)
    }
    #[doc = "Bit 20 - Wake on Connect Enable (WKCNNT_E)"]
    #[inline(always)]
    pub fn wake_on_connect_enbl_wkcnnte(&mut self) -> WakeOnConnectEnblWkcnnteW<Ehci064Spec> {
        WakeOnConnectEnblWkcnnteW::new(self, 20)
    }
    #[doc = "Bit 21 - Wake on Disconnect Enable (WKDSCNNT_E)"]
    #[inline(always)]
    pub fn wake_on_disconnect_enbl_wkdscnnte(
        &mut self,
    ) -> WakeOnDisconnectEnblWkdscnnteW<Ehci064Spec> {
        WakeOnDisconnectEnblWkdscnnteW::new(self, 21)
    }
}
#[doc = "Port1 Status/Control Register (PORTSC1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ehci064::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehci064::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ehci064Spec;
impl crate::RegisterSpec for Ehci064Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ehci064::R`](R) reader structure"]
impl crate::Readable for Ehci064Spec {}
#[doc = "`write(|w| ..)` method takes [`ehci064::W`](W) writer structure"]
impl crate::Writable for Ehci064Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EHCI064 to value 0x3000"]
impl crate::Resettable for Ehci064Spec {
    const RESET_VALUE: u32 = 0x3000;
}
