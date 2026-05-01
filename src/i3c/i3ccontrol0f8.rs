#[doc = "Register `I3CCONTROL0F8` reader"]
pub type R = crate::R<I3ccontrol0f8Spec>;
#[doc = "Register `I3CCONTROL0F8` writer"]
pub type W = crate::W<I3ccontrol0f8Spec>;
#[doc = "Field `REGIBITIMEOUT` reader - REG_IBI_TIMEOUT"]
pub type RegibitimeoutR = crate::FieldReader<u16>;
#[doc = "Field `REGIBITIMEOUT` writer - REG_IBI_TIMEOUT"]
pub type RegibitimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `REGIBITIMEOUTEN` reader - REG_IBI_TIMEOUT_EN"]
pub type RegibitimeoutenR = crate::BitReader;
#[doc = "Field `REGIBITIMEOUTEN` writer - REG_IBI_TIMEOUT_EN"]
pub type RegibitimeoutenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - REG_IBI_TIMEOUT"]
    #[inline(always)]
    pub fn regibitimeout(&self) -> RegibitimeoutR {
        RegibitimeoutR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - REG_IBI_TIMEOUT_EN"]
    #[inline(always)]
    pub fn regibitimeouten(&self) -> RegibitimeoutenR {
        RegibitimeoutenR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - REG_IBI_TIMEOUT"]
    #[inline(always)]
    pub fn regibitimeout(&mut self) -> RegibitimeoutW<I3ccontrol0f8Spec> {
        RegibitimeoutW::new(self, 0)
    }
    #[doc = "Bit 16 - REG_IBI_TIMEOUT_EN"]
    #[inline(always)]
    pub fn regibitimeouten(&mut self) -> RegibitimeoutenW<I3ccontrol0f8Spec> {
        RegibitimeoutenW::new(self, 16)
    }
}
#[doc = "I3C\\_IBI\\_TIMEOUT\\_F8\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol0f8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol0f8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol0f8Spec;
impl crate::RegisterSpec for I3ccontrol0f8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol0f8::R`](R) reader structure"]
impl crate::Readable for I3ccontrol0f8Spec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol0f8::W`](W) writer structure"]
impl crate::Writable for I3ccontrol0f8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL0F8 to value 0xea60"]
impl crate::Resettable for I3ccontrol0f8Spec {
    const RESET_VALUE: u32 = 0xea60;
}
