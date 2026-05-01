#[doc = "Register `IPC02C` reader"]
pub type R = crate::R<Ipc02cSpec>;
#[doc = "Register `IPC02C` writer"]
pub type W = crate::W<Ipc02cSpec>;
#[doc = "Field `REGTXIPI07` reader - REG_TX_IPI0_7"]
pub type Regtxipi07R = crate::FieldReader<u32>;
#[doc = "Field `REGTXIPI07` writer - REG_TX_IPI0_7"]
pub type Regtxipi07W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_TX_IPI0_7"]
    #[inline(always)]
    pub fn regtxipi07(&self) -> Regtxipi07R {
        Regtxipi07R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_TX_IPI0_7"]
    #[inline(always)]
    pub fn regtxipi07(&mut self) -> Regtxipi07W<Ipc02cSpec> {
        Regtxipi07W::new(self, 0)
    }
}
#[doc = "tx ipi0 reg7\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc02c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc02c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ipc02cSpec;
impl crate::RegisterSpec for Ipc02cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipc02c::R`](R) reader structure"]
impl crate::Readable for Ipc02cSpec {}
#[doc = "`write(|w| ..)` method takes [`ipc02c::W`](W) writer structure"]
impl crate::Writable for Ipc02cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPC02C to value 0"]
impl crate::Resettable for Ipc02cSpec {}
