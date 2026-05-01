#[doc = "Register `IPC070` reader"]
pub type R = crate::R<Ipc070Spec>;
#[doc = "Register `IPC070` writer"]
pub type W = crate::W<Ipc070Spec>;
#[doc = "Field `REGTXIPI30` reader - REG_TX_IPI3_0"]
pub type Regtxipi30R = crate::FieldReader<u32>;
#[doc = "Field `REGTXIPI30` writer - REG_TX_IPI3_0"]
pub type Regtxipi30W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_TX_IPI3_0"]
    #[inline(always)]
    pub fn regtxipi30(&self) -> Regtxipi30R {
        Regtxipi30R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_TX_IPI3_0"]
    #[inline(always)]
    pub fn regtxipi30(&mut self) -> Regtxipi30W<Ipc070Spec> {
        Regtxipi30W::new(self, 0)
    }
}
#[doc = "tx ipi3 reg0\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc070::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc070::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ipc070Spec;
impl crate::RegisterSpec for Ipc070Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipc070::R`](R) reader structure"]
impl crate::Readable for Ipc070Spec {}
#[doc = "`write(|w| ..)` method takes [`ipc070::W`](W) writer structure"]
impl crate::Writable for Ipc070Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPC070 to value 0"]
impl crate::Resettable for Ipc070Spec {}
