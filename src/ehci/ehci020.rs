#[doc = "Register `EHCI020` reader"]
pub type R = crate::R<Ehci020Spec>;
#[doc = "Register `EHCI020` writer"]
pub type W = crate::W<Ehci020Spec>;
#[doc = "Field `RunStopRS` reader - Run/Stop (RS)"]
pub type RunStopRsR = crate::BitReader;
#[doc = "Field `RunStopRS` writer - Run/Stop (RS)"]
pub type RunStopRsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HostCtrlRstHCRESET` reader - Host Controller Reset (HCRESET)"]
pub type HostCtrlRstHcresetR = crate::BitReader;
#[doc = "Field `HostCtrlRstHCRESET` writer - Host Controller Reset (HCRESET)"]
pub type HostCtrlRstHcresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "List Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ListSize {
    #[doc = "0: 1024 elements (4096 bytes) Default value"]
    _1024Elements4096BytesDefaultValue = 0,
    #[doc = "1: 512 elements (2048 bytes)"]
    _512Elements2048Bytes = 1,
    #[doc = "2: 256 elements (1024 bytes) -- for resource-constrained environments"]
    _256Elements1024Bytes_ForResourceconstrainedEnvironments = 2,
}
impl From<ListSize> for u8 {
    #[inline(always)]
    fn from(variant: ListSize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ListSize {
    type Ux = u8;
}
impl crate::IsEnum for ListSize {}
#[doc = "Field `ListSize` reader - List Size"]
pub type ListSizeR = crate::FieldReader<ListSize>;
impl ListSizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ListSize {
        match self.bits {
            0 => ListSize::_1024Elements4096BytesDefaultValue,
            1 => ListSize::_512Elements2048Bytes,
            2 => ListSize::_256Elements1024Bytes_ForResourceconstrainedEnvironments,
            _ => unreachable!(),
        }
    }
    #[doc = "1024 elements (4096 bytes) Default value"]
    #[inline(always)]
    pub fn is_1024_elements_4096_bytes_default_value(&self) -> bool {
        *self == ListSize::_1024Elements4096BytesDefaultValue
    }
    #[doc = "512 elements (2048 bytes)"]
    #[inline(always)]
    pub fn is_512_elements_2048_bytes(&self) -> bool {
        *self == ListSize::_512Elements2048Bytes
    }
    #[doc = "256 elements (1024 bytes) -- for resource-constrained environments"]
    #[inline(always)]
    pub fn is_256_elements_1024_bytes__for_resourceconstrained_environments(&self) -> bool {
        *self == ListSize::_256Elements1024Bytes_ForResourceconstrainedEnvironments
    }
}
#[doc = "Field `ListSize` writer - List Size"]
pub type ListSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, ListSize>;
impl<'a, REG> ListSizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1024 elements (4096 bytes) Default value"]
    #[inline(always)]
    pub fn _1024_elements_4096_bytes_default_value(self) -> &'a mut crate::W<REG> {
        self.variant(ListSize::_1024Elements4096BytesDefaultValue)
    }
    #[doc = "512 elements (2048 bytes)"]
    #[inline(always)]
    pub fn _512_elements_2048_bytes(self) -> &'a mut crate::W<REG> {
        self.variant(ListSize::_512Elements2048Bytes)
    }
    #[doc = "256 elements (1024 bytes) -- for resource-constrained environments"]
    #[inline(always)]
    pub fn _256_elements_1024_bytes__for_resourceconstrained_environments(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(ListSize::_256Elements1024Bytes_ForResourceconstrainedEnvironments)
    }
}
#[doc = "Periodic Schedule Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PeriodicScheduleEnbl {
    #[doc = "0: Do not process the Periodic Schedule"]
    DoNotProcessThePeriodicSchedule = 0,
    #[doc = "1: Use the PERIODICLISTBASE register to access the Periodic Schedule."]
    UseThePeriodiclistbaseRegisterToAccessThePeriodicSchedule = 1,
}
impl From<PeriodicScheduleEnbl> for bool {
    #[inline(always)]
    fn from(variant: PeriodicScheduleEnbl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PeriodicScheduleEnbl` reader - Periodic Schedule Enable"]
