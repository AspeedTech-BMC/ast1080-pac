#[doc = "Register `IPC000` reader"]
pub type R = crate::R<Ipc000Spec>;
#[doc = "Register `IPC000` writer"]
pub type W = crate::W<Ipc000Spec>;
#[doc = "Field `REGIPITRIG` reader - REG_IPI_TRIG"]
pub type RegipitrigR = crate::FieldReader;
#[doc = "Field `REGIPITRIG` writer - REG_IPI_TRIG"]
pub type RegipitrigW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - REG_IPI_TRIG"]
    #[inline(always)]
    pub fn regipitrig(&self) -> RegipitrigR {
        RegipitrigR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - REG_IPI_TRIG"]
    #[inline(always)]
    pub fn regipitrig(&mut self) -> RegipitrigW<Ipc000Spec> {
        RegipitrigW::new(self, 0)
    }
}
#[doc = "IPI trig , w1 trigger\n\nYou can [`read`](crate::Reg::read) this register and get [`ipc000::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipc000::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ipc000Spec;
impl crate::RegisterSpec for Ipc000Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipc000::R`](R) reader structure"]
impl crate::Readable for Ipc000Spec {}
#[doc = "`write(|w| ..)` method takes [`ipc000::W`](W) writer structure"]
impl crate::Writable for Ipc000Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IPC000 to value 0"]
impl crate::Resettable for Ipc000Spec {}
