#[doc = "Register `I3CPHYCTRLREG0C4` reader"]
pub type R = crate::R<I3cphyctrlreg0c4Spec>;
#[doc = "Register `I3CPHYCTRLREG0C4` writer"]
pub type W = crate::W<I3cphyctrlreg0c4Spec>;
#[doc = "Field `REGSCLSYNCI` reader - REG_SCL_SYNC_I"]
pub type RegsclsynciR = crate::BitReader;
#[doc = "Field `REGSDASYNCI` reader - REG_SDA_SYNC_I"]
pub type RegsdasynciR = crate::BitReader;
#[doc = "Field `REGBUSSTOPOK` reader - REG_BUS_STOP_OK"]
pub type RegbusstopokR = crate::BitReader;
#[doc = "Field `REGSDASTUCKLOW` reader - REG_SDA_STUCK_LOW"]
pub type RegsdastucklowR = crate::BitReader;
#[doc = "Field `REGSDASTUCKHIGH` reader - REG_SDA_STUCK_HIGH"]
pub type RegsdastuckhighR = crate::BitReader;
#[doc = "Field `REGSCLSTUCK` reader - REG_SCL_STUCK"]
pub type RegsclstuckR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - REG_SCL_SYNC_I"]
    #[inline(always)]
    pub fn regsclsynci(&self) -> RegsclsynciR {
        RegsclsynciR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - REG_SDA_SYNC_I"]
    #[inline(always)]
    pub fn regsdasynci(&self) -> RegsdasynciR {
        RegsdasynciR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - REG_BUS_STOP_OK"]
    #[inline(always)]
    pub fn regbusstopok(&self) -> RegbusstopokR {
        RegbusstopokR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - REG_SDA_STUCK_LOW"]
    #[inline(always)]
    pub fn regsdastucklow(&self) -> RegsdastucklowR {
        RegsdastucklowR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - REG_SDA_STUCK_HIGH"]
    #[inline(always)]
    pub fn regsdastuckhigh(&self) -> RegsdastuckhighR {
        RegsdastuckhighR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - REG_SCL_STUCK"]
    #[inline(always)]
    pub fn regsclstuck(&self) -> RegsclstuckR {
        RegsclstuckR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {}
#[doc = "SDA\\_STUCK\\_READ\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cphyctrlreg0c4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cphyctrlreg0c4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cphyctrlreg0c4Spec;
impl crate::RegisterSpec for I3cphyctrlreg0c4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cphyctrlreg0c4::R`](R) reader structure"]
impl crate::Readable for I3cphyctrlreg0c4Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cphyctrlreg0c4::W`](W) writer structure"]
impl crate::Writable for I3cphyctrlreg0c4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CPHYCTRLREG0C4 to value 0"]
impl crate::Resettable for I3cphyctrlreg0c4Spec {}
