#[doc = "Register `HUB08` reader"]
pub type R = crate::R<Hub08Spec>;
#[doc = "Register `HUB08` writer"]
pub type W = crate::W<Hub08Spec>;
#[doc = "Field `EnblHubEP0SETUPDataPktACKINT` reader - Enable Hub EP0 SETUP Data packet ACK Interrupt"]
pub type EnblHubEp0setupdataPktAckintR = crate::BitReader;
#[doc = "Field `EnblHubEP0SETUPDataPktACKINT` writer - Enable Hub EP0 SETUP Data packet ACK Interrupt"]
pub type EnblHubEp0setupdataPktAckintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblHubEP0OUTDataPktACKSTALLINT` reader - Enable Hub EP0 OUT Data packet ACK/STALL Interrupt"]
pub type EnblHubEp0outdataPktAckstallintR = crate::BitReader;
#[doc = "Field `EnblHubEP0OUTDataPktACKSTALLINT` writer - Enable Hub EP0 OUT Data packet ACK/STALL Interrupt"]
pub type EnblHubEp0outdataPktAckstallintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblHubEP0OUTDataPktNAKINT` reader - Enable Hub EP0 OUT Data packet NAK Interrupt"]
pub type EnblHubEp0outdataPktNakintR = crate::BitReader;
#[doc = "Field `EnblHubEP0OUTDataPktNAKINT` writer - Enable Hub EP0 OUT Data packet NAK Interrupt"]
pub type EnblHubEp0outdataPktNakintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblHubEP0INDataPktACKSTALLINT` reader - Enable Hub EP0 IN Data packet ACK/STALL Interrupt"]
pub type EnblHubEp0indataPktAckstallintR = crate::BitReader;
#[doc = "Field `EnblHubEP0INDataPktACKSTALLINT` writer - Enable Hub EP0 IN Data packet ACK/STALL Interrupt"]
pub type EnblHubEp0indataPktAckstallintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblHubEP0INDataPktNAKINT` reader - Enable Hub EP0 IN Data packet NAK Interrupt"]
pub type EnblHubEp0indataPktNakintR = crate::BitReader;
#[doc = "Field `EnblHubEP0INDataPktNAKINT` writer - Enable Hub EP0 IN Data packet NAK Interrupt"]
pub type EnblHubEp0indataPktNakintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblHubEP1INDataPktACKINT` reader - Enable Hub EP1 IN Data packet ACK Interrupt"]
pub type EnblHubEp1indataPktAckintR = crate::BitReader;
#[doc = "Field `EnblHubEP1INDataPktACKINT` writer - Enable Hub EP1 IN Data packet ACK Interrupt"]
pub type EnblHubEp1indataPktAckintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblUSBBusRstINT` reader - Enable USB Bus Reset Interrupt"]
pub type EnblUsbbusRstIntR = crate::BitReader;
#[doc = "Field `EnblUSBBusRstINT` writer - Enable USB Bus Reset Interrupt"]
pub type EnblUsbbusRstIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblUSBSuspendEntryINT` reader - Enable USB Suspend Entry Interrupt"]
pub type EnblUsbsuspendEntryIntR = crate::BitReader;
#[doc = "Field `EnblUSBSuspendEntryINT` writer - Enable USB Suspend Entry Interrupt"]
pub type EnblUsbsuspendEntryIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblUSBSuspendResumeINT` reader - Enable USB Suspend Resume Interrupt"]
pub type EnblUsbsuspendResumeIntR = crate::BitReader;
#[doc = "Field `EnblUSBSuspendResumeINT` writer - Enable USB Suspend Resume Interrupt"]
pub type EnblUsbsuspendResumeIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblDev1CtrlINT` reader - Enable Device #1 Controller Interrupt"]
pub type EnblDev1ctrlIntR = crate::BitReader;
#[doc = "Field `EnblDev1CtrlINT` writer - Enable Device #1 Controller Interrupt"]
pub type EnblDev1ctrlIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblDev2CtrlINT` reader - Enable Device #2 Controller Interrupt"]
pub type EnblDev2ctrlIntR = crate::BitReader;
#[doc = "Field `EnblDev2CtrlINT` writer - Enable Device #2 Controller Interrupt"]
pub type EnblDev2ctrlIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblDev3CtrlINT` reader - Enable Device #3 Controller Interrupt"]
pub type EnblDev3ctrlIntR = crate::BitReader;
#[doc = "Field `EnblDev3CtrlINT` writer - Enable Device #3 Controller Interrupt"]
pub type EnblDev3ctrlIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblDev4CtrlINT` reader - Enable Device #4 Controller Interrupt"]
pub type EnblDev4ctrlIntR = crate::BitReader;
#[doc = "Field `EnblDev4CtrlINT` writer - Enable Device #4 Controller Interrupt"]
pub type EnblDev4ctrlIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblDev5CtrlINT` reader - Enable Device #5 Controller Interrupt"]
pub type EnblDev5ctrlIntR = crate::BitReader;
#[doc = "Field `EnblDev5CtrlINT` writer - Enable Device #5 Controller Interrupt"]
pub type EnblDev5ctrlIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblDev6CtrlINT` reader - Enable Device #6 Controller Interrupt"]
pub type EnblDev6ctrlIntR = crate::BitReader;
#[doc = "Field `EnblDev6CtrlINT` writer - Enable Device #6 Controller Interrupt"]
pub type EnblDev6ctrlIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblDev7CtrlINT` reader - Enable Device #7 Controller Interrupt"]
pub type EnblDev7ctrlIntR = crate::BitReader;
#[doc = "Field `EnblDev7CtrlINT` writer - Enable Device #7 Controller Interrupt"]
pub type EnblDev7ctrlIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblProgrammableEndpointPoolACKSTALLINT` reader - Enable Programmable Endpoint Pool ACK/STALL Interrupt"]
pub type EnblProgrammableEndpointPoolAckstallintR = crate::BitReader;
#[doc = "Field `EnblProgrammableEndpointPoolACKSTALLINT` writer - Enable Programmable Endpoint Pool ACK/STALL Interrupt"]
pub type EnblProgrammableEndpointPoolAckstallintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblProgrammableEndpointPoolNAKINT` reader - Enable Programmable Endpoint Pool NAK Interrupt"]
pub type EnblProgrammableEndpointPoolNakintR = crate::BitReader;
#[doc = "Field `EnblProgrammableEndpointPoolNAKINT` writer - Enable Programmable Endpoint Pool NAK Interrupt"]
pub type EnblProgrammableEndpointPoolNakintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Enable Hub EP0 SETUP Data packet ACK Interrupt"]
    #[inline(always)]
    pub fn enbl_hub_ep0setupdata_pkt_ackint(&self) -> EnblHubEp0setupdataPktAckintR {
        EnblHubEp0setupdataPktAckintR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Hub EP0 OUT Data packet ACK/STALL Interrupt"]
    #[inline(always)]
    pub fn enbl_hub_ep0outdata_pkt_ackstallint(&self) -> EnblHubEp0outdataPktAckstallintR {
        EnblHubEp0outdataPktAckstallintR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Hub EP0 OUT Data packet NAK Interrupt"]
    #[inline(always)]
    pub fn enbl_hub_ep0outdata_pkt_nakint(&self) -> EnblHubEp0outdataPktNakintR {
        EnblHubEp0outdataPktNakintR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Hub EP0 IN Data packet ACK/STALL Interrupt"]
    #[inline(always)]
    pub fn enbl_hub_ep0indata_pkt_ackstallint(&self) -> EnblHubEp0indataPktAckstallintR {
        EnblHubEp0indataPktAckstallintR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Hub EP0 IN Data packet NAK Interrupt"]
    #[inline(always)]
    pub fn enbl_hub_ep0indata_pkt_nakint(&self) -> EnblHubEp0indataPktNakintR {
        EnblHubEp0indataPktNakintR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Hub EP1 IN Data packet ACK Interrupt"]
    #[inline(always)]
    pub fn enbl_hub_ep1indata_pkt_ackint(&self) -> EnblHubEp1indataPktAckintR {
        EnblHubEp1indataPktAckintR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable USB Bus Reset Interrupt"]
    #[inline(always)]
    pub fn enbl_usbbus_rst_int(&self) -> EnblUsbbusRstIntR {
        EnblUsbbusRstIntR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable USB Suspend Entry Interrupt"]
    #[inline(always)]
    pub fn enbl_usbsuspend_entry_int(&self) -> EnblUsbsuspendEntryIntR {
        EnblUsbsuspendEntryIntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable USB Suspend Resume Interrupt"]
    #[inline(always)]
    pub fn enbl_usbsuspend_resume_int(&self) -> EnblUsbsuspendResumeIntR {
        EnblUsbsuspendResumeIntR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Device #1 Controller Interrupt"]
    #[inline(always)]
    pub fn enbl_dev1ctrl_int(&self) -> EnblDev1ctrlIntR {
        EnblDev1ctrlIntR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Device #2 Controller Interrupt"]
    #[inline(always)]
    pub fn enbl_dev2ctrl_int(&self) -> EnblDev2ctrlIntR {
        EnblDev2ctrlIntR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Device #3 Controller Interrupt"]
    #[inline(always)]
    pub fn enbl_dev3ctrl_int(&self) -> EnblDev3ctrlIntR {
        EnblDev3ctrlIntR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Device #4 Controller Interrupt"]
    #[inline(always)]
    pub fn enbl_dev4ctrl_int(&self) -> EnblDev4ctrlIntR {
        EnblDev4ctrlIntR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Device #5 Controller Interrupt"]
    #[inline(always)]
    pub fn enbl_dev5ctrl_int(&self) -> EnblDev5ctrlIntR {
        EnblDev5ctrlIntR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Device #6 Controller Interrupt"]
    #[inline(always)]
    pub fn enbl_dev6ctrl_int(&self) -> EnblDev6ctrlIntR {
        EnblDev6ctrlIntR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Device #7 Controller Interrupt"]
    #[inline(always)]
    pub fn enbl_dev7ctrl_int(&self) -> EnblDev7ctrlIntR {
        EnblDev7ctrlIntR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Programmable Endpoint Pool ACK/STALL Interrupt"]
    #[inline(always)]
    pub fn enbl_programmable_endpoint_pool_ackstallint(
        &self,
    ) -> EnblProgrammableEndpointPoolAckstallintR {
        EnblProgrammableEndpointPoolAckstallintR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Programmable Endpoint Pool NAK Interrupt"]
    #[inline(always)]
    pub fn enbl_programmable_endpoint_pool_nakint(&self) -> EnblProgrammableEndpointPoolNakintR {
        EnblProgrammableEndpointPoolNakintR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Hub EP0 SETUP Data packet ACK Interrupt"]
    #[inline(always)]
    pub fn enbl_hub_ep0setupdata_pkt_ackint(&mut self) -> EnblHubEp0setupdataPktAckintW<Hub08Spec> {
        EnblHubEp0setupdataPktAckintW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Hub EP0 OUT Data packet ACK/STALL Interrupt"]
    #[inline(always)]
    pub fn enbl_hub_ep0outdata_pkt_ackstallint(
        &mut self,
    ) -> EnblHubEp0outdataPktAckstallintW<Hub08Spec> {
        EnblHubEp0outdataPktAckstallintW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Hub EP0 OUT Data packet NAK Interrupt"]
    #[inline(always)]
    pub fn enbl_hub_ep0outdata_pkt_nakint(&mut self) -> EnblHubEp0outdataPktNakintW<Hub08Spec> {
        EnblHubEp0outdataPktNakintW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Hub EP0 IN Data packet ACK/STALL Interrupt"]
    #[inline(always)]
    pub fn enbl_hub_ep0indata_pkt_ackstallint(
        &mut self,
    ) -> EnblHubEp0indataPktAckstallintW<Hub08Spec> {
        EnblHubEp0indataPktAckstallintW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Hub EP0 IN Data packet NAK Interrupt"]
    #[inline(always)]
    pub fn enbl_hub_ep0indata_pkt_nakint(&mut self) -> EnblHubEp0indataPktNakintW<Hub08Spec> {
        EnblHubEp0indataPktNakintW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Hub EP1 IN Data packet ACK Interrupt"]
    #[inline(always)]
    pub fn enbl_hub_ep1indata_pkt_ackint(&mut self) -> EnblHubEp1indataPktAckintW<Hub08Spec> {
        EnblHubEp1indataPktAckintW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable USB Bus Reset Interrupt"]
    #[inline(always)]
    pub fn enbl_usbbus_rst_int(&mut self) -> EnblUsbbusRstIntW<Hub08Spec> {
        EnblUsbbusRstIntW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable USB Suspend Entry Interrupt"]
    #[inline(always)]
    pub fn enbl_usbsuspend_entry_int(&mut self) -> EnblUsbsuspendEntryIntW<Hub08Spec> {
        EnblUsbsuspendEntryIntW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable USB Suspend Resume Interrupt"]
    #[inline(always)]
    pub fn enbl_usbsuspend_resume_int(&mut self) -> EnblUsbsuspendResumeIntW<Hub08Spec> {
        EnblUsbsuspendResumeIntW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Device #1 Controller Interrupt"]
    #[inline(always)]
    pub fn enbl_dev1ctrl_int(&mut self) -> EnblDev1ctrlIntW<Hub08Spec> {
        EnblDev1ctrlIntW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Device #2 Controller Interrupt"]
    #[inline(always)]
    pub fn enbl_dev2ctrl_int(&mut self) -> EnblDev2ctrlIntW<Hub08Spec> {
        EnblDev2ctrlIntW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Device #3 Controller Interrupt"]
    #[inline(always)]
    pub fn enbl_dev3ctrl_int(&mut self) -> EnblDev3ctrlIntW<Hub08Spec> {
        EnblDev3ctrlIntW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Device #4 Controller Interrupt"]
    #[inline(always)]
    pub fn enbl_dev4ctrl_int(&mut self) -> EnblDev4ctrlIntW<Hub08Spec> {
        EnblDev4ctrlIntW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Device #5 Controller Interrupt"]
    #[inline(always)]
    pub fn enbl_dev5ctrl_int(&mut self) -> EnblDev5ctrlIntW<Hub08Spec> {
        EnblDev5ctrlIntW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Device #6 Controller Interrupt"]
    #[inline(always)]
    pub fn enbl_dev6ctrl_int(&mut self) -> EnblDev6ctrlIntW<Hub08Spec> {
        EnblDev6ctrlIntW::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Device #7 Controller Interrupt"]
    #[inline(always)]
    pub fn enbl_dev7ctrl_int(&mut self) -> EnblDev7ctrlIntW<Hub08Spec> {
        EnblDev7ctrlIntW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Programmable Endpoint Pool ACK/STALL Interrupt"]
    #[inline(always)]
    pub fn enbl_programmable_endpoint_pool_ackstallint(
        &mut self,
    ) -> EnblProgrammableEndpointPoolAckstallintW<Hub08Spec> {
        EnblProgrammableEndpointPoolAckstallintW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Programmable Endpoint Pool NAK Interrupt"]
    #[inline(always)]
    pub fn enbl_programmable_endpoint_pool_nakint(
        &mut self,
    ) -> EnblProgrammableEndpointPoolNakintW<Hub08Spec> {
        EnblProgrammableEndpointPoolNakintW::new(self, 17)
    }
}
#[doc = "Interrupt Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hub08::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hub08::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hub08Spec;
impl crate::RegisterSpec for Hub08Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hub08::R`](R) reader structure"]
impl crate::Readable for Hub08Spec {}
#[doc = "`write(|w| ..)` method takes [`hub08::W`](W) writer structure"]
impl crate::Writable for Hub08Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HUB08 to value 0"]
impl crate::Resettable for Hub08Spec {}
