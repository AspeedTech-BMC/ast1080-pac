#[doc = "Register `EHCI028` reader"]
pub type R = crate::R<Ehci028Spec>;
#[doc = "Register `EHCI028` writer"]
pub type W = crate::W<Ehci028Spec>;
#[doc = "Field `USBINTEnbl` reader - USB Interrupt Enable"]
pub type UsbintenblR = crate::BitReader;
#[doc = "Field `USBINTEnbl` writer - USB Interrupt Enable"]
pub type UsbintenblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBErrINTEnbl` reader - USB Error Interrupt Enable"]
pub type UsberrIntenblR = crate::BitReader;
#[doc = "Field `USBErrINTEnbl` writer - USB Error Interrupt Enable"]
pub type UsberrIntenblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PortChangeINTEnbl` reader - Port Change Interrupt Enable"]
pub type PortChangeIntenblR = crate::BitReader;
#[doc = "Field `PortChangeINTEnbl` writer - Port Change Interrupt Enable"]
pub type PortChangeIntenblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FrameListRolloverEnbl` reader - Frame List Rollover Enable"]
pub type FrameListRolloverEnblR = crate::BitReader;
#[doc = "Field `FrameListRolloverEnbl` writer - Frame List Rollover Enable"]
pub type FrameListRolloverEnblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HostSysErrEnbl` reader - Host System Error Enable"]
pub type HostSysErrEnblR = crate::BitReader;
#[doc = "Field `HostSysErrEnbl` writer - Host System Error Enable"]
pub type HostSysErrEnblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTOnAsyncAdvanceEnbl` reader - Interrupt on Async Advance Enable"]
pub type IntonAsyncAdvanceEnblR = crate::BitReader;
#[doc = "Field `INTOnAsyncAdvanceEnbl` writer - Interrupt on Async Advance Enable"]
pub type IntonAsyncAdvanceEnblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - USB Interrupt Enable"]
    #[inline(always)]
    pub fn usbintenbl(&self) -> UsbintenblR {
        UsbintenblR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB Error Interrupt Enable"]
    #[inline(always)]
    pub fn usberr_intenbl(&self) -> UsberrIntenblR {
        UsberrIntenblR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port Change Interrupt Enable"]
    #[inline(always)]
    pub fn port_change_intenbl(&self) -> PortChangeIntenblR {
        PortChangeIntenblR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Frame List Rollover Enable"]
    #[inline(always)]
    pub fn frame_list_rollover_enbl(&self) -> FrameListRolloverEnblR {
        FrameListRolloverEnblR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Host System Error Enable"]
    #[inline(always)]
    pub fn host_sys_err_enbl(&self) -> HostSysErrEnblR {
        HostSysErrEnblR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt on Async Advance Enable"]
    #[inline(always)]
    pub fn inton_async_advance_enbl(&self) -> IntonAsyncAdvanceEnblR {
        IntonAsyncAdvanceEnblR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - USB Interrupt Enable"]
    #[inline(always)]
    pub fn usbintenbl(&mut self) -> UsbintenblW<Ehci028Spec> {
        UsbintenblW::new(self, 0)
    }
    #[doc = "Bit 1 - USB Error Interrupt Enable"]
    #[inline(always)]
    pub fn usberr_intenbl(&mut self) -> UsberrIntenblW<Ehci028Spec> {
        UsberrIntenblW::new(self, 1)
    }
    #[doc = "Bit 2 - Port Change Interrupt Enable"]
    #[inline(always)]
    pub fn port_change_intenbl(&mut self) -> PortChangeIntenblW<Ehci028Spec> {
        PortChangeIntenblW::new(self, 2)
    }
    #[doc = "Bit 3 - Frame List Rollover Enable"]
    #[inline(always)]
    pub fn frame_list_rollover_enbl(&mut self) -> FrameListRolloverEnblW<Ehci028Spec> {
        FrameListRolloverEnblW::new(self, 3)
    }
    #[doc = "Bit 4 - Host System Error Enable"]
    #[inline(always)]
    pub fn host_sys_err_enbl(&mut self) -> HostSysErrEnblW<Ehci028Spec> {
        HostSysErrEnblW::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt on Async Advance Enable"]
    #[inline(always)]
    pub fn inton_async_advance_enbl(&mut self) -> IntonAsyncAdvanceEnblW<Ehci028Spec> {
        IntonAsyncAdvanceEnblW::new(self, 5)
    }
}
#[doc = "USB Interrupt Enable Register (USBINTR)\n\nYou can [`read`](crate::Reg::read) this register and get [`ehci028::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehci028::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ehci028Spec;
impl crate::RegisterSpec for Ehci028Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ehci028::R`](R) reader structure"]
impl crate::Readable for Ehci028Spec {}
#[doc = "`write(|w| ..)` method takes [`ehci028::W`](W) writer structure"]
impl crate::Writable for Ehci028Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EHCI028 to value 0"]
impl crate::Resettable for Ehci028Spec {}
