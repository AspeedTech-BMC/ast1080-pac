#[doc = "Register `IPC074` reader"]
pub type R = crate::R<Ipc074Spec>;
#[doc = "Register `IPC074` writer"]
pub type W = crate::W<Ipc074Spec>;
#[doc = "Field `REGTXIPI31` reader - REG_TX_IPI3_1"]
pub type Regtxipi31R = crate::FieldReader<u32>;
#[doc = "Field `REGTXIPI31` writer - REG_TX_IPI3_1"]
pub type Regtxipi31W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_TX_IPI3_1"]
    #[inline(always)]
    pub fn regtxipi31(&self) -> Regtxipi31R {
        Regtxipi31R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_TX_IPI3_1"]
    #[inline(always)]
    pub fn regtxipi31(&mut self) -> Regtxipi31W<Ipc074Spec> {
        Regtxipi31W::new(self, 0)
    }
}
#[doc = "tx ipi3 reg1\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc074::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc074::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ipc074Spec;
impl crate::RegisterSpec for Ipc074Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipc074::R`](R) reader structure"]
impl crate::Readable for Ipc074Spec {}
#[doc = "`write(|w| ..)` method takes [`ipc074::W`](W) writer structure"]
impl crate::Writable for Ipc074Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPC074 to value 0"]
impl crate::Resettable for Ipc074Spec {}
