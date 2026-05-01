#[doc = "Register `IPC020` reader"]
pub type R = crate::R<Ipc020Spec>;
#[doc = "Register `IPC020` writer"]
pub type W = crate::W<Ipc020Spec>;
#[doc = "Field `REGTXIPI04` reader - REG_TX_IPI0_4"]
pub type Regtxipi04R = crate::FieldReader<u32>;
#[doc = "Field `REGTXIPI04` writer - REG_TX_IPI0_4"]
pub type Regtxipi04W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_TX_IPI0_4"]
    #[inline(always)]
    pub fn regtxipi04(&self) -> Regtxipi04R {
        Regtxipi04R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_TX_IPI0_4"]
    #[inline(always)]
    pub fn regtxipi04(&mut self) -> Regtxipi04W<Ipc020Spec> {
        Regtxipi04W::new(self, 0)
    }
}
#[doc = "tx ipi0 reg4\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc020::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc020::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ipc020Spec;
impl crate::RegisterSpec for Ipc020Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipc020::R`](R) reader structure"]
impl crate::Readable for Ipc020Spec {}
#[doc = "`write(|w| ..)` method takes [`ipc020::W`](W) writer structure"]
impl crate::Writable for Ipc020Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPC020 to value 0"]
impl crate::Resettable for Ipc020Spec {}
