#[doc = "Register `IPC03C` reader"]
pub type R = crate::R<Ipc03cSpec>;
#[doc = "Register `IPC03C` writer"]
pub type W = crate::W<Ipc03cSpec>;
#[doc = "Field `REGTXIPI13` reader - REG_TX_IPI1_3"]
pub type Regtxipi13R = crate::FieldReader<u32>;
#[doc = "Field `REGTXIPI13` writer - REG_TX_IPI1_3"]
pub type Regtxipi13W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_TX_IPI1_3"]
    #[inline(always)]
    pub fn regtxipi13(&self) -> Regtxipi13R {
        Regtxipi13R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_TX_IPI1_3"]
    #[inline(always)]
    pub fn regtxipi13(&mut self) -> Regtxipi13W<Ipc03cSpec> {
        Regtxipi13W::new(self, 0)
    }
}
#[doc = "tx ipi1 reg3\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc03c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc03c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ipc03cSpec;
impl crate::RegisterSpec for Ipc03cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipc03c::R`](R) reader structure"]
impl crate::Readable for Ipc03cSpec {}
#[doc = "`write(|w| ..)` method takes [`ipc03c::W`](W) writer structure"]
impl crate::Writable for Ipc03cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPC03C to value 0"]
impl crate::Resettable for Ipc03cSpec {}
