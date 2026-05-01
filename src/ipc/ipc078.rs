#[doc = "Register `IPC078` reader"]
pub type R = crate::R<Ipc078Spec>;
#[doc = "Register `IPC078` writer"]
pub type W = crate::W<Ipc078Spec>;
#[doc = "Field `REGTXIPI32` reader - REG_TX_IPI3_2"]
pub type Regtxipi32R = crate::FieldReader<u32>;
#[doc = "Field `REGTXIPI32` writer - REG_TX_IPI3_2"]
pub type Regtxipi32W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_TX_IPI3_2"]
    #[inline(always)]
    pub fn regtxipi32(&self) -> Regtxipi32R {
        Regtxipi32R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_TX_IPI3_2"]
    #[inline(always)]
    pub fn regtxipi32(&mut self) -> Regtxipi32W<Ipc078Spec> {
        Regtxipi32W::new(self, 0)
    }
}
#[doc = "tx ipi3 reg2\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc078::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc078::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ipc078Spec;
impl crate::RegisterSpec for Ipc078Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipc078::R`](R) reader structure"]
impl crate::Readable for Ipc078Spec {}
#[doc = "`write(|w| ..)` method takes [`ipc078::W`](W) writer structure"]
impl crate::Writable for Ipc078Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPC078 to value 0"]
impl crate::Resettable for Ipc078Spec {}