pub type PeriodicScheduleEnblR = crate::BitReader<PeriodicScheduleEnbl>;
impl PeriodicScheduleEnblR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PeriodicScheduleEnbl {
        match self.bits {
            false => PeriodicScheduleEnbl::DoNotProcessThePeriodicSchedule,
            true => PeriodicScheduleEnbl::UseThePeriodiclistbaseRegisterToAccessThePeriodicSchedule,
        }
    }
    #[doc = "Do not process the Periodic Schedule"]
    #[inline(always)]
    pub fn is_do_not_process_the_periodic_schedule(&self) -> bool {
        *self == PeriodicScheduleEnbl::DoNotProcessThePeriodicSchedule
    }
    #[doc = "Use the PERIODICLISTBASE register to access the Periodic Schedule."]
    #[inline(always)]
    pub fn is_use_the_periodiclistbase_register_to_access_the_periodic_schedule(&self) -> bool {
        *self == PeriodicScheduleEnbl::UseThePeriodiclistbaseRegisterToAccessThePeriodicSchedule
    }
}
#[doc = "Field `PeriodicScheduleEnbl` writer - Periodic Schedule Enable"]
pub type PeriodicScheduleEnblW<'a, REG> = crate::BitWriter<'a, REG, PeriodicScheduleEnbl>;
impl<'a, REG> PeriodicScheduleEnblW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not process the Periodic Schedule"]
    #[inline(always)]
    pub fn do_not_process_the_periodic_schedule(self) -> &'a mut crate::W<REG> {
        self.variant(PeriodicScheduleEnbl::DoNotProcessThePeriodicSchedule)
    }
    #[doc = "Use the PERIODICLISTBASE register to access the Periodic Schedule."]
    #[inline(always)]
    pub fn use_the_periodiclistbase_register_to_access_the_periodic_schedule(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(
            PeriodicScheduleEnbl::UseThePeriodiclistbaseRegisterToAccessThePeriodicSchedule,
        )
    }
}
#[doc = "Asynchronous Schedule Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AsynchronousScheduleEnbl {
    #[doc = "0: Do not process the Asynchronous Schedule"]
    DoNotProcessTheAsynchronousSchedule = 0,
    #[doc = "1: Use the ASYNCLISTADDR register to access the Asynchronous Schedule."]
    UseTheAsynclistaddrRegisterToAccessTheAsynchronousSchedule = 1,
}
impl From<AsynchronousScheduleEnbl> for bool {
    #[inline(always)]
    fn from(variant: AsynchronousScheduleEnbl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AsynchronousScheduleEnbl` reader - Asynchronous Schedule Enable"]
pub type AsynchronousScheduleEnblR = crate::BitReader<AsynchronousScheduleEnbl>;
impl AsynchronousScheduleEnblR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AsynchronousScheduleEnbl {
        match self.bits {
            false => AsynchronousScheduleEnbl::DoNotProcessTheAsynchronousSchedule,
            true => {
                AsynchronousScheduleEnbl::UseTheAsynclistaddrRegisterToAccessTheAsynchronousSchedule
            }
        }
    }
    #[doc = "Do not process the Asynchronous Schedule"]
    #[inline(always)]
    pub fn is_do_not_process_the_asynchronous_schedule(&self) -> bool {
        *self == AsynchronousScheduleEnbl::DoNotProcessTheAsynchronousSchedule
    }
    #[doc = "Use the ASYNCLISTADDR register to access the Asynchronous Schedule."]
    #[inline(always)]
    pub fn is_use_the_asynclistaddr_register_to_access_the_asynchronous_schedule(&self) -> bool {
        *self
            == AsynchronousScheduleEnbl::UseTheAsynclistaddrRegisterToAccessTheAsynchronousSchedule
    }
}
#[doc = "Field `AsynchronousScheduleEnbl` writer - Asynchronous Schedule Enable"]
pub type AsynchronousScheduleEnblW<'a, REG> = crate::BitWriter<'a, REG, AsynchronousScheduleEnbl>;
impl<'a, REG> AsynchronousScheduleEnblW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not process the Asynchronous Schedule"]
    #[inline(always)]
    pub fn do_not_process_the_asynchronous_schedule(self) -> &'a mut crate::W<REG> {
        self.variant(AsynchronousScheduleEnbl::DoNotProcessTheAsynchronousSchedule)
    }
    #[doc = "Use the ASYNCLISTADDR register to access the Asynchronous Schedule."]
    #[inline(always)]
    pub fn use_the_asynclistaddr_register_to_access_the_asynchronous_schedule(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(
            AsynchronousScheduleEnbl::UseTheAsynclistaddrRegisterToAccessTheAsynchronousSchedule,
        )
    }
}
#[doc = "Field `INTOnAsyncAdvanceDoorbell` reader - Interrupt on Async Advance Doorbell"]
pub type IntonAsyncAdvanceDoorbellR = crate::BitReader;
#[doc = "Field `INTOnAsyncAdvanceDoorbell` writer - Interrupt on Async Advance Doorbell"]
pub type IntonAsyncAdvanceDoorbellW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved03` reader - Reserved (0)"]
pub type Reserved03R = crate::BitReader;
#[doc = "Field `AsynchronousScheduleParkModeCount` reader - Asynchronous Schedule Park Mode Count"]
pub type AsynchronousScheduleParkModeCountR = crate::FieldReader;
#[doc = "Field `AsynchronousScheduleParkModeCount` writer - Asynchronous Schedule Park Mode Count"]
pub type AsynchronousScheduleParkModeCountW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved02` reader - Reserved (0)"]
pub type Reserved02R = crate::BitReader;
#[doc = "Field `AsynchronousScheduleParkModeEnbl` reader - Asynchronous Schedule Park Mode Enable"]
pub type AsynchronousScheduleParkModeEnblR = crate::BitReader;
#[doc = "Field `AsynchronousScheduleParkModeEnbl` writer - Asynchronous Schedule Park Mode Enable"]
pub type AsynchronousScheduleParkModeEnblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved01` reader - Reserved (0)"]
pub type Reserved01R = crate::FieldReader;
#[doc = "Field `INTThresholdCtrl` reader - Interrupt Threshold Control"]
pub type IntthresholdCtrlR = crate::FieldReader;
#[doc = "Field `INTThresholdCtrl` writer - Interrupt Threshold Control"]
pub type IntthresholdCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader;
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
    #[doc = "Bits 2:3 - List Size"]
    #[inline(always)]
    pub fn list_size(&self) -> ListSizeR {
        ListSizeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Periodic Schedule Enable"]
    #[inline(always)]
    pub fn periodic_schedule_enbl(&self) -> PeriodicScheduleEnblR {
        PeriodicScheduleEnblR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Asynchronous Schedule Enable"]
    #[inline(always)]
    pub fn asynchronous_schedule_enbl(&self) -> AsynchronousScheduleEnblR {
        AsynchronousScheduleEnblR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt on Async Advance Doorbell"]
    #[inline(always)]
    pub fn inton_async_advance_doorbell(&self) -> IntonAsyncAdvanceDoorbellR {
        IntonAsyncAdvanceDoorbellR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved03(&self) -> Reserved03R {
        Reserved03R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Asynchronous Schedule Park Mode Count"]
    #[inline(always)]
    pub fn asynchronous_schedule_park_mode_count(&self) -> AsynchronousScheduleParkModeCountR {
        AsynchronousScheduleParkModeCountR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved02(&self) -> Reserved02R {
        Reserved02R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Asynchronous Schedule Park Mode Enable"]
    #[inline(always)]
    pub fn asynchronous_schedule_park_mode_enbl(&self) -> AsynchronousScheduleParkModeEnblR {
        AsynchronousScheduleParkModeEnblR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt Threshold Control"]
    #[inline(always)]
    pub fn intthreshold_ctrl(&self) -> IntthresholdCtrlR {
        IntthresholdCtrlR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Run/Stop (RS)"]
    #[inline(always)]
    pub fn run_stop_rs(&mut self) -> RunStopRsW<Ehci020Spec> {
        RunStopRsW::new(self, 0)
    }
    #[doc = "Bit 1 - Host Controller Reset (HCRESET)"]
    #[inline(always)]
    pub fn host_ctrl_rst_hcreset(&mut self) -> HostCtrlRstHcresetW<Ehci020Spec> {
        HostCtrlRstHcresetW::new(self, 1)
    }
    #[doc = "Bits 2:3 - List Size"]
    #[inline(always)]
    pub fn list_size(&mut self) -> ListSizeW<Ehci020Spec> {
        ListSizeW::new(self, 2)
    }
    #[doc = "Bit 4 - Periodic Schedule Enable"]
    #[inline(always)]
    pub fn periodic_schedule_enbl(&mut self) -> PeriodicScheduleEnblW<Ehci020Spec> {
        PeriodicScheduleEnblW::new(self, 4)
    }
    #[doc = "Bit 5 - Asynchronous Schedule Enable"]
    #[inline(always)]
    pub fn asynchronous_schedule_enbl(&mut self) -> AsynchronousScheduleEnblW<Ehci020Spec> {
        AsynchronousScheduleEnblW::new(self, 5)
    }
    #[doc = "Bit 6 - Interrupt on Async Advance Doorbell"]
    #[inline(always)]
    pub fn inton_async_advance_doorbell(&mut self) -> IntonAsyncAdvanceDoorbellW<Ehci020Spec> {
        IntonAsyncAdvanceDoorbellW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Asynchronous Schedule Park Mode Count"]
    #[inline(always)]
    pub fn asynchronous_schedule_park_mode_count(
        &mut self,
    ) -> AsynchronousScheduleParkModeCountW<Ehci020Spec> {
        AsynchronousScheduleParkModeCountW::new(self, 8)
    }
    #[doc = "Bit 11 - Asynchronous Schedule Park Mode Enable"]
    #[inline(always)]
    pub fn asynchronous_schedule_park_mode_enbl(
        &mut self,
    ) -> AsynchronousScheduleParkModeEnblW<Ehci020Spec> {
        AsynchronousScheduleParkModeEnblW::new(self, 11)
    }
    #[doc = "Bits 16:23 - Interrupt Threshold Control"]
    #[inline(always)]
    pub fn intthreshold_ctrl(&mut self) -> IntthresholdCtrlW<Ehci020Spec> {
        IntthresholdCtrlW::new(self, 16)
    }
}
#[doc = "USB Command Register (USBCMD)\n\nYou can [`read`](crate::Reg::read) this register and get [`ehci020::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehci020::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ehci020Spec;
impl crate::RegisterSpec for Ehci020Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ehci020::R`](R) reader structure"]
impl crate::Readable for Ehci020Spec {}
#[doc = "`write(|w| ..)` method takes [`ehci020::W`](W) writer structure"]
impl crate::Writable for Ehci020Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EHCI020 to value 0x0008_0b00"]
impl crate::Resettable for Ehci020Spec {
    const RESET_VALUE: u32 = 0x0008_0b00;
}
