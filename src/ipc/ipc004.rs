#[doc = "Register `IPC004` reader"]
pub type R = crate::R<Ipc004Spec>;
#[doc = "Register `IPC004` writer"]
pub type W = crate::W<Ipc004Spec>;
#[doc = "Field `REGIPIENABLE` reader - REG_IPI_ENABLE"]
pub type RegipienableR = crate::FieldReader;
#[doc = "Field `REGIPIENABLE` writer - REG_IPI_ENABLE"]
pub type RegipienableW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - REG_IPI_ENABLE"]
    #[inline(always)]
    pub fn regipienable(&self) -> RegipienableR {
        RegipienableR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - REG_IPI_ENABLE"]
    #[inline(always)]
    pub fn regipienable(&mut self) -> RegipienableW<Ipc004Spec> {
        RegipienableW::new(self, 0)
    }
}
#[doc = "IPI enable\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc004::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc004::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ipc004Spec;
impl crate::RegisterSpec for Ipc004Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipc004::R`](R) reader structure"]
impl crate::Readable for Ipc004Spec {}
#[doc = "`write(|w| ..)` method takes [`ipc004::W`](W) writer structure"]
impl crate::Writable for Ipc004Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPC004 to value 0"]
impl crate::Resettable for Ipc004Spec {}
