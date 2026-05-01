#[doc = "Register `EHCI024` reader"]
pub type R = crate::R<Ehci024Spec>;
#[doc = "Register `EHCI024` writer"]
pub type W = crate::W<Ehci024Spec>;
#[doc = "Field `USBINTUSBINTWC` reader - USB Interrupt (USBINT) (WC)"]
pub type UsbintusbintwcR = crate::BitReader;
#[doc = "Field `USBINTUSBINTWC` writer - USB Interrupt (USBINT) (WC)"]
pub type UsbintusbintwcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBErrINTWC` reader - USB Error Interrupt (WC)"]
pub type UsberrIntwcR = crate::BitReader;
#[doc = "Field `USBErrINTWC` writer - USB Error Interrupt (WC)"]
pub type UsberrIntwcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PortChangeDetectWC` reader - Port Change Detect (WC)"]
pub type PortChangeDetectWcR = crate::BitReader;
#[doc = "Field `PortChangeDetectWC` writer - Port Change Detect (WC)"]
pub type PortChangeDetectWcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FrameListRolloverWC` reader - Frame List Rollover (WC)"]
pub type FrameListRolloverWcR = crate::BitReader;
#[doc = "Field `FrameListRolloverWC` writer - Frame List Rollover (WC)"]
pub type FrameListRolloverWcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HostSysErrWC` reader - Host System Error (WC)"]
pub type HostSysErrWcR = crate::BitReader;
#[doc = "Field `HostSysErrWC` writer - Host System Error (WC)"]
pub type HostSysErrWcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTOnAsyncAdvanceWC` reader - Interrupt on Async Advance (WC)"]
pub type IntonAsyncAdvanceWcR = crate::BitReader;
#[doc = "Field `INTOnAsyncAdvanceWC` writer - Interrupt on Async Advance (WC)"]
pub type IntonAsyncAdvanceWcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved01` reader - Reserved (0)"]
pub type Reserved01R = crate::FieldReader;
#[doc = "Field `HCHaltedBit` reader - HCHalted bit"]
pub type HchaltedBitR = crate::BitReader;
#[doc = "Field `Reclamation` reader - Reclamation"]
pub type ReclamationR = crate::BitReader;
#[doc = "Field `PeriodicScheduleStatus` reader - Periodic Schedule Status"]
pub type PeriodicScheduleStatusR = crate::BitReader;
#[doc = "Field `AsynchronousScheduleStatus` reader - Asynchronous Schedule Status"]
pub type AsynchronousScheduleStatusR = crate::BitReader;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - USB Interrupt (USBINT) (WC)"]
    #[inline(always)]
    pub fn usbintusbintwc(&self) -> UsbintusbintwcR {
        UsbintusbintwcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB Error Interrupt (WC)"]
    #[inline(always)]
    pub fn usberr_intwc(&self) -> UsberrIntwcR {
        UsberrIntwcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port Change Detect (WC)"]
    #[inline(always)]
    pub fn port_change_detect_wc(&self) -> PortChangeDetectWcR {
        PortChangeDetectWcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Frame List Rollover (WC)"]
    #[inline(always)]
    pub fn frame_list_rollover_wc(&self) -> FrameListRolloverWcR {
        FrameListRolloverWcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Host System Error (WC)"]
    #[inline(always)]
    pub fn host_sys_err_wc(&self) -> HostSysErrWcR {
        HostSysErrWcR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt on Async Advance (WC)"]
    #[inline(always)]
    pub fn inton_async_advance_wc(&self) -> IntonAsyncAdvanceWcR {
        IntonAsyncAdvanceWcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:11 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bit 12 - HCHalted bit"]
    #[inline(always)]
    pub fn hchalted_bit(&self) -> HchaltedBitR {
        HchaltedBitR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Reclamation"]
    #[inline(always)]
    pub fn reclamation(&self) -> ReclamationR {
        ReclamationR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Periodic Schedule Status"]
    #[inline(always)]
    pub fn periodic_schedule_status(&self) -> PeriodicScheduleStatusR {
        PeriodicScheduleStatusR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Asynchronous Schedule Status"]
    #[inline(always)]
    pub fn asynchronous_schedule_status(&self) -> AsynchronousScheduleStatusR {
        AsynchronousScheduleStatusR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - USB Interrupt (USBINT) (WC)"]
    #[inline(always)]
    pub fn usbintusbintwc(&mut self) -> UsbintusbintwcW<Ehci024Spec> {
        UsbintusbintwcW::new(self, 0)
    }
    #[doc = "Bit 1 - USB Error Interrupt (WC)"]
    #[inline(always)]
    pub fn usberr_intwc(&mut self) -> UsberrIntwcW<Ehci024Spec> {
        UsberrIntwcW::new(self, 1)
    }
    #[doc = "Bit 2 - Port Change Detect (WC)"]
    #[inline(always)]
    pub fn port_change_detect_wc(&mut self) -> PortChangeDetectWcW<Ehci024Spec> {
        PortChangeDetectWcW::new(self, 2)
    }
    #[doc = "Bit 3 - Frame List Rollover (WC)"]
    #[inline(always)]
    pub fn frame_list_rollover_wc(&mut self) -> FrameListRolloverWcW<Ehci024Spec> {
        FrameListRolloverWcW::new(self, 3)
    }
    #[doc = "Bit 4 - Host System Error (WC)"]
    #[inline(always)]
    pub fn host_sys_err_wc(&mut self) -> HostSysErrWcW<Ehci024Spec> {
        HostSysErrWcW::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt on Async Advance (WC)"]
    #[inline(always)]
    pub fn inton_async_advance_wc(&mut self) -> IntonAsyncAdvanceWcW<Ehci024Spec> {
        IntonAsyncAdvanceWcW::new(self, 5)
    }
}
#[doc = "USB Status Register (USBSTS)\n\nYou can [`read`](crate::Reg::read) this register and get [`ehci024::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehci024::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ehci024Spec;
impl crate::RegisterSpec for Ehci024Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ehci024::R`](R) reader structure"]
impl crate::Readable for Ehci024Spec {}
#[doc = "`write(|w| ..)` method takes [`ehci024::W`](W) writer structure"]
impl crate::Writable for Ehci024Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EHCI024 to value 0x1000"]
impl crate::Resettable for Ehci024Spec {
    const RESET_VALUE: u32 = 0x1000;
}
