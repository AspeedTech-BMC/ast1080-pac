#[doc = "Register `IPC050` reader"]
pub type R = crate::R<Ipc050Spec>;
#[doc = "Register `IPC050` writer"]
pub type W = crate::W<Ipc050Spec>;
#[doc = "Field `REGTXIPI20` reader - REG_TX_IPI2_0"]
pub type Regtxipi20R = crate::FieldReader<u32>;
#[doc = "Field `REGTXIPI20` writer - REG_TX_IPI2_0"]
pub type Regtxipi20W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_TX_IPI2_0"]
    #[inline(always)]
    pub fn regtxipi20(&self) -> Regtxipi20R {
        Regtxipi20R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_TX_IPI2_0"]
    #[inline(always)]
    pub fn regtxipi20(&mut self) -> Regtxipi20W<Ipc050Spec> {
        Regtxipi20W::new(self, 0)
    }
}
#[doc = "tx ipi2 reg0\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc050::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc050::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ipc050Spec;
impl crate::RegisterSpec for Ipc050Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipc050::R`](R) reader structure"]
impl crate::Readable for Ipc050Spec {}
#[doc = "`write(|w| ..)` method takes [`ipc050::W`](W) writer structure"]
impl crate::Writable for Ipc050Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPC050 to value 0"]
impl crate::Resettable for Ipc050Spec {}
