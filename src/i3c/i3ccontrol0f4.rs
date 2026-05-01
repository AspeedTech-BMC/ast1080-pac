#[doc = "Register `I3CCONTROL0F4` reader"]
pub type R = crate::R<I3ccontrol0f4Spec>;
#[doc = "Register `I3CCONTROL0F4` writer"]
pub type W = crate::W<I3ccontrol0f4Spec>;
#[doc = "Field `REGI3CINTRPROCESS` reader - REG_I3C_INTR_PROCESS"]
pub type Regi3cintrprocessR = crate::BitReader;
#[doc = "Field `REGI3CINTRPROCESS` writer - REG_I3C_INTR_PROCESS"]
pub type Regi3cintrprocessW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - REG_I3C_INTR_PROCESS"]
    #[inline(always)]
    pub fn regi3cintrprocess(&self) -> Regi3cintrprocessR {
        Regi3cintrprocessR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - REG_I3C_INTR_PROCESS"]
    #[inline(always)]
    pub fn regi3cintrprocess(&mut self) -> Regi3cintrprocessW<I3ccontrol0f4Spec> {
        Regi3cintrprocessW::new(self, 0)
    }
}
#[doc = "I3C\\_INTR\\_PROCESS\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol0f4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol0f4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol0f4Spec;
impl crate::RegisterSpec for I3ccontrol0f4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol0f4::R`](R) reader structure"]
impl crate::Readable for I3ccontrol0f4Spec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol0f4::W`](W) writer structure"]
impl crate::Writable for I3ccontrol0f4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL0F4 to value 0"]
impl crate::Resettable for I3ccontrol0f4Spec {}
