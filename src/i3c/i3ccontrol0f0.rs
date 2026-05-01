#[doc = "Register `I3CCONTROL0F0` reader"]
pub type R = crate::R<I3ccontrol0f0Spec>;
#[doc = "Register `I3CCONTROL0F0` writer"]
pub type W = crate::W<I3ccontrol0f0Spec>;
#[doc = "Field `REGCAPSTATUS` reader - REG_CAP_STATUS"]
pub type RegcapstatusR = crate::BitReader;
#[doc = "Field `REGPIOSTATUS` reader - REG_PIO_STATUS"]
pub type RegpiostatusR = crate::BitReader;
#[doc = "Field `REGRHSSTATUS` reader - REG_RHS_STATUS"]
pub type RegrhsstatusR = crate::BitReader;
#[doc = "Field `REGINHOUSESTATUS` reader - REG_INHOUSE_STATUS"]
pub type ReginhousestatusR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - REG_CAP_STATUS"]
    #[inline(always)]
    pub fn regcapstatus(&self) -> RegcapstatusR {
        RegcapstatusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - REG_PIO_STATUS"]
    #[inline(always)]
    pub fn regpiostatus(&self) -> RegpiostatusR {
        RegpiostatusR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - REG_RHS_STATUS"]
    #[inline(always)]
    pub fn regrhsstatus(&self) -> RegrhsstatusR {
        RegrhsstatusR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - REG_INHOUSE_STATUS"]
    #[inline(always)]
    pub fn reginhousestatus(&self) -> ReginhousestatusR {
        ReginhousestatusR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {}
#[doc = "I3C\\_INTR\\_STATUS\\_F0\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol0f0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol0f0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol0f0Spec;
impl crate::RegisterSpec for I3ccontrol0f0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol0f0::R`](R) reader structure"]
impl crate::Readable for I3ccontrol0f0Spec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol0f0::W`](W) writer structure"]
impl crate::Writable for I3ccontrol0f0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL0F0 to value 0"]
impl crate::Resettable for I3ccontrol0f0Spec {}
