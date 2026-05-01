#[doc = "Register `IPC060` reader"]
pub type R = crate::R<Ipc060Spec>;
#[doc = "Register `IPC060` writer"]
pub type W = crate::W<Ipc060Spec>;
#[doc = "Field `REGTXIPI24` reader - REG_TX_IPI2_4"]
pub type Regtxipi24R = crate::FieldReader<u32>;
#[doc = "Field `REGTXIPI24` writer - REG_TX_IPI2_4"]
pub type Regtxipi24W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_TX_IPI2_4"]
    #[inline(always)]
    pub fn regtxipi24(&self) -> Regtxipi24R {
        Regtxipi24R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_TX_IPI2_4"]
    #[inline(always)]
    pub fn regtxipi24(&mut self) -> Regtxipi24W<Ipc060Spec> {
        Regtxipi24W::new(self, 0)
    }
}
#[doc = "tx ipi2 reg4\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc060::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc060::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ipc060Spec;
impl crate::RegisterSpec for Ipc060Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipc060::R`](R) reader structure"]
impl crate::Readable for Ipc060Spec {}
#[doc = "`write(|w| ..)` method takes [`ipc060::W`](W) writer structure"]
impl crate::Writable for Ipc060Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPC060 to value 0"]
impl crate::Resettable for Ipc060Spec {}
