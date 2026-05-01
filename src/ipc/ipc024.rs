#[doc = "Register `IPC024` reader"]
pub type R = crate::R<Ipc024Spec>;
#[doc = "Register `IPC024` writer"]
pub type W = crate::W<Ipc024Spec>;
#[doc = "Field `REGTXIPI05` reader - REG_TX_IPI0_5"]
pub type Regtxipi05R = crate::FieldReader<u32>;
#[doc = "Field `REGTXIPI05` writer - REG_TX_IPI0_5"]
pub type Regtxipi05W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_TX_IPI0_5"]
    #[inline(always)]
    pub fn regtxipi05(&self) -> Regtxipi05R {
        Regtxipi05R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_TX_IPI0_5"]
    #[inline(always)]
    pub fn regtxipi05(&mut self) -> Regtxipi05W<Ipc024Spec> {
        Regtxipi05W::new(self, 0)
    }
}
#[doc = "tx ipi0 reg5\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc024::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc024::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ipc024Spec;
impl crate::RegisterSpec for Ipc024Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipc024::R`](R) reader structure"]
impl crate::Readable for Ipc024Spec {}
#[doc = "`write(|w| ..)` method takes [`ipc024::W`](W) writer structure"]
impl crate::Writable for Ipc024Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPC024 to value 0"]
impl crate::Resettable for Ipc024Spec {}
