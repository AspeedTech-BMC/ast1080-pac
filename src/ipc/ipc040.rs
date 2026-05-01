#[doc = "Register `IPC040` reader"]
pub type R = crate::R<Ipc040Spec>;
#[doc = "Register `IPC040` writer"]
pub type W = crate::W<Ipc040Spec>;
#[doc = "Field `REGTXIPI14` reader - REG_TX_IPI1_4"]
pub type Regtxipi14R = crate::FieldReader<u32>;
#[doc = "Field `REGTXIPI14` writer - REG_TX_IPI1_4"]
pub type Regtxipi14W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_TX_IPI1_4"]
    #[inline(always)]
    pub fn regtxipi14(&self) -> Regtxipi14R {
        Regtxipi14R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_TX_IPI1_4"]
    #[inline(always)]
    pub fn regtxipi14(&mut self) -> Regtxipi14W<Ipc040Spec> {
        Regtxipi14W::new(self, 0)
    }
}
#[doc = "tx ipi1 reg4\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc040::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc040::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ipc040Spec;
impl crate::RegisterSpec for Ipc040Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipc040::R`](R) reader structure"]
impl crate::Readable for Ipc040Spec {}
#[doc = "`write(|w| ..)` method takes [`ipc040::W`](W) writer structure"]
impl crate::Writable for Ipc040Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPC040 to value 0"]
impl crate::Resettable for Ipc040Spec {}
