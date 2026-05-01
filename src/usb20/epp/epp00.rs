#[doc = "Register `EPP00` reader"]
pub type R = crate::R<Epp00Spec>;
#[doc = "Register `EPP00` writer"]
pub type W = crate::W<Epp00Spec>;
#[doc = "Field `EnblEndpoint` reader - Enable Endpoint"]
pub type EnblEndpointR = crate::BitReader;
#[doc = "Field `EnblEndpoint` writer - Enable Endpoint"]
pub type EnblEndpointW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AllocatedDevPortNumber` reader - Allocated Device Port Number"]
pub type AllocatedDevPortNumberR = crate::FieldReader;
#[doc = "Field `AllocatedDevPortNumber` writer - Allocated Device Port Number"]
pub type AllocatedDevPortNumberW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `EndpointTypeSel` reader - Endpoint type selection"]
pub type EndpointTypeSelR = crate::FieldReader;
#[doc = "Field `EndpointTypeSel` writer - Endpoint type selection"]
pub type EndpointTypeSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved01` reader - Reserved (0)"]
pub type Reserved01R = crate::BitReader;
#[doc = "Field `EndpointNumber` reader - Endpoint Number"]
pub type EndpointNumberR = crate::FieldReader;
#[doc = "Field `EndpointNumber` writer - Endpoint Number"]
pub type EndpointNumberW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EndpointStallCtrl` reader - Endpoint Stall Control"]
pub type EndpointStallCtrlR = crate::BitReader;
#[doc = "Field `EndpointStallCtrl` writer - Endpoint Stall Control"]
pub type EndpointStallCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EndpointAutoDataToggleDis` reader - Endpoint Auto Data Toggle Disable"]
pub type EndpointAutoDataToggleDisR = crate::BitReader;
#[doc = "Field `EndpointAutoDataToggleDis` writer - Endpoint Auto Data Toggle Disable"]
pub type EndpointAutoDataToggleDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EndpointDataFetchCtrl` reader - Endpoint Data Fetch Control"]
pub type EndpointDataFetchCtrlR = crate::FieldReader;
#[doc = "Field `EndpointDataFetchCtrl` writer - Endpoint Data Fetch Control"]
pub type EndpointDataFetchCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EndpointMaximumPktSizeDefinition` reader - Endpoint Maximum Packet Size Definition"]
pub type EndpointMaximumPktSizeDefinitionR = crate::FieldReader<u16>;
#[doc = "Field `EndpointMaximumPktSizeDefinition` writer - Endpoint Maximum Packet Size Definition"]
pub type EndpointMaximumPktSizeDefinitionW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::BitReader;
#[doc = "Field `BurstTransactionsLongIdleINTCtrl` reader - Burst transactions long idle interrupt control"]
pub type BurstTransactionsLongIdleIntctrlR = crate::BitReader;
#[doc = "Field `BurstTransactionsLongIdleINTCtrl` writer - Burst transactions long idle interrupt control"]
pub type BurstTransactionsLongIdleIntctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ListModePreemptyINTCtrl` reader - List mode pre-empty interrupt control"]
pub type ListModePreemptyIntctrlR = crate::FieldReader;
#[doc = "Field `ListModePreemptyINTCtrl` writer - List mode pre-empty interrupt control"]
pub type ListModePreemptyIntctrlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Enable Endpoint"]
    #[inline(always)]
    pub fn enbl_endpoint(&self) -> EnblEndpointR {
        EnblEndpointR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Allocated Device Port Number"]
    #[inline(always)]
    pub fn allocated_dev_port_number(&self) -> AllocatedDevPortNumberR {
        AllocatedDevPortNumberR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:6 - Endpoint type selection"]
    #[inline(always)]
    pub fn endpoint_type_sel(&self) -> EndpointTypeSelR {
        EndpointTypeSelR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Endpoint Number"]
    #[inline(always)]
    pub fn endpoint_number(&self) -> EndpointNumberR {
        EndpointNumberR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Endpoint Stall Control"]
    #[inline(always)]
    pub fn endpoint_stall_ctrl(&self) -> EndpointStallCtrlR {
        EndpointStallCtrlR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Endpoint Auto Data Toggle Disable"]
    #[inline(always)]
    pub fn endpoint_auto_data_toggle_dis(&self) -> EndpointAutoDataToggleDisR {
        EndpointAutoDataToggleDisR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Endpoint Data Fetch Control"]
    #[inline(always)]
    pub fn endpoint_data_fetch_ctrl(&self) -> EndpointDataFetchCtrlR {
        EndpointDataFetchCtrlR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:25 - Endpoint Maximum Packet Size Definition"]
    #[inline(always)]
    pub fn endpoint_maximum_pkt_size_definition(&self) -> EndpointMaximumPktSizeDefinitionR {
        EndpointMaximumPktSizeDefinitionR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Burst transactions long idle interrupt control"]
    #[inline(always)]
    pub fn burst_transactions_long_idle_intctrl(&self) -> BurstTransactionsLongIdleIntctrlR {
        BurstTransactionsLongIdleIntctrlR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31 - List mode pre-empty interrupt control"]
    #[inline(always)]
    pub fn list_mode_preempty_intctrl(&self) -> ListModePreemptyIntctrlR {
        ListModePreemptyIntctrlR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Endpoint"]
    #[inline(always)]
    pub fn enbl_endpoint(&mut self) -> EnblEndpointW<Epp00Spec> {
        EnblEndpointW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Allocated Device Port Number"]
    #[inline(always)]
    pub fn allocated_dev_port_number(&mut self) -> AllocatedDevPortNumberW<Epp00Spec> {
        AllocatedDevPortNumberW::new(self, 1)
    }
    #[doc = "Bits 4:6 - Endpoint type selection"]
    #[inline(always)]
    pub fn endpoint_type_sel(&mut self) -> EndpointTypeSelW<Epp00Spec> {
        EndpointTypeSelW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Endpoint Number"]
    #[inline(always)]
    pub fn endpoint_number(&mut self) -> EndpointNumberW<Epp00Spec> {
        EndpointNumberW::new(self, 8)
    }
    #[doc = "Bit 12 - Endpoint Stall Control"]
    #[inline(always)]
    pub fn endpoint_stall_ctrl(&mut self) -> EndpointStallCtrlW<Epp00Spec> {
        EndpointStallCtrlW::new(self, 12)
    }
    #[doc = "Bit 13 - Endpoint Auto Data Toggle Disable"]
    #[inline(always)]
    pub fn endpoint_auto_data_toggle_dis(&mut self) -> EndpointAutoDataToggleDisW<Epp00Spec> {
        EndpointAutoDataToggleDisW::new(self, 13)
    }
    #[doc = "Bits 14:15 - Endpoint Data Fetch Control"]
    #[inline(always)]
    pub fn endpoint_data_fetch_ctrl(&mut self) -> EndpointDataFetchCtrlW<Epp00Spec> {
        EndpointDataFetchCtrlW::new(self, 14)
    }
    #[doc = "Bits 16:25 - Endpoint Maximum Packet Size Definition"]
    #[inline(always)]
    pub fn endpoint_maximum_pkt_size_definition(
        &mut self,
    ) -> EndpointMaximumPktSizeDefinitionW<Epp00Spec> {
        EndpointMaximumPktSizeDefinitionW::new(self, 16)
    }
    #[doc = "Bit 27 - Burst transactions long idle interrupt control"]
    #[inline(always)]
    pub fn burst_transactions_long_idle_intctrl(
        &mut self,
    ) -> BurstTransactionsLongIdleIntctrlW<Epp00Spec> {
        BurstTransactionsLongIdleIntctrlW::new(self, 27)
    }
    #[doc = "Bits 28:31 - List mode pre-empty interrupt control"]
    #[inline(always)]
    pub fn list_mode_preempty_intctrl(&mut self) -> ListModePreemptyIntctrlW<Epp00Spec> {
        ListModePreemptyIntctrlW::new(self, 28)
    }
}
#[doc = "Endpoint Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`epp00::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epp00::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Epp00Spec;
impl crate::RegisterSpec for Epp00Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`epp00::R`](R) reader structure"]
impl crate::Readable for Epp00Spec {}
#[doc = "`write(|w| ..)` method takes [`epp00::W`](W) writer structure"]
impl crate::Writable for Epp00Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EPP00 to value 0"]
impl crate::Resettable for Epp00Spec {}
