#[doc = "Register `IPC044` reader"]
pub type R = crate::R<Ipc044Spec>;
#[doc = "Register `IPC044` writer"]
pub type W = crate::W<Ipc044Spec>;
#[doc = "Field `REGTXIPI15` reader - REG_TX_IPI1_5"]
pub type Regtxipi15R = crate::FieldReader<u32>;
#[doc = "Field `REGTXIPI15` writer - REG_TX_IPI1_5"]
pub type Regtxipi15W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_TX_IPI1_5"]
    #[inline(always)]
    pub fn regtxipi15(&self) -> Regtxipi15R {
        Regtxipi15R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_TX_IPI1_5"]
    #[inline(always)]
    pub fn regtxipi15(&mut self) -> Regtxipi15W<Ipc044Spec> {
        Regtxipi15W::new(self, 0)
    }
}
#[doc = "tx ipi1 reg5\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc044::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc044::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ipc044Spec;
impl crate::RegisterSpec for Ipc044Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipc044::R`](R) reader structure"]
impl crate::Readable for Ipc044Spec {}
#[doc = "`write(|w| ..)` method takes [`ipc044::W`](W) writer structure"]
impl crate::Writable for Ipc044Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPC044 to value 0"]
impl crate::Resettable for Ipc044Spec {}
