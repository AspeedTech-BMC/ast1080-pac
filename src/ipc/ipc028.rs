#[doc = "Register `IPC028` reader"]
pub type R = crate::R<Ipc028Spec>;
#[doc = "Register `IPC028` writer"]
pub type W = crate::W<Ipc028Spec>;
#[doc = "Field `REGTXIPI06` reader - REG_TX_IPI0_6"]
pub type Regtxipi06R = crate::FieldReader<u32>;
#[doc = "Field `REGTXIPI06` writer - REG_TX_IPI0_6"]
pub type Regtxipi06W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_TX_IPI0_6"]
    #[inline(always)]
    pub fn regtxipi06(&self) -> Regtxipi06R {
        Regtxipi06R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_TX_IPI0_6"]
    #[inline(always)]
    pub fn regtxipi06(&mut self) -> Regtxipi06W<Ipc028Spec> {
        Regtxipi06W::new(self, 0)
    }
}
#[doc = "tx ipi0 reg6\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc028::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc028::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ipc028Spec;
impl crate::RegisterSpec for Ipc028Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipc028::R`](R) reader structure"]
impl crate::Readable for Ipc028Spec {}
#[doc = "`write(|w| ..)` method takes [`ipc028::W`](W) writer structure"]
impl crate::Writable for Ipc028Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPC028 to value 0"]
impl crate::Resettable for Ipc028Spec {}
