#[doc = "Register `IPC05C` reader"]
pub type R = crate::R<Ipc05cSpec>;
#[doc = "Register `IPC05C` writer"]
pub type W = crate::W<Ipc05cSpec>;
#[doc = "Field `REGTXIPI23` reader - REG_TX_IPI2_3"]
pub type Regtxipi23R = crate::FieldReader<u32>;
#[doc = "Field `REGTXIPI23` writer - REG_TX_IPI2_3"]
pub type Regtxipi23W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_TX_IPI2_3"]
    #[inline(always)]
    pub fn regtxipi23(&self) -> Regtxipi23R {
        Regtxipi23R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_TX_IPI2_3"]
    #[inline(always)]
    pub fn regtxipi23(&mut self) -> Regtxipi23W<Ipc05cSpec> {
        Regtxipi23W::new(self, 0)
    }
}
#[doc = "tx ipi2 reg3\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc05c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc05c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ipc05cSpec;
impl crate::RegisterSpec for Ipc05cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipc05c::R`](R) reader structure"]
impl crate::Readable for Ipc05cSpec {}
#[doc = "`write(|w| ..)` method takes [`ipc05c::W`](W) writer structure"]
impl crate::Writable for Ipc05cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPC05C to value 0"]
impl crate::Resettable for Ipc05cSpec {}
