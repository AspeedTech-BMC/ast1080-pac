#[doc = "Register `IPC048` reader"]
pub type R = crate::R<Ipc048Spec>;
#[doc = "Register `IPC048` writer"]
pub type W = crate::W<Ipc048Spec>;
#[doc = "Field `REGTXIPI16` reader - REG_TX_IPI1_6"]
pub type Regtxipi16R = crate::FieldReader<u32>;
#[doc = "Field `REGTXIPI16` writer - REG_TX_IPI1_6"]
pub type Regtxipi16W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_TX_IPI1_6"]
    #[inline(always)]
    pub fn regtxipi16(&self) -> Regtxipi16R {
        Regtxipi16R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_TX_IPI1_6"]
    #[inline(always)]
    pub fn regtxipi16(&mut self) -> Regtxipi16W<Ipc048Spec> {
        Regtxipi16W::new(self, 0)
    }
}
#[doc = "tx ipi1 reg6\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc048::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc048::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ipc048Spec;
impl crate::RegisterSpec for Ipc048Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipc048::R`](R) reader structure"]
impl crate::Readable for Ipc048Spec {}
#[doc = "`write(|w| ..)` method takes [`ipc048::W`](W) writer structure"]
impl crate::Writable for Ipc048Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPC048 to value 0"]
impl crate::Resettable for Ipc048Spec {}
