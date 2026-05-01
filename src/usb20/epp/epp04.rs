#[doc = "Register `EPP04` reader"]
pub type R = crate::R<Epp04Spec>;
#[doc = "Register `EPP04` writer"]
pub type W = crate::W<Epp04Spec>;
#[doc = "Field `DescriptorListOpEnbl` reader - Descriptor List Operation Enable"]
pub type DescriptorListOpEnblR = crate::BitReader;
#[doc = "Field `DescriptorListOpEnbl` writer - Descriptor List Operation Enable"]
pub type DescriptorListOpEnblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SingleStageDescriptorMode` reader - Single Stage Descriptor Mode"]
pub type SingleStageDescriptorModeR = crate::BitReader;
#[doc = "Field `SingleStageDescriptorMode` writer - Single Stage Descriptor Mode"]
pub type SingleStageDescriptorModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DescriptorListOpRst` reader - Descriptor List Operation Reset"]
pub type DescriptorListOpRstR = crate::BitReader;
#[doc = "Field `DescriptorListOpRst` writer - Descriptor List Operation Reset"]
pub type DescriptorListOpRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMABufModeSel` reader - DMA buffer mode selection"]
pub type DmabufModeSelR = crate::BitReader;
#[doc = "Field `CurDescriptorProcessingStatusRegdebug` reader - Current Descriptor Processing Status regdebug"]
pub type CurDescriptorProcessingStatusRegdebugR = crate::FieldReader;
#[doc = "Field `StartSPLITCycleRegdebug` reader - Start SPLIT Cycle regdebug"]
pub type StartSplitcycleRegdebugR = crate::BitReader;
#[doc = "Field `AutoDataToggleCountRegdebug` reader - Auto Data Toggle Count regdebug"]
pub type AutoDataToggleCountRegdebugR = crate::FieldReader;
#[doc = "Field `CSPLITINWaitRegdebug` reader - CSPLIT IN Wait regdebug"]
pub type CsplitinwaitRegdebugR = crate::BitReader;
#[doc = "Field `TheCurINTGenerationFlagRegdebug` reader - The Current Interrupt Generation Flag regdebug"]
pub type TheCurIntgenerationFlagRegdebugR = crate::BitReader;
#[doc = "Field `OUTTransactionShortPktINTSts` reader - OUT transaction short packet interrupt status"]
pub type OuttransactionShortPktIntstsR = crate::BitReader;
#[doc = "Field `OUTTransactionShortPktINTSts` writer - OUT transaction short packet interrupt status"]
pub type OuttransactionShortPktIntstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DescrptorPreemptyINTSts` reader - Descrptor pre-empty interrupt status"]
pub type DescrptorPreemptyIntstsR = crate::BitReader;
#[doc = "Field `DescrptorPreemptyINTSts` writer - Descrptor pre-empty interrupt status"]
pub type DescrptorPreemptyIntstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LongIdleINTSts` reader - Long idle interrupt status"]
pub type LongIdleIntstsR = crate::BitReader;
#[doc = "Field `LongIdleINTSts` writer - Long idle interrupt status"]
pub type LongIdleIntstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OccupiedTxINBufferIndexRegdebug` reader - Occupied Transmit IN Buffer Index regdebug"]
pub type OccupiedTxInbufferIndexRegdebugR = crate::FieldReader;
#[doc = "Field `OccupiedTxINBufferStatusRegdebug` reader - Occupied Transmit IN Buffer Status regdebug"]
pub type OccupiedTxInbufferStatusRegdebugR = crate::BitReader;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u16>;
#[doc = "Field `DMABaseAddr3332` reader - DMA Base address\\[33:32\\]"]
pub type DmabaseAddr3332R = crate::FieldReader;
#[doc = "Field `DMABaseAddr3332` writer - DMA Base address\\[33:32\\]"]
pub type DmabaseAddr3332W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Descriptor List Operation Enable"]
    #[inline(always)]
    pub fn descriptor_list_op_enbl(&self) -> DescriptorListOpEnblR {
        DescriptorListOpEnblR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Single Stage Descriptor Mode"]
    #[inline(always)]
    pub fn single_stage_descriptor_mode(&self) -> SingleStageDescriptorModeR {
        SingleStageDescriptorModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Descriptor List Operation Reset"]
    #[inline(always)]
    pub fn descriptor_list_op_rst(&self) -> DescriptorListOpRstR {
        DescriptorListOpRstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA buffer mode selection"]
    #[inline(always)]
    pub fn dmabuf_mode_sel(&self) -> DmabufModeSelR {
        DmabufModeSelR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Current Descriptor Processing Status regdebug"]
    #[inline(always)]
    pub fn cur_descriptor_processing_status_regdebug(
        &self,
    ) -> CurDescriptorProcessingStatusRegdebugR {
        CurDescriptorProcessingStatusRegdebugR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Start SPLIT Cycle regdebug"]
    #[inline(always)]
    pub fn start_splitcycle_regdebug(&self) -> StartSplitcycleRegdebugR {
        StartSplitcycleRegdebugR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Auto Data Toggle Count regdebug"]
    #[inline(always)]
    pub fn auto_data_toggle_count_regdebug(&self) -> AutoDataToggleCountRegdebugR {
        AutoDataToggleCountRegdebugR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - CSPLIT IN Wait regdebug"]
    #[inline(always)]
    pub fn csplitinwait_regdebug(&self) -> CsplitinwaitRegdebugR {
        CsplitinwaitRegdebugR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The Current Interrupt Generation Flag regdebug"]
    #[inline(always)]
    pub fn the_cur_intgeneration_flag_regdebug(&self) -> TheCurIntgenerationFlagRegdebugR {
        TheCurIntgenerationFlagRegdebugR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - OUT transaction short packet interrupt status"]
    #[inline(always)]
    pub fn outtransaction_short_pkt_intsts(&self) -> OuttransactionShortPktIntstsR {
        OuttransactionShortPktIntstsR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Descrptor pre-empty interrupt status"]
    #[inline(always)]
    pub fn descrptor_preempty_intsts(&self) -> DescrptorPreemptyIntstsR {
        DescrptorPreemptyIntstsR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Long idle interrupt status"]
    #[inline(always)]
    pub fn long_idle_intsts(&self) -> LongIdleIntstsR {
        LongIdleIntstsR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Occupied Transmit IN Buffer Index regdebug"]
    #[inline(always)]
    pub fn occupied_tx_inbuffer_index_regdebug(&self) -> OccupiedTxInbufferIndexRegdebugR {
        OccupiedTxInbufferIndexRegdebugR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - Occupied Transmit IN Buffer Status regdebug"]
    #[inline(always)]
    pub fn occupied_tx_inbuffer_status_regdebug(&self) -> OccupiedTxInbufferStatusRegdebugR {
        OccupiedTxInbufferStatusRegdebugR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:29 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    #[doc = "Bits 30:31 - DMA Base address\\[33:32\\]"]
    #[inline(always)]
    pub fn dmabase_addr3332(&self) -> DmabaseAddr3332R {
        DmabaseAddr3332R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Descriptor List Operation Enable"]
    #[inline(always)]
    pub fn descriptor_list_op_enbl(&mut self) -> DescriptorListOpEnblW<Epp04Spec> {
        DescriptorListOpEnblW::new(self, 0)
    }
    #[doc = "Bit 1 - Single Stage Descriptor Mode"]
    #[inline(always)]
    pub fn single_stage_descriptor_mode(&mut self) -> SingleStageDescriptorModeW<Epp04Spec> {
        SingleStageDescriptorModeW::new(self, 1)
    }
    #[doc = "Bit 2 - Descriptor List Operation Reset"]
    #[inline(always)]
    pub fn descriptor_list_op_rst(&mut self) -> DescriptorListOpRstW<Epp04Spec> {
        DescriptorListOpRstW::new(self, 2)
    }
    #[doc = "Bit 13 - OUT transaction short packet interrupt status"]
    #[inline(always)]
    pub fn outtransaction_short_pkt_intsts(&mut self) -> OuttransactionShortPktIntstsW<Epp04Spec> {
        OuttransactionShortPktIntstsW::new(self, 13)
    }
    #[doc = "Bit 14 - Descrptor pre-empty interrupt status"]
    #[inline(always)]
    pub fn descrptor_preempty_intsts(&mut self) -> DescrptorPreemptyIntstsW<Epp04Spec> {
        DescrptorPreemptyIntstsW::new(self, 14)
    }
    #[doc = "Bit 15 - Long idle interrupt status"]
    #[inline(always)]
    pub fn long_idle_intsts(&mut self) -> LongIdleIntstsW<Epp04Spec> {
        LongIdleIntstsW::new(self, 15)
    }
    #[doc = "Bits 30:31 - DMA Base address\\[33:32\\]"]
    #[inline(always)]
    pub fn dmabase_addr3332(&mut self) -> DmabaseAddr3332W<Epp04Spec> {
        DmabaseAddr3332W::new(self, 30)
    }
}
#[doc = "DMA Descriptor List Control/Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`epp04::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epp04::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Epp04Spec;
impl crate::RegisterSpec for Epp04Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`epp04::R`](R) reader structure"]
impl crate::Readable for Epp04Spec {}
#[doc = "`write(|w| ..)` method takes [`epp04::W`](W) writer structure"]
impl crate::Writable for Epp04Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EPP04 to value 0"]
impl crate::Resettable for Epp04Spec {}
