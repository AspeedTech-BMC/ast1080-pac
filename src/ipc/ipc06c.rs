#[doc = "Register `IPC06C` reader"]
pub type R = crate::R<Ipc06cSpec>;
#[doc = "Register `IPC06C` writer"]
pub type W = crate::W<Ipc06cSpec>;
#[doc = "Field `REGTXIPI27` reader - REG_TX_IPI2_7"]
pub type Regtxipi27R = crate::FieldReader<u32>;
#[doc = "Field `REGTXIPI27` writer - REG_TX_IPI2_7"]
pub type Regtxipi27W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_TX_IPI2_7"]
    #[inline(always)]
    pub fn regtxipi27(&self) -> Regtxipi27R {
        Regtxipi27R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_TX_IPI2_7"]
    #[inline(always)]
    pub fn regtxipi27(&mut self) -> Regtxipi27W<Ipc06cSpec> {
        Regtxipi27W::new(self, 0)
    }
}
#[doc = "tx ipi2 reg7\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc06c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc06c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ipc06cSpec;
impl crate::RegisterSpec for Ipc06cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipc06c::R`](R) reader structure"]
impl crate::Readable for Ipc06cSpec {}
#[doc = "`write(|w| ..)` method takes [`ipc06c::W`](W) writer structure"]
impl crate::Writable for Ipc06cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPC06C to value 0"]
impl crate::Resettable for Ipc06cSpec {}
