#[doc = "Register `DEV08` reader"]
pub type R = crate::R<Dev08Spec>;
#[doc = "Register `DEV08` writer"]
pub type W = crate::W<Dev08Spec>;
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
#[doc = "Field `Reserved03` reader - Reserved (0)"]
pub type Reserved03R = crate::FieldReader;
#[doc = "Field `Endpoint0INDataByteCountForXfer` reader - Endpoint 0 IN data byte count for transfer"]
pub type Endpoint0indataByteCountForXferR = crate::FieldReader;
#[doc = "Field `Endpoint0INDataByteCountForXfer` writer - Endpoint 0 IN data byte count for transfer"]
pub type Endpoint0indataByteCountForXferW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `Reserved02` reader - Reserved (0)"]
pub type Reserved02R = crate::BitReader;
#[doc = "Field `Endpoint0OUTRxdDataByteCount` reader - Endpoint 0 OUT received data byte count"]
pub type Endpoint0outrxdDataByteCountR = crate::FieldReader;
#[doc = "Field `Reserved01` reader - Reserved (0)"]
pub type Reserved01R = crate::BitReader;
#[doc = "Field `StatusOfTxDMAStateMachineRegdebug` reader - Status of Transmit DMA State Machine regdebug"]
pub type StatusOfTxDmastateMachineRegdebugR = crate::FieldReader;
#[doc = "Field `StartSPLITCycleRegdebug` reader - Start SPLIT Cycle regdebug"]
pub type StartSplitcycleRegdebugR = crate::BitReader;
#[doc = "Field `NormalINWaitRegdebug` reader - Normal IN Wait regdebug"]
pub type NormalInwaitRegdebugR = crate::BitReader;
#[doc = "Field `CSPLITINWaitRegdebug` reader - CSPLIT IN Wait regdebug"]
pub type CsplitinwaitRegdebugR = crate::BitReader;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::BitReader;
#[doc = "Field `BaseAddrOfDataBuf3332` reader - Base address of data buffer\\[33:32\\]"]
pub type BaseAddrOfDataBuf3332R = crate::FieldReader;
#[doc = "Field `BaseAddrOfDataBuf3332` writer - Base address of data buffer\\[33:32\\]"]
pub type BaseAddrOfDataBuf3332W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
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
    pub fn reserved03(&self) -> Reserved03R {
        Reserved03R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:14 - Endpoint 0 IN data byte count for transfer"]
    #[inline(always)]
    pub fn endpoint0indata_byte_count_for_xfer(&self) -> Endpoint0indataByteCountForXferR {
        Endpoint0indataByteCountForXferR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved02(&self) -> Reserved02R {
        Reserved02R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:22 - Endpoint 0 OUT received data byte count"]
    #[inline(always)]
    pub fn endpoint0outrxd_data_byte_count(&self) -> Endpoint0outrxdDataByteCountR {
        Endpoint0outrxdDataByteCountR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Status of Transmit DMA State Machine regdebug"]
    #[inline(always)]
    pub fn status_of_tx_dmastate_machine_regdebug(&self) -> StatusOfTxDmastateMachineRegdebugR {
        StatusOfTxDmastateMachineRegdebugR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Start SPLIT Cycle regdebug"]
    #[inline(always)]
    pub fn start_splitcycle_regdebug(&self) -> StartSplitcycleRegdebugR {
        StartSplitcycleRegdebugR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Normal IN Wait regdebug"]
    #[inline(always)]
    pub fn normal_inwait_regdebug(&self) -> NormalInwaitRegdebugR {
        NormalInwaitRegdebugR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - CSPLIT IN Wait regdebug"]
    #[inline(always)]
    pub fn csplitinwait_regdebug(&self) -> CsplitinwaitRegdebugR {
        CsplitinwaitRegdebugR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Base address of data buffer\\[33:32\\]"]
    #[inline(always)]
    pub fn base_addr_of_data_buf3332(&self) -> BaseAddrOfDataBuf3332R {
        BaseAddrOfDataBuf3332R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Endpoint 0 STALL control"]
    #[inline(always)]
    pub fn endpoint0stallctrl(&mut self) -> Endpoint0stallctrlW<Dev08Spec> {
        Endpoint0stallctrlW::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint 0 IN buffer ready for transferring data"]
    #[inline(always)]
    pub fn endpoint0inbuf_ready_for_xferring_data(
        &mut self,
    ) -> Endpoint0inbufReadyForXferringDataW<Dev08Spec> {
        Endpoint0inbufReadyForXferringDataW::new(self, 1)
    }
    #[doc = "Bit 2 - Endpoint 0 OUT buffer ready for receiving data"]
    #[inline(always)]
    pub fn endpoint0outbuf_ready_for_receiving_data(
        &mut self,
    ) -> Endpoint0outbufReadyForReceivingDataW<Dev08Spec> {
        Endpoint0outbufReadyForReceivingDataW::new(self, 2)
    }
    #[doc = "Bits 8:14 - Endpoint 0 IN data byte count for transfer"]
    #[inline(always)]
    pub fn endpoint0indata_byte_count_for_xfer(
        &mut self,
    ) -> Endpoint0indataByteCountForXferW<Dev08Spec> {
        Endpoint0indataByteCountForXferW::new(self, 8)
    }
    #[doc = "Bits 30:31 - Base address of data buffer\\[33:32\\]"]
    #[inline(always)]
    pub fn base_addr_of_data_buf3332(&mut self) -> BaseAddrOfDataBuf3332W<Dev08Spec> {
        BaseAddrOfDataBuf3332W::new(self, 30)
    }
}
#[doc = "Endpoint 0 Control/Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dev08::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev08::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dev08Spec;
impl crate::RegisterSpec for Dev08Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev08::R`](R) reader structure"]
impl crate::Readable for Dev08Spec {}
#[doc = "`write(|w| ..)` method takes [`dev08::W`](W) writer structure"]
impl crate::Writable for Dev08Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEV08 to value 0"]
impl crate::Resettable for Dev08Spec {}
