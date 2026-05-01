#[doc = "Register `HUB30` reader"]
pub type R = crate::R<Hub30Spec>;
#[doc = "Register `HUB30` writer"]
pub type W = crate::W<Hub30Spec>;
#[doc = "Field `Endpoint0STALLCtrl` reader - Endpoint 0 STALL control"]
pub type Endpoint0stallctrlR = crate::BitReader;
#[doc = "Field `Endpoint0STALLCtrl` writer - Endpoint 0 STALL control"]
pub type Endpoint0stallctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Endpoint0INBufReadyForXferringData` reader - Endpoint 0 IN buffer ready for transferring data"]
pub type Endpoint0inbufReadyForXferringDataR = crate::BitReader;
#[doc = "Field `Endpoint0INBufReadyForXferringData` writer - Endpoint 0 IN buffer ready for transferring data"]
pub type Endpoint0inbufReadyForXferringDataW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Endpoint0OUTBufReadyForReceivingData` reader - Endpoint 0 OUT buffer ready for receiving data"]
pub type Endpoint0outbufReadyForReceivingDataR = crate::BitReader;
#[doc = "Field `Endpoint0OUTBufReadyForReceivingData` writer - Endpoint 0 OUT buffer ready for receiving data"]
pub type Endpoint0outbufReadyForReceivingDataW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved02` reader - Reserved (0)"]
pub type Reserved02R = crate::FieldReader;
#[doc = "Field `Endpoint0INDataByteCountForXfer` reader - Endpoint 0 IN data byte count for transfer"]
pub type Endpoint0indataByteCountForXferR = crate::FieldReader;
#[doc = "Field `Endpoint0INDataByteCountForXfer` writer - Endpoint 0 IN data byte count for transfer"]
pub type Endpoint0indataByteCountForXferW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `Reserved01` reader - Reserved (0)"]
pub type Reserved01R = crate::BitReader;
#[doc = "Field `Endpoint0OUTRxdDataByteCount` reader - Endpoint 0 OUT received data byte count"]
pub type Endpoint0outrxdDataByteCountR = crate::FieldReader;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `BaseAddrOfEndpoint0INOUTDataBuf3332` reader - Base address of Endpoint 0 IN/OUT data buffer\\[33:32\\]"]
pub type BaseAddrOfEndpoint0inoutdataBuf3332R = crate::FieldReader;
#[doc = "Field `BaseAddrOfEndpoint0INOUTDataBuf3332` writer - Base address of Endpoint 0 IN/OUT data buffer\\[33:32\\]"]
pub type BaseAddrOfEndpoint0inoutdataBuf3332W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Endpoint 0 STALL control"]
    #[inline(always)]
    pub fn endpoint0stallctrl(&self) -> Endpoint0stallctrlR {
        Endpoint0stallctrlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint 0 IN buffer ready for transferring data"]
    #[inline(always)]
    pub fn endpoint0inbuf_ready_for_xferring_data(&self) -> Endpoint0inbufReadyForXferringDataR {
        Endpoint0inbufReadyForXferringDataR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Endpoint 0 OUT buffer ready for receiving data"]
    #[inline(always)]
    pub fn endpoint0outbuf_ready_for_receiving_data(
        &self,
    ) -> Endpoint0outbufReadyForReceivingDataR {
        Endpoint0outbufReadyForReceivingDataR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:7 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved02(&self) -> Reserved02R {
        Reserved02R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:14 - Endpoint 0 IN data byte count for transfer"]
    #[inline(always)]
    pub fn endpoint0indata_byte_count_for_xfer(&self) -> Endpoint0indataByteCountForXferR {
        Endpoint0indataByteCountForXferR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:22 - Endpoint 0 OUT received data byte count"]
    #[inline(always)]
    pub fn endpoint0outrxd_data_byte_count(&self) -> Endpoint0outrxdDataByteCountR {
        Endpoint0outrxdDataByteCountR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 23:29 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 23) & 0x7f) as u8)
    }
    #[doc = "Bits 30:31 - Base address of Endpoint 0 IN/OUT data buffer\\[33:32\\]"]
    #[inline(always)]
    pub fn base_addr_of_endpoint0inoutdata_buf3332(&self) -> BaseAddrOfEndpoint0inoutdataBuf3332R {
        BaseAddrOfEndpoint0inoutdataBuf3332R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Endpoint 0 STALL control"]
    #[inline(always)]
    pub fn endpoint0stallctrl(&mut self) -> Endpoint0stallctrlW<Hub30Spec> {
        Endpoint0stallctrlW::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint 0 IN buffer ready for transferring data"]
    #[inline(always)]
    pub fn endpoint0inbuf_ready_for_xferring_data(
        &mut self,
    ) -> Endpoint0inbufReadyForXferringDataW<Hub30Spec> {
        Endpoint0inbufReadyForXferringDataW::new(self, 1)
    }
    #[doc = "Bit 2 - Endpoint 0 OUT buffer ready for receiving data"]
    #[inline(always)]
    pub fn endpoint0outbuf_ready_for_receiving_data(
        &mut self,
    ) -> Endpoint0outbufReadyForReceivingDataW<Hub30Spec> {
        Endpoint0outbufReadyForReceivingDataW::new(self, 2)
    }
    #[doc = "Bits 8:14 - Endpoint 0 IN data byte count for transfer"]
    #[inline(always)]
    pub fn endpoint0indata_byte_count_for_xfer(
        &mut self,
    ) -> Endpoint0indataByteCountForXferW<Hub30Spec> {
        Endpoint0indataByteCountForXferW::new(self, 8)
    }
    #[doc = "Bits 30:31 - Base address of Endpoint 0 IN/OUT data buffer\\[33:32\\]"]
    #[inline(always)]
    pub fn base_addr_of_endpoint0inoutdata_buf3332(
        &mut self,
    ) -> BaseAddrOfEndpoint0inoutdataBuf3332W<Hub30Spec> {
        BaseAddrOfEndpoint0inoutdataBuf3332W::new(self, 30)
    }
}
#[doc = "Endpoint 0 Control/Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hub30::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hub30::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hub30Spec;
impl crate::RegisterSpec for Hub30Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hub30::R`](R) reader structure"]
impl crate::Readable for Hub30Spec {}
#[doc = "`write(|w| ..)` method takes [`hub30::W`](W) writer structure"]
impl crate::Writable for Hub30Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HUB30 to value 0"]
impl crate::Resettable for Hub30Spec {}
