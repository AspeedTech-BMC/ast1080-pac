#[doc = "Register `I3CCONTROL00C` reader"]
pub type R = crate::R<I3ccontrol00cSpec>;
#[doc = "Register `I3CCONTROL00C` writer"]
pub type W = crate::W<I3ccontrol00cSpec>;
#[doc = "Field `REGMSTWRITESTATE` reader - REG_MST_WRITE_STATE"]
pub type RegmstwritestateR = crate::FieldReader;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `REGMSTWRITEDDRSTATE` reader - REG_MST_WRITE_DDR_STATE"]
pub type RegmstwriteddrstateR = crate::FieldReader;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `REGMSTREADSTATE` reader - REG_MST_READ_STATE"]
pub type RegmstreadstateR = crate::FieldReader;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGMSTREADDDRSTATE` reader - REG_MST_READ_DDR_STATE"]
pub type RegmstreadddrstateR = crate::FieldReader;
#[doc = "Field `REGMSTINTERNALSTATE` reader - REG_MST_INTERNAL_STATE"]
pub type RegmstinternalstateR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - REG_MST_WRITE_STATE"]
    #[inline(always)]
    pub fn regmstwritestate(&self) -> RegmstwritestateR {
        RegmstwritestateR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:10 - REG_MST_WRITE_DDR_STATE"]
    #[inline(always)]
    pub fn regmstwriteddrstate(&self) -> RegmstwriteddrstateR {
        RegmstwriteddrstateR::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:16 - REG_MST_READ_STATE"]
    #[inline(always)]
    pub fn regmstreadstate(&self) -> RegmstreadstateR {
        RegmstreadstateR::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 17:19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:24 - REG_MST_READ_DDR_STATE"]
    #[inline(always)]
    pub fn regmstreadddrstate(&self) -> RegmstreadddrstateR {
        RegmstreadddrstateR::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 28:31 - REG_MST_INTERNAL_STATE"]
    #[inline(always)]
    pub fn regmstinternalstate(&self) -> RegmstinternalstateR {
        RegmstinternalstateR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "I3C\\_STATUS\\_C\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol00c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol00c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol00cSpec;
impl crate::RegisterSpec for I3ccontrol00cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol00c::R`](R) reader structure"]
impl crate::Readable for I3ccontrol00cSpec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol00c::W`](W) writer structure"]
impl crate::Writable for I3ccontrol00cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL00C to value 0"]
impl crate::Resettable for I3ccontrol00cSpec {}
