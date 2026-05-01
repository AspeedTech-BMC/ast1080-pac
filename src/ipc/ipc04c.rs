#[doc = "Register `IPC04C` reader"]
pub type R = crate::R<Ipc04cSpec>;
#[doc = "Register `IPC04C` writer"]
pub type W = crate::W<Ipc04cSpec>;
#[doc = "Field `REGTXIPI17` reader - REG_TX_IPI1_7"]
pub type Regtxipi17R = crate::FieldReader<u32>;
#[doc = "Field `REGTXIPI17` writer - REG_TX_IPI1_7"]
pub type Regtxipi17W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_TX_IPI1_7"]
    #[inline(always)]
    pub fn regtxipi17(&self) -> Regtxipi17R {
        Regtxipi17R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_TX_IPI1_7"]
    #[inline(always)]
    pub fn regtxipi17(&mut self) -> Regtxipi17W<Ipc04cSpec> {
        Regtxipi17W::new(self, 0)
    }
}
#[doc = "tx ipi1 reg7\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc04c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc04c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ipc04cSpec;
impl crate::RegisterSpec for Ipc04cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipc04c::R`](R) reader structure"]
impl crate::Readable for Ipc04cSpec {}
#[doc = "`write(|w| ..)` method takes [`ipc04c::W`](W) writer structure"]
impl crate::Writable for Ipc04cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPC04C to value 0"]
impl crate::Resettable for Ipc04cSpec {}
