#[doc = "Register `I3CCONTROL004` reader"]
pub type R = crate::R<I3ccontrol004Spec>;
#[doc = "Register `I3CCONTROL004` writer"]
pub type W = crate::W<I3ccontrol004Spec>;
#[doc = "Field `REGMODEPRIMARYMST` reader - REG_MODE_PRIMARY_MST"]
pub type RegmodeprimarymstR = crate::BitReader;
#[doc = "Field `REGMODEPRIMARYSLV` reader - REG_MODE_PRIMARY_SLV"]
pub type RegmodeprimaryslvR = crate::BitReader;
#[doc = "Field `REGMODEPRIMARYMSTTOSLV` reader - REG_MODE_PRIMARY_MST_TO_SLV"]
pub type RegmodeprimarymsttoslvR = crate::BitReader;
#[doc = "Field `REGMODEPRIMARYSLVTOMST` reader - REG_MODE_PRIMARY_SLV_TO_MST"]
pub type RegmodeprimaryslvtomstR = crate::BitReader;
#[doc = "Field `REGMODESECONDARYMST` reader - REG_MODE_SECONDARY_MST"]
pub type RegmodesecondarymstR = crate::BitReader;
#[doc = "Field `REGMODESECONDARYSLV` reader - REG_MODE_SECONDARY_SLV"]
pub type RegmodesecondaryslvR = crate::BitReader;
#[doc = "Field `REGMODESECONDARYMSTTOSLV` reader - REG_MODE_SECONDARY_MST_TO_SLV"]
pub type RegmodesecondarymsttoslvR = crate::BitReader;
#[doc = "Field `REGMODESECONDARYSLVTOMST` reader - REG_MODE_SECONDARY_SLV_TO_MST"]
pub type RegmodesecondaryslvtomstR = crate::BitReader;
#[doc = "Field `REGMODEPURESLV` reader - REG_MODE_PURE_SLV"]
pub type RegmodepureslvR = crate::BitReader;
#[doc = "Field `REGDMAABORTED` reader - REG_DMA_ABORTED"]
pub type RegdmaabortedR = crate::BitReader;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `REGMSTINCRITICALSECTION` reader - REG_MST_IN_CRITICAL_SECTION"]
pub type RegmstincriticalsectionR = crate::BitReader;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGSLVDYNAMICADDRESS` reader - REG_SLV_DYNAMIC_ADDRESS"]
pub type RegslvdynamicaddressR = crate::FieldReader;
#[doc = "Field `REGSLVDYNAMICADDRESSVALID` reader - REG_SLV_DYNAMIC_ADDRESS_VALID"]
pub type RegslvdynamicaddressvalidR = crate::BitReader;
#[doc = "Field `REGMSTTRANSSTATE` reader - REG_MST_TRANS_STATE"]
pub type RegmsttransstateR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - REG_MODE_PRIMARY_MST"]
    #[inline(always)]
    pub fn regmodeprimarymst(&self) -> RegmodeprimarymstR {
        RegmodeprimarymstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - REG_MODE_PRIMARY_SLV"]
    #[inline(always)]
    pub fn regmodeprimaryslv(&self) -> RegmodeprimaryslvR {
        RegmodeprimaryslvR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - REG_MODE_PRIMARY_MST_TO_SLV"]
    #[inline(always)]
    pub fn regmodeprimarymsttoslv(&self) -> RegmodeprimarymsttoslvR {
        RegmodeprimarymsttoslvR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - REG_MODE_PRIMARY_SLV_TO_MST"]
    #[inline(always)]
    pub fn regmodeprimaryslvtomst(&self) -> RegmodeprimaryslvtomstR {
        RegmodeprimaryslvtomstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - REG_MODE_SECONDARY_MST"]
    #[inline(always)]
    pub fn regmodesecondarymst(&self) -> RegmodesecondarymstR {
        RegmodesecondarymstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - REG_MODE_SECONDARY_SLV"]
    #[inline(always)]
    pub fn regmodesecondaryslv(&self) -> RegmodesecondaryslvR {
        RegmodesecondaryslvR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - REG_MODE_SECONDARY_MST_TO_SLV"]
    #[inline(always)]
    pub fn regmodesecondarymsttoslv(&self) -> RegmodesecondarymsttoslvR {
        RegmodesecondarymsttoslvR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - REG_MODE_SECONDARY_SLV_TO_MST"]
    #[inline(always)]
    pub fn regmodesecondaryslvtomst(&self) -> RegmodesecondaryslvtomstR {
        RegmodesecondaryslvtomstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - REG_MODE_PURE_SLV"]
    #[inline(always)]
    pub fn regmodepureslv(&self) -> RegmodepureslvR {
        RegmodepureslvR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - REG_DMA_ABORTED"]
    #[inline(always)]
    pub fn regdmaaborted(&self) -> RegdmaabortedR {
        RegdmaabortedR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - REG_MST_IN_CRITICAL_SECTION"]
    #[inline(always)]
    pub fn regmstincriticalsection(&self) -> RegmstincriticalsectionR {
        RegmstincriticalsectionR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:22 - REG_SLV_DYNAMIC_ADDRESS"]
    #[inline(always)]
    pub fn regslvdynamicaddress(&self) -> RegslvdynamicaddressR {
        RegslvdynamicaddressR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - REG_SLV_DYNAMIC_ADDRESS_VALID"]
    #[inline(always)]
    pub fn regslvdynamicaddressvalid(&self) -> RegslvdynamicaddressvalidR {
        RegslvdynamicaddressvalidR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - REG_MST_TRANS_STATE"]
    #[inline(always)]
    pub fn regmsttransstate(&self) -> RegmsttransstateR {
        RegmsttransstateR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "I3C\\_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol004::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol004::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol004Spec;
impl crate::RegisterSpec for I3ccontrol004Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol004::R`](R) reader structure"]
impl crate::Readable for I3ccontrol004Spec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol004::W`](W) writer structure"]
impl crate::Writable for I3ccontrol004Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL004 to value 0"]
impl crate::Resettable for I3ccontrol004Spec {}
