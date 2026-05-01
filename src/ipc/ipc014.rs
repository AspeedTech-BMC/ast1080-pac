#[doc = "Register `IPC014` reader"]
pub type R = crate::R<Ipc014Spec>;
#[doc = "Register `IPC014` writer"]
pub type W = crate::W<Ipc014Spec>;
#[doc = "Field `REGTXIPI01` reader - REG_TX_IPI0_1"]
pub type Regtxipi01R = crate::FieldReader<u32>;
#[doc = "Field `REGTXIPI01` writer - REG_TX_IPI0_1"]
pub type Regtxipi01W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_TX_IPI0_1"]
    #[inline(always)]
    pub fn regtxipi01(&self) -> Regtxipi01R {
        Regtxipi01R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_TX_IPI0_1"]
    #[inline(always)]
    pub fn regtxipi01(&mut self) -> Regtxipi01W<Ipc014Spec> {
        Regtxipi01W::new(self, 0)
    }
}
#[doc = "tx ipi0 reg1\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc014::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc014::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ipc014Spec;
impl crate::RegisterSpec for Ipc014Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipc014::R`](R) reader structure"]
impl crate::Readable for Ipc014Spec {}
#[doc = "`write(|w| ..)` method takes [`ipc014::W`](W) writer structure"]
impl crate::Writable for Ipc014Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPC014 to value 0"]
impl crate::Resettable for Ipc014Spec {}
