#[doc = "Register `IPC07C` reader"]
pub type R = crate::R<Ipc07cSpec>;
#[doc = "Register `IPC07C` writer"]
pub type W = crate::W<Ipc07cSpec>;
#[doc = "Field `REGTXIPI33` reader - REG_TX_IPI3_3"]
pub type Regtxipi33R = crate::FieldReader<u32>;
#[doc = "Field `REGTXIPI33` writer - REG_TX_IPI3_3"]
pub type Regtxipi33W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_TX_IPI3_3"]
    #[inline(always)]
    pub fn regtxipi33(&self) -> Regtxipi33R {
        Regtxipi33R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_TX_IPI3_3"]
    #[inline(always)]
    pub fn regtxipi33(&mut self) -> Regtxipi33W<Ipc07cSpec> {
        Regtxipi33W::new(self, 0)
    }
}
#[doc = "tx ipi3 reg3\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc07c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc07c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ipc07cSpec;
impl crate::RegisterSpec for Ipc07cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipc07c::R`](R) reader structure"]
impl crate::Readable for Ipc07cSpec {}
#[doc = "`write(|w| ..)` method takes [`ipc07c::W`](W) writer structure"]
impl crate::Writable for Ipc07cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPC07C to value 0"]
impl crate::Resettable for Ipc07cSpec {}
