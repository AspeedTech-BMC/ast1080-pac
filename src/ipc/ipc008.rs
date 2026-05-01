#[doc = "Register `IPC008` reader"]
pub type R = crate::R<Ipc008Spec>;
#[doc = "Register `IPC008` writer"]
pub type W = crate::W<Ipc008Spec>;
#[doc = "Field `REGIPISTATUS` reader - REG_IPI_STATUS"]
pub type RegipistatusR = crate::FieldReader;
#[doc = "Field `REGIPISTATUS` writer - REG_IPI_STATUS"]
pub type RegipistatusW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - REG_IPI_STATUS"]
    #[inline(always)]
    pub fn regipistatus(&self) -> RegipistatusR {
        RegipistatusR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - REG_IPI_STATUS"]
    #[inline(always)]
    pub fn regipistatus(&mut self) -> RegipistatusW<Ipc008Spec> {
        RegipistatusW::new(self, 0)
    }
}
#[doc = "ipi status\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc008::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc008::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ipc008Spec;
impl crate::RegisterSpec for Ipc008Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipc008::R`](R) reader structure"]
impl crate::Readable for Ipc008Spec {}
#[doc = "`write(|w| ..)` method takes [`ipc008::W`](W) writer structure"]
impl crate::Writable for Ipc008Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPC008 to value 0"]
impl crate::Resettable for Ipc008Spec {}
