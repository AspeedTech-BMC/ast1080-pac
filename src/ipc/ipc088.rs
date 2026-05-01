#[doc = "Register `IPC088` reader"]
pub type R = crate::R<Ipc088Spec>;
#[doc = "Register `IPC088` writer"]
pub type W = crate::W<Ipc088Spec>;
#[doc = "Field `REGTXIPI36` reader - REG_TX_IPI3_6"]
pub type Regtxipi36R = crate::FieldReader<u32>;
#[doc = "Field `REGTXIPI36` writer - REG_TX_IPI3_6"]
pub type Regtxipi36W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_TX_IPI3_6"]
    #[inline(always)]
    pub fn regtxipi36(&self) -> Regtxipi36R {
        Regtxipi36R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_TX_IPI3_6"]
    #[inline(always)]
    pub fn regtxipi36(&mut self) -> Regtxipi36W<Ipc088Spec> {
        Regtxipi36W::new(self, 0)
    }
}
#[doc = "tx ipi3 reg6\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc088::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc088::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ipc088Spec;
impl crate::RegisterSpec for Ipc088Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipc088::R`](R) reader structure"]
impl crate::Readable for Ipc088Spec {}
#[doc = "`write(|w| ..)` method takes [`ipc088::W`](W) writer structure"]
impl crate::Writable for Ipc088Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPC088 to value 0"]
impl crate::Resettable for Ipc088Spec {}
