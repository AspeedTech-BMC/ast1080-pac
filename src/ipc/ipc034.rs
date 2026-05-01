#[doc = "Register `IPC034` reader"]
pub type R = crate::R<Ipc034Spec>;
#[doc = "Register `IPC034` writer"]
pub type W = crate::W<Ipc034Spec>;
#[doc = "Field `REGTXIPI11` reader - REG_TX_IPI1_1"]
pub type Regtxipi11R = crate::FieldReader<u32>;
#[doc = "Field `REGTXIPI11` writer - REG_TX_IPI1_1"]
pub type Regtxipi11W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_TX_IPI1_1"]
    #[inline(always)]
    pub fn regtxipi11(&self) -> Regtxipi11R {
        Regtxipi11R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_TX_IPI1_1"]
    #[inline(always)]
    pub fn regtxipi11(&mut self) -> Regtxipi11W<Ipc034Spec> {
        Regtxipi11W::new(self, 0)
    }
}
#[doc = "tx ipi1 reg1\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc034::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc034::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ipc034Spec;
impl crate::RegisterSpec for Ipc034Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipc034::R`](R) reader structure"]
impl crate::Readable for Ipc034Spec {}
#[doc = "`write(|w| ..)` method takes [`ipc034::W`](W) writer structure"]
impl crate::Writable for Ipc034Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPC034 to value 0"]
impl crate::Resettable for Ipc034Spec {}
