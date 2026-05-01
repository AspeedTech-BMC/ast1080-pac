#[doc = "Register `IPC010` reader"]
pub type R = crate::R<Ipc010Spec>;
#[doc = "Register `IPC010` writer"]
pub type W = crate::W<Ipc010Spec>;
#[doc = "Field `REGTXIPI00` reader - REG_TX_IPI0_0"]
pub type Regtxipi00R = crate::FieldReader<u32>;
#[doc = "Field `REGTXIPI00` writer - REG_TX_IPI0_0"]
pub type Regtxipi00W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_TX_IPI0_0"]
    #[inline(always)]
    pub fn regtxipi00(&self) -> Regtxipi00R {
        Regtxipi00R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_TX_IPI0_0"]
    #[inline(always)]
    pub fn regtxipi00(&mut self) -> Regtxipi00W<Ipc010Spec> {
        Regtxipi00W::new(self, 0)
    }
}
#[doc = "tx ipi0 reg0\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc010::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc010::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ipc010Spec;
impl crate::RegisterSpec for Ipc010Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipc010::R`](R) reader structure"]
impl crate::Readable for Ipc010Spec {}
#[doc = "`write(|w| ..)` method takes [`ipc010::W`](W) writer structure"]
impl crate::Writable for Ipc010Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPC010 to value 0"]
impl crate::Resettable for Ipc010Spec {}
