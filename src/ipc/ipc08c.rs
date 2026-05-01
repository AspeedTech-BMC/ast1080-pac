#[doc = "Register `IPC08C` reader"]
pub type R = crate::R<Ipc08cSpec>;
#[doc = "Register `IPC08C` writer"]
pub type W = crate::W<Ipc08cSpec>;
#[doc = "Field `REGTXIPI37` reader - REG_TX_IPI3_7"]
pub type Regtxipi37R = crate::FieldReader<u32>;
#[doc = "Field `REGTXIPI37` writer - REG_TX_IPI3_7"]
pub type Regtxipi37W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_TX_IPI3_7"]
    #[inline(always)]
    pub fn regtxipi37(&self) -> Regtxipi37R {
        Regtxipi37R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_TX_IPI3_7"]
    #[inline(always)]
    pub fn regtxipi37(&mut self) -> Regtxipi37W<Ipc08cSpec> {
        Regtxipi37W::new(self, 0)
    }
}
#[doc = "tx ipi3 reg7\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc08c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc08c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ipc08cSpec;
impl crate::RegisterSpec for Ipc08cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipc08c::R`](R) reader structure"]
impl crate::Readable for Ipc08cSpec {}
#[doc = "`write(|w| ..)` method takes [`ipc08c::W`](W) writer structure"]
impl crate::Writable for Ipc08cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPC08C to value 0"]
impl crate::Resettable for Ipc08cSpec {}
