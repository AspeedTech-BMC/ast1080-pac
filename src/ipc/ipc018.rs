#[doc = "Register `IPC018` reader"]
pub type R = crate::R<Ipc018Spec>;
#[doc = "Register `IPC018` writer"]
pub type W = crate::W<Ipc018Spec>;
#[doc = "Field `REGTXIPI02` reader - REG_TX_IPI0_2"]
pub type Regtxipi02R = crate::FieldReader<u32>;
#[doc = "Field `REGTXIPI02` writer - REG_TX_IPI0_2"]
pub type Regtxipi02W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_TX_IPI0_2"]
    #[inline(always)]
    pub fn regtxipi02(&self) -> Regtxipi02R {
        Regtxipi02R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_TX_IPI0_2"]
    #[inline(always)]
    pub fn regtxipi02(&mut self) -> Regtxipi02W<Ipc018Spec> {
        Regtxipi02W::new(self, 0)
    }
}
#[doc = "tx ipi0 reg2\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc018::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc018::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ipc018Spec;
impl crate::RegisterSpec for Ipc018Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipc018::R`](R) reader structure"]
impl crate::Readable for Ipc018Spec {}
#[doc = "`write(|w| ..)` method takes [`ipc018::W`](W) writer structure"]
impl crate::Writable for Ipc018Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPC018 to value 0"]
impl crate::Resettable for Ipc018Spec {}
