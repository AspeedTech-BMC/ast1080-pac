#[doc = "Register `IPC080` reader"]
pub type R = crate::R<Ipc080Spec>;
#[doc = "Register `IPC080` writer"]
pub type W = crate::W<Ipc080Spec>;
#[doc = "Field `REGTXIPI34` reader - REG_TX_IPI3_4"]
pub type Regtxipi34R = crate::FieldReader<u32>;
#[doc = "Field `REGTXIPI34` writer - REG_TX_IPI3_4"]
pub type Regtxipi34W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_TX_IPI3_4"]
    #[inline(always)]
    pub fn regtxipi34(&self) -> Regtxipi34R {
        Regtxipi34R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_TX_IPI3_4"]
    #[inline(always)]
    pub fn regtxipi34(&mut self) -> Regtxipi34W<Ipc080Spec> {
        Regtxipi34W::new(self, 0)
    }
}
#[doc = "tx ipi3 reg4\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc080::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc080::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ipc080Spec;
impl crate::RegisterSpec for Ipc080Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipc080::R`](R) reader structure"]
impl crate::Readable for Ipc080Spec {}
#[doc = "`write(|w| ..)` method takes [`ipc080::W`](W) writer structure"]
impl crate::Writable for Ipc080Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPC080 to value 0"]
impl crate::Resettable for Ipc080Spec {}
