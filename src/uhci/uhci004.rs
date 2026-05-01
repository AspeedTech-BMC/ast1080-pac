#[doc = "Register `UHCI004` reader"]
pub type R = crate::R<Uhci004Spec>;
#[doc = "Register `UHCI004` writer"]
pub type W = crate::W<Uhci004Spec>;
#[doc = "Field `USBINTUSBINTWC` reader - USB Interrupt (USBINT) (WC)"]
pub type UsbintusbintwcR = crate::BitReader;
#[doc = "Field `USBINTUSBINTWC` writer - USB Interrupt (USBINT) (WC)"]
pub type UsbintusbintwcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBErrINTWC` reader - USB Error Interrupt (WC)"]
pub type UsberrIntwcR = crate::BitReader;
#[doc = "Field `USBErrINTWC` writer - USB Error Interrupt (WC)"]
pub type UsberrIntwcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ResumeDetectWC` reader - Resume Detect (WC)"]
pub type ResumeDetectWcR = crate::BitReader;
#[doc = "Field `ResumeDetectWC` writer - Resume Detect (WC)"]
pub type ResumeDetectWcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HostSysErrWC` reader - Host System Error (WC)"]
pub type HostSysErrWcR = crate::BitReader;
#[doc = "Field `HostSysErrWC` writer - Host System Error (WC)"]
pub type HostSysErrWcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HostCtrlProcessErrWC` reader - Host Controller Process Error (WC)"]
pub type HostCtrlProcessErrWcR = crate::BitReader;
#[doc = "Field `HostCtrlProcessErrWC` writer - Host Controller Process Error (WC)"]
pub type HostCtrlProcessErrWcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCHaltedBitWC` reader - HCHalted bit (WC)"]
pub type HchaltedBitWcR = crate::BitReader;
#[doc = "Field `HCHaltedBitWC` writer - HCHalted bit (WC)"]
pub type HchaltedBitWcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u32>;
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
    #[doc = "Bit 2 - Resume Detect (WC)"]
    #[inline(always)]
    pub fn resume_detect_wc(&self) -> ResumeDetectWcR {
        ResumeDetectWcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Host System Error (WC)"]
    #[inline(always)]
    pub fn host_sys_err_wc(&self) -> HostSysErrWcR {
        HostSysErrWcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Host Controller Process Error (WC)"]
    #[inline(always)]
    pub fn host_ctrl_process_err_wc(&self) -> HostCtrlProcessErrWcR {
        HostCtrlProcessErrWcR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HCHalted bit (WC)"]
    #[inline(always)]
    pub fn hchalted_bit_wc(&self) -> HchaltedBitWcR {
        HchaltedBitWcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - USB Interrupt (USBINT) (WC)"]
    #[inline(always)]
    pub fn usbintusbintwc(&mut self) -> UsbintusbintwcW<Uhci004Spec> {
        UsbintusbintwcW::new(self, 0)
    }
    #[doc = "Bit 1 - USB Error Interrupt (WC)"]
    #[inline(always)]
    pub fn usberr_intwc(&mut self) -> UsberrIntwcW<Uhci004Spec> {
        UsberrIntwcW::new(self, 1)
    }
    #[doc = "Bit 2 - Resume Detect (WC)"]
    #[inline(always)]
    pub fn resume_detect_wc(&mut self) -> ResumeDetectWcW<Uhci004Spec> {
        ResumeDetectWcW::new(self, 2)
    }
    #[doc = "Bit 3 - Host System Error (WC)"]
    #[inline(always)]
    pub fn host_sys_err_wc(&mut self) -> HostSysErrWcW<Uhci004Spec> {
        HostSysErrWcW::new(self, 3)
    }
    #[doc = "Bit 4 - Host Controller Process Error (WC)"]
    #[inline(always)]
    pub fn host_ctrl_process_err_wc(&mut self) -> HostCtrlProcessErrWcW<Uhci004Spec> {
        HostCtrlProcessErrWcW::new(self, 4)
    }
    #[doc = "Bit 5 - HCHalted bit (WC)"]
    #[inline(always)]
    pub fn hchalted_bit_wc(&mut self) -> HchaltedBitWcW<Uhci004Spec> {
        HchaltedBitWcW::new(self, 5)
    }
}
#[doc = "USB Status Register (USBSTS)\n\nYou can [`read`](crate::Reg::read) this register and get [`uhci004::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhci004::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uhci004Spec;
impl crate::RegisterSpec for Uhci004Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uhci004::R`](R) reader structure"]
impl crate::Readable for Uhci004Spec {}
#[doc = "`write(|w| ..)` method takes [`uhci004::W`](W) writer structure"]
impl crate::Writable for Uhci004Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UHCI004 to value 0"]
impl crate::Resettable for Uhci004Spec {}
