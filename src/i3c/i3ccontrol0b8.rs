#[doc = "Register `I3CCONTROL0B8` reader"]
pub type R = crate::R<I3ccontrol0b8Spec>;
#[doc = "Register `I3CCONTROL0B8` writer"]
pub type W = crate::W<I3ccontrol0b8Spec>;
#[doc = "Field `REGMRL` reader - REG_MRL"]
pub type RegmrlR = crate::FieldReader<u16>;
#[doc = "Field `REGMRL` writer - REG_MRL"]
pub type RegmrlW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `REGMWL` reader - REG_MWL"]
pub type RegmwlR = crate::FieldReader<u16>;
#[doc = "Field `REGMWL` writer - REG_MWL"]
pub type RegmwlW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - REG_MRL"]
    #[inline(always)]
    pub fn regmrl(&self) -> RegmrlR {
        RegmrlR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - REG_MWL"]
    #[inline(always)]
    pub fn regmwl(&self) -> RegmwlR {
        RegmwlR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - REG_MRL"]
    #[inline(always)]
    pub fn regmrl(&mut self) -> RegmrlW<I3ccontrol0b8Spec> {
        RegmrlW::new(self, 0)
    }
    #[doc = "Bits 16:31 - REG_MWL"]
    #[inline(always)]
    pub fn regmwl(&mut self) -> RegmwlW<I3ccontrol0b8Spec> {
        RegmwlW::new(self, 16)
    }
}
#[doc = "I3C\\_SLV\\_CTL\\_0B8\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol0b8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol0b8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol0b8Spec;
impl crate::RegisterSpec for I3ccontrol0b8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol0b8::R`](R) reader structure"]
impl crate::Readable for I3ccontrol0b8Spec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol0b8::W`](W) writer structure"]
impl crate::Writable for I3ccontrol0b8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL0B8 to value 0x0080_0080"]
impl crate::Resettable for I3ccontrol0b8Spec {
    const RESET_VALUE: u32 = 0x0080_0080;
}
