#[doc = "Register `IPC030` reader"]
pub type R = crate::R<Ipc030Spec>;
#[doc = "Register `IPC030` writer"]
pub type W = crate::W<Ipc030Spec>;
#[doc = "Field `REGTXIPI10` reader - REG_TX_IPI1_0"]
pub type Regtxipi10R = crate::FieldReader<u32>;
#[doc = "Field `REGTXIPI10` writer - REG_TX_IPI1_0"]
pub type Regtxipi10W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_TX_IPI1_0"]
    #[inline(always)]
    pub fn regtxipi10(&self) -> Regtxipi10R {
        Regtxipi10R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_TX_IPI1_0"]
    #[inline(always)]
    pub fn regtxipi10(&mut self) -> Regtxipi10W<Ipc030Spec> {
        Regtxipi10W::new(self, 0)
    }
}
#[doc = "tx ipi1 reg0\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc030::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc030::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ipc030Spec;
impl crate::RegisterSpec for Ipc030Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipc030::R`](R) reader structure"]
impl crate::Readable for Ipc030Spec {}
#[doc = "`write(|w| ..)` method takes [`ipc030::W`](W) writer structure"]
impl crate::Writable for Ipc030Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPC030 to value 0"]
impl crate::Resettable for Ipc030Spec {}
