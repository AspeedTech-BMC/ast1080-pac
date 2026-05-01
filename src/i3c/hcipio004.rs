#[doc = "Register `HCIPIO004` reader"]
pub type R = crate::R<Hcipio004Spec>;
#[doc = "Register `HCIPIO004` writer"]
pub type W = crate::W<Hcipio004Spec>;
#[doc = "Field `REGRESPONSEQUEUEPORT` reader - REG_RESPONSE_QUEUE_PORT"]
pub type RegresponsequeueportR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - REG_RESPONSE_QUEUE_PORT"]
    #[inline(always)]
    pub fn regresponsequeueport(&self) -> RegresponsequeueportR {
        RegresponsequeueportR::new(self.bits)
    }
}
impl W {}
#[doc = "RESPONSE\\_QUEUE\\_PORT\n\nYou can [`read`](crate::Reg::read) this register and get [`hcipio004::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcipio004::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcipio004Spec;
impl crate::RegisterSpec for Hcipio004Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcipio004::R`](R) reader structure"]
impl crate::Readable for Hcipio004Spec {}
#[doc = "`write(|w| ..)` method takes [`hcipio004::W`](W) writer structure"]
impl crate::Writable for Hcipio004Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIPIO004 to value 0"]
impl crate::Resettable for Hcipio004Spec {}
