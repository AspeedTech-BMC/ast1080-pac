#[doc = "Register `HCIPIO000` reader"]
pub type R = crate::R<Hcipio000Spec>;
#[doc = "Register `HCIPIO000` writer"]
pub type W = crate::W<Hcipio000Spec>;
#[doc = "Field `REGCOMMANDQUEUEPORT` reader - REG_COMMAND_QUEUE_PORT"]
pub type RegcommandqueueportR = crate::FieldReader<u32>;
#[doc = "Field `REGCOMMANDQUEUEPORT` writer - REG_COMMAND_QUEUE_PORT"]
pub type RegcommandqueueportW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - REG_COMMAND_QUEUE_PORT"]
    #[inline(always)]
    pub fn regcommandqueueport(&self) -> RegcommandqueueportR {
        RegcommandqueueportR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - REG_COMMAND_QUEUE_PORT"]
    #[inline(always)]
    pub fn regcommandqueueport(&mut self) -> RegcommandqueueportW<Hcipio000Spec> {
        RegcommandqueueportW::new(self, 0)
    }
}
#[doc = "COMMAND\\_QUEUE\\_PORT\n\nYou can [`read`](crate::Reg::read) this register and get [`hcipio000::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcipio000::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcipio000Spec;
impl crate::RegisterSpec for Hcipio000Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcipio000::R`](R) reader structure"]
impl crate::Readable for Hcipio000Spec {}
#[doc = "`write(|w| ..)` method takes [`hcipio000::W`](W) writer structure"]
impl crate::Writable for Hcipio000Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIPIO000 to value 0"]
impl crate::Resettable for Hcipio000Spec {}
