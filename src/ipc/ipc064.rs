#[doc = "Register `IPC064` reader"]
pub type R = crate::R<Ipc064Spec>;
#[doc = "Register `IPC064` writer"]
pub type W = crate::W<Ipc064Spec>;
#[doc = "Field `REGTXIPI25` reader - REG_TX_IPI2_5"]
pub type Regtxipi25R = crate::FieldReader<u32>;
#[doc = "Field `REGTXIPI25` writer - REG_TX_IPI2_5"]
pub type Regtxipi25W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_TX_IPI2_5"]
    #[inline(always)]
    pub fn regtxipi25(&self) -> Regtxipi25R {
        Regtxipi25R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_TX_IPI2_5"]
    #[inline(always)]
    pub fn regtxipi25(&mut self) -> Regtxipi25W<Ipc064Spec> {
        Regtxipi25W::new(self, 0)
    }
}
#[doc = "tx ipi2 reg5\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc064::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc064::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ipc064Spec;
impl crate::RegisterSpec for Ipc064Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipc064::R`](R) reader structure"]
impl crate::Readable for Ipc064Spec {}
#[doc = "`write(|w| ..)` method takes [`ipc064::W`](W) writer structure"]
impl crate::Writable for Ipc064Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPC064 to value 0"]
impl crate::Resettable for Ipc064Spec {}
