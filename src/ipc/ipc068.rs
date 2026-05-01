#[doc = "Register `IPC068` reader"]
pub type R = crate::R<Ipc068Spec>;
#[doc = "Register `IPC068` writer"]
pub type W = crate::W<Ipc068Spec>;
#[doc = "Field `REGTXIPI26` reader - REG_TX_IPI2_6"]
pub type Regtxipi26R = crate::FieldReader<u32>;
#[doc = "Field `REGTXIPI26` writer - REG_TX_IPI2_6"]
pub type Regtxipi26W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_TX_IPI2_6"]
    #[inline(always)]
    pub fn regtxipi26(&self) -> Regtxipi26R {
        Regtxipi26R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_TX_IPI2_6"]
    #[inline(always)]
    pub fn regtxipi26(&mut self) -> Regtxipi26W<Ipc068Spec> {
        Regtxipi26W::new(self, 0)
    }
}
#[doc = "tx ipi2 reg6\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc068::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc068::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ipc068Spec;
impl crate::RegisterSpec for Ipc068Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipc068::R`](R) reader structure"]
impl crate::Readable for Ipc068Spec {}
#[doc = "`write(|w| ..)` method takes [`ipc068::W`](W) writer structure"]
impl crate::Writable for Ipc068Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPC068 to value 0"]
impl crate::Resettable for Ipc068Spec {}
