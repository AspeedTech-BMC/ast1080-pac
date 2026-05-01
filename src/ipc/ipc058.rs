#[doc = "Register `IPC058` reader"]
pub type R = crate::R<Ipc058Spec>;
#[doc = "Register `IPC058` writer"]
pub type W = crate::W<Ipc058Spec>;
#[doc = "Field `REGTXIPI22` reader - REG_TX_IPI2_2"]
pub type Regtxipi22R = crate::FieldReader<u32>;
#[doc = "Field `REGTXIPI22` writer - REG_TX_IPI2_2"]
pub type Regtxipi22W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_TX_IPI2_2"]
    #[inline(always)]
    pub fn regtxipi22(&self) -> Regtxipi22R {
        Regtxipi22R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_TX_IPI2_2"]
    #[inline(always)]
    pub fn regtxipi22(&mut self) -> Regtxipi22W<Ipc058Spec> {
        Regtxipi22W::new(self, 0)
    }
}
#[doc = "tx ipi2 reg2\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc058::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc058::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ipc058Spec;
impl crate::RegisterSpec for Ipc058Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipc058::R`](R) reader structure"]
impl crate::Readable for Ipc058Spec {}
#[doc = "`write(|w| ..)` method takes [`ipc058::W`](W) writer structure"]
impl crate::Writable for Ipc058Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPC058 to value 0"]
impl crate::Resettable for Ipc058Spec {}
