#[doc = "Register `EHCI000` reader"]
pub type R = crate::R<Ehci000Spec>;
#[doc = "Register `EHCI000` writer"]
pub type W = crate::W<Ehci000Spec>;
#[doc = "Field `OffsetToOpalRegs` reader - Offset to Operational Registers"]
pub type OffsetToOpalRegsR = crate::FieldReader;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `HostCtrlInterfaceVersionNumberHCIVERSION` reader - Host Controller Interface Version Number (HCIVERSION)"]
pub type HostCtrlInterfaceVersionNumberHciversionR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:7 - Offset to Operational Registers"]
    #[inline(always)]
    pub fn offset_to_opal_regs(&self) -> OffsetToOpalRegsR {
        OffsetToOpalRegsR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Host Controller Interface Version Number (HCIVERSION)"]
    #[inline(always)]
    pub fn host_ctrl_interface_version_number_hciversion(
        &self,
    ) -> HostCtrlInterfaceVersionNumberHciversionR {
        HostCtrlInterfaceVersionNumberHciversionR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Capability Registers Length (CAPLENGTH)\n\nYou can [`read`](crate::Reg::read) this register and get [`ehci000::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehci000::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ehci000Spec;
impl crate::RegisterSpec for Ehci000Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ehci000::R`](R) reader structure"]
impl crate::Readable for Ehci000Spec {}
#[doc = "`write(|w| ..)` method takes [`ehci000::W`](W) writer structure"]
impl crate::Writable for Ehci000Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EHCI000 to value 0x0100_0020"]
impl crate::Resettable for Ehci000Spec {
    const RESET_VALUE: u32 = 0x0100_0020;
}
