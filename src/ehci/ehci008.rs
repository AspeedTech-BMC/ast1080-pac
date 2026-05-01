#[doc = "Register `EHCI008` reader"]
pub type R = crate::R<Ehci008Spec>;
#[doc = "Register `EHCI008` writer"]
pub type W = crate::W<Ehci008Spec>;
#[doc = "64-bit Addressing Capability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum _64bitAddringCapability {
    #[doc = "0: data structures using 32-bit address memory pointers"]
    DataStructuresUsing32bitAddressMemoryPointers = 0,
    #[doc = "1: data structures using 64-bit address memory pointers"]
    DataStructuresUsing64bitAddressMemoryPointers = 1,
}
impl From<_64bitAddringCapability> for bool {
    #[inline(always)]
    fn from(variant: _64bitAddringCapability) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `64bitAddringCapability` reader - 64-bit Addressing Capability"]
pub type _64bitAddringCapabilityR = crate::BitReader<_64bitAddringCapability>;
impl _64bitAddringCapabilityR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> _64bitAddringCapability {
        match self.bits {
            false => _64bitAddringCapability::DataStructuresUsing32bitAddressMemoryPointers,
            true => _64bitAddringCapability::DataStructuresUsing64bitAddressMemoryPointers,
        }
    }
    #[doc = "data structures using 32-bit address memory pointers"]
    #[inline(always)]
    pub fn is_data_structures_using_32bit_address_memory_pointers(&self) -> bool {
        *self == _64bitAddringCapability::DataStructuresUsing32bitAddressMemoryPointers
    }
    #[doc = "data structures using 64-bit address memory pointers"]
    #[inline(always)]
    pub fn is_data_structures_using_64bit_address_memory_pointers(&self) -> bool {
        *self == _64bitAddringCapability::DataStructuresUsing64bitAddressMemoryPointers
    }
}
#[doc = "Field `ProgrammableFrameListFlag` reader - Programmable Frame List Flag"]
pub type ProgrammableFrameListFlagR = crate::BitReader;
#[doc = "Field `AsynchronousScheduleParkCapability` reader - Asynchronous Schedule Park Capability"]
pub type AsynchronousScheduleParkCapabilityR = crate::BitReader;
#[doc = "Field `Reserved01` reader - Reserved (0)"]
pub type Reserved01R = crate::BitReader;
#[doc = "Field `IsochronousSchedulingThreshold` reader - Isochronous Scheduling Threshold"]
pub type IsochronousSchedulingThresholdR = crate::FieldReader;
#[doc = "Field `EHCIExtendedCapabilitiesPointerEECP` reader - EHCI Extended Capabilities Pointer (EECP)"]
pub type EhciextendedCapabilitiesPointerEecpR = crate::FieldReader;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - 64-bit Addressing Capability"]
    #[inline(always)]
    pub fn _64bit_addring_capability(&self) -> _64bitAddringCapabilityR {
        _64bitAddringCapabilityR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Programmable Frame List Flag"]
    #[inline(always)]
    pub fn programmable_frame_list_flag(&self) -> ProgrammableFrameListFlagR {
        ProgrammableFrameListFlagR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Asynchronous Schedule Park Capability"]
    #[inline(always)]
    pub fn asynchronous_schedule_park_capability(&self) -> AsynchronousScheduleParkCapabilityR {
        AsynchronousScheduleParkCapabilityR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Isochronous Scheduling Threshold"]
    #[inline(always)]
    pub fn isochronous_scheduling_threshold(&self) -> IsochronousSchedulingThresholdR {
        IsochronousSchedulingThresholdR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - EHCI Extended Capabilities Pointer (EECP)"]
    #[inline(always)]
    pub fn ehciextended_capabilities_pointer_eecp(&self) -> EhciextendedCapabilitiesPointerEecpR {
        EhciextendedCapabilitiesPointerEecpR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Capability Parameters (HCCPARAMS)\n\nYou can [`read`](crate::Reg::read) this register and get [`ehci008::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehci008::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ehci008Spec;
impl crate::RegisterSpec for Ehci008Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ehci008::R`](R) reader structure"]
impl crate::Readable for Ehci008Spec {}
#[doc = "`write(|w| ..)` method takes [`ehci008::W`](W) writer structure"]
impl crate::Writable for Ehci008Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EHCI008 to value 0x16"]
impl crate::Resettable for Ehci008Spec {
    const RESET_VALUE: u32 = 0x16;
}
