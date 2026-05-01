#[doc = "Register `EPP0C` reader"]
pub type R = crate::R<Epp0cSpec>;
#[doc = "Register `EPP0C` writer"]
pub type W = crate::W<Epp0cSpec>;
#[doc = "Field `DescriptorListCPUWrPointerDefault0` reader - Descriptor List CPU Write Pointer (Default=0)"]
pub type DescriptorListCpuwrPointerDefault0R = crate::FieldReader;
#[doc = "Field `DescriptorListCPUWrPointerDefault0` writer - Descriptor List CPU Write Pointer (Default=0)"]
pub type DescriptorListCpuwrPointerDefault0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DescriptorListDMAReadPointerDefault0` reader - Descriptor List DMA Read Pointer (Default=0)"]
pub type DescriptorListDmareadPointerDefault0R = crate::FieldReader;
#[doc = "Field `DescriptorListDMAReadPointerDefault0` writer - Descriptor List DMA Read Pointer (Default=0)"]
pub type DescriptorListDmareadPointerDefault0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PktSizeDefaultX` reader - Packet Size (Default=X)"]
pub type PktSizeDefaultXR = crate::FieldReader<u16>;
#[doc = "Field `PktSizeDefaultX` writer - Packet Size (Default=X)"]
pub type PktSizeDefaultXW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::BitReader;
#[doc = "Field `EndpointCurDataToggleSequenceValueDefault0` reader - Endpoint Current Data Toggle Sequence Value (Default=0)"]
pub type EndpointCurDataToggleSequenceValueDefault0R = crate::FieldReader;
#[doc = "Field `EndpointCurDataToggleSequenceValueDefault0` writer - Endpoint Current Data Toggle Sequence Value (Default=0)"]
pub type EndpointCurDataToggleSequenceValueDefault0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LongIdleINTSts` reader - Long idle interrupt status"]
pub type LongIdleIntstsR = crate::BitReader;
#[doc = "Field `LongIdleINTSts` writer - Long idle interrupt status"]
pub type LongIdleIntstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DescriptorListEmptyFlagDefault0` reader - Descriptor List Empty Flag (Default=0)"]
pub type DescriptorListEmptyFlagDefault0R = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - Descriptor List CPU Write Pointer (Default=0)"]
    #[inline(always)]
    pub fn descriptor_list_cpuwr_pointer_default0(&self) -> DescriptorListCpuwrPointerDefault0R {
        DescriptorListCpuwrPointerDefault0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Descriptor List DMA Read Pointer (Default=0)"]
    #[inline(always)]
    pub fn descriptor_list_dmaread_pointer_default0(
        &self,
    ) -> DescriptorListDmareadPointerDefault0R {
        DescriptorListDmareadPointerDefault0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:26 - Packet Size (Default=X)"]
    #[inline(always)]
    pub fn pkt_size_default_x(&self) -> PktSizeDefaultXR {
        PktSizeDefaultXR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bit 27 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Endpoint Current Data Toggle Sequence Value (Default=0)"]
    #[inline(always)]
    pub fn endpoint_cur_data_toggle_sequence_value_default0(
        &self,
    ) -> EndpointCurDataToggleSequenceValueDefault0R {
        EndpointCurDataToggleSequenceValueDefault0R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Long idle interrupt status"]
    #[inline(always)]
    pub fn long_idle_intsts(&self) -> LongIdleIntstsR {
        LongIdleIntstsR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Descriptor List Empty Flag (Default=0)"]
    #[inline(always)]
    pub fn descriptor_list_empty_flag_default0(&self) -> DescriptorListEmptyFlagDefault0R {
        DescriptorListEmptyFlagDefault0R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Descriptor List CPU Write Pointer (Default=0)"]
    #[inline(always)]
    pub fn descriptor_list_cpuwr_pointer_default0(
        &mut self,
    ) -> DescriptorListCpuwrPointerDefault0W<Epp0cSpec> {
        DescriptorListCpuwrPointerDefault0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Descriptor List DMA Read Pointer (Default=0)"]
    #[inline(always)]
    pub fn descriptor_list_dmaread_pointer_default0(
        &mut self,
    ) -> DescriptorListDmareadPointerDefault0W<Epp0cSpec> {
        DescriptorListDmareadPointerDefault0W::new(self, 8)
    }
    #[doc = "Bits 16:26 - Packet Size (Default=X)"]
    #[inline(always)]
    pub fn pkt_size_default_x(&mut self) -> PktSizeDefaultXW<Epp0cSpec> {
        PktSizeDefaultXW::new(self, 16)
    }
    #[doc = "Bits 28:29 - Endpoint Current Data Toggle Sequence Value (Default=0)"]
    #[inline(always)]
    pub fn endpoint_cur_data_toggle_sequence_value_default0(
        &mut self,
    ) -> EndpointCurDataToggleSequenceValueDefault0W<Epp0cSpec> {
        EndpointCurDataToggleSequenceValueDefault0W::new(self, 28)
    }
    #[doc = "Bit 30 - Long idle interrupt status"]
    #[inline(always)]
    pub fn long_idle_intsts(&mut self) -> LongIdleIntstsW<Epp0cSpec> {
        LongIdleIntstsW::new(self, 30)
    }
}
#[doc = "DMA Descriptor List Read(DMA)/Write(CPU) Pointer and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`epp0c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epp0c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Epp0cSpec;
impl crate::RegisterSpec for Epp0cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`epp0c::R`](R) reader structure"]
impl crate::Readable for Epp0cSpec {}
#[doc = "`write(|w| ..)` method takes [`epp0c::W`](W) writer structure"]
impl crate::Writable for Epp0cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EPP0C to value 0"]
impl crate::Resettable for Epp0cSpec {}
