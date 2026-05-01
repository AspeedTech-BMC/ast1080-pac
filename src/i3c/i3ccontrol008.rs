#[doc = "Register `I3CCONTROL008` reader"]
pub type R = crate::R<I3ccontrol008Spec>;
#[doc = "Register `I3CCONTROL008` writer"]
pub type W = crate::W<I3ccontrol008Spec>;
#[doc = "Field `REGMSTIBILENGTH` reader - REG_MST_IBI_LENGTH"]
pub type RegmstibilengthR = crate::FieldReader<u16>;
#[doc = "Field `REGMSTIBILENGTH` writer - REG_MST_IBI_LENGTH"]
pub type RegmstibilengthW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `REGTERMINATEIFIBILENGTH` reader - REG_TERMINATE_IF_IBI_LENGTH"]
pub type RegterminateifibilengthR = crate::BitReader;
#[doc = "Field `REGTERMINATEIFIBILENGTH` writer - REG_TERMINATE_IF_IBI_LENGTH"]
pub type RegterminateifibilengthW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGMSTDAASTATE` reader - REG_MST_DAA_STATE"]
pub type RegmstdaastateR = crate::FieldReader;
#[doc = "Field `REGMSTIBISTATE` reader - REG_MST_IBI_STATE"]
pub type RegmstibistateR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - REG_MST_IBI_LENGTH"]
    #[inline(always)]
    pub fn regmstibilength(&self) -> RegmstibilengthR {
        RegmstibilengthR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - REG_TERMINATE_IF_IBI_LENGTH"]
    #[inline(always)]
    pub fn regterminateifibilength(&self) -> RegterminateifibilengthR {
        RegterminateifibilengthR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:23 - REG_MST_DAA_STATE"]
    #[inline(always)]
    pub fn regmstdaastate(&self) -> RegmstdaastateR {
        RegmstdaastateR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:28 - REG_MST_IBI_STATE"]
    #[inline(always)]
    pub fn regmstibistate(&self) -> RegmstibistateR {
        RegmstibistateR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - REG_MST_IBI_LENGTH"]
    #[inline(always)]
    pub fn regmstibilength(&mut self) -> RegmstibilengthW<I3ccontrol008Spec> {
        RegmstibilengthW::new(self, 0)
    }
    #[doc = "Bit 16 - REG_TERMINATE_IF_IBI_LENGTH"]
    #[inline(always)]
    pub fn regterminateifibilength(&mut self) -> RegterminateifibilengthW<I3ccontrol008Spec> {
        RegterminateifibilengthW::new(self, 16)
    }
}
#[doc = "I3C\\_MST\\_MRL\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol008::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol008::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol008Spec;
impl crate::RegisterSpec for I3ccontrol008Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol008::R`](R) reader structure"]
impl crate::Readable for I3ccontrol008Spec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol008::W`](W) writer structure"]
impl crate::Writable for I3ccontrol008Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL008 to value 0"]
impl crate::Resettable for I3ccontrol008Spec {}
