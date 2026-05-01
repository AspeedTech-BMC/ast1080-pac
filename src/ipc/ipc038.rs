#[doc = "Register `IPC038` reader"]
pub type R = crate::R<Ipc038Spec>;
#[doc = "Register `IPC038` writer"]
pub type W = crate::W<Ipc038Spec>;
#[doc = "Field `REGTXIPI12` reader - REG_TX_IPI1_2"]
pub type Regtxipi12R = crate::FieldReader<u32>;
#[doc = "Field `REGTXIPI12` writer - REG_TX_IPI1_2"]
pub type Regtxipi12W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_TX_IPI1_2"]
    #[inline(always)]
    pub fn regtxipi12(&self) -> Regtxipi12R {
        Regtxipi12R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_TX_IPI1_2"]
    #[inline(always)]
    pub fn regtxipi12(&mut self) -> Regtxipi12W<Ipc038Spec> {
        Regtxipi12W::new(self, 0)
    }
}
#[doc = "tx ipi1 reg2\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc038::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc038::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ipc038Spec;
impl crate::RegisterSpec for Ipc038Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipc038::R`](R) reader structure"]
impl crate::Readable for Ipc038Spec {}
#[doc = "`write(|w| ..)` method takes [`ipc038::W`](W) writer structure"]
impl crate::Writable for Ipc038Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPC038 to value 0"]
impl crate::Resettable for Ipc038Spec {}
