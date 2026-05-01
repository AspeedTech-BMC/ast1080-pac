#[doc = "Register `IPC084` reader"]
pub type R = crate::R<Ipc084Spec>;
#[doc = "Register `IPC084` writer"]
pub type W = crate::W<Ipc084Spec>;
#[doc = "Field `REGTXIPI35` reader - REG_TX_IPI3_5"]
pub type Regtxipi35R = crate::FieldReader<u32>;
#[doc = "Field `REGTXIPI35` writer - REG_TX_IPI3_5"]
pub type Regtxipi35W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_TX_IPI3_5"]
    #[inline(always)]
    pub fn regtxipi35(&self) -> Regtxipi35R {
        Regtxipi35R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_TX_IPI3_5"]
    #[inline(always)]
    pub fn regtxipi35(&mut self) -> Regtxipi35W<Ipc084Spec> {
        Regtxipi35W::new(self, 0)
    }
}
#[doc = "tx ipi3 reg5\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc084::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc084::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ipc084Spec;
impl crate::RegisterSpec for Ipc084Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipc084::R`](R) reader structure"]
impl crate::Readable for Ipc084Spec {}
#[doc = "`write(|w| ..)` method takes [`ipc084::W`](W) writer structure"]
impl crate::Writable for Ipc084Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPC084 to value 0"]
impl crate::Resettable for Ipc084Spec {}
