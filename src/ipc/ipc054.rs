#[doc = "Register `IPC054` reader"]
pub type R = crate::R<Ipc054Spec>;
#[doc = "Register `IPC054` writer"]
pub type W = crate::W<Ipc054Spec>;
#[doc = "Field `REGTXIPI21` reader - REG_TX_IPI2_1"]
pub type Regtxipi21R = crate::FieldReader<u32>;
#[doc = "Field `REGTXIPI21` writer - REG_TX_IPI2_1"]
pub type Regtxipi21W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_TX_IPI2_1"]
    #[inline(always)]
    pub fn regtxipi21(&self) -> Regtxipi21R {
        Regtxipi21R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_TX_IPI2_1"]
    #[inline(always)]
    pub fn regtxipi21(&mut self) -> Regtxipi21W<Ipc054Spec> {
        Regtxipi21W::new(self, 0)
    }
}
#[doc = "tx ipi2 reg1\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc054::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc054::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ipc054Spec;
impl crate::RegisterSpec for Ipc054Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipc054::R`](R) reader structure"]
impl crate::Readable for Ipc054Spec {}
#[doc = "`write(|w| ..)` method takes [`ipc054::W`](W) writer structure"]
impl crate::Writable for Ipc054Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPC054 to value 0"]
impl crate::Resettable for Ipc054Spec {}
